use crate::core::channels::{ChannelConfig, ChannelType, ChannelClient, ChannelMessage};
use crate::core::channels::{FeishuWebSocketClient, FeishuMessage, FeishuMessageHandler, FeishuConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde_json::{Value, json};

pub struct ChannelManager {
    channels: Arc<RwLock<HashMap<ChannelType, ChannelConfig>>>,
    feishu_ws_client: Arc<Mutex<Option<FeishuWebSocketClient>>>,
    message_handler: Arc<dyn FeishuMessageHandler>,
}

impl ChannelManager {
    pub fn new(message_handler: Arc<dyn FeishuMessageHandler>) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            feishu_ws_client: Arc::new(Mutex::new(None)),
            message_handler,
        }
    }
    
    pub async fn add_channel(&self, config: ChannelConfig) {
        let mut channels = self.channels.write().await;
        
        // 如果是飞书渠道且启用了长连接，启动 WebSocket
        if config.channel_type == ChannelType::Feishu {
            if let Err(e) = self.start_feishu_websocket_if_needed(&config).await {
                log::error!("Failed to start Feishu WebSocket: {}", e);
            }
        }
        
        channels.insert(config.channel_type.clone(), config);
    }
    
    pub async fn get_channel(&self, channel_type: &ChannelType) -> Option<ChannelConfig> {
        let channels = self.channels.read().await;
        channels.get(channel_type).cloned()
    }
    
    pub async fn list_channels(&self) -> Vec<ChannelConfig> {
        let channels = self.channels.read().await;
        channels.values().cloned().collect()
    }
    
    pub async fn remove_channel(&self, channel_type: &ChannelType) {
        let mut channels = self.channels.write().await;
        
        // 如果是飞书渠道，停止 WebSocket
        if *channel_type == ChannelType::Feishu {
            self.stop_feishu_websocket().await;
        }
        
        channels.remove(channel_type);
    }
    
    pub async fn get_enabled_channels(&self) -> Vec<ChannelConfig> {
        let channels = self.channels.read().await;
        channels.values()
            .filter(|c| c.enabled)
            .cloned()
            .collect()
    }
    
    /// 启动飞书 WebSocket 长连接（如果需要）
    async fn start_feishu_websocket_if_needed(&self, config: &ChannelConfig) -> Result<(), String> {
        let feishu_config: crate::core::channels::FeishuConfig = 
            serde_json::from_value(config.config.clone())
            .map_err(|e| format!("Failed to parse Feishu config: {}", e))?;
        
        // 检查是否启用了长连接
        let use_long_connection = feishu_config.use_long_connection.unwrap_or(false);
        if !use_long_connection {
            return Ok(());
        }
        
        // 停止现有的 WebSocket 连接
        self.stop_feishu_websocket().await;
        
        // 创建新的 WebSocket 客户端
        let mut client = FeishuWebSocketClient::new(
            feishu_config.app_id,
            feishu_config.app_secret,
            feishu_config.encrypt_key,
        );
        
        // 获取消息接收器
        let message_rx = client.get_message_receiver();
        let handler = self.message_handler.clone();
        
        // 启动消息处理任务
        tokio::spawn(async move {
            let mut rx = message_rx.lock().await;
            while let Some(message) = rx.recv().await {
                if let Err(e) = handler.handle_message(message).await {
                    log::error!("Failed to handle Feishu message: {}", e);
                }
            }
        });
        
        // 保存客户端
        *self.feishu_ws_client.lock().await = Some(client);
        
        // 启动 WebSocket 连接（在后台运行）
        let ws_client = self.feishu_ws_client.clone();
        tokio::spawn(async move {
            if let Some(ref mut client) = *ws_client.lock().await {
                if let Err(e) = client.start().await {
                    log::error!("Feishu WebSocket error: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// 停止飞书 WebSocket 连接
    async fn stop_feishu_websocket(&self) {
        if let Some(ref mut client) = *self.feishu_ws_client.lock().await {
            client.stop().await;
        }
        *self.feishu_ws_client.lock().await = None;
    }
    
    /// 检查飞书 WebSocket 连接状态
    pub async fn is_feishu_ws_connected(&self) -> bool {
        if let Some(ref client) = *self.feishu_ws_client.lock().await {
            client.is_connected().await
        } else {
            false
        }
    }
    
    /// 通过 WebSocket 发送消息（如果可用）
    pub async fn send_message_via_ws(&self, content: &str, receive_id: &str) -> Result<String, String> {
        if let Some(ref client) = *self.feishu_ws_client.lock().await {
            if client.is_connected().await {
                // 构造消息格式
                let message = json! ({
                    "type": "message",
                    "receive_id": receive_id,
                    "content": content,
                    "msg_type": "text"
                });
                
                // 发送消息
                client.send_message(tokio_tungstenite::tungstenite::Message::Text(message.to_string()))
                    .await
                    .map_err(|e| format!("发送消息失败: {}", e))?;
                
                Ok("消息发送成功".to_string())
            } else {
                Err("WebSocket 未连接".to_string())
            }
        } else {
            Err("WebSocket 客户端未初始化".to_string())
        }
    }
}

/// LLM 集成的飞书消息处理器
pub struct LlmFeishuMessageHandler;

#[async_trait::async_trait]
impl FeishuMessageHandler for LlmFeishuMessageHandler {
    async fn handle_message(&self, message: FeishuMessage) -> Result<String, String> {
        log::info!(
            "Received Feishu message from {}: {}",
            message.sender.sender_id,
            message.content
        );
        
        // 1. 加载 LLM 配置
        let llm_config = match crate::core::config_store::ConfigStore::new() {
            Ok(store) => match store.load_config() {
                Ok(config) => config,
                Err(_) => {
                    log::warn!("LLM config not found, using default reply");
                    return Ok("请先配置 LLM 才能使用智能回复功能".to_string());
                }
            },
            Err(_) => {
                log::warn!("Failed to load config store, using default reply");
                return Ok("配置加载失败，请检查配置".to_string());
            }
        };
        
        // 2. 创建 LLM 客户端并发送请求
        let llm_client = crate::llm::client::LlmClient::new(
            llm_config.api_key,
            llm_config.base_url,
            llm_config.model,
        );
        
        let messages = vec![
            serde_json::json!({
                "role": "system", 
                "content": "你是 OpenMonkey，一个智能 AI 助手。请提供友好、有帮助的回复。"
            }),
            serde_json::json!({
                "role": "user", 
                "content": message.content
            })
        ];
        
        match llm_client.chat(messages).await {
            Ok(response) => {
                log::info!("LLM response generated successfully");
                
                // 3. 发送回复到飞书
                // 这里我们只是返回 LLM 的回复，实际发送需要通过其他机制
                Ok(response.content)
            }
            Err(e) => {
                log::error!("LLM request failed: {}", e);
                Ok(format!("抱歉，处理请求时出错：{}", e))
            }
        }
    }
}

/// 默认的飞书消息处理器（向后兼容）
pub struct DefaultFeishuMessageHandler;

#[async_trait::async_trait]
impl FeishuMessageHandler for DefaultFeishuMessageHandler {
    async fn handle_message(&self, message: FeishuMessage) -> Result<String, String> {
        log::info!(
            "Received Feishu message from {}: {}",
            message.sender.sender_id,
            message.content
        );
        
        Ok("Message processed".to_string())
    }
}
