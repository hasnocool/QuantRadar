use quantaradar_multi_exchange::*;
#[test]
fn multi_exchange_fetch() {
    let m = MultiExchange;
    assert_eq!(Exchange::fetch_price(&m, "BTC"), 100.0);
}
