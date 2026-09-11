# CRATE_CANONICAL_dashboard.md — quantaradar-dashboard

## quantaradar-dashboard

### Purpose
Shared crate `quantaradar-dashboard` in the QuantRadar workspace (`crates/shared/dashboard`).
# crates/shared/dashboard/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/dashboard/codemap.md)
```text
# crates/shared/dashboard/codemap

## Responsibility
Web/dashboard reporting and visualization.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → dashboard computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct DashMetric (lib.rs)`
- `fn new (lib.rs)`
- `struct FeedHealthCard (lib.rs)`
- `struct DashboardFeeds (lib.rs)`
- `fn upsert (lib.rs)`
- `fn snapshot (lib.rs)`
- `fn criticals (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-dashboard"
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
- Workspace member: `crates/shared/dashboard` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-dashboard` / `cargo doc -p quantaradar-dashboard --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-dashboard` passes
- [ ] `cargo doc -p quantaradar-dashboard` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
