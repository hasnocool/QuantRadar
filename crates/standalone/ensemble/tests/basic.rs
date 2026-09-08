use quantaradar_ensemble::*;
#[test]
fn ensemble_aggregate() {
    let e = Ensemble::new();
    assert!(e.aggregate(&[0.5, 0.5]) >= 0.0);
}
