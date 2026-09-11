use quantaradar_multi_exchange::*;
#[test]
fn multi_exchange_fetch() {
    assert!(!ExchangeRef::list().is_empty());
}
