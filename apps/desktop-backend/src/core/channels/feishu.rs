use reqwest::Client;
use serde_json::{json, Value};
use crate::core::channels::{ChannelClient, ChannelMessage, ChannelType};
use async_trait::async_trait;

pub struct FeishuClient {
    app_id: String,
    app_secret: String,
    encrypt_key: Option<String>,
    verification_token: Option<String>,
    client: Client,
}

impl FeishuClient {
    pub fn new(app_id: String, app_secret: String, encrypt_key: Option<String>, verification_token: Option<String>) -> Self {
        Self {
            app_id,
            app_secret,
            encrypt_key,
            verification_token,
            client: Client::new(),
        }
    }
    
    async fn get_access_token(&self) -> Result<String, String> {
        let url = "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal";
        let body = json!({
            "app_id": self.app_id,
            "app_secret": self.app_secret,
        });
        
        let response = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("获取token失败: {}", error_text));
        }
        
        let json: Value = response.json().await
            .map_err(|e| format!("解析响应失败: {}", e))?;
        
        let tenant_access_token = json["tenant_access_token"]
            .as_str()
            .ok_or("响应中没有tenant_access_token")?;
        
        Ok(tenant_access_token.to_string())
    }
    
    async fn get_user_id_by_phone(&self, phone: &str, token: &str) -> Result<String, String> {
        let url = "https://open.feishu.cn/open-apis/contact/v3/users/batch_get_id";
        let body = json!({
            "mobiles": [phone],
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        if !response.status().is_success() {
            return Err("获取用户ID失败".to_string());
        }
        
        let json: Value = response.json().await
            .map_err(|e| format!("解析响应失败: {}", e))?;
        
        let user_id = json["data"]["users"][0]["user_id"]
            .as_str()
            .ok_or("响应中没有user_id")?;
        
        Ok(user_id.to_string())
    }
    
    pub async fn send_message_async(&self, receive_id: &str, content: &str, token: &str, receive_id_type: &str) -> Result<String, String> {
        // receive_id_type 作为 URL 查询参数
        let url = format!(
            "https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type={}",
            receive_id_type
        );
        
        // content 需要是 JSON 字符串格式
        let content_json = json!({
            "text": content
        });
        
        let body = json!({
            "receive_id": receive_id,
            "content": content_json.to_string(),
            "msg_type": "text",
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("发送消息失败: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("发送失败: {}", error_text));
        }
        
        Ok("消息发送成功".to_string())
    }
}

#[async_trait::async_trait]
impl ChannelClient for FeishuClient {
    async fn send_message(&self, message: &ChannelMessage) -> Result<String, String> {
        let token = self.get_access_token().await?;
        
        // 这里需要根据实际配置获取目标用户ID
        // 暂时使用示例，实际应该从配置中获取
        let receive_id = "user_id_placeholder";
        let receive_id_type = "user_id"; // 可选值: user_id, open_id, chat_id, email
        
        self.send_message_async(receive_id, &message.content, &token, receive_id_type).await
    }
    
    fn verify_config(&self) -> Result<bool, String> {
        if self.app_id.is_empty() || self.app_secret.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}
