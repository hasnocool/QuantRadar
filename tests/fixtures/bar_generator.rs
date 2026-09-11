/// Synthetic Bar generator with deterministic seeds for testing.
/// No network access required. Bar invariants: high >= max(open,close), low <= min(open,close), volume > 0.

use quantaradar_core::{Bar, MarketId};
use proptest::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;

/// Generate a random Bar with deterministic seed.
/// Invariant: high >= max(open,close), low <= min(open,close), volume > 0.
fn bar_with_seed(seed: u64, market_id: MarketId) -> Bar {
    let mut rng = SmallRng::seed_from_seed(seed);

    let base_price: f64 = 100.0 + (seed as f64) * 0.1;
    let open_price = base_price + rng.gen_range(-10.0..10.0);
    let high_price = open_price + rng.gen_range(0.0..20.0).max(open_price + 1.0);
    let low_price = open_price - rng.gen_range(0.0..20.0).max(open_price - 1.0);
    let close_price = open_price + rng.gen_range(-15.0..15.0);
    let volume: u64 = rng.gen_range(100..1000000);

    // Ensure invariants
    let high = high_price.max(close_price).max(open_price);
    let low = low_price.min(close_price).min(open_price);

    Bar {
        market_id,
        timestamp: seed as u64,
        open: open_price.max(1.0),
        high: high.max(low + 0.01),
        low: low.min(high - 0.01),
        close: close_price.max(1.0),
        volume,
        quote_asset: "USD".to_string(),
        base_asset: market_id.base().to_string(),
    }
}

/// proptest strategy for Bar that maintains invariants
pub fn bar_strategy() -> impl Strategy<Value = Bar> {
    any::<u64>()
        .prop_map(|seed| bar_with_seed(seed, MarketId::from("BTC/USD")))
        .prop_filter("Bar invariants", |bar| {
            bar.high >= bar.open && bar.high >= bar.close &&
                bar.low <= bar.open && bar.low <= bar.close &&
                bar.volume > 0
        })
}

// Property tests for Bar invariants (standalone, not inside proptest! block)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_volume_positive() {
        // Test with specific seeds
        for seed in 0..100 {
            let bar = bar_with_seed(seed, MarketId::from("BTC/USD"));
            assert!(bar.volume > 0, "Seed {}: volume should be positive", seed);
        }
    }

    #[test]
    fn test_bar_high_at_least_open() {
        for seed in 0..100 {
            let bar = bar_with_seed(seed, MarketId::from("BTC/USD"));
            assert!(bar.high >= bar.open, "Seed {}: high should be >= open", seed);
        }
    }

    #[test]
    fn test_bar_high_at_least_close() {
        for seed in 0..100 {
            let bar = bar_with_seed(seed, MarketId::from("BTC/USD"));
            assert!(bar.high >= bar.close, "Seed {}: high should be >= close", seed);
        }
    }

    #[test]
    fn test_bar_low_at_most_open() {
        for seed in 0..100 {
            let bar = bar_with_seed(seed, MarketId::from("BTC/USD"));
            assert!(bar.low <= bar.open, "Seed {}: low should be <= open", seed);
        }
    }

    #[test]
    fn test_bar_low_at_most_close() {
        for seed in 0..100 {
            let bar = bar_with_seed(seed, MarketId::from("BTC/USD"));
            assert!(bar.low <= bar.close, "Seed {}: low should be <= close", seed);
        }
    }
}