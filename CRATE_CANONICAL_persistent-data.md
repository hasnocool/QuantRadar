# CRATE_CANONICAL_persistent-data.md — quantaradar-persistent-data

## quantaradar-persistent-data

### Purpose
Standalone crate `quantaradar-persistent-data` in the QuantRadar workspace (`crates/standalone/persistent-data`).
# crates/standalone/persistent-data/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/persistent-data/codemap.md)
```text
# crates/standalone/persistent-data/codemap

## Responsibility
Persistent market data and replay.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → persistent-data computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct DataManifest (lib.rs)`
- `struct DatasetMeta (lib.rs)`
- `struct QualityFlag (lib.rs)`
- `struct DataStore (lib.rs)`
- `fn new (lib.rs)`
- `fn insert (lib.rs)`
- `fn query (lib.rs)`
- `fn query_range (lib.rs)`
- `fn get_meta (lib.rs)`
- `fn save_manifest (lib.rs)`
- `fn load_manifest (lib.rs)`
- `fn save_dataset (lib.rs)`
- `fn delete (lib.rs)`
- `fn list_keys (lib.rs)`
- `fn check_quality (lib.rs)`
- `fn add_dataset (lib.rs)`
- `fn verify_checksum (lib.rs)`
- `fn persist_dataset (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-persistent-data"
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
- Workspace member: `crates/standalone/persistent-data` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-persistent-data` / `cargo doc -p quantaradar-persistent-data --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-persistent-data` passes
- [ ] `cargo doc -p quantaradar-persistent-data` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
