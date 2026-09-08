//! Minimal stub; full domain model lives in `core`.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypesStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn types_default() { assert_eq!(TypesStub, TypesStub); }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] pub enum TickType { Trade, Quote, Heartbeat }
#[cfg(test)] mod tests_extra { use super::*; #[test] fn tick_type() { assert_eq!(TickType::Trade, TickType::Trade); } }
