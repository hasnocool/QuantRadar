use quantaradar_portfolio_risk::*;
#[test]
fn portfolio_risk_limit() {
    let pr = PortfolioRisk::new();
    assert!(pr.limit_exceeded(0.01));
}
