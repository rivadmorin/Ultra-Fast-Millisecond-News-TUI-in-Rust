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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Black,
    White,
    DeepBlue,
    Matrix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub retention: RetentionPolicy,
    pub fetch_interval_active_seconds: u64,
    pub fetch_interval_idle_seconds: u64,
    pub active_hours_start: u32,
    pub active_hours_end: u32,
    pub worker_threads: usize,
    pub theme: Theme,
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
            theme: Theme::Black,
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
}
