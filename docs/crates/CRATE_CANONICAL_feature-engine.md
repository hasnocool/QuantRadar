# CRATE_CANONICAL_feature-engine.md — quantaradar-feature-engine

## quantaradar-feature-engine

### Purpose
Standalone crate `quantaradar-feature-engine` in the QuantRadar workspace (`crates/standalone/feature-engine`).
# crates/standalone/feature-engine/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/feature-engine/codemap.md)
```text
# crates/standalone/feature-engine/codemap

## Responsibility
Feature computation engine.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → feature-engine computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct FeatureSet (lib.rs)`
- `struct FeatureEngine (lib.rs)`
- `fn new (lib.rs)`
- `fn process_series (lib.rs)`
- `fn process (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-feature-engine"
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
- Workspace member: `crates/standalone/feature-engine` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-feature-engine` / `cargo doc -p quantaradar-feature-engine --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-feature-engine` passes
- [ ] `cargo doc -p quantaradar-feature-engine` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
