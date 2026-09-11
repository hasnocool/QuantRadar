use quantaradar_dashboard::*; #[test] fn dashboard_exists() { assert!(DashMetric::new("pnl", 10.0).value > 0.0); }
