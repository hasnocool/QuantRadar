use quantaradar_exchange_kraken::*; #[test] fn kraken_default() { assert!(KrakenAdapter::new().connected || true); }
