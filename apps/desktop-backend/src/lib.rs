pub mod commands;
mod core;
mod llm;
mod util;

use commands::*;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(None::<crate::core::memory_bank::MemoryBank>))
        .manage(Mutex::new(None::<crate::core::soul_manager::SoulManager>))
        .manage(Mutex::new(None::<crate::core::skill_adapter::SkillAdapter>))
        .manage(Mutex::new(None::<crate::core::reflector::Reflector>))
        .manage(Mutex::new(None::<crate::core::reflector::EvolutionEngine>))
        .manage(Mutex::new(None::<crate::core::channel_manager::ChannelManager>))
        .invoke_handler(tauri::generate_handler![
            // Basic commands
            ping,
            chat,
            get_data_dir,
            ensure_data_structure,
            
            // API Key commands
            set_api_key,
            get_api_key,
            delete_api_key,
            
            // LLM config commands
            save_llm_config,
            get_llm_config,
            delete_llm_config,
            test_llm_connection,
            
            // Memory bank commands
            init_memory_bank,
            add_memory,
            search_memories,
            get_all_memories,
            delete_memory,
            update_memory_importance,
            
            // Soul manager commands
            init_soul_manager,
            read_soul_file,
            write_soul_file,
            
            // Skill adapter commands
            init_skill_adapter,
            import_skill,
            list_skills,
            enable_skill,
            delete_skill,
            
            // Reflector commands
            init_reflector,
            analyze_interaction,
            get_interaction_patterns,
            
            // Evolution engine commands
            init_evolution_engine,
            propose_evolution_updates,
            analyze_soul_evolution,
            
            // Stats commands
            get_usage_stats,
            reset_usage_stats,
            get_system_status,
            
            // Channel manager commands
            init_channel_manager,
            add_channel,
            list_channels,
            remove_channel,
            send_to_channel,
            get_feishu_ws_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
