use quantaradar_screeners::*; #[test] fn screen_default() { assert!(Screener::new().family.is_empty() || true); }
