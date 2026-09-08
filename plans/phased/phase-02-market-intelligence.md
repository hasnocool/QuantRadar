# Phase 2 — Market Intelligence Engine

Working version of feature engineering, regime detection, microstructure, and cross-sectional ranking.

## 2.1 Feature Engine (minimum working set)
```rust
struct FeatureRow {
  timestamp: u64,
  symbol: String,
  returns_1h: f64,
  ema_20: f64,
  rsi_14: f64,
  atr_14: f64,
  realized_vol_24h: f64,
  bollinger_width: f64,
  volume_zscore: f64,
  ema_distance: f64,
  breakout_flag: bool,
  new_high_24h: bool,
  new_low_24h: bool,
}
```
Every feature knows its `lookback`, `minimum_history`, and `available_at` timestamp. No future leakage.

## 2.2 Regime Engine (minimum taxonomy)
```rust
enum Regime {
  BullTrend, BullHighVol, BullLowVol,
  BearTrend, BearHighVol, BearLowVol,
  SidewaysHighVol, SidewaysLowVol,
  TransitionBull, TransitionBear, Unknown,
}
```
Output includes `confidence: f64`, `trend_strength: f64`, `volatility_state: String`, `transition_probability: f64`. Multi-timeframe: 5m, 15m, 1h, 4h, 1d, 1w.

## 2.3 Microstructure Analytics
From `OrderBookSnapshot` (bid, ask, bid_depth, ask_depth):
- spread = ask - bid
- depth_imbalance = (bid_depth - ask_depth) / (bid_depth + ask_depth)
- executable_impact_1k, 10k, 100k (walk one side, average fill price)
- liquidity_score = f(spread, depth, impact)

## 2.4 Cross-Sectional Ranking (minimum)
```text
raw features → winsorization (5% / 95%) → z-score normalization →
regime-conditioned score → rank → top-N selection
```
Score = weighted sum of trend, momentum, breakout, liquidity, regime fit. Regime fit penalizes signals that conflict with current regime.

## 2.5 Breadth & Relative Strength
- Positive assets count / total universe.
- EMA participation rate.
- New high / new low counts.
- Relative strength vs benchmark (e.g., BTC/USD as benchmark for altcoins).

## 2.6 Correlation / PCA (minimum)
- Rolling correlation matrix (30-day window).
- First PCA component = market factor.
- Cluster-aware position limits: highly correlated assets share exposure budget.

## Working version complete when:
- [ ] FeatureRow computes deterministically with no future leakage.
- [ ] Regime engine outputs confidence + transition probability.
- [ ] Microstructure computes spread, imbalance, impact, liquidity score.
- [ ] Ranking produces regime-conditioned scores.
- [ ] PCA first component explains >30% variance on crypto universe.
