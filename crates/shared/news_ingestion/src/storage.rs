use anyhow::Result;
use std::path::Path;
use crate::{NewsItem, OnChainEvent};
use chrono::Utc;

pub struct NewsStorage {
    base_path: String,
}

impl NewsStorage {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn save_news(&self, items: &[NewsItem]) -> Result<()> {
        // Parquet write placeholder – in production use arrow writer
        let path = Path::new(&self.base_path).join(format!("news_{}.json", Utc::now().timestamp()));
        std::fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(items)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn save_onchain(&self, events: &[OnChainEvent]) -> Result<()> {
        let path = Path::new(&self.base_path).join(format!("onchain_{}.json", Utc::now().timestamp()));
        std::fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(events)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
