use quantaradar_monitoring::*; #[test] fn monitoring_default() { assert!(MonitorHealth::check().healthy); }
