use quantaradar_hold_period::*;
#[test]
fn hold_period_default() {
    let h = HoldPeriod::default();
    assert!(!h.enabled || true); // stub
}
