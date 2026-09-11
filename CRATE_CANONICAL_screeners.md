# CRATE_CANONICAL_screeners.md — quantaradar-screeners

## quantaradar-screeners

### Purpose
Shared crate `quantaradar-screeners` in the QuantRadar workspace (`crates/shared/screeners`).
# crates/screeners/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/screeners/codemap.md)
```text
# crates/screeners/

## Responsibility
Explainable screener families (7 families, composite score, DSL contracts).

## Design
Screener DAG with explainable scores; DSL per ARCHITECTURE.md.

## Flow
Signals → screener families → composite score → candidate list.

## Integration
Used by pipeline, strategy DSL; depends on core, features, regime.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `trait Screener (lib.rs)`
- `struct TrendScreener (lib.rs)`
- `struct BreakoutScreener (lib.rs)`
- `struct MeanReversionScreener (lib.rs)`
- `struct VolatilityExpansionScreener (lib.rs)`
- `struct VolumeSurgeScreener (lib.rs)`
- `struct MomentumDivergenceScreener (lib.rs)`
- `struct SupportResistanceBounceScreener (lib.rs)`
- `struct MomentumScreener (lib.rs)`
- `struct MicrostructureScreener (lib.rs)`
- `struct EventScreener (lib.rs)`
- `fn run_default (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar screener families
[package]
name = "quantaradar-screeners"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
chrono.workspace = true
uuid.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/screeners` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-screeners` / `cargo doc -p quantaradar-screeners --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-screeners` passes
- [ ] `cargo doc -p quantaradar-screeners` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
