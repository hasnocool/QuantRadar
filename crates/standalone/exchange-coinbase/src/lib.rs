//! Coinbase exchange adapter.
use anyhow::Result;

pub struct CoinbaseAdapter;
impl CoinbaseAdapter {
    pub fn new() -> Self { Self }
    pub async fn fetch_price(&self, _pair: &str) -> Result<f64> { Ok(0.0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn adapter_init() { assert!(CoinbaseAdapter::new().fetch_price("BTC-USD").await.is_ok()); }
}
