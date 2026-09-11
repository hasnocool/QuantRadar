# CRATE_CANONICAL_research.md — quantaradar-research

## quantaradar-research

### Purpose
Shared crate `quantaradar-research` in the QuantRadar workspace (`crates/shared/research`).
# crates/research/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/research/codemap.md)
```text
# crates/research/

## Responsibility
Breadth, ranking, relative-strength, PCA, event studies.

## Design
PCA engine, ranking algorithms, event-study statistics.

## Flow
Market data → ranking/PCA → research reports → promotion gates.

## Integration
Used by screening, champion/challenger; depends on core, features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct AssetSnapshot (lib.rs)`
- `struct Breadth (lib.rs)`
- `struct RelativeStrength (lib.rs)`
- `struct RankedAsset (lib.rs)`
- `struct Event (lib.rs)`
- `struct StrategySpec (lib.rs)`
- `fn breadth (lib.rs)`
- `fn relative_strength (lib.rs)`
- `fn rank (lib.rs)`
- `fn events (lib.rs)`
- `fn correlation_matrix (lib.rs)`
- `fn pca_first_component (lib.rs)`
- `fn generate_strategies (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar research crate manifest
[package]
name = "quantaradar-research"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
[dependencies]
chrono.workspace = true
serde.workspace = true
serde_json.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/research` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-research` / `cargo doc -p quantaradar-research --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-research` passes
- [ ] `cargo doc -p quantaradar-research` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
