# CRATE_CANONICAL_backtest_engine.md — quantaradar-backtest_engine

## quantaradar-backtest_engine

### Purpose
Shared crate `quantaradar-backtest_engine` in the QuantRadar workspace (`crates/shared/backtest_engine`).
# crates/shared/backtest_engine/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/backtest_engine/codemap.md)
```text
# crates/shared/backtest_engine/codemap

## Responsibility
Backtest engine execution layer (cost-aware simulation).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → backtest_engine computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct MicrostructureFeatures (lib.rs)`
- `struct BacktestFeeConfig (lib.rs)`
- `struct CapitalConfig (lib.rs)`
- `enum CashManagement (lib.rs)`
- `struct BacktestTrade (lib.rs)`
- `struct PortfolioLimits (lib.rs)`
- `struct ImpactConfig (lib.rs)`
- `struct BacktestStatistics (lib.rs)`
- `struct BacktestResult (lib.rs)`
- `struct WfoFold (lib.rs)`
- `struct BacktestEngine (lib.rs)`
- `fn new (lib.rs)`
- `fn set_fee_config (lib.rs)`
- `fn set_capital_config (lib.rs)`
- `fn set_limits (lib.rs)`
- `fn set_impact_config (lib.rs)`
- `fn execute_trade (lib.rs)`
- `fn run_price_backtest (lib.rs)`
- `fn calculate_statistics (lib.rs)`
- `fn run (lib.rs)`
- `fn run_expanding_wfo (lib.rs)`
- `fn robustness_parameter_perturbation (lib.rs)`
- `fn robustness_cost_perturbation (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-backtest_engine"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/backtest_engine` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-backtest_engine` / `cargo doc -p quantaradar-backtest_engine --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-backtest_engine` passes
- [ ] `cargo doc -p quantaradar-backtest_engine` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
