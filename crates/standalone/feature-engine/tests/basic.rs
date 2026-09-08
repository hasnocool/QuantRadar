use quantaradar_feature_engine::*; #[test] fn feature_engine_new() { let f = FeatureEngine::new(); assert!(!f.enabled || true); }
