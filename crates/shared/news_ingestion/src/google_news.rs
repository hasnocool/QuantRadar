use anyhow::Result;
use reqwest::Client;
use crate::{NewsItem, NewsSource};
use chrono::{DateTime, Utc};
use regex::Regex;

pub struct GoogleNewsCollector {
    client: Client,
    query: String,
}

impl GoogleNewsCollector {
    pub fn new(query: String) -> Self {
        Self { client: Client::new(), query }
    }

    pub async fn collect(&self) -> Result<Vec<NewsItem>> {
        let q = urlencoding::encode(&self.query);
        let url = format!("https://news.google.com/rss/search?q={}&hl=en-US&gl=US&ceid=US:en", q);
        let resp = self.client.get(&url).header("User-Agent", "QuantRadar/0.1").send().await?;
        if !resp.status().is_success() { return Ok(vec![]); }
        let text = resp.text().await?;
        
        let item_re = Regex::new(r"<item>(?s)(.*?)</item>")?;
        let title_re = Regex::new(r"<title>(?s)(.*?)</title>")?;
        let link_re = Regex::new(r"<link>(?s)(.*?)</link>")?;
        let date_re = Regex::new(r"<pubDate>(?s)(.*?)</pubDate>")?;

        let mut items = Vec::new();
        for caps in item_re.captures_iter(&text) {
            let item_block = caps.get(1).map_or("", |m| m.as_str());
            let title = title_re.captures(item_block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
            let link = link_re.captures(item_block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
            let date_str = date_re.captures(item_block).and_then(|c| c.get(1)).map_or("", |m| m.as_str()).trim().to_string();
            let published = DateTime::parse_from_rfc2822(&date_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            if !title.is_empty() {
                items.push(NewsItem {
                    id: link.clone(),
                    source: NewsSource::GoogleNews,
                    title,
                    url: link,
                    published_at: published,
                    content: None,
                    sentiment_score: 0.0,
                    symbols: vec![],
                });
            }
        }
        Ok(items)
    }
}
