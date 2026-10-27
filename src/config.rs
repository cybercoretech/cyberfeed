use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeName {
    #[default]
    Cyberpunk,
    Dracula,
    Nord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedConfig {
    pub name: String,
    pub url: String,
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

fn default_refresh_interval() -> u64 {
    300
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeName::Cyberpunk,
            refresh_interval_secs: 300,
            feeds: vec![
                FeedConfig {
                    name: "Rust Blog".into(),
                    url: "https://blog.rust-lang.org/feed.xml".into(),
                },
                FeedConfig {
                    name: "Hacker News".into(),
                    url: "https://news.ycombinator.com/rss".into(),
                },
            ],
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("cyberfeed");
    fs::create_dir_all(&path).ok();
    path.push("config.yaml");
    path
}

pub fn load_config() -> Result<Config> {
    let path = get_config_path();
    if !path.exists() {
        let default_config = Config::default();
        let yaml = serde_yaml::to_string(&default_config)?;
        fs::write(&path, yaml)?;
        return Ok(default_config);
    }
    let content = fs::read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
