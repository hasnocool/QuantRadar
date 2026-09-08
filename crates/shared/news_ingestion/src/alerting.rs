use crate::{NewsItem, OnChainEvent};

pub struct AlertConfig {
    pub sentiment_threshold: f64,
    pub keyword: String,
}

pub struct Alerter {
    config: AlertConfig,
}

impl Alerter {
    pub fn new(config: AlertConfig) -> Self {
        Self { config }
    }

    pub fn check_news(&self, items: &[NewsItem]) -> Vec<NewsItem> {
        items.iter()
            .filter(|n| n.sentiment_score.abs() >= self.config.sentiment_threshold && n.title.to_lowercase().contains(&self.config.keyword.to_lowercase()))
            .cloned()
            .collect()
    }

    pub fn check_onchain(&self, events: &[OnChainEvent]) -> Vec<OnChainEvent> {
        events.iter()
            .filter(|e| e.event_type.to_lowercase().contains(&self.config.keyword.to_lowercase()))
            .cloned()
            .collect()
    }
}
