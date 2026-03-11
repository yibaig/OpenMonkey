use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmConfig {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub model: String,
}

pub struct ConfigStore {
    config_path: PathBuf,
}

impl ConfigStore {
    pub fn new() -> Result<Self, String> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("OpenMonkey");
        
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
        
        let config_path = config_dir.join("llm_config.json");
        
        Ok(Self { config_path })
    }
    
    pub fn save_config(&self, config: &LlmConfig) -> Result<(), String> {
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| e.to_string())?;
        fs::write(&self.config_path, json)
            .map_err(|e| e.to_string())?;
        eprintln!("Config saved to: {:?}", self.config_path);
        Ok(())
    }
    
    pub fn load_config(&self) -> Result<LlmConfig, String> {
        if !self.config_path.exists() {
            return Err("Config file not found".to_string());
        }
        
        let json = fs::read_to_string(&self.config_path)
            .map_err(|e| e.to_string())?;
        let config: LlmConfig = serde_json::from_str(&json)
            .map_err(|e| e.to_string())?;
        eprintln!("Config loaded from: {:?}", self.config_path);
        Ok(config)
    }
    
    pub fn delete_config(&self) -> Result<(), String> {
        if self.config_path.exists() {
            fs::remove_file(&self.config_path)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
