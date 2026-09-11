use quantaradar_features::*; #[test] fn features_default() { assert!(ema(&[1.0, 2.0, 3.0, 4.0, 5.0], 3)[2].is_some()); }
