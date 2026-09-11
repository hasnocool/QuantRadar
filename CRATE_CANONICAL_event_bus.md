# CRATE_CANONICAL_event_bus.md — quantaradar-event_bus

## quantaradar-event_bus

### Purpose
Standalone crate `quantaradar-event_bus` in the QuantRadar workspace (`crates/standalone/event_bus`).
# crates/standalone/event_bus/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/event_bus/codemap.md)
```text
# crates/standalone/event_bus/codemap

## Responsibility
Event bus for market-data distribution.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → event_bus computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct EventBus (lib.rs)`
- `fn new (lib.rs)`
- `fn subscribe (lib.rs)`
- `fn emit (lib.rs)`
- `fn listen (lib.rs)`
- `fn events (lib.rs)`
- `fn count (lib.rs)`
- `fn route_event (lib.rs)`
- `fn persist_events (lib.rs)`
- `fn persist_to_disk (lib.rs)`
- `fn replay_from_disk (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-event_bus"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
quantaradar-core = { path = "../../shared/core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/standalone/event_bus` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-event_bus` / `cargo doc -p quantaradar-event_bus --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-event_bus` passes
- [ ] `cargo doc -p quantaradar-event_bus` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
