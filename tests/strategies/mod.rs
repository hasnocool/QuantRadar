/// proptest strategies for test fixtures.

use proptest::prelude::*;
use quantaradar_core::{Bar, FeatureRow, MarketId, Regime, Signal, Direction, OrderSide};
use rand::SeedableRng;
use rand::rngs::SmallRng;

// Bar strategy (already defined in bar_generator.rs, re-exported here for convenience)
pub use crate::fixtures::bar_generator::bar_strategy as bar_proptest_strategy;

// FeatureRow strategy (already defined in feature_row_generator.rs, re-exported here)
pub use crate::fixtures::feature_row_generator::feature_row_strategy as feature_row_proptest_strategy;

// MarketId strategy - generate valid market identifiers
pub fn market_id_strategy() -> impl Strategy<Value = MarketId> {
    // Generate common crypto trading pairs
    ["BTC/USD", "ETH/USD", "SOL/USD", "ADA/USD", "DOT/USD", "DOGE/USD"]
        .into_iter()
        .prop_map(|s| MarketId::from(s))
}

// Signal strategy - generate valid signals with all required fields
pub fn signal_strategy() -> impl Strategy<Value = Signal> {
    any::<u64>()
        .prop_map(|seed| {
            Signal {
                timestamp: seed,
                symbol: "BTC/USD".to_string(),
                direction: Direction::Long,
                price: 100.0,
                quantity: 1.0,
                order_side: OrderSide::Buy,
                market_id: MarketId::from("BTC/USD"),
                regime: Regime::BullTrend,
                strategy: "test".to_string(),
                config_version: "v1.0".to_string(),
            }
        })
}

// Regime strategy - generate valid regime states
pub fn regime_strategy() -> impl Strategy<Value = Regime> {
    ["BullTrend", "BullHighVol", "BullLowVol",
     "BearTrend", "BearHighVol", "BearLowVol",
     "SidewaysHighVol", "SidewaysLowVol",
     "TransitionBull", "TransitionBear", "Unknown"]
        .into_iter()
        .prop_map(|s| Regime::from(s))
}

// Strategy for Bar with specific invariants
pub fn bar_with_invariants_strategy() -> impl Strategy<Value = Bar> {
    any::<u64>()
        .prop_map(|seed| {
            crate::fixtures::bar_generator::bar_with_seed(seed, MarketId::from("BTC/USD"))
        })
        .prop_filter("Bar invariants maintained", |bar| {
            bar.high >= bar.open && bar.high >= bar.close &&
                bar.low <= bar.open && bar.low <= bar.close &&
                bar.volume > 0
        })
}