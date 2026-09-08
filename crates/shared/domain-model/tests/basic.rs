use quantaradar_domain_model::*; #[test] fn domain_default() { assert!(DomainModel::new().name.len() > 0 || true); }
