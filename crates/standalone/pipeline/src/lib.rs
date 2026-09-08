//! pipeline stub crate.
pub struct Pipeline { stages: Vec<String>, active: bool, results: Vec<String> }
impl Pipeline {
    pub fn new() -> Self { Self { stages: vec!["market".into(), "feature".into(), "regime".into(), "screen".into(), "rank".into(), "risk".into(), "intent".into(), "sim".into()], active: false, results: vec![] } }
    pub fn start(&mut self) -> bool { self.active = true; true }
    pub fn stop(&mut self) -> bool { self.active = false; true }
    pub fn run_stage(&mut self, stage: &str, input: String) -> String { let out = format!("{} -> {}", stage, input); self.results.push(out.clone()); out }
    pub fn get_results(&self) -> Vec<String> { self.results.clone() }
    pub fn get_stage_count(&self) -> usize { self.stages.len() }
}
