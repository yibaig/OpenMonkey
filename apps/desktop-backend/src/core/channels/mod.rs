use serde::{Deserialize, Serialize};
use std::fmt;
use async_trait::async_trait;

pub mod feishu;
pub mod wecom;
pub mod telegram;
pub mod feishu_ws;

pub use feishu::FeishuClient;
pub use wecom::WeComClient;
pub use telegram::TelegramClient;
pub use feishu_ws::{FeishuWebSocketClient, FeishuMessage, FeishuSender, FeishuMessageHandler, start_feishu_websocket};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub channel_type: ChannelType,
    pub enabled: bool,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Feishu,
    WeCom,
    Telegram,
    DingTalk,
    Slack,
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelType::Feishu => write!(f, "飞书"),
            ChannelType::WeCom => write!(f, "企业微信"),
            ChannelType::Telegram => write!(f, "Telegram"),
            ChannelType::DingTalk => write!(f, "钉钉"),
            ChannelType::Slack => write!(f, "Slack"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuConfig {
    pub app_id: String,
    pub app_secret: String,
    pub encrypt_key: Option<String>,
    pub verification_token: Option<String>,
    pub use_long_connection: Option<bool>,
    pub receive_id: Option<String>,
    pub receive_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeComConfig {
    pub corp_id: String,
    pub agent_id: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub channel_type: ChannelType,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

#[async_trait]
pub trait ChannelClient: Send + Sync {
    async fn send_message(&self, message: &ChannelMessage) -> Result<String, String>;
    fn verify_config(&self) -> Result<bool, String>;
}

pub async fn send_to_telegram(bot_token: String, chat_id: Option<String>, content: String) -> Result<String, String> {
    use reqwest::Client;
    use serde_json::{json, Value};
    
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    // 如果没有指定 chat_id，使用一个默认的处理方式
    // 实际使用时应该从配置或消息中获取
    let chat_id = chat_id.unwrap_or_else(|| "".to_string());
    
    if chat_id.is_empty() {
        return Err("Chat ID 不能为空".to_string());
    }
    
    let body = json!({
        "chat_id": chat_id,
        "text": content,
        "parse_mode": "HTML"
    });
    
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("发送消息失败: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("发送消息失败: {}", error_text));
    }
    
    let json: Value = response.json().await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    if !json["ok"].as_bool().unwrap_or(false) {
        let error = json["description"].as_str().unwrap_or("未知错误");
        return Err(format!("发送消息失败: {}", error));
    }
    
    Ok("消息发送成功".to_string())
}

pub async fn send_to_feishu(app_id: String, app_secret: String, content: String, receive_id: Option<String>, receive_id_type: Option<String>) -> Result<String, String> {
    use reqwest::Client;
    use serde_json::{json, Value};
    
    // 获取access_token
    let client = Client::new();
    let token_url = "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal";
    let token_body = json!({
        "app_id": app_id,
        "app_secret": app_secret
    });
    
    let token_response = client
        .post(token_url)
        .json(&token_body)
        .send()
        .await
        .map_err(|e| format!("获取access_token失败: {}", e))?;
    
    if !token_response.status().is_success() {
        let error_text = token_response.text().await.unwrap_or_default();
        return Err(format!("获取access_token失败: {}", error_text));
    }
    
    let token_json: Value = token_response.json().await
        .map_err(|e| format!("解析access_token响应失败: {}", e))?;
    
    let code = token_json["code"].as_i64().unwrap_or(1);
    if code != 0 {
        let message = token_json["msg"].as_str().unwrap_or("未知错误");
        return Err(format!("获取access_token失败: {}", message));
    }
    
    let tenant_access_token = token_json["tenant_access_token"].as_str()
        .ok_or("响应中没有tenant_access_token")?;
    
    // 发送消息 - receive_id_type 作为 URL 查询参数
    let receive_id_type = receive_id_type.unwrap_or_else(|| "chat_id".to_string());
    let message_url = format!(
        "https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type={}",
        receive_id_type
    );
    
    // content 需要是 JSON 字符串格式
    let content_json = json!({
        "text": content
    });
    
    // 使用传入的 receive_id 或默认值
    let receive_id = receive_id.unwrap_or_else(|| "oc_123456".to_string());
    
    let message_body = json!({
        "receive_id": receive_id,
        "content": content_json.to_string(),
        "msg_type": "text"
    });
    
    let message_response = client
        .post(&message_url)
        .header("Authorization", format!("Bearer {}", tenant_access_token))
        .json(&message_body)
        .send()
        .await
        .map_err(|e| format!("发送消息失败: {}", e))?;
    
    if !message_response.status().is_success() {
        let error_text = message_response.text().await.unwrap_or_default();
        return Err(format!("发送消息失败: {}", error_text));
    }
    
    let message_json: Value = message_response.json().await
        .map_err(|e| format!("解析消息响应失败: {}", e))?;
    
    let msg_code = message_json["code"].as_i64().unwrap_or(1);
    if msg_code != 0 {
        let message = message_json["msg"].as_str().unwrap_or("未知错误");
        return Err(format!("发送消息失败: {}", message));
    }
    
    Ok("消息发送成功".to_string())
}

pub async fn send_to_wecom(corp_id: String, agent_id: String, secret: String, content: String) -> Result<String, String> {
    use reqwest::Client;
    use serde_json::{json, Value};
    
    let client = Client::new();
    
    // 获取access_token
    let token_url = format!(
        "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
        corp_id, secret
    );
    
    let token_response = client
        .get(&token_url)
        .send()
        .await
        .map_err(|e| format!("获取access_token失败: {}", e))?;
    
    if !token_response.status().is_success() {
        let error_text = token_response.text().await.unwrap_or_default();
        return Err(format!("获取access_token失败: {}", error_text));
    }
    
    let token_json: Value = token_response.json().await
        .map_err(|e| format!("解析access_token响应失败: {}", e))?;
    
    let access_token = token_json["access_token"].as_str()
        .ok_or("响应中没有access_token")?;
    
    // 发送消息
    let message_url = format!(
        "https://qyapi.weixin.qq.com/cgi-bin/message/send?access_token={}",
        access_token
    );
    
    let message_body = json!({
        "touser": "@all",
        "msgtype": "text",
        "agentid": agent_id,
        "text": {
            "content": content
        },
        "safe": 0
    });
    
    let message_response = client
        .post(&message_url)
        .json(&message_body)
        .send()
        .await
        .map_err(|e| format!("发送消息失败: {}", e))?;
    
    if !message_response.status().is_success() {
        let error_text = message_response.text().await.unwrap_or_default();
        return Err(format!("发送消息失败: {}", error_text));
    }
    
    let message_json: Value = message_response.json().await
        .map_err(|e| format!("解析消息响应失败: {}", e))?;
    
    let errcode = message_json["errcode"].as_i64().unwrap_or(1);
    if errcode != 0 {
        let errmsg = message_json["errmsg"].as_str().unwrap_or("未知错误");
        return Err(format!("发送消息失败: {}", errmsg));
    }
    
    Ok("消息发送成功".to_string())
}
