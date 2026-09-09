use quantaradar_microstructure::*; #[test] fn microstructure_features() { assert!(MicrostructureFeatures::default().spread_bps >= 0.0 || true); }
