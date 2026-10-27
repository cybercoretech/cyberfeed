use crate::config::Config;
use crate::db::Database;
use reqwest::Client;
use std::time::Duration;

pub async fn fetch_all_feeds(config: &Config, db: &Database) {
    let client = match Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .timeout(Duration::from_secs(6))
        .build()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    for feed in &config.feeds {
        let resp = match client.get(&feed.url).send().await {
            Ok(r) => r,
            Err(_) => continue,
        };

        let bytes = match resp.bytes().await {
            Ok(b) => b,
            Err(_) => continue,
        };

        // Try RSS 2.0 Channel
        if let Ok(channel) = rss::Channel::read_from(&bytes[..]) {
            for item in channel.items() {
                let title = item.title().unwrap_or("No Title");
                let link = item.link().unwrap_or("");
                let content = item.description().unwrap_or(item.content().unwrap_or(""));
                if !link.is_empty() {
                    let _ = db.insert_article(title, &feed.name, content, link);
                }
            }
        }
        // Try Atom Feed
        else if let Ok(atom_feed) = atom_syndication::Feed::read_from(&bytes[..]) {
            for entry in atom_feed.entries() {
                let title = entry.title();
                let link = entry.links().first().map(|l| l.href.as_str()).unwrap_or("");
                let content = entry
                    .content()
                    .and_then(|c| c.value.as_deref())
                    .or_else(|| entry.summary().map(|s| s.as_str()))
                    .unwrap_or("");
                if !link.is_empty() {
                    let _ = db.insert_article(title, &feed.name, content, link);
                }
            }
        }
    }
}
