use quantaradar_core::{Direction, FeatureRow, Regime};
use quantaradar_execution::RiskLimits;
use quantaradar_pipeline::{MarketData, SignalPipeline};
use quantaradar_signal_ensemble::SignalEnsembleConfig;

fn feature_row(symbol: &str, returns_24: f64) -> FeatureRow {
    FeatureRow {
        timestamp: 1_700_000_000_000,
        symbol: symbol.to_string(),
        returns_1h: 0.01,
        ema_20: 105.0,
        ema_50: Some(102.0),
        ema_200: Some(99.0),
        rsi_14: Some(62.0),
        atr_14: Some(2.0),
        atr_pct: Some(0.02),
        realized_vol_24h: 0.03,
        bollinger_width: 0.05,
        volume_zscore: 2.5,
        ema_distance: 0.3,
        breakout_flag: true,
        new_high_24h: true,
        new_low_24h: false,
        lookback: 24,
        minimum_history: 30,
        availability_at: 1_700_000_000_000,
        returns_1: Some(0.01),
        returns_4: Some(0.02),
        returns_24: Some(returns_24),
        returns_72: Some(0.05),
        distance_ema20_atr: Some(0.3),
        breakout_20: true,
        new_high_20: true,
        new_low_20: false,
        sector: Some("test".to_string()),
    }
}

#[test]
fn pipeline_screens_ensembles_and_gates() {
    let pipeline =
        SignalPipeline::new(SignalEnsembleConfig::default(), RiskLimits::default(), 100_000.0);
    let rows = vec![
        feature_row("AAA", 0.06),
        feature_row("BBB", 0.12),
    ];
    let market = vec![
        MarketData {
            symbol: "AAA".to_string(),
            entry: 100.0,
            stop: 90.0,
            liquidity_score: 0.9,
        },
        MarketData {
            symbol: "BBB".to_string(),
            entry: 100.0,
            stop: 90.0,
            liquidity_score: 0.9,
        },
    ];
    let out = pipeline.process(&rows, Regime::BullTrend, &market, 0);
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].symbol, "BBB");
    assert_eq!(out[0].rank, 1);
    assert_eq!(out[1].symbol, "AAA");
    assert_eq!(out[1].rank, 2);
    assert!(out.iter().all(|p| p.direction == Direction::Long));
    assert!(out.iter().all(|p| p.order_intent.is_some()));
}