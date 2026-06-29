use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionPolicy {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Custom(u64), // In seconds
}

impl RetentionPolicy {
    pub fn as_seconds(&self) -> u64 {
        match self {
            RetentionPolicy::Hourly => 3600,
            RetentionPolicy::Daily => 86400,
            RetentionPolicy::Weekly => 604800,
            RetentionPolicy::Monthly => 2592000,
            RetentionPolicy::Custom(s) => *s,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub retention: RetentionPolicy,
    pub fetch_interval_active_seconds: u64,
    pub fetch_interval_idle_seconds: u64,
    pub active_hours_start: u32, // 0-23
    pub active_hours_end: u32,   // 0-23
    pub worker_threads: usize,
    pub db_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            retention: RetentionPolicy::Daily,
            fetch_interval_active_seconds: 60, // 1 minute during active hours
            fetch_interval_idle_seconds: 300,  // 5 minutes during idle hours
            active_hours_start: 6,             // 6 AM
            active_hours_end: 22,              // 10 PM
            worker_threads: 4,                 // Max concurrent fetches
            db_path: None,
        }
    }
}
