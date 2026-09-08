//! Strategy DSL stub with basic expression parser.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct StrategyExpr { pub expr: String }
impl StrategyExpr { pub fn parse(s: &str) -> Self { Self{expr:s.into()} } pub fn evaluate(&self) -> bool { !self.expr.is_empty() } }
#[cfg(test)] mod tests { use super::*; #[test] fn parse_ok() { assert!(StrategyExpr::parse("buy").evaluate()); } }
