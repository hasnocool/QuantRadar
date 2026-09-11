use quantaradar_research::*; #[test] fn research_default() { assert_eq!(correlation_matrix(&vec![vec![1.0, 2.0]]).len(), 1); }
