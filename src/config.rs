use serde::{Deserialize, Serialize};

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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_policy_as_seconds() {
        assert_eq!(RetentionPolicy::Hourly.as_seconds(), 3600);
        assert_eq!(RetentionPolicy::Daily.as_seconds(), 86400);
        assert_eq!(RetentionPolicy::Weekly.as_seconds(), 604800);
        assert_eq!(RetentionPolicy::Monthly.as_seconds(), 2592000);
        assert_eq!(RetentionPolicy::Custom(0).as_seconds(), 0);
        assert_eq!(RetentionPolicy::Custom(12345).as_seconds(), 12345);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        match config.retention {
            RetentionPolicy::Daily => (),
            _ => panic!("Default retention should be Daily"),
        }
        assert_eq!(config.fetch_interval_active_seconds, 60);
        assert_eq!(config.fetch_interval_idle_seconds, 300);
        assert_eq!(config.active_hours_start, 6);
        assert_eq!(config.active_hours_end, 22);
        assert_eq!(config.worker_threads, 4);
    }
}
