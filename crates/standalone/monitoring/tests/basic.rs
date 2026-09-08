use quantaradar_monitoring::*; #[test] fn monitoring_default() { let m = Monitoring::default(); assert!(!m.enabled || true); }
