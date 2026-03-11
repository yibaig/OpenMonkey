//! 协调器 - 多 Agent 任务编排

use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tracing::info;

use crate::agent::{Agent, AgentState};

/// 任务定义
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub assigned_to: Option<String>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed(String),
    Failed(String),
}

/// 协调器
pub struct Coordinator {
    agents: HashMap<String, Agent>,
    tasks: HashMap<String, Task>,
    event_tx: broadcast::Sender<Event>,
}

#[derive(Debug, Clone)]
pub enum Event {
    TaskAssigned(String, String),
    TaskCompleted(String, String),
    TaskFailed(String, String),
}

impl Coordinator {
    /// 创建新的协调器
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(100);
        
        Self {
            agents: HashMap::new(),
            tasks: HashMap::new(),
            event_tx,
        }
    }
    
    /// 注册 Agent
    pub fn register_agent(&mut self, agent: Agent) {
        let id = agent.config.id.clone();
        info!("注册 Agent: {} ({})", agent.config.name, id);
        self.agents.insert(id, agent);
    }
    
    /// 创建任务
    pub fn create_task(&mut self, description: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let task = Task {
            id: id.clone(),
            description: description.to_string(),
            assigned_to: None,
            status: TaskStatus::Pending,
        };
        
        info!("创建任务：{}", id);
        self.tasks.insert(id.clone(), task);
        id
    }
    
    /// 分配任务给 Agent
    pub fn assign_task(&mut self, task_id: &str, agent_id: &str) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.assigned_to = Some(agent_id.to_string());
            task.status = TaskStatus::InProgress;
            
            let _ = self.event_tx.send(Event::TaskAssigned(
                task_id.to_string(),
                agent_id.to_string(),
            ));
            
            info!("任务 {} 分配给 Agent {}", task_id, agent_id);
            Ok(())
        } else {
            anyhow::bail!("任务不存在：{}", task_id)
        }
    }
    
    /// 获取空闲 Agent
    pub fn get_idle_agent(&self) -> Option<&Agent> {
        self.agents.values().find(|agent| {
            matches!(agent.state, AgentState::Idle)
        })
    }
    
    /// 自动分配任务
    pub fn auto_assign_task(&mut self, task_id: &str) -> Result<()> {
        if let Some(idle_agent) = self.get_idle_agent() {
            let agent_id = idle_agent.config.id.clone();
            self.assign_task(task_id, &agent_id)?;
            Ok(())
        } else {
            anyhow::bail!("没有空闲的 Agent")
        }
    }
    
    /// 完成任务
    pub fn complete_task(&mut self, task_id: &str, result: &str) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.status = TaskStatus::Completed(result.to_string());
            
            if let Some(ref agent_id) = task.assigned_to {
                let _ = self.event_tx.send(Event::TaskCompleted(
                    task_id.to_string(),
                    agent_id.clone(),
                ));
            }
            
            info!("任务 {} 完成", task_id);
            Ok(())
        } else {
            anyhow::bail!("任务不存在：{}", task_id)
        }
    }
    
    /// 订阅事件
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }
    
    /// 列出所有任务
    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    /// 列出所有 Agent
    pub fn list_agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }
}

impl Default for Coordinator {
    fn default() -> Self {
        Self::new()
    }
}
