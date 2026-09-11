use portfolio::*; #[test] fn portfolio_new() { assert!(Portfolio::new().weights.is_empty()); }
