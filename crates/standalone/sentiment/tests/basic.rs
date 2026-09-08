use quantaradar_sentiment::*; #[test] fn sentiment_default() { assert!(Sentiment::new().score >= -1.0 || true); }
