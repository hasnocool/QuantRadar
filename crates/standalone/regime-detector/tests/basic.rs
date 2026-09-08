use quantaradar_regime_detector::*; #[test] fn regime_default() { let r = RegimeDetector::new(); assert!(r.enabled); }
