# CRATE_CANONICAL_events.md — quantaradar-events

## quantaradar-events

### Purpose
Standalone crate `quantaradar-events` in the QuantRadar workspace (`crates/standalone/events`).
# crates/standalone/events/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/events/codemap.md)
```text
# crates/standalone/events/codemap

## Responsibility
Event definitions (trade, quote, order).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → events computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Event (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-events"
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
- Workspace member: `crates/standalone/events` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-events` / `cargo doc -p quantaradar-events --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-events` passes
- [ ] `cargo doc -p quantaradar-events` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
