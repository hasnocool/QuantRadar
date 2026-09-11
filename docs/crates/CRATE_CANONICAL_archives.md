# CRATE_CANONICAL_archives.md — quantaradar-archives

## quantaradar-archives

### Purpose
Shared crate `quantaradar-archives` in the QuantRadar workspace (`crates/shared/archives`).
# crates/shared/archives/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/archives/codemap.md)
```text
# crates/shared/archives/codemap

## Responsibility
Archive/storage of historical datasets and replay artifacts.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → archives computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct ArchivesConfig (lib.rs)`
- `struct DataGap (lib.rs)`
- `enum GapSeverity (lib.rs)`
- `struct SymbolArchive (lib.rs)`
- `fn new (lib.rs)`
- `struct GapStats (lib.rs)`
- `struct BookReconstructor (lib.rs)`
- `fn reconstruct (lib.rs)`
- `fn validate_sequence (lib.rs)`
- `struct ArchiveManager (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-archives"
version.workspace = true
edition.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
quantaradar-data-model = { path = "../data-model" }
quantaradar-storage = { path = "../storage" }
anyhow = { workspace = true }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
tokio = { workspace = true, features = ["sync", "time"] }
tracing = { workspace = true }
uuid = { workspace = true, features = ["v4", "serde"] }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-data-model`, `quantaradar-storage`, `anyhow`, `chrono`, `serde`, `tokio`, `tracing`, `uuid`, `tempfile`

### Integration points
- Workspace member: `crates/shared/archives` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-archives` / `cargo doc -p quantaradar-archives --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-archives` passes
- [ ] `cargo doc -p quantaradar-archives` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
