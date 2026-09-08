use anyhow::Result;
use reqwest::Client;
use crate::{NewsItem, NewsSource};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NewsApiArticle {
    title: String,
    url: String,
    publishedAt: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NewsApiResponse {
    articles: Vec<NewsApiArticle>,
}

pub struct NewsApiCollector {
    client: Client,
    api_key: String,
    query: String,
}

impl NewsApiCollector {
    pub fn new(api_key: String, query: String) -> Self {
        Self { client: Client::new(), api_key, query }
    }

    pub async fn collect(&self) -> Result<Vec<NewsItem>> {
        let url = format!(
            "https://newsapi.org/v2/everything?q={}&apiKey={}&language=en&sortBy=publishedAt&pageSize=100",
            urlencoding::encode(&self.query),
            self.api_key
        );
        let resp = self.client.get(&url).header("User-Agent", "QuantRadar/0.1").send().await?;
        if !resp.status().is_success() { return Ok(vec![]); }
        let data: NewsApiResponse = resp.json().await?;
        let mut items = Vec::new();
        for a in data.articles {
            let published = DateTime::parse_from_rfc3339(&a.publishedAt)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            items.push(NewsItem {
                id: a.url.clone(),
                source: NewsSource::OfficialFeed,
                title: a.title,
                url: a.url,
                published_at: published,
                content: a.description,
                sentiment_score: 0.0,
                symbols: vec![],
            });
        }
        Ok(items)
    }
}
