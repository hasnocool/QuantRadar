use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::{NewsItem, NewsSource};

#[derive(Debug, Deserialize)]
struct RssItem {
    title: String,
    link: String,
    guid: Option<String>,
    pubDate: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RssChannel {
    item: Vec<RssItem>,
}

#[derive(Debug, Deserialize)]
struct RssRoot {
    channel: RssChannel,
}

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
        for sub in &self.subreddits {
            let url = format!("https://www.reddit.com/r/{}/.rss", sub);
            let resp = self.client.get(&url).header("User-Agent", "QuantRadar/0.1").send().await?;
            if !resp.status().is_success() { continue; }
            let rss: RssRoot = resp.json().await?;
            for it in rss.channel.item {
                let published = it.pubDate
                    .and_then(|s| DateTime::parse_from_rfc2822(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                items.push(NewsItem {
                    id: it.guid.unwrap_or_else(|| it.link.clone()),
                    source: NewsSource::RedditRss,
                    title: it.title,
                    url: it.link,
                    published_at: published,
                    content: it.description,
                    sentiment_score: 0.0,
                    symbols: vec![],
                });
            }
        }
        Ok(items)
    }
}
