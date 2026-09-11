/// Core crate unit tests - extends existing core crate test suite.
/// Covers domain model types, serde round-trip, and Observation validation.

use quantaradar_core::*;
use quantaradar_core::QualityFlag;

#[test]
fn core_bar_range_and_typical() {
    let bar = Bar { ts: chrono::Utc::now(), open: 100.0, high: 110.0, low: 90.0, close: 105.0, volume: 10.0, trades: None };
    assert_eq!(bar.range(), 20.0);
    assert_eq!(bar.typical_price(), (110.0 + 90.0 + 105.0) / 3.0);
}

#[test]
fn core_safe_return_edges() {
    let v = safe_return(110.0, 100.0).unwrap();
    assert!((v - 0.1).abs() < 1e-12);
    assert_eq!(safe_return(100.0, 0.0), None);
    assert_eq!(safe_return(f64::NAN, 100.0), None);
    assert_eq!(safe_return(100.0, f64::INFINITY), None);
}

#[test]
fn core_valid_price_volume() {
    assert!(is_valid_price(1.0));
    assert!(!is_valid_price(0.0));
    assert!(!is_valid_price(f64::NAN));
    assert!(is_valid_volume(0.0));
    assert!(!is_valid_volume(-1.0));
}

#[test]
fn core_direction_serde() {
    assert_eq!(Direction::Long.to_string(), "long");
    assert_eq!(Direction::Short.to_string(), "short");
    assert_eq!(Direction::Flat.to_string(), "flat");
    assert_eq!(OrderSide::Buy.to_string(), "buy");
    assert_eq!(OrderSide::Sell.to_string(), "sell");
    let d: Direction = serde_json::from_str("\"long\"").unwrap();
    assert_eq!(d, Direction::Long);
    let s = serde_json::to_string(&OrderSide::Sell).unwrap();
    assert_eq!(s, "\"sell\"");
}

#[test]
fn core_enum_display() {
    // Direction display
    assert_eq!(format!("{}", Direction::Long), "long");
    assert_eq!(format!("{}", Direction::Short), "short");
    assert_eq!(format!("{}", Direction::Flat), "flat");
    // OrderSide display
    assert_eq!(format!("{}", OrderSide::Buy), "buy");
    assert_eq!(format!("{}", OrderSide::Sell), "sell");
    // SignalFamily display
    let sf = SignalFamily::Trend;
    let sf_str = format!("{}", sf);
    // EventKind display
    let ek = EventKind::Breakout;
    let ek_str = format!("{}", ek);
    // QualityFlag display
    let qf = QualityFlag::Valid;
    let qf_str = format!("{}", qf);
    assert_eq!(qf_str, "valid");
}

#[test]
fn core_observation_validate_happy() {
    let obs = Observation::new(10, "kraken".into(), "BTC/USD".into(), "BTC".into(), "USD".into(),
        OHLCV { open: 100.0, high: 110.0, low: 90.0, close: 105.0, volume: 10.0 }, 5, 104.0, 105.0,
        vec![(104.0, 1.0)], vec![(105.0, 1.0)],
        SourceKind::Rest, 11, vec![]);
    let r = validate_observation(&obs, Some(9));
    assert!(r.is_valid);
    // Invalid timestamp
    let r = validate_observation(&obs, Some(10));
    assert!(!r.is_valid);
    assert!(r.flags.contains(&QualityFlag::InvalidTimestamp));
}

#[test]
fn core_observation_negative_spread() {
    let mut bad = Observation::new(10, "kraken".into(), "BTC/USD".into(), "BTC".into(), "USD".into(),
        OHLCV { open: 100.0, high: 110.0, low: 90.0, close: 105.0, volume: 10.0 }, 5, 104.0, 105.0,
        vec![(104.0, 1.0)], vec![(105.0, 1.0)],
        SourceKind::Rest, 11, vec![]);
    // ask < bid creates negative spread
    bad.ask = 103.0;
    let r = validate_observation(&bad, None);
    assert!(r.flags.contains(&QualityFlag::NegativeSpread));
}

#[test]
fn core_observation_negative_price() {
    let mut bad = Observation::new(10, "kraken".into(), "BTC/USD".into(), "BTC".into(), "USD".into(),
        OHLCV { open: 100.0, high: 110.0, low: 90.0, close: 105.0, volume: 10.0 }, 5, 104.0, 105.0,
        vec![(104.0, 1.0)], vec![(105.0, 1.0)],
        SourceKind::Rest, 11, vec![]);
    bad.ohlcv.close = -5.0;
    let r = validate_observation(&bad, None);
    assert!(r.flags.contains(&QualityFlag::NegativePrice));
}

#[test]
fn core_bar_negative_spread() {
    let bar = Bar { ts: chrono::Utc::now(), open: 1.0, high: 1.0, low: 2.0, close: 1.0, volume: 1.0, trades: None };
    let r = validate_bar(&bar, None);
    assert!(r.flags.contains(&QualityFlag::NegativeSpread));
}