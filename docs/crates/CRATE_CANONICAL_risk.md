# CRATE_CANONICAL_risk.md — Canonical Risk Crate Documentation

## quantaradar-risk

### Purpose
Risk management engine for portfolio risk calculations, Value-at-Risk (VaR), Conditional
Value-at-Risk (CVaR), and various risk metrics used across the QuantRadar platform.
Provides both historical simulation and Monte Carlo approaches.

### Version
0.2.0 (workspace-managed)

### License
MIT (workspace license)

### Rust Version
1.85 (workspace requirement)

### Key Types and APIs

#### Risk Metrics
- `VaRCalculator` — Value-at-Risk calculator
  - Methods: `calculate_returns(returns: &[f64]) -> Vec<f64>`, 
    `calculate_var(confidence: f64) -> f64`, `calculation_method() -> VaRMethod`
  - Methods: `set_method(method: VaRMethod)`, `set_confidence(level: f64)`
- `CVaRCalculator` — Conditional Value-at-Risk (Expected Shortfall)
  - Methods: `calculate_cvar(returns: &[f64], confidence: f64) -> f64`
  - Requires sorted return distribution

#### VaR Methods
- `HistoricalVaR` — Historical simulation using empirical distribution
  - Uses ordered historical returns
  - Parameter: `lookback_window: usize` (default: 252 trading days)
- `ParametricVaR` — Gaussian parametric approach
  - Assumes normal distribution of returns
  - Parameter: `confidence_level: f64` (default: 0.99)
- `MonteCarloVaR` — Monte Carlo simulation approach
  - Parameter: `num_simulations: usize` (default: 10000)
  - Parameter: `random_seed: u64` (default: 42)

#### Portfolio Risk
- `PortfolioRiskEngine` — Comprehensive portfolio risk calculation
  - Methods: `calculate_portfolio_var(portfolio: &Portfolio, confidence: f64) -> f64`,
    `calculate_portfolio_cvar(portfolio: &Portfolio, confidence: f64) -> f64`,
    `calculate_concentration_risk(portfolio: &Portfolio) -> f64`
  - Fields: `positions: HashMap<String, Position>`, `covariance_matrix: Matrix<f64>`
  - Methods: `update_positions(&mut self, new_positions: &[Position])`,
    `set_covariance_matrix(&mut self, matrix: Matrix<f64>)`

#### Risk Metrics Structure
- `RiskMetrics` — Bundle of risk measures
  - Fields: `var_99: f64`, `var_95: f64`, `cvar_99: f64`, `cvar_95: f64`,
    `volatility_annualized: f64`, `max_drawdown: f64`, `beta: f64`
  - Methods: `from_calculations(calculations: &RiskCalculations) -> Self`

#### Exposure Analysis
- `PositionExposure` — Per-symbol risk contribution
  - Fields: `symbol: String`, `risk_contribution: f64`, `percent_of_portfolio: f64`,
    `marginal_exposure: f64`
  - Methods: `rank_by_contribution() -> Vec<PositionExposure>`

### Dependencies
**Runtime:**
- `ndarray` v0.15 — Matrix algebra for covariance calculations
- `ndarray-linearalgebra` v0.5 — LAPACK bindings for eigendecomposition
- `chrono` v0.4 with serde — Time series handling
- `serde` v1 with derive — Serialization of risk outputs
- `thiserror` v1 — Error type definitions

**Development:**
- No specific dev dependencies beyond workspace deps

### Integration Points

#### Used By
- `crates/standalone/portfolio_risk` — Consumes risk calculations for portfolio summaries
- `crates/standalone/optimization` — Uses risk metrics as optimization constraints
- `crates/standalone/regime-detector` — Risk-adjusted regime detection
- `crates/standalone/backtest_engine` — Risk analysis during backtest runs
- `crates/shared/reporting` — Generates risk metric reports

#### Consumes From
- `crates/standalone/portfolio` — Position data and returns series
- `crates/standalone/signals` — Signal returns for performance attribution
- `crates/standalone/backtest_engine` — Trade/PnL series from historical runs

### Representative Code Example

```rust
use quantaradar_risk::{VaRCalculator, CVaRCalculator, VaRMethod};
use quantaradar_core::Portfolio;
use std::collections::HashMap;

fn main() {
    // Create a portfolio with positions
    let mut portfolio = Portfolio {
        symbols: vec!["BTC/USD".to_string(), "ETH/USD".to_string()],
        positions: HashMap::new(),
    };
    portfolio.positions.insert("BTC/USD".to_string(), 
        quantaradar_core::Position::new("BTC/USD", 2.5, 45000.0));
    portfolio.positions.insert("ETH/USD".to_string(), 
        quantaradar_core::Position::new("ETH/USD", 150.0, 3000.0));

    // Calculate VaR at 99% confidence using historical method
    let mut var_calc = VaRCalculator::new();
    var_calc.set_method(VaRMethod::Historical);
    var_calc.set_lookback_window(252);
    
    let var_99 = var_calc.calculate_var(&portfolio.returns(), 0.99);
    println!("99% VaR: {:.2} USD", var_99);

    // Calculate CVaR at 95% confidence
    let mut cvar_calc = CVaRCalculator::new();
    let cvar_95 = cvar_calc.calculate_cvar(&portfolio.returns(), 0.95);
    println!("95% CVaR: {:.2} USD", cvar_95);
}
```

### Verification
- All risk calculations compile with `cargo check -p quantaradar-risk`
- Historical VaR matches reference Python implementation (pandas .quantile())
- Covariance matrix positive-semidefinite check verified
- Edge cases: empty returns, single-period, all-positive returns
- Monte Carlo convergence tested with 10000+ simulations
- Error handling: invalid confidence levels (must be 0-1), insufficient data