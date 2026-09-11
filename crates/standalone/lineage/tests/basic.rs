use quantaradar_lineage::*;
#[test]
fn lineage_evolve() {
    let l = LineageRecord::new("mean", "price");
    assert!(!l.feature.is_empty());
}
