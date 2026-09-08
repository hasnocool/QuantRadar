use quantaradar_model_registry::*; #[test] fn registry_add() { let mut r = ModelRegistry::new(); r.register("m".into(),"v1".into()); assert!(!r.list().is_empty()); }
