use tauri::State;
use crate::core::channel_manager::{ChannelManager, DefaultFeishuMessageHandler};
use crate::core::channels::{ChannelConfig, ChannelType};
use std::sync::Arc;
use tokio::sync::Mutex;

// ==================== Basic Commands ====================

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
pub async fn chat(message: String) -> Result<String, String> {
    // TODO: Implement chat functionality
    Ok(format!("Echo: {}", message))
}

#[tauri::command]
pub fn get_data_dir() -> Result<String, String> {
    crate::util::get_app_data_dir()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn ensure_data_structure() -> Result<(), String> {
    let data_dir = crate::util::get_app_data_dir()?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    Ok(())
}

// ==================== API Key Commands ====================

#[tauri::command]
pub fn set_api_key(key: String) -> Result<(), String> {
    let store = crate::core::secure_vault::SecureVault::new("openmonkey", "api_key")
        .map_err(|e| format!("创建安全存储失败: {}", e))?;
    store.set_api_key(&key)
        .map_err(|e| format!("保存API密钥失败: {}", e))
}

#[tauri::command]
pub fn get_api_key() -> Result<String, String> {
    let store = crate::core::secure_vault::SecureVault::new("openmonkey", "api_key")
        .map_err(|e| format!("创建安全存储失败: {}", e))?;
    store.get_api_key()
        .map_err(|e| format!("获取API密钥失败: {}", e))
}

#[tauri::command]
pub fn delete_api_key() -> Result<(), String> {
    let store = crate::core::secure_vault::SecureVault::new("openmonkey", "api_key")
        .map_err(|e| format!("创建安全存储失败: {}", e))?;
    store.delete_api_key()
        .map_err(|e| format!("删除API密钥失败: {}", e))
}

// ==================== LLM Config Commands ====================

#[tauri::command]
pub async fn save_llm_config(
    api_key: String,
    base_url: String,
    model: String,
) -> Result<(), String> {
    let config = crate::core::config_store::LlmConfig {
        api_key,
        base_url,
        model,
    };
    
    let store = crate::core::config_store::ConfigStore::new()
        .map_err(|e| format!("创建配置存储失败: {}", e))?;
    store.save_config(&config)
        .map_err(|e| format!("保存配置失败: {}", e))
}

#[tauri::command]
pub async fn get_llm_config() -> Result<crate::core::config_store::LlmConfig, String> {
    let store = crate::core::config_store::ConfigStore::new()
        .map_err(|e| format!("创建配置存储失败: {}", e))?;
    store.load_config()
        .map_err(|e| format!("获取配置失败: {}", e))
}

#[tauri::command]
pub async fn delete_llm_config() -> Result<(), String> {
    let store = crate::core::config_store::ConfigStore::new()
        .map_err(|e| format!("创建配置存储失败: {}", e))?;
    store.delete_config()
        .map_err(|e| format!("删除配置失败: {}", e))
}

#[tauri::command]
pub async fn test_llm_connection(
    api_key: String,
    base_url: String,
    model: String,
) -> Result<String, String> {
    let client = crate::llm::client::LlmClient::new(api_key, base_url, model);
    
    let messages = vec![
        serde_json::json!({
            "role": "user",
            "content": "Hello"
        })
    ];
    
    client.chat(messages).await
        .map(|_| "连接成功".to_string())
        .map_err(|e| format!("连接测试失败: {}", e))
}

// ==================== Memory Bank Commands ====================

#[tauri::command]
pub async fn init_memory_bank(
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let data_dir = crate::util::get_app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    let db_path = data_dir.join("memories.db").to_string_lossy().to_string();
    let bank = crate::core::memory_bank::MemoryBank::new(&db_path).await
        .map_err(|e| format!("初始化记忆库失败: {}", e))?;
    *guard = Some(bank);
    Ok(())
}

#[tauri::command]
pub async fn add_memory(
    content: String,
    importance: f32,
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<String, String> {
    let guard = state.lock().await;
    let bank = guard.as_ref().ok_or("MemoryBank not initialized")?;
    let importance_i32 = importance as i32;
    let result = bank.add_memory(&content, None, importance_i32).await
        .map_err(|e| format!("添加记忆失败: {}", e))?;
    Ok(result.to_string())
}

#[tauri::command]
pub async fn search_memories(
    query: String,
    limit: usize,
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<Vec<crate::core::memory_bank::Memory>, String> {
    let guard = state.lock().await;
    let bank = guard.as_ref().ok_or("MemoryBank not initialized")?;
    bank.search_memories(&query, limit as i32).await
        .map_err(|e| format!("搜索记忆失败: {}", e))
}

#[tauri::command]
pub async fn get_all_memories(
    limit: usize,
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<Vec<crate::core::memory_bank::Memory>, String> {
    let guard = state.lock().await;
    let bank = guard.as_ref().ok_or("MemoryBank not initialized")?;
    bank.get_all_memories(limit as i32).await
        .map_err(|e| format!("获取记忆失败: {}", e))
}

#[tauri::command]
pub async fn delete_memory(
    id: String,
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let bank = guard.as_ref().ok_or("MemoryBank not initialized")?;
    let id_i64 = id.parse::<i64>().map_err(|e| format!("无效的ID: {}", e))?;
    bank.delete_memory(id_i64).await
        .map_err(|e| format!("删除记忆失败: {}", e))
}

#[tauri::command]
pub async fn update_memory_importance(
    id: String,
    importance: f32,
    state: State<'_, Mutex<Option<crate::core::memory_bank::MemoryBank>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let bank = guard.as_ref().ok_or("MemoryBank not initialized")?;
    let id_i64 = id.parse::<i64>().map_err(|e| format!("无效的ID: {}", e))?;
    let importance_i32 = importance as i32;
    bank.update_importance(id_i64, importance_i32).await
        .map_err(|e| format!("更新记忆重要性失败: {}", e))
}

// ==================== Soul Manager Commands ====================

#[tauri::command]
pub async fn init_soul_manager(
    state: State<'_, Mutex<Option<crate::core::soul_manager::SoulManager>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let data_dir = crate::util::get_app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    let manager = crate::core::soul_manager::SoulManager::new(data_dir);
    manager.initialize_defaults().map_err(|e| format!("初始化默认文件失败: {}", e))?;
    *guard = Some(manager);
    Ok(())
}

#[tauri::command]
pub async fn read_soul_file(
    filename: String,
    state: State<'_, Mutex<Option<crate::core::soul_manager::SoulManager>>>,
) -> Result<String, String> {
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("SoulManager not initialized")?;
    match filename.as_str() {
        "SOUL.md" => manager.read_soul().map_err(|e| e.to_string()),
        "USER.md" => manager.read_user().map_err(|e| e.to_string()),
        "AGENTS.md" => manager.read_agents().map_err(|e| e.to_string()),
        _ => Err("不支持的文件类型".to_string()),
    }
}

#[tauri::command]
pub async fn write_soul_file(
    filename: String,
    content: String,
    state: State<'_, Mutex<Option<crate::core::soul_manager::SoulManager>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("SoulManager not initialized")?;
    match filename.as_str() {
        "SOUL.md" => manager.write_soul(&content).map_err(|e| e.to_string()),
        "USER.md" => manager.write_user(&content).map_err(|e| e.to_string()),
        "AGENTS.md" => manager.write_agents(&content).map_err(|e| e.to_string()),
        _ => Err("不支持的文件类型".to_string()),
    }
}

// ==================== Skill Adapter Commands ====================

#[tauri::command]
pub async fn init_skill_adapter(
    state: State<'_, Mutex<Option<crate::core::skill_adapter::SkillAdapter>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let data_dir = crate::util::get_app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    let skills_dir = data_dir.join("skills");
    std::fs::create_dir_all(&skills_dir).map_err(|e| format!("创建技能目录失败: {}", e))?;
    let adapter = crate::core::skill_adapter::SkillAdapter::new(skills_dir);
    *guard = Some(adapter);
    Ok(())
}

#[tauri::command]
pub async fn import_skill(
    path: String,
    state: State<'_, Mutex<Option<crate::core::skill_adapter::SkillAdapter>>>,
) -> Result<String, String> {
    let guard = state.lock().await;
    let adapter = guard.as_ref().ok_or("SkillAdapter not initialized")?;
    let skill = adapter.parse_external(&path).await?;
    adapter.save_skill(&skill)
}

#[tauri::command]
pub async fn list_skills(
    state: State<'_, Mutex<Option<crate::core::skill_adapter::SkillAdapter>>>,
) -> Result<Vec<crate::core::skill_adapter::OmSkill>, String> {
    let guard = state.lock().await;
    let adapter = guard.as_ref().ok_or("SkillAdapter not initialized")?;
    adapter.list_skills()
}

#[tauri::command]
pub async fn enable_skill(
    name: String,
    enabled: bool,
    state: State<'_, Mutex<Option<crate::core::skill_adapter::SkillAdapter>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let adapter = guard.as_ref().ok_or("SkillAdapter not initialized")?;
    adapter.enable_skill(&name, enabled)
}

#[tauri::command]
pub async fn delete_skill(
    name: String,
    state: State<'_, Mutex<Option<crate::core::skill_adapter::SkillAdapter>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let adapter = guard.as_ref().ok_or("SkillAdapter not initialized")?;
    adapter.delete_skill(&name)
}

// ==================== Reflector Commands ====================

#[tauri::command]
pub async fn init_reflector(
    state: State<'_, Mutex<Option<crate::core::reflector::Reflector>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let reflector = crate::core::reflector::Reflector::new();
    *guard = Some(reflector);
    Ok(())
}

#[tauri::command]
pub async fn analyze_interaction(
    input: String,
    output: String,
    state: State<'_, Mutex<Option<crate::core::reflector::Reflector>>>,
) -> Result<Option<crate::core::reflector::LessonLearned>, String> {
    let guard = state.lock().await;
    let reflector = guard.as_ref().ok_or("Reflector not initialized")?;
    Ok(reflector.analyze_interaction(input, output, true, "success".to_string()).await)
}

#[tauri::command]
pub async fn get_interaction_patterns(
    state: State<'_, Mutex<Option<crate::core::reflector::Reflector>>>,
) -> Result<Vec<String>, String> {
    let guard = state.lock().await;
    let reflector = guard.as_ref().ok_or("Reflector not initialized")?;
    Ok(reflector.analyze_patterns())
}

// ==================== Evolution Engine Commands ====================

#[tauri::command]
pub async fn init_evolution_engine(
    state: State<'_, Mutex<Option<crate::core::reflector::EvolutionEngine>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    let memory_bank = crate::core::memory_bank::MemoryBank::new(":memory:").await
        .map_err(|e| format!("创建MemoryBank失败: {}", e))?;
    let engine = crate::core::reflector::EvolutionEngine::new(std::sync::Arc::new(memory_bank));
    *guard = Some(engine);
    Ok(())
}

#[tauri::command]
pub async fn propose_evolution_updates(
    state: State<'_, Mutex<Option<crate::core::reflector::EvolutionEngine>>>,
) -> Result<Vec<crate::core::reflector::EvolutionProposal>, String> {
    let guard = state.lock().await;
    let engine = guard.as_ref().ok_or("EvolutionEngine not initialized")?;
    engine.propose_updates().await
}

#[tauri::command]
pub async fn analyze_soul_evolution(
    state: State<'_, Mutex<Option<crate::core::reflector::EvolutionEngine>>>,
) -> Result<Vec<String>, String> {
    let guard = state.lock().await;
    let engine = guard.as_ref().ok_or("EvolutionEngine not initialized")?;
    Ok(engine.analyze_soul_evolution(""))
}

// ==================== Stats Commands ====================

#[tauri::command]
pub async fn get_usage_stats() -> Result<crate::core::stats::UsageStats, String> {
    let data_dir = crate::util::get_app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    let manager = crate::core::stats::StatsManager::new(data_dir.to_string_lossy().as_ref())
        .map_err(|e| format!("创建统计管理器失败: {}", e))?;
    manager.load_stats()
}

#[tauri::command]
pub async fn reset_usage_stats() -> Result<(), String> {
    let data_dir = crate::util::get_app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;
    let manager = crate::core::stats::StatsManager::new(data_dir.to_string_lossy().as_ref())
        .map_err(|e| format!("创建统计管理器失败: {}", e))?;
    manager.reset_stats()
}

#[tauri::command]
pub async fn get_system_status() -> Result<String, String> {
    Ok("系统状态正常".to_string())
}

// ==================== Channel Manager Commands ====================

#[tauri::command]
pub async fn init_channel_manager(
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<(), String> {
    let mut guard = state.lock().await;
    
    // 创建支持 LLM 的消息处理器
    let message_handler: Arc<dyn crate::core::channels::FeishuMessageHandler> = 
        Arc::new(crate::core::channel_manager::LlmFeishuMessageHandler);
    
    let manager = ChannelManager::new(message_handler);
    *guard = Some(manager);
    
    log::info!("ChannelManager initialized with LLM support");
    Ok(())
}

#[tauri::command]
pub async fn add_channel(
    config: ChannelConfig,
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<(), String> {
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
    manager.add_channel(config).await;
    Ok(())
}

#[tauri::command]
pub async fn list_channels(
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<Vec<ChannelConfig>, String> {
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
    
    Ok(manager.list_channels().await)
}

#[tauri::command]
pub async fn remove_channel(
    channel_type: String,
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<(), String> {
    let channel = match channel_type.as_str() {
        "feishu" => ChannelType::Feishu,
        "we_com" => ChannelType::WeCom,
        "telegram" => ChannelType::Telegram,
        "dingtalk" => ChannelType::DingTalk,
        "slack" => ChannelType::Slack,
        _ => return Err("不支持的渠道类型".to_string()),
    };
    
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
    manager.remove_channel(&channel).await;
    Ok(())
}

#[tauri::command]
pub async fn send_to_channel(
    channel_type: String,
    content: String,
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<String, String> {
    let channel = match channel_type.as_str() {
        "feishu" => ChannelType::Feishu,
        "we_com" => ChannelType::WeCom,
        "telegram" => ChannelType::Telegram,
        "dingtalk" => ChannelType::DingTalk,
        "slack" => ChannelType::Slack,
        _ => return Err("不支持的渠道类型".to_string()),
    };
    
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
    
    let config = manager.get_channel(&channel).await
        .ok_or("渠道未配置")?;
    
    if !config.enabled {
        return Err("渠道未启用".to_string());
    }
    
    // 释放锁后再进行网络请求
    drop(guard);
    
    match &channel {
        ChannelType::Telegram => {
            let telegram_config: crate::core::channels::TelegramConfig = 
                serde_json::from_value(config.config)
                .map_err(|e| format!("解析Telegram配置失败: {}", e))?;
            
            crate::core::channels::send_to_telegram(
                telegram_config.bot_token,
                telegram_config.chat_id,
                content
            ).await
        }
        ChannelType::Feishu => {
            let feishu_config: crate::core::channels::FeishuConfig = 
                serde_json::from_value(config.config.clone())
                .map_err(|e| format!("解析飞书配置失败: {}", e))?;
            
            // 尝试使用 WebSocket 发送消息（如果启用了长连接）
            if feishu_config.use_long_connection.unwrap_or(false) {
                let guard = state.lock().await;
                let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
                
                if manager.is_feishu_ws_connected().await {
                    let receive_id = feishu_config.receive_id.clone().unwrap_or_else(|| "".to_string());
                    if !receive_id.is_empty() {
                        let result = manager.send_message_via_ws(&content, &receive_id).await;
                        drop(guard);
                        return result;
                    }
                }
                drop(guard);
            }
            
            // 如果 WebSocket 不可用，使用 HTTP API
            crate::core::channels::send_to_feishu(
                feishu_config.app_id,
                feishu_config.app_secret,
                content,
                feishu_config.receive_id,
                feishu_config.receive_id_type,
            ).await
        }
        ChannelType::WeCom => {
            let wecom_config: crate::core::channels::WeComConfig = 
                serde_json::from_value(config.config)
                .map_err(|e| format!("解析企业微信配置失败: {}", e))?;
            
            crate::core::channels::send_to_wecom(
                wecom_config.corp_id,
                wecom_config.agent_id,
                wecom_config.secret,
                content
            ).await
        }
        _ => Err("该渠道暂未实现".to_string())
    }
}

#[tauri::command]
pub async fn get_feishu_ws_status(
    state: State<'_, Mutex<Option<ChannelManager>>>,
) -> Result<bool, String> {
    let guard = state.lock().await;
    let manager = guard.as_ref().ok_or("ChannelManager not initialized")?;
    
    Ok(manager.is_feishu_ws_connected().await)
}
