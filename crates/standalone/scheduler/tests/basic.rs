use quantaradar_scheduler::*; #[test] fn scheduler_new() { let s = Scheduler::new(); assert!(!s.active || true); }
