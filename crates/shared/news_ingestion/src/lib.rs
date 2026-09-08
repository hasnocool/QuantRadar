use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String,
    pub source: NewsSource,
    pub title: String,
    pub url: String,
    pub published_at: DateTime<Utc>,
    pub content: Option<String>,
    pub sentiment_score: f64,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NewsSource {
    RedditRss,
    RedditJson,
    GoogleNews,
    OfficialFeed,
    OnChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnChainEvent {
    pub tx_hash: String,
    pub block_number: u64,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub address: String,
    pub value: Option<f64>,
    pub raw: serde_json::Value,
}

pub mod reddit_rss;
pub mod reddit_json;
pub mod google_news;
pub mod news_api;
pub mod onchain;
pub mod storage;
pub mod alerting;

pub struct IngestionConfig {
    pub reddit_subreddits: Vec<String>,
    pub google_query: String,
    pub news_api_key: Option<String>,
    pub news_api_query: String,
    pub rpc_url: String,
    pub storage_path: String,
}

impl Default for IngestionConfig {
    fn default() -> Self {
        Self {
            reddit_subreddits: vec!["CryptoCurrency".into(), "Bitcoin".into()],
            google_query: "cryptocurrency market".into(),
            news_api_key: None,
            news_api_query: "cryptocurrency".into(),
            rpc_url: "http://localhost:8545".into(),
            storage_path: "data/news".into(),
        }
    }
}

pub async fn run(config: IngestionConfig) -> anyhow::Result<()> {
    use crate::{reddit_rss::RedditRssCollector, reddit_json::RedditJsonCollector, google_news::GoogleNewsCollector, news_api::NewsApiCollector, onchain::OnChainRpcClient, storage::NewsStorage, alerting::{Alerter, AlertConfig}};
    
    println!("Starting news ingestion pipeline");
    
    let rss_collector = RedditRssCollector::new(config.reddit_subreddits.clone());
    let rss_items = rss_collector.collect().await?;
    println!("Reddit RSS collected {} items", rss_items.len());
    
    let json_collector = RedditJsonCollector::new(config.reddit_subreddits.clone());
    let json_items = json_collector.collect().await?;
    println!("Reddit JSON collected {} items", json_items.len());
    
    let google_collector = GoogleNewsCollector::new(config.google_query.clone());
    let google_items = google_collector.collect().await?;
    println!("Google News collected {} items", google_items.len());
    
    let mut api_items = Vec::new();
    if let Some(key) = config.news_api_key {
        let api_collector = NewsApiCollector::new(key, config.news_api_query.clone());
        api_items = api_collector.collect().await?;
        println!("NewsAPI collected {} items", api_items.len());
    } else {
        println!("NewsAPI skipped: no key");
    }
    
    let rpc = OnChainRpcClient::new(config.rpc_url.clone());
    let latest = rpc.get_latest_block().await.unwrap_or(0);
    println!("On-chain latest block: {}", latest);
    let events = rpc.get_logs("0x0000000000000000000000000000000000000000", latest.saturating_sub(100), latest).await.unwrap_or_default();
    println!("On-chain events collected {}", events.len());
    
    let storage = NewsStorage::new(config.storage_path.clone());
    let all_news = [rss_items, json_items, google_items, api_items].concat();
    storage.save_news(&all_news)?;
    storage.save_onchain(&events)?;
    println!("Saved {} news items and {} on-chain events to {}", all_news.len(), events.len(), config.storage_path);
    
    let alerter = Alerter::new(AlertConfig { sentiment_threshold: 0.7, keyword: "bitcoin".into() });
    let alerts = alerter.check_news(&all_news);
    println!("Alerts triggered: {}", alerts.len());
    for a in alerts.iter().take(5) {
        println!("ALERT: {:?} - {}", a.source, a.title);
    }
    
    Ok(())
}
