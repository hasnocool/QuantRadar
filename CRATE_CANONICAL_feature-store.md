# CRATE_CANONICAL_feature-store.md — quantaradar-feature-store

## quantaradar-feature-store

### Purpose
Standalone crate `quantaradar-feature-store` in the QuantRadar workspace (`crates/standalone/feature-store`).
# crates/standalone/feature-store/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/feature-store/codemap.md)
```text
# crates/standalone/feature-store/codemap

## Responsibility
Feature persistence/store.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → feature-store computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `enum FeatureCategory (lib.rs)`
- `struct FeatureStore (lib.rs)`
- `struct FeatureMetadata (lib.rs)`
- `struct FeatureRow (lib.rs)`
- `fn new (lib.rs)`
- `fn register_feature (lib.rs)`
- `fn save (lib.rs)`
- `fn load (lib.rs)`
- `fn as_of (lib.rs)`
- `fn compute_features (lib.rs)`
- `struct FeatureLineage (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-feature-store"
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
- Workspace member: `crates/standalone/feature-store` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-feature-store` / `cargo doc -p quantaradar-feature-store --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-feature-store` passes
- [ ] `cargo doc -p quantaradar-feature-store` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
