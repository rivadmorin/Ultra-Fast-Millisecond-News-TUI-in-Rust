use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionPolicy {
    Hourly,
    Daily,
    Monthly,
    Custom(u64), // In seconds
}

impl RetentionPolicy {
    pub fn as_seconds(&self) -> u64 {
        match self {
            RetentionPolicy::Hourly => 3600,
            RetentionPolicy::Daily => 86400,
            RetentionPolicy::Monthly => 2592000,
            RetentionPolicy::Custom(s) => *s,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub retention: RetentionPolicy,
    pub fetch_interval_seconds: u64,
    pub db_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            retention: RetentionPolicy::Daily,
            fetch_interval_seconds: 60, // Fetch every minute
            db_path: None,
        }
    }
}
