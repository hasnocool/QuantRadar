use quantaradar_events::*;
#[test]
fn market_event_classify() {
    let mut ev = MarketEvent::new("BTC".into(), 100.0);
    ev.classify("anomaly");
    assert_eq!(ev.kind, "anomaly");
}
