# CRATE_CANONICAL_registry.md — quantaradar-registry

## quantaradar-registry

### Purpose
Standalone crate `quantaradar-registry` in the QuantRadar workspace (`crates/standalone/registry`).
# crates/standalone/registry/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/registry/codemap.md)
```text
# crates/standalone/registry/codemap

## Responsibility
Experiment/registry management.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → registry computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Experiment (lib.rs)`
- `fn new (lib.rs)`
- `fn set_metric (lib.rs)`
- `fn promote (lib.rs)`
- `fn reject (lib.rs)`
- `struct Registry (lib.rs)`
- `fn new (lib.rs)`
- `fn add (lib.rs)`
- `fn get (lib.rs)`
- `fn count (lib.rs)`
- `fn list_champions (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-registry"
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
- Workspace member: `crates/standalone/registry` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-registry` / `cargo doc -p quantaradar-registry --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-registry` passes
- [ ] `cargo doc -p quantaradar-registry` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
