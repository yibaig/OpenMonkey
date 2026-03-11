use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

#[derive(Clone)]
pub struct MemoryBank {
    pool: SqlitePool,
}

impl MemoryBank {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(db_path)?
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                embedding_vector BLOB,
                tags TEXT,
                importance INTEGER DEFAULT 1,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn add_memory(
        &self,
        content: &str,
        tags: Option<&str>,
        importance: i32,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO memories (content, tags, importance) VALUES (?, ?, ?)"
        )
        .bind(content)
        .bind(tags)
        .bind(importance)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn search_memories(
        &self,
        query: &str,
        limit: i32,
    ) -> Result<Vec<Memory>, sqlx::Error> {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, Memory>(
            "SELECT id, content, tags, importance, created_at FROM memories WHERE content LIKE ? ORDER BY importance DESC, created_at DESC LIMIT ?"
        )
        .bind(&pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_all_memories(&self, limit: i32) -> Result<Vec<Memory>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Memory>(
            "SELECT id, content, tags, importance, created_at FROM memories ORDER BY created_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn delete_memory(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM memories WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_importance(&self, id: i64, importance: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE memories SET importance = ? WHERE id = ?")
            .bind(importance)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Memory {
    pub id: i64,
    pub content: String,
    pub tags: Option<String>,
    pub importance: i32,
    pub created_at: String,
}
