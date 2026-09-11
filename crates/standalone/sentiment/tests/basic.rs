use quantaradar_sentiment::*; #[test] fn sentiment_default() { assert!(Sentiment::score_text("good news").score > 0.0); }
