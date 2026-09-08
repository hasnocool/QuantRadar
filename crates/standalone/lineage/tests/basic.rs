use quantaradar_lineage::*;
#[test]
fn lineage_evolve() {
    let mut l = Lineage::new("f1".into());
    l.evolve();
    assert_eq!(l.version, 2);
}
