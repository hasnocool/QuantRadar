//! registry stub crate.
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Experiment { pub id: String, pub strategy_id: String, pub metrics: HashMap<String, f64>, pub decision: String, pub timestamp: u64 }
impl Experiment { pub fn new(id: String, sid: String) -> Self { Self { id, strategy_id: sid, metrics: HashMap::new(), decision: "pending".into(), timestamp: 0 } } pub fn set_metric(&mut self, k: String, v: f64) { self.metrics.insert(k, v); } pub fn promote(&mut self) { self.decision = "champion".into(); } pub fn reject(&mut self) { self.decision = "rejected".into(); } }
pub struct Registry { experiments: Vec<Experiment> }
impl Registry { pub fn new() -> Self { Self { experiments: vec![] } } pub fn add(&mut self, e: Experiment) { self.experiments.push(e); } pub fn get(&self, id: &str) -> Option<&Experiment> { self.experiments.iter().find(|e| e.id == id) } pub fn count(&self) -> usize { self.experiments.len() } pub fn list_champions(&self) -> Vec<&Experiment> { self.experiments.iter().filter(|e| e.decision == "champion").collect() } }
