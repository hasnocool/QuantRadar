use quantaradar_domain_model::*;
#[test] fn domain_default() { let t = Trade::new("BTC".into(), 1.0, 100.0, quantaradar_core::Direction::Long); assert!(!t.symbol.is_empty()); }
