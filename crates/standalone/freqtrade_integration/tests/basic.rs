use quantaradar_freqtrade_integration::*;
#[test]
fn freqtrade_connect() {
    let mut c = FreqtradeClient::new();
    c.connect();
    assert!(c.connected);
}
