use quantaradar_orderbook::*; #[test] fn orderbook_spread() { assert!(OrderBook::new(100.0, 100.5).spread() == 0.5); }
