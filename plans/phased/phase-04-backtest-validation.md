# Phase 4 — Backtest, Validation & Anti-Overfitting

Working version of cost-aware backtesting, expanding walk-forward, robustness, and champion/challenger.

## 4.1 Backtest Engine (minimum realistic)
Inputs: historical candles + trades + order books + spread + liquidity + latency + fees + market impact.
Execution rules:
- Signal at `t` → order submitted at `t + latency`.
- Book state at arrival → available liquidity → partial fill.
- Price impact = f(quantity, depth, spread).
- Fees applied per fill. Slippage = spread + impact.

## 4.2 Portfolio Realism (minimum)
- Multiple simultaneous symbols.
- Cash management: cash = previous_cash + realized_pnl - new_position_cost.
- Position sizing from equity, entry, stop distance.
- Per-position cap, portfolio heat cap, concurrent-position limit.
- Liquidity gate: only trade if executable depth >= position_size * 2.

## 4.3 Walk-Forward Validation (mandatory)
```text
TRAIN ────────► TEST
             ↓
        move forward
             ↓
TRAIN ─────────────► TEST
```
Example folds: 2019-2021 train / 2022 test; 2020-2022 train / 2023 test; 2021-2023 train / 2024 test. Aggregate OOS performance across all folds.

## 4.4 Robustness Testing (minimum)
- Parameter perturbation: ±20% on all numeric params.
- Transaction-cost perturbation: fees × 2, slippage × 1.5.
- Trade-order randomization: shuffle trade sequence, measure PnL variance.
- Regime segmentation: report performance per regime.
- Cross-asset testing: same strategy on different symbols.

## 4.5 Anti-Overfitting Stack
Every strategy is guilty until proven robust:
```text
Walk-forward → OOS performance → Parameter perturbation →
Transaction-cost perturbation → Bootstrap → Monte Carlo →
Trade-order randomization → Regime testing → Cross-asset testing →
Multiple-hypothesis correction → Overfitting diagnostics
```
Multiple-hypothesis correction: Bonferroni or Benjamini-Hochberg on p-values from backtest matrix.

## 4.6 Champion / Challenger Gate
```text
CHAMPION
   │
   ├── Challenger A → Validation → Reject / Promote
   ├── Challenger B → Validation → Reject / Promote
   └── Challenger C → Validation → Reject / Promote
```
Promotion requires: OOS Sharpe > champion OOS Sharpe, max drawdown < champion max drawdown, profit factor > 1.5, robustness pass rate > 70%.

## Working version complete when:
- [ ] Backtest includes fees, slippage, partial fills, liquidity gate.
- [ ] Expanding WFO produces aggregated OOS metrics.
- [ ] Robustness tests run automatically on every strategy.
- [ ] Champion/challenger promotion requires OOS improvement + risk constraints.
