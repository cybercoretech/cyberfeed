use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeName {
    #[default]
    Cyberpunk,
    Dracula,
    Nord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub theme: ThemeName,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_secs: u64,
    #[serde(default)]
    pub feeds: Vec<FeedConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedConfig {
    pub name: String,
    pub url: String,
}

fn default_refresh_interval() -> u64 {
    300
}

impl Config {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("cyberfeed"))
            .unwrap_or_else(|| PathBuf::from("."));
        
        let config_path = config_dir.join("config.yaml");

        if config_path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = serde_yaml::from_str(&contents) {
                    return config;
                }
            }
        }

        Self {
            theme: ThemeName::Cyberpunk,
            refresh_interval_secs: 300,
            feeds: vec![
                FeedConfig {
                    name: "Rust Blog".to_string(),
                    url: "https://blog.rust-lang.org/feed.xml".to_string(),
                },
                FeedConfig {
                    name: "Hacker News".to_string(),
                    url: "https://news.ycombinator.com/rss".to_string(),
                },
            ],
        }
    }
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    Ok(Config::load())
}
