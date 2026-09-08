//! Sentiment scoring model.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct Sentiment { pub score: f64 }
impl Sentiment { pub fn score_text(text: &str) -> Self { let s = if text.contains("good") { 0.8 } else if text.contains("bad") { -0.5 } else { 0.0 }; Self{score:s} } }
#[cfg(test)] mod tests { use super::*; #[test] fn sentiment_positive() { assert!(Sentiment::score_text("good news").score > 0.0); } }
