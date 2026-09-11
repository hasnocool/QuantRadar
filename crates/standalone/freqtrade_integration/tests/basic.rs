use quantaradar_freqtrade_integration::*;
#[test]
fn freqtrade_connect() {
    let c = FreqAdapter::connect();
    assert!(c.connected);
}
