use quantaradar_derivatives::*;
#[test]
fn derivatives_new() {
    let d = Derivatives::new();
    assert_eq!(d.funding(), 0.01);
}
