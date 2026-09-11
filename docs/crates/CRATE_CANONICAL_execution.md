# CRATE_CANONICAL_execution.md — quantaradar-execution

## quantaradar-execution

### Purpose
Shared crate `quantaradar-execution` in the QuantRadar workspace (`crates/shared/execution`).
# crates/execution/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/execution/codemap.md)
```text
# crates/execution/

## Responsibility
Risk controls + paper execution (execution orders, paper-state machine, reconciliation).

## Design
Paper-state machine with risk limits; order execution tracking.

## Flow
Signals → execution orders → paper state → reconciliation.

## Integration
Depends on portfolio, backtest; produces paper-trading state.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct RiskLimits (lib.rs)`
- `struct Position (lib.rs)`
- `struct OrderIntent (lib.rs)`
- `struct Fill (lib.rs)`
- `struct PaperAccount (lib.rs)`
- `struct RiskEvent (lib.rs)`
- `struct PendingOrder (lib.rs)`
- `fn effective_positions_num (lib.rs)`
- `fn portfolio_var (lib.rs)`
- `fn volatility_target_size (lib.rs)`
- `fn gross_exposure (lib.rs)`
- `fn size_position (lib.rs)`
- `fn approve (lib.rs)`
- `fn paper_fill (lib.rs)`
- `fn mark_to_market (lib.rs)`
- `fn update_account (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar execution crate manifest
[package]
name = "quantaradar-execution"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
[dependencies]
chrono.workspace = true
serde.workspace = true
uuid.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/execution` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-execution` / `cargo doc -p quantaradar-execution --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-execution` passes
- [ ] `cargo doc -p quantaradar-execution` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
