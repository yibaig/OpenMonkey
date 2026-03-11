//! Agent 定义

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Agent 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Busy,
    Error(String),
}

/// Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    pub role: String,
    pub description: Option<String>,
    pub skills: Vec<String>,
}

/// Agent
pub struct Agent {
    pub config: AgentConfig,
    pub state: AgentState,
}

impl Agent {
    /// 创建新 Agent
    pub fn new(name: &str, role: &str) -> Self {
        Self {
            config: AgentConfig {
                id: Uuid::new_v4().to_string(),
                name: name.to_string(),
                role: role.to_string(),
                description: None,
                skills: Vec::new(),
            },
            state: AgentState::Idle,
        }
    }
    
    /// 设置描述
    pub fn with_description(mut self, desc: &str) -> Self {
        self.config.description = Some(desc.to_string());
        self
    }
    
    /// 添加技能
    pub fn add_skill(&mut self, skill_id: &str) {
        self.config.skills.push(skill_id.to_string());
    }
    
    /// 执行任务
    pub async fn execute(&mut self, task: &str) -> Result<String> {
        self.state = AgentState::Busy;
        
        // 模拟任务执行
        let result = format!("Agent {} 执行任务：{}", self.config.name, task);
        
        self.state = AgentState::Idle;
        Ok(result)
    }
}
