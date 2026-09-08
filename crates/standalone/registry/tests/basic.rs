use quantaradar_registry::*; #[test] fn registry_new() { let r = Registry::new(); assert_eq!(r.count(), 0); }
