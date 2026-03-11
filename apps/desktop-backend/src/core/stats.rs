use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UsageStats {
    pub total_requests: u64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_tokens: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub last_request_time: Option<String>,
}

pub struct StatsManager {
    stats_path: PathBuf,
}

impl StatsManager {
    pub fn new(data_dir: &str) -> Result<Self, String> {
        let stats_path = PathBuf::from(data_dir).join("stats.json");
        Ok(Self { stats_path })
    }
    
    pub fn load_stats(&self) -> Result<UsageStats, String> {
        if !self.stats_path.exists() {
            return Ok(UsageStats::default());
        }
        
        let content = fs::read_to_string(&self.stats_path)
            .map_err(|e| format!("Failed to read stats: {}", e))?;
        
        let stats: UsageStats = serde_json::from_str(&content)
            .unwrap_or_default();
        
        Ok(stats)
    }
    
    pub fn save_stats(&self, stats: &UsageStats) -> Result<(), String> {
        let content = serde_json::to_string_pretty(stats)
            .map_err(|e| format!("Failed to serialize stats: {}", e))?;
        
        fs::write(&self.stats_path, content)
            .map_err(|e| format!("Failed to write stats: {}", e))?;
        
        Ok(())
    }
    
    pub fn record_request(&self, input_tokens: u64, output_tokens: u64, success: bool) -> Result<UsageStats, String> {
        let mut stats = self.load_stats()?;
        
        stats.total_requests += 1;
        stats.total_input_tokens += input_tokens;
        stats.total_output_tokens += output_tokens;
        stats.total_tokens += input_tokens + output_tokens;
        stats.last_request_time = Some(chrono::Utc::now().to_rfc3339());
        
        if success {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        
        self.save_stats(&stats)?;
        Ok(stats)
    }
    
    pub fn reset_stats(&self) -> Result<(), String> {
        let stats = UsageStats::default();
        self.save_stats(&stats)
    }
}
