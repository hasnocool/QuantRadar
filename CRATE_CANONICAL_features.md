# CRATE_CANONICAL_features.md — quantaradar-features

## quantaradar-features

### Purpose
Shared crate `quantaradar-features` in the QuantRadar workspace (`crates/shared/features`).
# crates/features/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/features/codemap.md)
```text
# crates/features/
## Responsibility
Deterministic technical features (FeatureRow, feature store, feature-engine pipeline).
## Design
Polars/duckdb-backed; deterministic calculations; no ML within this crate.
## Flow
Market data → feature computation → feature store → screening/backtest.
## Integration
Used by regime, screening, backtest crates; depends on core.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Lookback (lib.rs)`
- `const fn (lib.rs)`
- `fn ok_at (lib.rs)`
- `fn ema (lib.rs)`
- `fn ema_lookback (lib.rs)`
- `fn rsi (lib.rs)`
- `fn rsi_lookback (lib.rs)`
- `fn atr (lib.rs)`
- `fn atr_lookback (lib.rs)`
- `fn rolling_std (lib.rs)`
- `fn feature_rows (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar technical feature engine
[package]
name = "quantaradar-features"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
chrono.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/features` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-features` / `cargo doc -p quantaradar-features --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-features` passes
- [ ] `cargo doc -p quantaradar-features` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
