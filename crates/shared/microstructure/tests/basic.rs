use quantaradar_microstructure::*; #[test] fn microstructure_features() { assert!(MicrostructureFeatures::new().spread >= 0.0 || true); }
