# Phase 3 — Screener & Signal Pipeline

Working version of independent screener families, signal ensemble, and real-time pipeline.

## 3.1 Screener Families (independent, explainable)
TREND         → moving-average trend, ADX strength, persistence
BREAKOUT      → range breakout, vol-confirmed breakout, price/volume expansion
MEAN REVERSION → oversold, deviation from mean, liquidity-supported reversal
MOMENTUM      → short/medium acceleration, residual momentum
VOL EXPANSION → volatility spike, compression breakout
MICROSTRUCTURE → order-book imbalance, aggressive flow, depth deterioration
EVENT         → volume anomaly, new high/low, regime change, correlation break

Each family produces a typed `Signal` with evidence. The Signal struct carries the full ARCHITECTURE.md:44-46 signal contract:
- timestamp
- symbol
- family
- direction: Direction (Long / Short / Flat)
- score: f64
- regime: Regime
- rationale: String
- feature_evidence: Vec<(String, f64)>
- strategy/config_version

## 3.2 Signal Ensemble (minimum)
Trend:              +0.82
Momentum:           +0.76
Relative strength:  +0.91
Liquidity:          +0.98
Order flow:         +0.73
Regime fit:         +0.88
Event strength:     +0.65
────────────────────────────
Composite:          +0.84

Composite = weighted average of family scores, weighted by regime compatibility. No family directly produces an order.

## 3.3 Real-Time Signal Pipeline (minimum working flow)
market event → state update → feature update → regime update →
screener evaluation → cross-sectional ranking → signal aggregation →
portfolio risk check → order intent → execution simulator

Latency budget: feature update < 100ms, signal aggregation < 50ms, risk check < 20ms.

## 3.4 Strategy DSL (minimum)
family: breakout
regime_filter: [BullTrend, BullLowVol]
parameters:
  lookback: 20
  min_volume_z: 1.5
  min_liquidity_score: 0.6
constraints:
  max_position_pct: 5.0
  max_portfolio_heat: 30.0

## 3.5 Expected-Return Model
For each signal, estimate:
- P(up)
- expected return
- expected adverse excursion / favorable excursion
- expected holding period
- expected volatility
- confidence

Used to compute expected alpha / expected risk / expected cost before sizing.

## 3.6 Holding-Period Model
Predictions for horizons: 5m, 15m, 1h, 4h, 1d, 3d, 7d. Enables combining strategies without mixing incompatible signal horizons.

## 3.7 Event Intelligence Layer
Event bus categories: market, asset, liquidity, volatility, derivatives, exchange, external/news.
Events: LIQUIDITY_COLLAPSE, FUNDING_EXTREME, OI_SURGE, LIQUIDATION_CASCADE, VOL_BREAKOUT, SPREAD_EXPANSION, ORDERBOOK_IMBALANCE, CORRELATION_BREAK, REGIME_CHANGE, MOMENTUM_FAILURE.

## 3.8 Strategy Discovery Loop
Continuous hypothesis generation: "When BTC in low-vol bullish regime, coins with strong 24h RS, positive order-flow imbalance, 20-period breakout may outperform over 24-72h." Machine-testable. Auto-asks: works? after fees? after slippage? OOS? across years/assets/regimes? lucky params? parameter perturbation? cost increases? randomized trade order?

## 3.9 Design Goals
- Goal 1: Dynamic asset discovery instead of hard-coded universes.
- Goal 3: No data leakage.
- Goal 5: Regime-conditional research.

## Working version complete when:
- [x] Each screener family produces typed `Signal` with evidence.
- [x] Signal ensemble computes composite score with regime weights.
- [x] Real-time pipeline runs end-to-end in < 200ms.
- [x] Strategy DSL parses and validates constraints.