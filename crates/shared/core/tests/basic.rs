use quantaradar_core::*; #[test] fn core_bar_range() { let b = Bar { ts: chrono::Utc::now(), open:1.0, high:2.0, low:0.5, close:1.5, volume:100.0, trades: None }; assert!(b.range() > 0.0); }
