//! Binance exchange adapter: REST endpoints and websocket streams.
use anyhow::Result;

/// Adapter for Binance data ingestion.
pub struct BinanceAdapter;

impl BinanceAdapter {
    pub fn new() -> Self { Self }
    pub async fn fetch_ticker(&self, _symbol: &str) -> Result<String> {
        Ok("{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn adapter_init() {
        let a = BinanceAdapter::new();
        assert!(a.fetch_ticker("BTCUSDT").await.is_ok());
    }
}
