//! 技能数据模型

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

/// 技能定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// 唯一标识
    pub id: String,
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: String,
    /// 技能版本
    pub version: String,
    /// 作者
    pub author: Option<String>,
    /// 是否是 WASM 技能
    pub is_wasm: bool,
    /// WASM 字节码（如果是 WASM 技能）
    #[serde(skip)]
    pub wasm_bytes: Vec<u8>,
    /// 工具定义
    pub tools: Vec<ToolDefinition>,
    /// 技能内容（Markdown）
    pub content: String,
}

impl Skill {
    /// 从 Markdown 解析技能
    pub fn from_markdown(content: &str) -> Result<Self> {
        // 简化的解析逻辑
        let mut name = String::new();
        let mut description = String::new();
        let mut version = "1.0.0".to_string();
        let mut author = None;
        
        for line in content.lines().take(20) {
            let line = line.trim();
            
            if line.starts_with("#") && name.is_empty() {
                name = line.trim_start_matches('#').trim().to_string();
            } else if line.starts_with(">") && description.is_empty() {
                description = line.trim_start_matches('>').trim().to_string();
            } else if line.starts_with("- **Version:**") {
                version = line.replace("- **Version:**", "").trim().to_string();
            } else if line.starts_with("- **Author:**") {
                author = Some(line.replace("- **Author:**", "").trim().to_string());
            }
        }
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: if name.is_empty() { "Unknown Skill".to_string() } else { name },
            description,
            version,
            author,
            is_wasm: false,
            wasm_bytes: Vec::new(),
            tools: Vec::new(),
            content: content.to_string(),
        })
    }
    
    /// 转换为 Markdown
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("# {}\n\n", self.name));
        md.push_str(&format!("> {}\n\n", self.description));
        md.push_str(&format!("- **Version:** {}\n", self.version));
        
        if let Some(ref author) = self.author {
            md.push_str(&format!("- **Author:** {}\n", author));
        }
        
        md.push_str("\n---\n\n");
        md.push_str(&self.content);
        
        md
    }
}

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 参数定义
    pub parameters: Vec<ParameterDefinition>,
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 是否必需
    pub required: bool,
    /// 描述
    pub description: Option<String>,
}
