//! 记忆数据模型

use serde::{Deserialize, Serialize};

/// 记忆类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// 短期记忆 - 最近对话
    ShortTerm,
    /// 长期记忆 - 重要信息
    LongTerm,
    /// 程序记忆 - 技能和知识
    Procedural,
    /// 情感记忆 - 情感体验
    Emotional,
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::ShortTerm => write!(f, "short_term"),
            MemoryType::LongTerm => write!(f, "long_term"),
            MemoryType::Procedural => write!(f, "procedural"),
            MemoryType::Emotional => write!(f, "emotional"),
        }
    }
}

/// 记忆元数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryMetadata {
    /// 来源（用户输入、AI 回复、系统事件等）
    pub source: Option<String>,
    /// 重要性评分（0-1）
    pub importance: Option<f32>,
    /// 情感极性（-1 到 1）
    pub sentiment: Option<f32>,
    /// 标签
    pub tags: Option<Vec<String>>,
    /// 关联的记忆 ID
    pub related_ids: Option<Vec<String>>,
    /// 向量嵌入（可选，用于检索）
    pub embedding: Option<Vec<f32>>,
}

/// 记忆结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 唯一标识
    pub id: String,
    /// 记忆内容
    pub content: String,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 创建时间戳（毫秒）
    pub created_at: i64,
    /// 更新时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// 元数据
    #[serde(default)]
    pub metadata: MemoryMetadata,
}

impl Memory {
    /// 创建新记忆
    pub fn new(
        content: String,
        memory_type: MemoryType,
        metadata: Option<MemoryMetadata>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            memory_type,
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: None,
            metadata: metadata.unwrap_or_default(),
        }
    }
    
    /// 设置重要性评分
    pub fn with_importance(mut self, importance: f32) -> Self {
        self.metadata.importance = Some(importance.clamp(0.0, 1.0));
        self
    }
    
    /// 添加标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = Some(tags);
        self
    }
}

/// SQLx 行转换实现
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Memory {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let id: String = row.try_get("id")?;
        let content: String = row.try_get("content")?;
        let memory_type_str: String = row.try_get("memory_type")?;
        let created_at: i64 = row.try_get("created_at")?;
        let updated_at: Option<i64> = row.try_get("updated_at")?;
        let metadata_json: String = row.try_get("metadata")?;
        
        let memory_type = match memory_type_str.as_str() {
            "short_term" => MemoryType::ShortTerm,
            "long_term" => MemoryType::LongTerm,
            "procedural" => MemoryType::Procedural,
            "emotional" => MemoryType::Emotional,
            _ => return Err(sqlx::Error::Decode("Invalid memory type".into())),
        };
        
        let metadata: MemoryMetadata = serde_json::from_str(&metadata_json)
            .map_err(|e| sqlx::Error::Decode(e.to_string().into()))
            .unwrap_or_default();
        
        Ok(Self {
            id,
            content,
            memory_type,
            created_at,
            updated_at,
            metadata,
        })
    }
}
