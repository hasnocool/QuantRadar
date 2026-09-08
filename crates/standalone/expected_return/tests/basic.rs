use quantaradar_expected_return::*;
#[test]
fn expected_return_sharpe() {
    assert!(sharpe(0.1, 0.2) > 0.0);
}
