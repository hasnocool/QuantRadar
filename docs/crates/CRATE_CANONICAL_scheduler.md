# CRATE_CANONICAL_scheduler.md — quantaradar-scheduler

## quantaradar-scheduler

### Purpose
Standalone crate `quantaradar-scheduler` in the QuantRadar workspace (`crates/standalone/scheduler`).
# crates/standalone/scheduler/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/scheduler/codemap.md)
```text
# crates/standalone/scheduler/codemap

## Responsibility
Task scheduling and execution.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → scheduler computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Scheduler (lib.rs)`
- `fn new (lib.rs)`
- `fn tick (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-scheduler"
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
- Workspace member: `crates/standalone/scheduler` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-scheduler` / `cargo doc -p quantaradar-scheduler --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-scheduler` passes
- [ ] `cargo doc -p quantaradar-scheduler` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
