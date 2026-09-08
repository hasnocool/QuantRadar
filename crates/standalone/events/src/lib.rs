//! Market event tracking (delisting, listings, events).
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct Event { pub symbol: String, pub event_type: String }
impl Event { pub fn new(s: &str, t: &str) -> Self { Self{symbol:s.into(), event_type:t.into()} } }
#[cfg(test)] mod tests { use super::*; #[test] fn event_create() { let e = Event::new("A","delist"); assert!(!e.symbol.is_empty()); } }
