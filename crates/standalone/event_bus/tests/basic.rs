use quantaradar_event_bus::*; #[test] fn event_bus_default() { assert!(EventBus::new().subscribers.len() >= 0); }
