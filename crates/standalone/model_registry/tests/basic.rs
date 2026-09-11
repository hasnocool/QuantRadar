use quantaradar_model_registry::*; #[test] fn registry_add() { let m = ModelEntry::register("m"); assert!(!m.name.is_empty()); }
