use reqwest::Client;
use serde_json::{json, Value};
use crate::core::channels::{ChannelClient, ChannelMessage};
use async_trait::async_trait;

pub struct TelegramClient {
    bot_token: String,
    chat_id: Option<String>,
    client: Client,
}

impl TelegramClient {
    pub fn new(bot_token: String, chat_id: Option<String>) -> Self {
        Self {
            bot_token,
            chat_id,
            client: Client::new(),
        }
    }
    
    pub async fn send_message_async(&self, content: &str) -> Result<String, String> {
        let chat_id = self.chat_id.as_ref()
            .ok_or("未配置chat_id")?;
        
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let body = json!({
            "chat_id": chat_id,
            "text": content,
            "parse_mode": "HTML",
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("发送消息失败: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("发送失败: {}", error_text));
        }
        
        let json: Value = response.json().await
            .map_err(|e| format!("解析响应失败: {}", e))?;
        
        let ok = json["ok"]
            .as_bool()
            .ok_or("响应中没有ok字段")?;
        
        if !ok {
            let description = json["description"]
                .as_str()
                .unwrap_or("未知错误");
            return Err(format!("Telegram错误: {}", description));
        }
        
        Ok("消息发送成功".to_string())
    }
    
    pub async fn get_me(&self) -> Result<Value, String> {
        let url = format!("https://api.telegram.org/bot{}/getMe", self.bot_token);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        if !response.status().is_success() {
            return Err("获取bot信息失败".to_string());
        }
        
        response.json().await
            .map_err(|e| format!("解析响应失败: {}", e))
    }
}

#[async_trait::async_trait]
impl ChannelClient for TelegramClient {
    async fn send_message(&self, message: &ChannelMessage) -> Result<String, String> {
        self.send_message_async(&message.content).await
    }
    
    fn verify_config(&self) -> Result<bool, String> {
        if self.bot_token.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}
