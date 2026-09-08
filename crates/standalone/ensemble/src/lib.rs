//! ensemble stub crate.
use serde::{Serialize, Deserialize};
pub struct Ensemble { pub weights:Vec<f64>, pub scores:Vec<f64> }
impl Ensemble { pub fn new()->Self{Self{weights:vec![0.5,0.5],scores:vec![]}} pub fn add_weight(&mut self,w:f64){self.weights.push(w);} pub fn score(&self)->f64{if self.weights.is_empty(){0.0}else{self.weights.iter().sum::<f64>()/self.weights.iter().sum::<f64>().max(1.0)}} pub fn add_score(&mut self,s:f64){self.scores.push(s);} pub fn aggregate(&self,scores:&[f64])->f64{if scores.is_empty(){0.0}else{scores.iter().sum::<f64>()/scores.len() as f64}} }
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct EnsembleVote { pub weight: f64 }
impl EnsembleVote { pub fn vote() -> f64 { 1.0 } }
#[cfg(test)] mod tests { use super::*; #[test] fn vote_positive() { assert!(EnsembleVote::vote() > 0.0); } }
