# CRATE_CANONICAL_ensemble.md — quantaradar-ensemble

## quantaradar-ensemble

### Purpose
Standalone crate `quantaradar-ensemble` in the QuantRadar workspace (`crates/standalone/ensemble`).
# crates/standalone/ensemble/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/ensemble/codemap.md)
```text
# crates/standalone/ensemble/codemap

## Responsibility
Ensemble modeling and aggregation.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → ensemble computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Ensemble (lib.rs)`
- `fn new (lib.rs)`
- `fn add_weight (lib.rs)`
- `fn score (lib.rs)`
- `fn add_score (lib.rs)`
- `fn aggregate (lib.rs)`
- `struct EnsembleVote (lib.rs)`
- `fn vote (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-ensemble"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/ensemble` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-ensemble` / `cargo doc -p quantaradar-ensemble --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-ensemble` passes
- [ ] `cargo doc -p quantaradar-ensemble` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
