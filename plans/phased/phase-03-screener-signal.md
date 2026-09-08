# Phase 3 — Screener & Signal Pipeline

Working version of independent screener families, signal ensemble, and real-time pipeline.

## 3.1 Screener Families (independent, explainable)
```text
TREND         → moving-average trend, ADX strength, persistence
BREAKOUT      → range breakout, vol-confirmed breakout, price/volume expansion
MEAN REVERSION → oversold, deviation from mean, liquidity-supported reversal
MOMENTUM      → short/medium acceleration, residual momentum
VOL EXPANSION → volatility spike, compression breakout
MICROSTRUCTURE → order-book imbalance, aggressive flow, depth deterioration
EVENT         → volume anomaly, new high/low, regime change, correlation break
```
Each family produces `Signal { timestamp, symbol, family, direction: Direction, score: f64, regime: Regime, rationale: String, feature_evidence: Vec<(String, f64)> }`.

## 3.2 Signal Ensemble (minimum)
```text
Trend:              +0.82
Momentum:           +0.76
Relative strength:  +0.91
Liquidity:          +0.98
Order flow:         +0.73
Regime fit:         +0.88
Event strength:     +0.65
────────────────────────────
Composite:          +0.84
```
Composite = weighted average of family scores, weighted by regime compatibility. No family directly produces an order.

## 3.3 Real-Time Signal Pipeline (minimum working flow)
```text
market event → state update → feature update → regime update →
screener evaluation → cross-sectional ranking → signal aggregation →
portfolio risk check → order intent → execution simulator
```
Latency budget: feature update < 100ms, signal aggregation < 50ms, risk check < 20ms.

## 3.4 Strategy DSL (minimum)
```yaml
family: breakout
regime_filter: [BullTrend, BullLowVol]
parameters:
  lookback: 20
  min_volume_z: 1.5
  min_liquidity_score: 0.6
constraints:
  max_position_pct: 5.0
  max_portfolio_heat: 30.0
```

## Working version complete when:
- [ ] Each screener family produces typed `Signal` with evidence.
- [ ] Signal ensemble computes composite score with regime weights.
- [ ] Real-time pipeline runs end-to-end in < 200ms.
- [ ] Strategy DSL parses and validates constraints.
