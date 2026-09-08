# Phase 5 — Portfolio, Risk & Paper Execution

Working version of portfolio optimizer, risk engine, paper trading state machine, and reconciliation.

## 5.1 Portfolio Optimizer (minimum)
Inputs: signals → expected returns → correlation matrix → liquidity → volatility.
Output: position weights (e.g., BTC 6.2%, ETH 3.1%, SOL 1.4%, AVAX 0.4%).
Constraints:
- Max position % (e.g., 10%).
- Max portfolio heat (e.g., 30%).
- Max gross/net exposure.
- Cluster limits: highly correlated assets share budget.
- Liquidity-adjusted exposure: weight × liquidity_score.

## 5.2 Risk Engine (minimum controls)
```text
per-trade risk → portfolio heat → max position → max cluster exposure →
max leverage → max drawdown → volatility targeting → VaR / CVaR →
stress tests → liquidity limits → correlation limits → regime-based reduction
```
Dynamic scaling:
- Normal regime → 100% risk.
- High vol → 50%.
- Extreme vol → 25%.
- Market dislocation → 0%.

## 5.3 Paper Trading State Machine
States: `Pending` → `Submitted` → `PartiallyFilled` → `Filled` / `Rejected` / `Cancelled` / `Expired`.
Account fields: `cash`, `equity` (cash + unrealized PnL), `peak_equity`, `realized_pnl`, `positions`, `fills`, `pending_orders`.
Unrealized PnL = sum(position_size × (current_price - entry_price)).
Reconciliation: compare paper fills vs simulated book state; flag discrepancies.

## 5.4 Paper Account Initialization
No `Default` derivation. Explicit initialization:
```rust
PaperAccount::new(initial_cash: f64, initial_equity: f64)
```
Numeric fields initialized to coherent starting state, not zero.

## 5.5 Execution Boundary (isolated, disabled by default)
```text
Research → Promotion Gate → Paper → Shadow → Canary → Live
```
Live adapter is a separate crate/module, disabled by default, with no exchange credentials in repo.

## Working version complete when:
- [ ] Portfolio optimizer produces liquidity-adjusted weights with cluster limits.
- [ ] Risk engine applies regime-based scaling and drawdown throttling.
- [ ] Paper account tracks unrealized PnL, pending orders, and reconciliation.
- [ ] Live adapter is isolated and disabled by default.
