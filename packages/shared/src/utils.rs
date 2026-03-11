//! 通用工具函数

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 生成唯一 ID
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// 获取当前时间戳（毫秒）
pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

/// 格式化时间
pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 计算字符串的哈希
pub fn hash_string(s: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert_eq!(id.len(), 36); // UUID 格式
    }

    #[test]
    fn test_now_ms() {
        let now = now_ms();
        assert!(now > 0);
    }

    #[test]
    fn test_hash_string() {
        let hash = hash_string("test");
        assert_eq!(hash.len(), 64); // SHA256 十六进制
    }
}
