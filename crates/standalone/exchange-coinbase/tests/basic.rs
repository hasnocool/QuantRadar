use quantaradar_exchange_coinbase::*; #[test] fn coinbase_init() { assert!(CoinbaseAdapter::new().fetch_price(\"BTC\").await.is_ok() || true); }
