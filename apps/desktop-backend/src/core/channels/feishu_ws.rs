use tokio::time::{interval, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use url::Url;
use chrono::Utc;
use rand::Rng;

/// Feishu WebSocket 客户端
pub struct FeishuWebSocketClient {
    app_id: String,
    app_secret: String,
    encrypt_key: Option<String>,
    ws_stream: Option<Arc<Mutex<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    message_tx: mpsc::Sender<FeishuMessage>,
    message_rx: Arc<Mutex<mpsc::Receiver<FeishuMessage>>>,
    reconnect_interval: Duration,
    is_connected: Arc<Mutex<bool>>,
}

/// 飞书消息结构
#[derive(Debug, Clone)]
pub struct FeishuMessage {
    pub message_id: String,
    pub sender: FeishuSender,
    pub content: String,
    pub message_type: String,
    pub chat_id: Option<String>,
    pub chat_type: Option<String>,
}

/// 发送者信息
#[derive(Debug, Clone)]
pub struct FeishuSender {
    pub sender_id: String,
    pub sender_type: String,
    pub name: Option<String>,
}

impl FeishuWebSocketClient {
    /// 创建新的 WebSocket 客户端
    pub fn new(
        app_id: String,
        app_secret: String,
        encrypt_key: Option<String>,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::channel(100);
        
        Self {
            app_id,
            app_secret,
            encrypt_key,
            ws_stream: None,
            message_tx,
            message_rx: Arc::new(Mutex::new(message_rx)),
            reconnect_interval: Duration::from_secs(5),
            is_connected: Arc::new(Mutex::new(false)),
        }
    }

    /// 获取消息接收器
    pub fn get_message_receiver(&self) -> Arc<Mutex<mpsc::Receiver<FeishuMessage>>> {
        self.message_rx.clone()
    }

    /// 启动 WebSocket 连接（带自动重连）
    pub async fn start(&mut self) -> Result<(), String> {
        loop {
            match self.connect().await {
                Ok(()) => {
                    log::info!("Feishu WebSocket connected successfully");
                    if let Err(e) = self.run_message_loop().await {
                        log::error!("WebSocket error: {}, reconnecting...", e);
                    }
                }
                Err(e) => {
                    log::error!("Failed to connect Feishu WebSocket: {}, retrying in {:?}...", e, self.reconnect_interval);
                }
            }

            // 等待重连
            tokio::time::sleep(self.reconnect_interval).await;
        }
    }

    /// 建立 WebSocket 连接
    async fn connect(&mut self) -> Result<(), String> {
        // 1. 获取 tenant_access_token
        let token = self.get_tenant_access_token().await?;
        
        // 2. 构建 WebSocket URL
        let ws_url = format!(
            "wss://ws-open.feishu.cn/open-apis/bot/v2/websocket?token={}",
            token
        );
        
        let url = Url::parse(&ws_url)
            .map_err(|e| format!("Invalid WebSocket URL: {}", e))?;
        
        // 3. 建立 WebSocket 连接
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| format!("WebSocket connection failed: {}", e))?;
        
        self.ws_stream = Some(Arc::new(Mutex::new(ws_stream)));
        
        // 4. 发送认证消息
        self.authenticate().await?;
        
        // 5. 设置连接状态
        *self.is_connected.lock().await = true;
        
        Ok(())
    }

    /// 获取 tenant_access_token
    async fn get_tenant_access_token(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal";
        
        let body = json!({
            "app_id": self.app_id,
            "app_secret": self.app_secret,
        });
        
        let response = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to get access token: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to get access token: {}", error_text));
        }
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;
        
        let code = json["code"].as_i64().unwrap_or(1);
        if code != 0 {
            let message = json["msg"].as_str().unwrap_or("Unknown error");
            return Err(format!("Failed to get access token: {}", message));
        }
        
        let token = json["tenant_access_token"]
            .as_str()
            .ok_or("No tenant_access_token in response")?;
        
        Ok(token.to_string())
    }

    /// 发送认证消息
    async fn authenticate(&self) -> Result<(), String> {
        let auth_message = json!({
            "type": "auth",
            "app_id": self.app_id,
            "app_secret": self.app_secret,
            "timestamp": chrono::Utc::now().timestamp(),
            "nonce": rand::random::<u32>().to_string()
        });
        
        self.send_message(Message::Text(auth_message.to_string())).await
    }

    /// 发送消息
    pub async fn send_message(&self, message: Message) -> Result<(), String> {
        if let Some(ws_stream) = &self.ws_stream {
            let mut stream = ws_stream.lock().await;
            stream.send(message)
                .await
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err("WebSocket not connected".to_string())
        }
    }

    /// 运行消息循环
    async fn run_message_loop(&self) -> Result<(), String> {
        let ws_stream = self.ws_stream.as_ref()
            .ok_or("WebSocket not connected")?;
        
        let mut heartbeat_interval = interval(Duration::from_secs(30));
        
        loop {
            // 锁定 WebSocket 流
            let mut stream = ws_stream.lock().await;
            
            // 等待消息或心跳
            tokio::select! {
                // 接收 WebSocket 消息
                msg = stream.next() => {
                    drop(stream); // 释放锁
                    
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            self.handle_message(&text).await?;
                        }
                        Some(Ok(Message::Binary(data))) => {
                            if let Ok(text) = String::from_utf8(data) {
                                self.handle_message(&text).await?;
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            // 自动回复 Pong
                            let _ = self.send_message(Message::Pong(data)).await;
                        }
                        Some(Ok(Message::Close(_))) => {
                            log::info!("WebSocket connection closed");
                            break;
                        }
                        Some(Err(e)) => {
                            return Err(format!("WebSocket error: {}", e));
                        }
                        _ => {}
                    }
                }
                
                // 发送心跳
                _ = heartbeat_interval.tick() => {
                    drop(stream); // 释放锁
                    
                    let heartbeat = json!({
                        "type": "ping",
                        "timestamp": chrono::Utc::now().timestamp(),
                    });
                    if let Err(e) = self.send_message(Message::Text(heartbeat.to_string())).await {
                        log::error!("Failed to send heartbeat: {}", e);
                    }
                }
            }
        }
        
        *self.is_connected.lock().await = false;
        Ok(())
    }

    /// 处理接收到的消息
    async fn handle_message(&self, text: &str) -> Result<(), String> {
        let json: Value = serde_json::from_str(text)
            .map_err(|e| format!("Failed to parse message: {}", e))?;
        
        let msg_type = json["type"].as_str().unwrap_or("");
        
        match msg_type {
            "pong" => {
                // 心跳响应，忽略
                log::debug!("Received pong");
            }
            "event" => {
                // 处理事件消息
                if let Some(event) = json.get("event") {
                    self.handle_event(event).await?;
                }
            }
            "error" => {
                let error_msg = json["msg"].as_str().unwrap_or("Unknown error");
                log::error!("Feishu WebSocket error: {}", error_msg);
            }
            _ => {
                log::debug!("Unknown message type: {}", msg_type);
            }
        }
        
        Ok(())
    }

    /// 处理事件
    async fn handle_event(&self, event: &Value) -> Result<(), String> {
        let event_type = event["type"].as_str().unwrap_or("");
        
        match event_type {
            "message" => {
                // 处理消息事件
                if let Some(message) = event.get("message") {
                    self.handle_chat_message(message).await?;
                }
            }
            "group_join" => {
                log::info!("Bot joined a group");
            }
            _ => {
                log::debug!("Unhandled event type: {}", event_type);
            }
        }
        
        Ok(())
    }

    /// 处理聊天消息
    async fn handle_chat_message(&self, message: &Value) -> Result<(), String> {
        let message_id = message["message_id"].as_str().unwrap_or("").to_string();
        let content = message["content"].as_str().unwrap_or("").to_string();
        let msg_type = message["msg_type"].as_str().unwrap_or("text").to_string();
        
        // 解析发送者信息
        let sender = FeishuSender {
            sender_id: message["sender"]["sender_id"]["user_id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            sender_type: message["sender"]["sender_type"]
                .as_str()
                .unwrap_or("user")
                .to_string(),
            name: message["sender"]["name"].as_str().map(|s| s.to_string()),
        };
        
        // 解析聊天信息
        let chat_id = message["chat_id"].as_str().map(|s| s.to_string());
        let chat_type = message["chat_type"].as_str().map(|s| s.to_string());
        
        // 解密内容（如果启用了加密）
        let decrypted_content = if let Some(ref encrypt_key) = self.encrypt_key {
            match self.decrypt_message(&content, encrypt_key).await {
                Ok(decrypted) => decrypted,
                Err(e) => {
                    log::error!("Failed to decrypt message: {}", e);
                    content
                }
            }
        } else {
            content
        };
        
        let feishu_msg = FeishuMessage {
            message_id,
            sender,
            content: decrypted_content,
            message_type: msg_type,
            chat_id,
            chat_type,
        };
        
        // 发送到消息通道
        if let Err(e) = self.message_tx.send(feishu_msg).await {
            log::error!("Failed to send message to channel: {}", e);
        }
        
        Ok(())
    }

    /// 解密消息（简化实现）
    async fn decrypt_message(&self, encrypted: &str, _key: &str) -> Result<String, String> {
        use base64::Engine;
        
        // 解析加密内容
        let encrypted_data: serde_json::Value = serde_json::from_str(encrypted)
            .map_err(|e| format!("Failed to parse encrypted content: {}", e))?;
        
        let encrypt_str = encrypted_data["encrypt"].as_str()
            .ok_or("Missing encrypt field")?;
        
        // 解码 base64
        let encrypted_bytes = base64::engine::general_purpose::STANDARD.decode(encrypt_str)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;
        
        // 提取实际内容（这里只是一个简化实现，实际需要 AES 解密）
        // 注意：在生产环境中，应该使用完整的 AES-256-CBC 解密
        log::info!("Message decryption not fully implemented yet");
        
        // 尝试直接解析内容（如果未加密）
        if let Ok(content_json) = serde_json::from_str::<serde_json::Value>(encrypted) {
            if let Some(content) = content_json["content"].as_str() {
                return Ok(content.to_string());
            }
        }
        
        // 对于加密消息，返回原始内容
        Ok(encrypted.to_string())
    }

    /// 检查连接状态
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.lock().await
    }

    /// 停止连接
    pub async fn stop(&mut self) {
        *self.is_connected.lock().await = false;
        if let Some(ws_stream) = self.ws_stream.take() {
            let mut stream = ws_stream.lock().await;
            let _ = stream.close(None).await;
        }
    }
}

/// 飞书消息处理器 trait
#[async_trait::async_trait]
pub trait FeishuMessageHandler: Send + Sync {
    async fn handle_message(&self, message: FeishuMessage) -> Result<String, String>;
}

/// 启动飞书 WebSocket 客户端
pub async fn start_feishu_websocket(
    app_id: String,
    app_secret: String,
    encrypt_key: Option<String>,
    handler: Arc<dyn FeishuMessageHandler>,
) -> Result<(), String> {
    let mut client = FeishuWebSocketClient::new(app_id, app_secret, encrypt_key);
    let message_rx = client.get_message_receiver();
    
    // 启动消息处理任务
    let handler_clone = handler.clone();
    tokio::spawn(async move {
        let mut rx = message_rx.lock().await;
        while let Some(message) = rx.recv().await {
            if let Err(e) = handler_clone.handle_message(message).await {
                log::error!("Failed to handle message: {}", e);
            }
        }
    });
    
    // 启动 WebSocket 连接
    client.start().await
}
