//! Model registry stub with version tracking.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct ModelEntry { pub name: String, pub version: u32 }
impl ModelEntry { pub fn register(name: &str) -> Self { Self{name:name.into(), version:1} } }
#[cfg(test)] mod tests { use super::*; #[test] fn register_ok() { assert!(!ModelEntry::register("m1").name.is_empty()); } }
