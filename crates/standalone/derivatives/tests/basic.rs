use quantaradar_derivatives::*;
#[test]
fn derivatives_new() {
    let d = DerivativeContract::new("BTC", 100);
    assert!(!d.symbol.is_empty());
}
