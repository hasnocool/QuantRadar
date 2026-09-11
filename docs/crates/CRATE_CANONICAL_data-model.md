# CRATE_CANONICAL_data-model.md — quantaradar-data-model

## quantaradar-data-model

### Purpose
Shared crate `quantaradar-data-model` in the QuantRadar workspace (`crates/shared/data-model`).
# crates/shared/data-model/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/data-model/codemap.md)
```text
# crates/shared/data-model/codemap

## Responsibility
Typed data-model definitions and schemas.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → data-model computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct MarketObservation (lib.rs)`
- `fn new (lib.rs)`
- `fn from_core (lib.rs)`
- `fn to_core (lib.rs)`
- `fn is_valid (lib.rs)`
- `fn mid_price (lib.rs)`
- `fn spread (lib.rs)`
- `fn spread_bps (lib.rs)`
- `struct TradeObservation (lib.rs)`
- `struct OrderBookObservation (lib.rs)`
- `struct MarketDataBatch (lib.rs)`
- `fn new (lib.rs)`
- `fn with_capacity (lib.rs)`
- `fn is_empty (lib.rs)`
- `fn len (lib.rs)`
- `struct DatasetManifest (lib.rs)`
- `enum DatasetType (lib.rs)`
- `fn compute_checksum (lib.rs)`
- `const STORAGE_LAYOUT (lib.rs)`
- `fn ensure_storage_layout (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-data-model"
version.workspace = true
edition.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
uuid = { workspace = true, features = ["v4", "serde"] }
anyhow = { workspace = true }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `chrono`, `serde`, `uuid`, `anyhow`

### Integration points
- Workspace member: `crates/shared/data-model` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-data-model` / `cargo doc -p quantaradar-data-model --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-data-model` passes
- [ ] `cargo doc -p quantaradar-data-model` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
