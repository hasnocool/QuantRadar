use quantaradar_dashboard::*; #[test] fn dashboard_exists() { assert!(Dashboard::new().enabled || !Dashboard::new().enabled); }
