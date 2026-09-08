//! Live execution boundary stub.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct LiveExec { pub active: bool }
impl LiveExec { pub fn start() -> Self { Self { active: true } } }
#[cfg(test)] mod tests { use super::*; #[test] fn live_start() { assert!(LiveExec::start().active); } }
