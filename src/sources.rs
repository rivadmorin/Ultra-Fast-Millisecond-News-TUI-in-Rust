use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedSource {
    pub url: String,
    pub category: String,
    pub source_name: String,
}

#[derive(Debug, Deserialize)]
struct SourcesConfig {
    sources: Vec<FeedSource>,
}

pub fn get_sources() -> Vec<FeedSource> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "LiveNewsTUI", "LiveNews") {
        let config_dir = proj_dirs.config_dir();
        let sources_path = config_dir.join("sources.toml");

        if let Ok(content) = fs::read_to_string(&sources_path) {
            match toml::from_str::<SourcesConfig>(&content) {
                Ok(config) => return config.sources,
                Err(e) => log::error!("Failed to parse sources.toml: {}", e),
            }
        }

        // If it doesn't exist in config dir, check current directory
        let local_path = PathBuf::from("sources.toml");
        if let Ok(content) = fs::read_to_string(&local_path) {
            match toml::from_str::<SourcesConfig>(&content) {
                Ok(config) => {
                    // Also copy it to the config dir for future use
                    let _ = fs::create_dir_all(config_dir);
                    let _ = fs::write(sources_path, content);
                    return config.sources;
                }
                Err(e) => log::error!("Failed to parse local sources.toml: {}", e),
            }
        }
    }

    // Fallback to a few default sources if everything fails
    vec![
        FeedSource {
            url: "https://news.ycombinator.com/rss".to_string(),
            category: "Tech".to_string(),
            source_name: "Hacker News".to_string(),
        },
        FeedSource {
            url: "http://feeds.bbci.co.uk/news/rss.xml".to_string(),
            category: "World".to_string(),
            source_name: "BBC News".to_string(),
        },
    ]
}

pub fn get_categories() -> Vec<&'static str> {
    // These could also be derived from sources, but keeping them static for UI order consistency
    vec![
        "All",
        "Indonesia",
        "World",
        "Finance",
        "Tech",
        "AI",
        "Crypto",
        "Science",
        "Health",
        "Lifestyle",
        "Sports",
        "Entertainment",
        "Gaming",
        "Auto",
        "Legal",
    ]
}
