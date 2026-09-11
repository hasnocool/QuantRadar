use quantaradar_portfolio_risk::{PortfolioRiskEngine, RiskPosition, RiskRegime, RiskConfig, VaRMethod};

#[test]
fn portfolio_risk_engine_creation() {
    let engine = PortfolioRiskEngine::new();
    assert_eq!(engine.regime(), RiskRegime::Normal);
}

#[test]
fn portfolio_risk_with_equity() {
    let engine = PortfolioRiskEngine::with_equity(1_000_000.0);
    assert_eq!(engine.current_equity(), 1_000_000.0);
}

#[test]
fn portfolio_risk_position_update() {
    let mut engine = PortfolioRiskEngine::new();
    let pos = RiskPosition {
        symbol: "AAPL".to_string(),
        quantity: 100.0,
        market_value: 15_000.0,
        notional_value: 15_000.0,
        sector: Some("Tech".to_string()),
        liquidity_score: 0.9,
        beta: Some(1.2),
        returns: vec![0.01, -0.01],
    };
    engine.update_position(pos);
    assert!(engine.get_position("AAPL").is_some());
}

#[test]
fn portfolio_risk_regime_scaling() {
    let mut engine = PortfolioRiskEngine::new();
    engine.set_regime(RiskRegime::HighVol);
    assert_eq!(engine.regime(), RiskRegime::HighVol);
    assert_eq!(engine.regime().scale_factor(), 0.5);

    let adjusted = engine.regime_adjusted_limits();
    assert_eq!(adjusted.max_portfolio_var, engine.config().max_portfolio_var * 0.5);
}

#[test]
fn portfolio_risk_var_computation() {
    let mut engine = PortfolioRiskEngine::with_equity(1_000_000.0);
    engine.add_returns(vec![0.01, -0.02, 0.015, -0.005, 0.005, -0.01, 0.02]);
    let var = engine.var_historical_public(0.95);
    assert!(var > 0.0);
}

#[test]
fn portfolio_risk_metrics() {
    let mut engine = PortfolioRiskEngine::with_equity(1_000_000.0);
    engine.add_returns(vec![0.01, -0.02, 0.015, -0.005, 0.005, -0.01, 0.02]);
    let metrics = engine.compute_metrics();
    assert!(metrics.var_95 >= 0.0);
    assert!(metrics.var_99 >= metrics.var_95);
    assert!(metrics.cvar_95 >= metrics.var_95);
}