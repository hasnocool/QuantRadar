//! Basic property/load verification stub.
#[test]
fn load_stable() {
    let s = quantaradar_test_scale::TestScale::new();
    assert!(s.load_benchmark(1) > 0.0);
}
