# CRATE_CANONICAL_monitoring.md — quantaradar-monitoring

## quantaradar-monitoring

### Purpose
Standalone crate `quantaradar-monitoring` in the QuantRadar workspace (`crates/standalone/monitoring`).
# crates/standalone/monitoring/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/monitoring/codemap.md)
```text
# crates/standalone/monitoring/codemap

## Responsibility
Monitoring and health checks.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → monitoring computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct MonitorHealth (lib.rs)`
- `fn check (lib.rs)`
- `struct FeedHealthSnapshot (lib.rs)`
- `struct HealthRegistry (lib.rs)`
- `fn record (lib.rs)`
- `fn criticals (lib.rs)`
- `fn len (lib.rs)`
- `fn is_empty (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-monitoring"
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
- Workspace member: `crates/standalone/monitoring` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-monitoring` / `cargo doc -p quantaradar-monitoring --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-monitoring` passes
- [ ] `cargo doc -p quantaradar-monitoring` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
