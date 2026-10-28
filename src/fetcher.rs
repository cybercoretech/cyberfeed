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

        for article in parse_feed(&bytes) {
            let _ = db.insert_article(&article.title, &feed.name, &article.content, &article.link);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsedArticle {
    title: String,
    content: String,
    link: String,
}

fn parse_feed(bytes: &[u8]) -> Vec<ParsedArticle> {
    if let Ok(channel) = rss::Channel::read_from(bytes) {
        return channel
            .items()
            .iter()
            .filter_map(|item| {
                let link = item.link().unwrap_or("");
                if link.is_empty() {
                    return None;
                }
                Some(ParsedArticle {
                    title: item.title().unwrap_or("No Title").to_string(),
                    content: item
                        .description()
                        .or(item.content())
                        .unwrap_or("")
                        .to_string(),
                    link: link.to_string(),
                })
            })
            .collect();
    }

    if let Ok(feed) = atom_syndication::Feed::read_from(bytes) {
        return feed
            .entries()
            .iter()
            .filter_map(|entry| {
                let link = entry.links().first().map(|l| l.href.as_str()).unwrap_or("");
                if link.is_empty() {
                    return None;
                }
                Some(ParsedArticle {
                    title: entry.title().to_string(),
                    content: entry
                        .content()
                        .and_then(|c| c.value.as_deref())
                        .or_else(|| entry.summary().map(|s| s.as_str()))
                        .unwrap_or("")
                        .to_string(),
                    link: link.to_string(),
                })
            })
            .collect();
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rss_title_link_and_description() {
        let xml = br#"<?xml version="1.0"?>
            <rss version="2.0"><channel><title>Example</title>
            <link>https://example.com</link><description>Test feed</description>
            <item><title>RSS title</title><link>https://example.com/1</link>
            <description>RSS description</description></item>
            </channel></rss>"#;

        assert_eq!(
            parse_feed(xml),
            vec![ParsedArticle {
                title: "RSS title".into(),
                content: "RSS description".into(),
                link: "https://example.com/1".into(),
            }]
        );
    }

    #[test]
    fn parses_atom_title_link_and_summary() {
        let xml = br#"<?xml version="1.0"?>
            <feed xmlns="http://www.w3.org/2005/Atom">
              <title>Example</title><id>https://example.com</id>
              <updated>2026-01-01T00:00:00Z</updated>
              <entry><title>Atom title</title><id>urn:entry:1</id>
                <updated>2026-01-01T00:00:00Z</updated>
                <link href="https://example.com/atom-1"/>
                <summary>Atom summary</summary>
              </entry>
            </feed>"#;

        assert_eq!(
            parse_feed(xml),
            vec![ParsedArticle {
                title: "Atom title".into(),
                content: "Atom summary".into(),
                link: "https://example.com/atom-1".into(),
            }]
        );
    }

    #[test]
    fn ignores_feed_items_without_links() {
        let xml = br#"<rss version="2.0"><channel><title>Example</title>
            <link>https://example.com</link><description>Test</description>
            <item><title>No link</title></item>
            </channel></rss>"#;
        assert!(parse_feed(xml).is_empty());
    }

    #[test]
    fn malformed_xml_returns_no_articles() {
        assert!(parse_feed(b"<not valid xml").is_empty());
    }
}
