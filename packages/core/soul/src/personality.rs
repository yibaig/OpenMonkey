//! 人格系统 v1 - 人格进化和管理

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// 人格特质
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    /// 特质名称
    pub name: String,
    /// 强度（0-100）
    pub strength: u8,
    /// 描述
    pub description: Option<String>,
}

/// 人格状态
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalityState {
    /// 情绪状态
    pub mood: String,
    /// 能量水平（0-100）
    pub energy: u8,
    /// 专注度（0-100）
    pub focus: u8,
}

/// 人格配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    /// 人格 ID
    pub id: String,
    /// 人格名称
    pub name: String,
    /// 人格特质
    pub traits: Vec<PersonalityTrait>,
    /// 当前状态
    pub state: PersonalityState,
    /// 进化日志
    pub evolution_log: Vec<EvolutionEntry>,
}

/// 进化记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEntry {
    /// 时间戳
    pub timestamp: i64,
    /// 变化描述
    pub change: String,
    /// 原因
    pub reason: String,
}

impl Personality {
    /// 创建新人格
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            traits: Vec::new(),
            state: PersonalityState::default(),
            evolution_log: Vec::new(),
        }
    }
    
    /// 添加特质
    pub fn add_trait(&mut self, name: &str, strength: u8, description: Option<&str>) {
        self.traits.push(PersonalityTrait {
            name: name.to_string(),
            strength: strength.min(100),
            description: description.map(String::from),
        });
        info!("人格 '{}' 添加特质：{} (强度：{})", self.name, name, strength);
    }
    
    /// 更新特质强度
    pub fn update_trait(&mut self, name: &str, delta: i8) -> Result<()> {
        if let Some(trait_) = self.traits.iter_mut().find(|t| t.name == name) {
            let new_strength = (trait_.strength as i16 + delta as i16).clamp(0, 100) as u8;
            let old_strength = trait_.strength;
            trait_.strength = new_strength;
            
            // 记录进化
            if old_strength != new_strength {
                self.log_evolution(
                    format!("特质 '{}' 强度变化", name),
                    format!("{} -> {}", old_strength, new_strength),
                );
            }
            
            info!("人格 '{}' 特质 '{}' 强度更新：{} -> {}", self.name, name, old_strength, new_strength);
        }
        Ok(())
    }
    
    /// 记录进化
    fn log_evolution(&mut self, change: String, reason: String) {
        self.evolution_log.push(EvolutionEntry {
            timestamp: chrono::Utc::now().timestamp_millis(),
            change,
            reason,
        });
    }
    
    /// 设置情绪
    pub fn set_mood(&mut self, mood: &str) {
        self.state.mood = mood.to_string();
        info!("人格 '{}' 情绪更新：{}", self.name, mood);
    }
    
    /// 获取特质
    pub fn get_trait(&self, name: &str) -> Option<&PersonalityTrait> {
        self.traits.iter().find(|t| t.name == name)
    }
    
    /// 列出所有特质
    pub fn list_traits(&self) -> Vec<&PersonalityTrait> {
        self.traits.iter().collect()
    }
}

/// 人格管理器
pub struct PersonalityManager {
    personalities: HashMap<String, Personality>,
    active_personality: Option<String>,
}

impl PersonalityManager {
    /// 创建新人格管理器
    pub fn new() -> Self {
        Self {
            personalities: HashMap::new(),
            active_personality: None,
        }
    }
    
    /// 创建人格
    pub fn create_personality(&mut self, name: &str) -> &Personality {
        let personality = Personality::new(name);
        let id = personality.id.clone();
        self.personalities.insert(id.clone(), personality);
        self.personalities.get(&id).unwrap()
    }
    
    /// 激活人格
    pub fn activate(&mut self, personality_id: &str) -> Result<()> {
        if self.personalities.contains_key(personality_id) {
            self.active_personality = Some(personality_id.to_string());
            info!("激活人格：{}", personality_id);
            Ok(())
        } else {
            anyhow::bail!("人格不存在：{}", personality_id)
        }
    }
    
    /// 获取当前人格
    pub fn get_active(&self) -> Option<&Personality> {
        self.active_personality
            .as_ref()
            .and_then(|id| self.personalities.get(id))
    }
    
    /// 获取可变当前人格
    pub fn get_active_mut(&mut self) -> Option<&mut Personality> {
        self.active_personality
            .as_ref()
            .and_then(|id| self.personalities.get_mut(id))
    }
    
    /// 列出所有人格
    pub fn list_personalities(&self) -> Vec<&Personality> {
        self.personalities.values().collect()
    }
}

impl Default for PersonalityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_personality() {
        let mut manager = PersonalityManager::new();
        let personality = manager.create_personality("旺财");
        assert_eq!(personality.name, "旺财");
    }

    #[test]
    fn test_add_trait() {
        let mut personality = Personality::new("测试");
        personality.add_trait("聪明", 80, Some("非常聪明"));
        assert_eq!(personality.traits.len(), 1);
        assert_eq!(personality.traits[0].strength, 80);
    }
}
