/// Synthetic FeatureRow generator with deterministic seeds for testing.
/// No network access required. FeatureRow knows its lookback, minimum_history, and availability_at timestamp.

use quantaradar_core::{FeatureRow, MarketId};
use proptest::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;

/// Generate a random FeatureRow with deterministic seed.
/// Fields: timestamp, symbol, returns_1h, ema_20, rsi_14, atr_14,
/// realized_vol_24h, bollinger_width, volume_zscore, ema_distance,
/// breakout_flag, new_high_24h, new_low_24h
fn feature_row_with_seed(seed: u64, market_id: MarketId) -> FeatureRow {
    let mut rng = SmallRng::seed_from_seed(seed);

    let base_price: f64 = 100.0 + (seed as f64) * 0.1;
    let open_price = base_price + rng.gen_range(-10.0..10.0);
    let high_price = open_price + rng.gen_range(0.0..20.0).max(open_price + 1.0);
    let low_price = open_price - rng.gen_range(0.0..20.0).max(open_price - 1.0);
    let close_price = open_price + rng.gen_range(-15.0..15.0);
    let volume: u64 = rng.gen_range(100..1000000);

    let returns_1h = rng.gen_range(-0.1..0.1);
    let ema_20 = base_price + rng.gen_range(-20.0..20.0);
    let rsi_14 = rng.gen_range(0.0..100.0);
    let atr_14 = rng.gen_range(0.1..10.0);
    let realized_vol_24h = rng.gen_range(0.0..0.5);
    let bollinger_width = rng.gen_range(0.1..1.0);
    let volume_zscore = rng.gen_range(-3.0..3.0);
    let ema_distance = rng.gen_range(-5.0..5.0);
    let breakout_flag = rng.gen_bool(0.3);
    let new_high_24h = rng.gen_bool(0.2);
    let new_low_24h = rng.gen_bool(0.2);

    FeatureRow {
        timestamp: seed as u64,
        symbol: market_id.to_string(),
        returns_1h,
        ema_20,
        rsi_14,
        atr_14,
        realized_vol_24h,
        bollinger_width,
        volume_zscore,
        ema_distance,
        breakout_flag,
        new_high_24h,
        new_low_24h,
    }
}

/// proptest strategy for FeatureRow that maintains invariants
pub fn feature_row_strategy() -> impl Strategy<Value = FeatureRow> {
    any::<u64>()
        .prop_map(|seed| feature_row_with_seed(seed, MarketId::from("BTC/USD")))
}

// Property tests for FeatureRow invariants
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_row_deterministic() {
        // Same seed should produce same FeatureRow
        let seed = 42u64;
        let row1 = feature_row_with_seed(seed, MarketId::from("BTC/USD"));
        let row2 = feature_row_with_seed(seed, MarketId::from("BTC/USD"));
        assert_eq!(row1.timestamp, row2.timestamp);
        assert_eq!(row1.returns_1h, row2.returns_1h);
        assert_eq!(row1.ema_20, row2.ema_20);
    }

    #[test]
    fn test_different_seeds_different_rows() {
        let row1 = feature_row_with_seed(1u64, MarketId::from("BTC/USD"));
        let row2 = feature_row_with_seed(2u64, MarketId::from("BTC/USD"));
        assert_ne!(row1.returns_1h, row2.returns_1h || row1.ema_20 != row2.ema_20);
    }
}