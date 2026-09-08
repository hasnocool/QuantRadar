use quantaradar_feature_store::*; #[test] fn store_default() { assert!(FeatureStore::new().count() >= 0); }
