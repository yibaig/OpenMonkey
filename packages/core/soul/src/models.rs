//! 灵魂系统数据模型

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

/// 灵魂配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulConfig {
    /// 名字
    pub name: String,
    /// 物种/身份
    pub creature: Option<String>,
    /// 性格特点
    pub vibe: Option<String>,
    /// Emoji
    pub emoji: Option<String>,
    /// 核心原则
    pub core_principles: Vec<String>,
    /// 边界
    pub boundaries: Vec<String>,
}

impl SoulConfig {
    /// 从 Markdown 解析
    pub fn from_markdown(content: &str) -> Result<Self> {
        // 简化的解析逻辑
        let mut name = String::new();
        let mut creature = None;
        let mut vibe = None;
        let mut emoji = None;
        let mut core_principles = Vec::new();
        let mut boundaries = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("- **Name:**") {
                name = line.replace("- **Name:**", "").trim().to_string();
            } else if line.starts_with("- **Creature:**") {
                creature = Some(line.replace("- **Creature:**", "").trim().to_string());
            } else if line.starts_with("- **Vibe:**") {
                vibe = Some(line.replace("- **Vibe:**", "").trim().to_string());
            } else if line.starts_with("- **Emoji:**") {
                emoji = Some(line.replace("- **Emoji:**", "").trim().to_string());
            }
        }
        
        Ok(Self {
            name,
            creature,
            vibe,
            emoji,
            core_principles,
            boundaries,
        })
    }
    
    /// 转换为 Markdown
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("- **Name:** {}\n", self.name));
        
        if let Some(ref creature) = self.creature {
            md.push_str(&format!("- **Creature:** {}\n", creature));
        }
        
        if let Some(ref vibe) = self.vibe {
            md.push_str(&format!("- **Vibe:** {}\n", vibe));
        }
        
        if let Some(ref emoji) = self.emoji {
            md.push_str(&format!("- **Emoji:** {}\n", emoji));
        }
        
        md
    }
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// 名字
    pub name: String,
    /// 称呼
    pub call_sign: Option<String>,
    /// 职业
    pub occupation: Option<String>,
    /// 所在地
    pub location: Option<String>,
    /// 偏好
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    /// 沟通风格
    pub communication_style: Option<String>,
    /// 语言
    pub language: Option<String>,
    /// 提醒方式
    pub reminder_style: Option<String>,
}

impl UserConfig {
    pub fn from_markdown(content: &str) -> Result<Self> {
        // 简化的解析逻辑
        Ok(Self {
            name: "User".to_string(),
            call_sign: None,
            occupation: None,
            location: None,
            preferences: UserPreferences::default(),
        })
    }
    
    pub fn to_markdown(&self) -> String {
        format!("- **Name:** {}\n", self.name)
    }
}

/// Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    /// Agent 列表
    pub agents: Vec<AgentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// 名字
    pub name: String,
    /// 角色
    pub role: String,
    /// 描述
    pub description: Option<String>,
}

impl AgentsConfig {
    pub fn from_markdown(content: &str) -> Result<Self> {
        Ok(Self {
            agents: Vec::new(),
        })
    }
}
