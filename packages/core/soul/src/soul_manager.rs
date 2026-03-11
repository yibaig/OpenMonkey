//! 灵魂管理器 - 人格配置管理

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

use crate::models::{SoulConfig, UserConfig, AgentsConfig};

/// 灵魂管理器
pub struct SoulManager {
    config_dir: PathBuf,
    soul_config: Option<SoulConfig>,
    user_config: Option<UserConfig>,
    agents_config: Option<AgentsConfig>,
}

impl SoulManager {
    /// 创建新的灵魂管理器
    pub fn new(config_dir: &Path) -> Self {
        Self {
            config_dir: config_dir.to_path_buf(),
            soul_config: None,
            user_config: None,
            agents_config: None,
        }
    }
    
    /// 加载所有配置
    pub async fn load_all(&mut self) -> Result<()> {
        self.load_soul().await?;
        self.load_user().await?;
        self.load_agents().await?;
        
        info!("灵魂配置加载完成");
        Ok(())
    }
    
    /// 加载灵魂配置
    pub async fn load_soul(&mut self) -> Result<()> {
        let path = self.config_dir.join("SOUL.md");
        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            self.soul_config = Some(SoulConfig::from_markdown(&content)?);
            info!("SOUL.md 加载成功");
        }
        Ok(())
    }
    
    /// 加载用户配置
    pub async fn load_user(&mut self) -> Result<()> {
        let path = self.config_dir.join("USER.md");
        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            self.user_config = Some(UserConfig::from_markdown(&content)?);
            info!("USER.md 加载成功");
        }
        Ok(())
    }
    
    /// 加载 Agent 配置
    pub async fn load_agents(&mut self) -> Result<()> {
        let path = self.config_dir.join("AGENTS.md");
        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            self.agents_config = Some(AgentsConfig::from_markdown(&content)?);
            info!("AGENTS.md 加载成功");
        }
        Ok(())
    }
    
    /// 获取灵魂配置
    pub fn soul(&self) -> Option<&SoulConfig> {
        self.soul_config.as_ref()
    }
    
    /// 获取用户配置
    pub fn user(&self) -> Option<&UserConfig> {
        self.user_config.as_ref()
    }
    
    /// 获取 Agent 配置
    pub fn agents(&self) -> Option<&AgentsConfig> {
        self.agents_config.as_ref()
    }
    
    /// 保存灵魂配置
    pub async fn save_soul(&self) -> Result<()> {
        if let Some(config) = &self.soul_config {
            let path = self.config_dir.join("SOUL.md");
            fs::write(&path, config.to_markdown()).await?;
            info!("SOUL.md 保存成功");
        }
        Ok(())
    }
    
    /// 保存用户配置
    pub async fn save_user(&self) -> Result<()> {
        if let Some(config) = &self.user_config {
            let path = self.config_dir.join("USER.md");
            fs::write(&path, config.to_markdown()).await?;
            info!("USER.md 保存成功");
        }
        Ok(())
    }
}
