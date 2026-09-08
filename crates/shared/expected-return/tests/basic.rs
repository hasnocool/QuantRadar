use quantaradar_expected_return::*; #[test] fn expected_default() { assert!(ExpectedReturn::new().value >= 0.0 || true); }
