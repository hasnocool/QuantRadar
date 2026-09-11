# CRATE_CANONICAL_backtest.md — quantaradar-backtest

## quantaradar-backtest

### Purpose
Shared crate `quantaradar-backtest` in the QuantRadar workspace (`crates/shared/backtest`).
# crates/backtest/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/backtest/codemap.md)
```text
# crates/backtest/

## Responsibility
Cost-aware backtesting (backtest engine, walk-forward, robustness, promotion gates).

## Design
Realistic execution with cost modeling; expanding WFO framework.

## Flow
Strategy candidates → backtest with costs → robustness → promotion.

## Integration
Used by portfolio, paper trading; depends on core, features, regime, screeners.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct BacktestConfig (lib.rs)`
- `struct BacktestResult (lib.rs)`
- `struct TradeRecord (lib.rs)`
- `fn run (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar deterministic strategy backtester
[package]
name = "quantaradar-backtest"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
chrono.workspace = true
serde.workspace = true
quantaradar-core = { path = "../core" }
quantaradar-microstructure = { path = "../microstructure" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-microstructure`

### Integration points
- Workspace member: `crates/shared/backtest` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-backtest` / `cargo doc -p quantaradar-backtest --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-backtest` passes
- [ ] `cargo doc -p quantaradar-backtest` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
