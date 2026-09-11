# CRATE_CANONICAL_optimization.md — optimization

## optimization

### Purpose
Standalone crate `optimization` in the QuantRadar workspace (`crates/standalone/optimization`).
# optimization

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/optimization/codemap.md)
```text
# optimization

## Responsibility
Portfolio optimization and position-sizing stubs (mean-variance deferred).

## Source Map
- `src/lib.rs`: `basic_size()`, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct PositionWeights (lib.rs)`
- `fn new (lib.rs)`
- `enum Side (lib.rs)`
- `struct OptimizerConfig (lib.rs)`
- `enum RegimeScaling (lib.rs)`
- `fn factor (lib.rs)`
- `fn regime_scaling_from_vol (lib.rs)`
- `struct OptimizerResult (lib.rs)`
- `fn portfolio_optimizer (lib.rs)`
- `fn optimize_portfolio_from_signals (lib.rs)`

### Cargo manifest (abridged)
```toml
# Portfolio optimization and position sizing.
[package]
name = "optimization"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
serde.workspace = true
ndarray = "0.15"
serde_json.workspace = true
```

### Dependencies (manifest keys)
`name`, `ndarray`

### Integration points
- Workspace member: `crates/standalone/optimization` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p optimization` / `cargo doc -p optimization --no-deps`.

### Verification checklist
- [ ] `cargo check -p optimization` passes
- [ ] `cargo doc -p optimization` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
