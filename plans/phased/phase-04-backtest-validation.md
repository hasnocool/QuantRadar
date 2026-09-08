# Phase 4 — Backtest, Validation & Anti-Overfitting

Working version of cost-aware backtesting, expanding walk-forward, robustness, and champion/challenger.

## 4.1 Backtest Engine (minimum realistic)
Inputs: historical candles + trades + order books + spread + liquidity + latency + fees + market impact + partial fills.
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
TRAIN ────────► TEST
             ↓
        move forward
             ↓
TRAIN ─────────────► TEST

Example folds: 2019-2021 train / 2022 test; 2020-2022 train / 2023 test; 2021-2023 train / 2024 test. Aggregate OOS performance across all folds.

## 4.4 Robustness Testing (minimum)
- Parameter perturbation: ±20% on all numeric params.
- Transaction-cost perturbation: fees × 2, slippage × 1.5.
- Trade-order randomization: shuffle trade sequence, measure PnL variance.
- Regime segmentation: report performance per regime.
- Cross-asset testing: same strategy on different symbols.

## 4.5 Anti-Overfitting Stack
Every strategy is guilty until proven robust:
Walk-forward → OOS performance → Parameter perturbation →
Transaction-cost perturbation → Bootstrap → Monte Carlo →
Trade-order randomization → Regime testing → Cross-asset testing →
Multiple-hypothesis correction → Overfitting diagnostics

Multiple-hypothesis correction methods: Bonferroni, Benjamini-Hochberg, Deflated Sharpe, PBO (Probability of Backtest Overfitting), White's Reality Check, Hansen SPA, nested walk-forward.

## 4.6 Regime-Segmented Reporting
Backtest results must be reported segmented by market regime, so performance is not masked by regime mix.

## 4.7 Champion / Challenger Gate
CHAMPION
   │
   ├── Challenger A → Validation → Reject / Promote
   ├── Challenger B → Validation → Reject / Promote
   └── Challenger C → Validation → Reject / Promote

Promotion requires: OOS Sharpe > champion OOS Sharpe, max drawdown < champion max drawdown, profit factor > 1.5, robustness pass rate > 70%.

## 4.8 Design Goals
- Goal 3: No data leakage.
- Goal 4: Explicit trading costs.
- Goal 5: Regime-conditional research.

## Working version complete when:
- [x] Backtest includes fees, slippage, partial fills, liquidity gate.
- [x] Expanding WFO produces aggregated OOS metrics.
- [x] Robustness tests run automatically on every strategy.
- [x] Champion/challenger promotion requires OOS improvement + risk constraints.