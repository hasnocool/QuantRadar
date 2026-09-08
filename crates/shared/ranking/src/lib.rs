//! Cross-sectional ranking model.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RankScore { pub symbol: String, pub score: f64 }
impl RankScore { pub fn rank_all(items: &[(String, f64)]) -> Vec<RankScore> { items.iter().map(|(s,v)| RankScore{symbol:s.clone(),score:*v}).collect() } }
#[cfg(test)] mod tests { use super::*; #[test] fn rank_ok() { assert!(RankScore::rank_all(&[("A".into(),1.0)]).len()==1); } }
