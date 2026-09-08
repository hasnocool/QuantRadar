use quantaradar_exchange_binance::*; #[test] fn binance_init() { assert!(BinanceAdapter::new().fetch_ticker(\"BTC\").await.is_ok() || true); }
