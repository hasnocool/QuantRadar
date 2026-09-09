use optimization::*;

#[test]
fn test_regime_scaling() {
    // Normal regime → 100%
    assert_eq!(RegimeScaling::Normal.factor(), 1.0);
    // High vol → 50%
    assert_eq!(RegimeScaling::HighVol.factor(), 0.5);
    // Extreme vol → 25%
    assert_eq!(RegimeScaling::ExtremeVol.factor(), 0.25);
    // Dislocation → 0%
    assert_eq!(RegimeScaling::Dislocation.factor(), 0.0);
}

#[test]
fn test_liquidity_adjustment() {
    let n = 3;
    let expected_returns = vec![0.08, 0.12, 0.05];

    // Equal correlation
    let mut corr = ndarray::Array2::eye(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                corr[[i, j]] = 0.1;
            }
        }
    }

    let liquidity = vec![1.0, 0.5, 0.1]; // One high-liquidity, one medium, one low
    let volatilities = vec![0.2, 0.2, 0.2];

    let config = OptimizerConfig {
        liquidity_floor: 0.1,
        ..Default::default()
    };

    let result = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        1.0,
    )
    .expect("optimizer should succeed");

    // Low liquidity asset should have lower weight
    let liq0 = result.weights.weights[0];
    let liq2 = result.weights.weights[2];
    assert!(liq0 >= liq2, "High liquidity asset should have higher weight than low liquidity asset");

    // Check percentages exist and are positive
    for p in &result.weights.percentages {
        assert!(*p >= 0.0);
    }
}

#[test]
fn test_max_position_constraint() {
    let n = 2;
    let expected_returns = vec![0.08, 0.12];

    let mut corr = ndarray::Array2::eye(n);
    corr[[0, 1]] = 0.0;
    corr[[1, 0]] = 0.0;

    let liquidity = vec![1.0, 1.0];
    let volatilities = vec![0.2, 0.2];

    let config = OptimizerConfig {
        max_position_pct: 0.30, // 30% max per position
        ..Default::default()
    };

    let result = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        1.0,
    )
    .expect("optimizer should succeed");

    // With 2 assets and 30% max each, weights should respect the constraint
    for w in &result.weights.weights {
        assert!(*w <= 0.30 + 1e-10, "position weight {} exceeds 30% limit", w);
    }
}

#[test]
fn test_portfolio_heat_constraint() {
    let n = 3;
    let expected_returns = vec![0.08, 0.12, 0.05];

    let mut corr = ndarray::Array2::eye(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                corr[[i, j]] = 0.0;
            }
        }
    }

    let liquidity = vec![1.0, 1.0, 1.0];
    let volatilities = vec![0.2, 0.2, 0.2];

    let config = OptimizerConfig {
        max_portfolio_heat: 0.50, // 50% gross exposure max
        ..Default::default()
    };

    let result = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        1.0,
    )
    .expect("optimizer should succeed");

    // Gross exposure should not exceed 50%
    let gross: f64 = result.weights.weights.iter().map(|&a| a.abs()).sum();
    assert!(gross <= 0.55, "gross exposure {} should not exceed 55% (with tolerance)", gross);
}

#[test]
fn test_dynamic_risk_scaling() {
    let n = 3;
    let expected_returns = vec![0.08, 0.12, 0.05];

    let mut corr = ndarray::Array2::eye(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                corr[[i, j]] = 0.0;
            }
        }
    }

    let liquidity = vec![1.0, 1.0, 1.0];
    let volatilities = vec![0.2, 0.2, 0.2];

    let config = OptimizerConfig::default();

    // Test normal regime (100% scaling)
    let normal = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        1.0,
    )
    .expect("should succeed");

    // Test high vol regime (50% scaling)
    let high_vol = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        0.5,
    )
    .expect("should succeed");

    // Test extreme vol regime (25% scaling)
    let extreme = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        0.25,
    )
    .expect("should succeed");

    // Test dislocation regime (0% scaling)
    let dislocation = portfolio_optimizer(
        &expected_returns,
        &corr,
        &liquidity,
        &volatilities,
        &config,
        0.0,
    )
    .expect("should succeed");

    // All should produce valid results
    assert!(normal.portfolio_volatility > 0.0);
    assert!(high_vol.portfolio_volatility > 0.0);
    assert!(extreme.portfolio_volatility > 0.0);
    assert!(dislocation.portfolio_volatility == 0.0 || dislocation.portfolio_volatility >= 0.0);

    // With dislocation (0% scaling), expected return should be effectively 0
    assert!(dislocation.expected_portfolio_return == 0.0 || dislocation.expected_portfolio_return.abs() < 1e-10);
}

#[test]
fn test_optimize_portfolio_from_signals() {
    let n = 3;
    let correlation_matrix = ndarray::Array2::eye(n);

    let liquidity_scores = vec![1.0, 0.8, 0.6];
    let volatilities = vec![0.2, 0.2, 0.2];

    let config = OptimizerConfig::default();

    // Test with normal regime scaling (100%)
    let signals = vec![
        Side::Long,
        Side::Long,
        Side::Long,
    ];

    let result = optimize_portfolio_from_signals(
        &signals,
        &correlation_matrix,
        &liquidity_scores,
        &volatilities,
        1.0, // normal regime
        &config,
    )
    .expect("should succeed");

    // Check weights are valid
    let wsum: f64 = result.weights.weights.iter().sum();
    // Allow sum <= 1.0 (cash remaining) or close to 1.0
    assert!(wsum <= 1.0 + 1e-10, "weights sum {} should not exceed 1.0 + tolerance", wsum);

    // Check each weight respects max position constraint
    for w in &result.weights.weights {
        assert!(*w <= 0.10 + 1e-10, "position weight {} exceeds 10% limit", w);
    }

    // Check percentages exist
    for p in &result.weights.percentages {
        assert!(*p >= 0.0, "percentage should be non-negative");
    }
}