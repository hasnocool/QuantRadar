# CRATE_CANONICAL_storage.md — quantaradar-storage

## quantaradar-storage

### Purpose
Shared crate `quantaradar-storage` in the QuantRadar workspace (`crates/shared/storage`).
# crates/shared/storage/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/storage/codemap.md)
```text
# crates/shared/storage/codemap

## Responsibility
Persistent storage layer (feature store, dataset persistence).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → storage computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct StorageConfig (lib.rs)`
- `enum Compression (lib.rs)`
- `fn writer_properties (lib.rs)`
- `struct MarketDataWriter (lib.rs)`
- `fn new (lib.rs)`
- `fn write_batch (lib.rs)`
- `struct MarketDataReader (lib.rs)`
- `fn new (lib.rs)`
- `fn read_observations (lib.rs)`
- `fn read_trades (lib.rs)`
- `fn read_order_books (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-storage"
version.workspace = true
edition.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
quantaradar-data-model = { path = "../data-model" }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
anyhow = { workspace = true }
uuid = { workspace = true, features = ["v4", "serde"] }
arrow = { workspace = true }
parquet = { workspace = true }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-data-model`, `chrono`, `serde`, `anyhow`, `uuid`, `arrow`, `parquet`, `tempfile`

### Integration points
- Workspace member: `crates/shared/storage` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-storage` / `cargo doc -p quantaradar-storage --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-storage` passes
- [ ] `cargo doc -p quantaradar-storage` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
