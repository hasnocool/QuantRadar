use quantaradar_universe_history::*;
#[test]
fn universe_history_add() {
    let mut u = UniverseHistory::new();
    u.add("BTC".into(), 100);
    assert!(!u.at_time(50).is_empty());
}
