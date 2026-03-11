//! 技能适配器 - 加载和管理技能

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

use crate::models::Skill;
use crate::wasm_executor::WasmExecutor;

/// 技能适配器
pub struct SkillAdapter {
    skills: HashMap<String, Skill>,
    wasm_executor: Option<WasmExecutor>,
    skills_dir: PathBuf,
}

impl SkillAdapter {
    /// 创建新的技能适配器
    pub fn new(skills_dir: &Path) -> Self {
        Self {
            skills: HashMap::new(),
            wasm_executor: None,
            skills_dir: skills_dir.to_path_buf(),
        }
    }
    
    /// 初始化 WASM 执行器
    pub fn init_wasm(&mut self) -> Result<()> {
        self.wasm_executor = Some(WasmExecutor::new()?);
        info!("WASM 执行器初始化完成");
        Ok(())
    }
    
    /// 执行 WASM 技能
    pub async fn execute_wasm_skill(&mut self, _skill_id: &str, wasm_bytes: &[u8]) -> Result<String> {
        if let Some(ref executor) = self.wasm_executor {
            executor.execute(wasm_bytes).await
        } else {
            anyhow::bail!("WASM 执行器未初始化")
        }
    }
    
    /// 加载所有技能
    pub async fn load_all(&mut self) -> Result<()> {
        if !self.skills_dir.exists() {
            info!("技能目录不存在，跳过加载");
            return Ok(());
        }
        
        let mut entries = fs::read_dir(&self.skills_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match self.load_skill(&path).await {
                    Ok(skill) => {
                        info!("加载技能：{}", skill.name);
                        self.skills.insert(skill.id.clone(), skill);
                    }
                    Err(e) => {
                        info!("加载技能失败 {:?}: {}", path, e);
                    }
                }
            }
        }
        
        info!("技能加载完成，共 {} 个技能", self.skills.len());
        Ok(())
    }
    
    /// 加载单个技能
    pub async fn load_skill(&self, path: &Path) -> Result<Skill> {
        let content = fs::read_to_string(path).await?;
        Skill::from_markdown(&content)
    }
    
    /// 获取技能
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }
    
    /// 执行技能
    pub async fn execute_skill(&mut self, skill_id: &str, input: &str) -> Result<String> {
        if let Some(skill) = self.skills.get(skill_id) {
            // 如果是 WASM 技能
            if skill.is_wasm {
                if let Some(ref executor) = self.wasm_executor {
                    return executor.execute(&skill.wasm_bytes).await;
                }
            }
            
            // 普通技能（返回技能描述）
            Ok(format!("执行技能：{}\n输入：{}", skill.name, input))
        } else {
            anyhow::bail!("技能不存在：{}", skill_id)
        }
    }
    
    /// 注册技能
    pub fn register_skill(&mut self, skill: Skill) {
        info!("注册新技能：{}", skill.name);
        self.skills.insert(skill.id.clone(), skill);
    }
    
    /// 卸载技能
    pub fn unregister_skill(&mut self, skill_id: &str) -> Option<Skill> {
        info!("卸载技能：{}", skill_id);
        self.skills.remove(skill_id)
    }
    
    /// 列出所有技能
    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
}
