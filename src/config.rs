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

        default_config()
    }
}

fn default_config() -> Config {
    Config {
        theme: ThemeName::Cyberpunk,
        refresh_interval_secs: 300,
        feeds: vec![
            FeedConfig {
                name: "Rust Blog".to_string(),
                url: concat!("https://", "blog.rust-lang.org", "/feed.xml").to_string(),
            },
            FeedConfig {
                name: "Hacker News".to_string(),
                url: concat!("https://", "news.ycombinator.com", "/rss").to_string(),
            },
        ],
    }
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    Ok(Config::load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_feed_urls_are_plain_https_urls() {
        let config = default_config();
        assert_eq!(config.feeds.len(), 2);
        assert_eq!(
            config.feeds[0].url,
            concat!("https://", "blog.rust-lang.org", "/feed.xml")
        );
        assert_eq!(
            config.feeds[1].url,
            concat!("https://", "news.ycombinator.com", "/rss")
        );
        for feed in &config.feeds {
            assert!(feed.url.starts_with("https://"));
            assert!(!feed.url.contains('[') && !feed.url.contains(']'));
        }
    }

    #[test]
    fn omitted_yaml_fields_use_documented_defaults() {
        let config: Config = serde_yaml::from_str("{}").unwrap();
        assert_eq!(config.theme, ThemeName::Cyberpunk);
        assert_eq!(config.refresh_interval_secs, 300);
        assert!(config.feeds.is_empty());
    }

    #[test]
    fn deserializes_supported_themes() {
        for (yaml, expected) in [
            ("theme: cyberpunk", ThemeName::Cyberpunk),
            ("theme: dracula", ThemeName::Dracula),
            ("theme: nord", ThemeName::Nord),
        ] {
            let config: Config = serde_yaml::from_str(yaml).unwrap();
            assert_eq!(config.theme, expected);
        }
    }

    #[test]
    fn rejects_unknown_theme_names() {
        let result = serde_yaml::from_str::<Config>("theme: unknown");
        assert!(result.is_err());
    }
}
