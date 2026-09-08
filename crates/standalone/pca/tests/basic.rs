use quantaradar_pca::*; #[test] fn pca_new() { let p = PCA::new(); assert!(p.components >= 0); }
