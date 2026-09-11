# CRATE_CANONICAL_ranking.md — quantaradar-ranking

## quantaradar-ranking

### Purpose
Shared crate `quantaradar-ranking` in the QuantRadar workspace (`crates/shared/ranking`).
# crates/shared/ranking/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/ranking/codemap.md)
```text
# crates/shared/ranking/codemap

## Responsibility
Cross-sectional ranking and relative-strength.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → ranking computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct RankScore (lib.rs)`
- `fn rank_all (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-ranking"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/ranking` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-ranking` / `cargo doc -p quantaradar-ranking --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-ranking` passes
- [ ] `cargo doc -p quantaradar-ranking` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
