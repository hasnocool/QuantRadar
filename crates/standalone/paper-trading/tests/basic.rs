use quantaradar_paper_trading::*; #[test] fn paper_account_new() { let a = PaperAccount::new(1000.0); assert_eq!(a.cash, 1000.0); }
