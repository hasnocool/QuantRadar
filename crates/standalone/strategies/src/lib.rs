//! Strategy DSL parameters stub (generator deferred to strategy_dsl #8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyType { Trend, Reversion }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn strategy_type_variants() { assert!(matches!(StrategyType::Trend, StrategyType::Trend)); }
}
