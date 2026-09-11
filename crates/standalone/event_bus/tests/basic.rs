use quantaradar_event_bus::*; #[test] fn event_bus_default() { assert_eq!(EventBus::new().count(), 0); }
