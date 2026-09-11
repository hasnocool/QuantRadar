use quantaradar_expected_return::*;
#[test]
fn expected_return_sharpe() {
    assert!(ExpectedReturn::compute(100.0).rate > 0.0);
}
