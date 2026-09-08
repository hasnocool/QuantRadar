use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::{NewsItem, NewsSource};

#[derive(Debug, Deserialize)]
struct RedditPost {
    id: String,
    title: String,
    url: String,
    selftext: Option<String>,
    created_utc: f64,
}

#[derive(Debug, Deserialize)]
struct RedditChildren {
    data: RedditPost,
}

#[derive(Debug, Deserialize)]
struct RedditListing {
    data: RedditChildrenData,
}

#[derive(Debug, Deserialize)]
struct RedditChildrenData {
    children: Vec<RedditChildren>,
}

pub struct RedditJsonCollector {
    client: Client,
    subreddits: Vec<String>,
}

impl RedditJsonCollector {
    pub fn new(subreddits: Vec<String>) -> Self {
        Self { client: Client::new(), subreddits }
    }

    pub async fn collect(&self) -> Result<Vec<NewsItem>> {
        let mut items = Vec::new();
        for sub in &self.subreddits {
            let url = format!("https://www.reddit.com/r/{}/new.json?limit=100", sub);
            let resp = self.client.get(&url).header("User-Agent", "QuantRadar/0.1").send().await?;
            if !resp.status().is_success() { continue; }
            let listing: serde_json::Value = resp.json().await?;
            if let Some(children) = listing.get("data").and_then(|d| d.get("children")) {
                if let Some(arr) = children.as_array() {
                    for child in arr {
                        if let Ok(post) = serde_json::from_value::<RedditPost>(child.get("data").cloned().unwrap_or_default()) {
                            let published = DateTime::<Utc>::from_timestamp(post.created_utc as i64, 0).unwrap_or_else(Utc::now);
                            items.push(NewsItem {
                                id: post.id,
                                source: NewsSource::RedditJson,
                                title: post.title,
                                url: post.url,
                                published_at: published,
                                content: post.selftext,
                                sentiment_score: 0.0,
                                symbols: vec![],
                            });
                        }
                    }
                }
            }
        }
        Ok(items)
    }
}
