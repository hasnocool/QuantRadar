use anyhow::Result;
use reqwest::Client;
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::{NewsItem, NewsSource};

pub struct RedditRssCollector {
    client: Client,
    subreddits: Vec<String>,
}

impl RedditRssCollector {
    pub fn new(subreddits: Vec<String>) -> Self {
        Self { client: Client::new(), subreddits }
    }

    pub async fn collect(&self) -> Result<Vec<NewsItem>> {
        let mut items = Vec::new();
        let item_re = Regex::new(r"<item>(?s)(.*?)</item>")?;
        let title_re = Regex::new(r"<title>(?s)<\!\[CDATA\[(.*?)\]\]>|(?s)<title>(.*?)</title>")?;
        let link_re = Regex::new(r"<link>(?s)(.*?)</link>")?;
        let guid_re = Regex::new(r"<guid>(?s)(.*?)</guid>")?;
        let date_re = Regex::new(r"<pubDate>(?s)(.*?)</pubDate>")?;
        let desc_re = Regex::new(r"<description>(?s)<\!\[CDATA\[(.*?)\]\]>|(?s)<description>(.*?)</description>")?;
        for sub in &self.subreddits {
            let url = format!("https://www.reddit.com/r/{}/.rss", sub);
            let resp = self.client.get(&url).header("User-Agent", "QuantRadar/0.1").send().await?;
            if !resp.status().is_success() { continue; }
            let text = resp.text().await?;
            for caps in item_re.captures_iter(&text) {
                let block = caps.get(1).map_or("", |m| m.as_str());
                let title = title_re.captures(block).and_then(|c| c.get(1).or_else(|| c.get(2))).map_or("", |m| m.as_str()).trim().to_string();
                let link = link_re.captures(block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
                let guid = guid_re.captures(block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
                let date_str = date_re.captures(block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
                let desc = desc_re.captures(block).and_then(|c| c.get(1).or_else(|| c.get(2))).map_or("", |m| m.as_str()).trim().to_string();
                if title.is_empty() { continue; }
                let published = DateTime::parse_from_rfc2822(&date_str).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now);
                items.push(NewsItem {
                    id: if guid.is_empty() { link.clone() } else { guid },
                    source: NewsSource::RedditRss,
                    title,
                    url: link,
                    published_at: published,
                    content: if desc.is_empty() { None } else { Some(desc) },
                    sentiment_score: 0.0,
                    symbols: vec![],
                });
            }
        }
        Ok(items)
    }
}
