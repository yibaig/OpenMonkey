use reqwest::Client;
use serde_json::{json, Value};
use crate::core::channels::{ChannelClient, ChannelMessage};
use async_trait::async_trait;

pub struct WeComClient {
    corp_id: String,
    agent_id: String,
    secret: String,
    token: Option<String>,
    client: Client,
}

impl WeComClient {
    pub fn new(corp_id: String, agent_id: String, secret: String, token: Option<String>) -> Self {
        Self {
            corp_id,
            agent_id,
            secret,
            token,
            client: Client::new(),
        }
    }
    
    async fn get_access_token(&self) -> Result<String, String> {
        let url = "https://qyapi.weixin.qq.com/cgi-bin/gettoken";
        let body = json!({
            "corpid": self.corp_id,
            "corpsecret": self.secret,
        });
        
        let response = self.client
            .get(url)
            .query(&[("corpid", &self.corp_id), ("corpsecret", &self.secret)])
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("获取token失败: {}", error_text));
        }
        
        let json: Value = response.json().await
            .map_err(|e| format!("解析响应失败: {}", e))?;
        
        let access_token = json["access_token"]
            .as_str()
            .ok_or("响应中没有access_token")?;
        
        Ok(access_token.to_string())
    }
    
    pub async fn send_message_async(&self, user_id: &str, content: &str, token: &str) -> Result<String, String> {
        let url = "https://qyapi.weixin.qq.com/cgi-bin/message/send";
        let body = json!({
            "touser": user_id,
            "msgtype": "text",
            "agentid": self.agent_id,
            "text": {
                "content": content
            }
        });
        
        let response = self.client
            .post(url)
            .query(&[("access_token", token)])
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
impl ChannelClient for WeComClient {
    async fn send_message(&self, message: &ChannelMessage) -> Result<String, String> {
        let token = self.get_access_token().await?;
        
        // 需要从配置中获取目标用户ID
        let user_id = "user_id_placeholder";
        
        self.send_message_async(user_id, &message.content, &token).await
    }
    
    fn verify_config(&self) -> Result<bool, String> {
        if self.corp_id.is_empty() || self.secret.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}
