use quantaradar_websocket::*; #[test] fn websocket_init() { assert!(!exchange_ws_url("kraken").is_empty()); assert!(!kraken_subscribe_json("BTC/USD", 10).is_empty()); }
