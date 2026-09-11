# CRATE_CANONICAL_replay.md — quantaradar-replay

## quantaradar-replay

### Purpose
Standalone crate `quantaradar-replay` in the QuantRadar workspace (`crates/standalone/replay`).
# crates/standalone/replay/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/replay/codemap.md)
```text
# crates/standalone/replay/codemap

## Responsibility
Replay/rebuild with dataset integration.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → replay computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct ReplayState (lib.rs)`
- `enum ReplayEvent (lib.rs)`
- `struct MarketStateEvent (lib.rs)`
- `struct SignalEvent (lib.rs)`
- `struct OrderEvent (lib.rs)`
- `struct FillEvent (lib.rs)`
- `struct PortfolioEvent (lib.rs)`
- `struct PnLEvent (lib.rs)`
- `struct ReplayResult (lib.rs)`
- `fn compute_checksum (lib.rs)`
- `struct ReplayEngine (lib.rs)`
- `fn new (lib.rs)`
- `fn replay_dataset (lib.rs)`
- `fn replay_gap (lib.rs)`
- `fn replay (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-replay"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
sha2.workspace = true

# Core domain models and types
quantaradar-core = { path = "../../shared/core" }
quantaradar-data-model = { path = "../../shared/data-model" }

# Parquet/Aarrow for raw archive reading
parquet = { version = "53", features = ["snap", "arrow"] }
arrow = { version = "53", features = ["csv", "json", "ipc"] }

# UUID for event IDs
uuid = { workspace = true, features = ["v4", "serde"] }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-data-model`, `parquet`, `arrow`, `uuid`, `tempfile`

### Integration points
- Workspace member: `crates/standalone/replay` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-replay` / `cargo doc -p quantaradar-replay --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-replay` passes
- [ ] `cargo doc -p quantaradar-replay` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
