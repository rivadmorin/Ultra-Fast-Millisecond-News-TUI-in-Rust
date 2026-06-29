use serde::{Deserialize, Serialize};
use std::fs;

use directories::ProjectDirs;

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
            fetch_interval_active_seconds: 60,
            fetch_interval_idle_seconds: 300,
            active_hours_start: 6,
            active_hours_end: 22,
            worker_threads: 4,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("com", "LiveNewsTUI", "LiveNews") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.toml");

            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            } else {
                // Try to create default config file
                let _ = fs::create_dir_all(config_dir);
                let config = Config::default();
                if let Ok(toml) = toml::to_string_pretty(&config) {
                    let _ = fs::write(config_path, toml);
                }
            }
        }
        Config::default()
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
