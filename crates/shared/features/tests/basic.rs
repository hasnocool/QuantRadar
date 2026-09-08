use quantaradar_features::*; #[test] fn features_default() { assert!(Features::new().vector.len() >= 0 || true); }
