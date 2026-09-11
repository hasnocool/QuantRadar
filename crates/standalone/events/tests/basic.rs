use quantaradar_events::*;
#[test]
fn market_event_classify() {
    let ev = Event::new("BTC", "anomaly");
    assert_eq!(ev.event_type, "anomaly");
}
