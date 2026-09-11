use quantaradar_feature_store::*;
#[test]
fn store_default() {
    let store = FeatureStore::new("/tmp/verify");
    let vstore = FeatureValueStore::new();
    assert!(store.as_of(chrono::Utc::now(), &vstore).is_empty());
}
