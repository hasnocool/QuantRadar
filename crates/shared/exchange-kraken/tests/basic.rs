use quantaradar_exchange_kraken::*;
#[test] fn kraken_default() { let c = KrakenClient::default(); assert!(format!("{:?}", c).contains("kraken")); }
