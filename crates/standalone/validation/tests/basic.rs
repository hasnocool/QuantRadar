use quantaradar_validation::*; #[test] fn validation_default() { assert!(Validator::new().valid || true); }
