use reqwest::Client;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub struct LlmClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LlmClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        }
    }

    pub async fn chat(&self, messages: Vec<Value>) -> Result<LlmResponse, String> {
        // Detect if using Alibaba Cloud Bailian (DashScope)
        let is_alibaba = self.base_url.contains("aliyun.com") || self.base_url.contains("dashscope");
        
        let (url, headers, body) = if is_alibaba {
            // Alibaba Cloud Bailian API format
            let url = format!("{}/compatible-mode/v1/chat/completions", self.base_url.trim_end_matches("/compatible-mode/v1").trim_end_matches("/v1"));
            
            // Normalize model name for Alibaba Cloud (lowercase, add qwen- prefix if needed)
            let model = if self.model.to_lowercase().starts_with("qwen") {
                self.model.to_lowercase()
            } else if self.model.to_lowercase().contains("qwen") {
                self.model.to_lowercase()
            } else {
                format!("qwen-{}", self.model.to_lowercase())
            };
            
            eprintln!("Normalized model name: {}", model);
            
            let headers = vec![
                ("Authorization", format!("Bearer {}", self.api_key)),
                ("Content-Type", "application/json".to_string()),
            ];
            let body = json!({
                "model": model,
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 2000
            });
            (url, headers, body)
        } else {
            // Standard OpenAI-compatible API
            let url = if self.base_url.ends_with("/v1") {
                format!("{}/chat/completions", self.base_url)
            } else {
                format!("{}/v1/chat/completions", self.base_url)
            };
            let headers = vec![
                ("Authorization", format!("Bearer {}", self.api_key)),
                ("Content-Type", "application/json".to_string()),
            ];
            let body = json!({
                "model": self.model,
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 2000
            });
            (url, headers, body)
        };
        
        eprintln!("LLM Request URL: {}", url);
        eprintln!("LLM Model: {}", self.model);
        eprintln!("Is Alibaba Cloud: {}", is_alibaba);

        let mut request = self.client
            .post(&url)
            .json(&body);
        
        for (key, value) in headers {
            request = request.header(key, value);
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            eprintln!("LLM API Error - Status: {}, Body: {}", status, error_text);
            return Err(format!("API error ({}): {}", status, error_text));
        }

        let response_json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Invalid response format")?
            .to_string();

        // Extract token usage
        let input_tokens = response_json["usage"]["prompt_tokens"]
            .as_u64()
            .unwrap_or(0);
        let output_tokens = response_json["usage"]["completion_tokens"]
            .as_u64()
            .unwrap_or(0);
        
        eprintln!("Token usage - Input: {}, Output: {}", input_tokens, output_tokens);

        Ok(LlmResponse {
            content,
            input_tokens,
            output_tokens,
        })
    }

    // Convenience method that returns just the content
    pub async fn chat_simple(&self, messages: Vec<Value>) -> Result<String, String> {
        self.chat(messages).await.map(|r| r.content)
    }

    pub async fn chat_with_skills(
        &self,
        user_message: &str,
        skills: &[crate::core::skill_adapter::OmSkill],
        soul_content: &str,
    ) -> Result<String, String> {
        let skills_context = if skills.is_empty() {
            "当前没有激活的技能。".to_string()
        } else {
            let skill_descriptions: Vec<String> = skills.iter()
                .map(|s| {
                    // Safe string truncation that handles UTF-8 correctly
                    let instructions = if s.instructions.len() > 200 {
                        match s.instructions.char_indices().nth(200) {
                            Some((idx, _)) => &s.instructions[..idx],
                            None => &s.instructions
                        }.to_string() + "..."
                    } else {
                        s.instructions.clone()
                    };
                    format!("- {}: {}\n  触发词: {}\n  说明: {}", 
                        s.name, 
                        s.description,
                        s.triggers.join(", "),
                        instructions
                    )
                })
                .collect();
            format!("当前激活的技能：\n{}", skill_descriptions.join("\n"))
        };

        let system_prompt = format!(
            r#"你是 OpenMonkey v2.0，一个智能 AI 助手。

## 你的灵魂（SOUL）
{}

## 可用技能
{}

## 指令
1. 根据用户的输入，判断是否需要使用某个技能
2. 如果需要使用技能，请明确说明你正在使用哪个技能
3. 提供有帮助、准确且友好的回复
4. 如果不确定如何回答，请诚实告知
"#,
            soul_content,
            skills_context
        );

        let messages = vec![
            json!({"role": "system", "content": system_prompt}),
            json!({"role": "user", "content": user_message}),
        ];

        self.chat(messages).await.map(|r| r.content)
    }

    pub async fn chat_with_skills_and_stats(
        &self,
        user_message: &str,
        skills: &[crate::core::skill_adapter::OmSkill],
        soul_content: &str,
    ) -> Result<LlmResponse, String> {
        let skills_context = if skills.is_empty() {
            "当前没有激活的技能。".to_string()
        } else {
            let skill_descriptions: Vec<String> = skills.iter()
                .map(|s| {
                    let instructions = if s.instructions.len() > 200 {
                        match s.instructions.char_indices().nth(200) {
                            Some((idx, _)) => &s.instructions[..idx],
                            None => &s.instructions
                        }.to_string() + "..."
                    } else {
                        s.instructions.clone()
                    };
                    format!("- {}: {}\n  触发词: {}\n  说明: {}", 
                        s.name, 
                        s.description,
                        s.triggers.join(", "),
                        instructions
                    )
                })
                .collect();
            format!("当前激活的技能：\n{}", skill_descriptions.join("\n"))
        };

        let system_prompt = format!(
            r#"你是 OpenMonkey v2.0，一个智能 AI 助手。

## 你的灵魂（SOUL）
{}

## 可用技能
{}

## 指令
1. 根据用户的输入，判断是否需要使用某个技能
2. 如果需要使用技能，请明确说明你正在使用哪个技能
3. 提供有帮助、准确且友好的回复
4. 如果不确定如何回答，请诚实告知
"#,
            soul_content,
            skills_context
        );

        let messages = vec![
            json!({"role": "system", "content": system_prompt}),
            json!({"role": "user", "content": user_message}),
        ];

        self.chat(messages).await
    }
}
