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
    pub enable_reddit: bool,
    pub google_query: String,
    pub news_api_key: Option<String>,
    pub news_api_query: String,
    pub rpc_url: String,
    pub onchain_address: String,
    pub storage_path: String,
}

impl Default for IngestionConfig {
    fn default() -> Self {
        Self {
            reddit_subreddits: vec!["CryptoCurrency".into(), "Bitcoin".into()],
            enable_reddit: false,
            google_query: "cryptocurrency market".into(),
            news_api_key: None,
            news_api_query: "cryptocurrency".into(),
            rpc_url: "https://eth.llamarpc.com".into(),
            onchain_address: "0x0000000000000000000000000000000000000000".into(),
            storage_path: "data/news".into(),
        }
    }
}

pub async fn run(config: IngestionConfig) -> anyhow::Result<()> {
    use crate::{reddit_rss::RedditRssCollector, reddit_json::RedditJsonCollector, google_news::GoogleNewsCollector, news_api::NewsApiCollector, onchain::OnChainRpcClient, storage::NewsStorage, alerting::{Alerter, AlertConfig}};
    
    println!("Starting news ingestion pipeline");
    
    let mut rss_items = Vec::new();
    let mut json_items = Vec::new();
    if config.enable_reddit {
        let rss_collector = RedditRssCollector::new(config.reddit_subreddits.clone());
        rss_items = match rss_collector.collect().await {
            Ok(v) => { println!("Reddit RSS collected {} items", v.len()); v },
            Err(e) => { eprintln!("Reddit RSS error: {}", e); Vec::new() }
        };
        let json_collector = RedditJsonCollector::new(config.reddit_subreddits.clone());
        json_items = match json_collector.collect().await {
            Ok(v) => { println!("Reddit JSON collected {} items", v.len()); v },
            Err(e) => { eprintln!("Reddit JSON error: {}", e); Vec::new() }
        };
    } else {
        println!("Reddit disabled, falling back to Google News + NewsAPI");
    }
    
    let google_collector = GoogleNewsCollector::new(config.google_query.clone());
    let google_items = match google_collector.collect().await {
        Ok(v) => { println!("Google News collected {} items", v.len()); v },
        Err(e) => { eprintln!("Google News error: {}", e); Vec::new() }
    };
    
    let mut api_items = Vec::new();
    if let Some(key) = config.news_api_key {
        let api_collector = NewsApiCollector::new(key, config.news_api_query.clone());
        match api_collector.collect().await {
            Ok(v) => { println!("NewsAPI collected {} items", v.len()); api_items = v; },
            Err(e) => { eprintln!("NewsAPI error: {}", e); }
        }
    } else {
        println!("NewsAPI skipped: no key");
    }
    
    let rpc = OnChainRpcClient::new(config.rpc_url.clone());
    let latest = match rpc.get_latest_block().await {
        Ok(b) => { println!("On-chain latest block: {}", b); b },
        Err(e) => { eprintln!("On-chain block error: {}", e); 0 }
    };
    let events = match rpc.get_logs(&config.onchain_address, latest.saturating_sub(100), latest).await {
        Ok(v) => { println!("On-chain events collected {}", v.len()); v },
        Err(e) => { eprintln!("On-chain logs error: {}", e); Vec::new() }
    };
    
    let storage = NewsStorage::new(config.storage_path.clone());
    let rss_count = rss_items.len();
    let json_count = json_items.len();
    let google_count = google_items.len();
    let api_count = api_items.len();
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
    
    let alerts_path = format!("{}/alerts.json", config.storage_path);
    let alerts_json = serde_json::to_string_pretty(&alerts).unwrap_or_default();
    let _ = std::fs::write(&alerts_path, alerts_json);
    println!("Alerts saved to {}", alerts_path);
    
    let dashboard = serde_json::json!({
        "timestamp": chrono::Utc::now(),
        "news_count": all_news.len(),
        "onchain_events": events.len(),
        "alerts": alerts.len(),
        "sources": {
            "reddit_rss": rss_count,
            "reddit_json": json_count,
            "google_news": google_count,
            "newsapi": api_count
        }
    });
    let dashboard_path = format!("{}/dashboard.json", config.storage_path);
    let _ = std::fs::write(&dashboard_path, serde_json::to_string_pretty(&dashboard).unwrap_or_default());
    println!("Dashboard saved to {}", dashboard_path);
    
    Ok(())
}
