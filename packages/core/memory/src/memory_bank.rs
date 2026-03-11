//! 记忆银行 - 四层记忆系统

use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::models::Memory;

/// 记忆银行 - 管理四层记忆
pub struct MemoryBank {
    pool: SqlitePool,
}

impl MemoryBank {
    /// 创建新的记忆银行
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // 初始化数据库表
        Self::init_tables(&pool).await?;
        
        Ok(Self { pool })
    }
    
    /// 初始化数据库表
    async fn init_tables(pool: &SqlitePool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                memory_type TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER,
                metadata TEXT
            )
            "#
        )
        .execute(pool)
        .await?;
        
        info!("记忆银行数据库表初始化完成");
        Ok(())
    }
    
    /// 添加记忆
    pub async fn add(&self, memory: Memory) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO memories (id, content, memory_type, created_at, metadata)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&memory.id)
        .bind(&memory.content)
        .bind(&memory.memory_type)
        .bind(memory.created_at)
        .bind(serde_json::to_string(&memory.metadata)?)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// 获取记忆
    pub async fn get(&self, id: &str) -> Result<Option<Memory>> {
        let memory = sqlx::query_as::<_, Memory>(
            r#"
            SELECT id, content, memory_type, created_at, updated_at, metadata
            FROM memories
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(memory)
    }
    
    /// 删除记忆
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM memories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// 按类型查询记忆
    pub async fn query_by_type(&self, memory_type: &str, limit: i32) -> Result<Vec<Memory>> {
        let memories = sqlx::query_as::<_, Memory>(
            r#"
            SELECT id, content, memory_type, created_at, updated_at, metadata
            FROM memories
            WHERE memory_type = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#
        )
        .bind(memory_type)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(memories)
    }
}
