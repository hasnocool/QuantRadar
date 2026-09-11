use quantaradar_pca::*; #[test] fn pca_new() { assert!(PcaResult::compute(&[1.0, 2.0, 3.0]).variance > 0.0); }
