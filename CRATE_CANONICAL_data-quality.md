# CRATE_CANONICAL_data-quality.md — data-quality

## data-quality

### Purpose
Standalone crate `data-quality` in the QuantRadar workspace (`crates/standalone/data-quality`).
# data-quality

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/data-quality/codemap.md)
```text
# data-quality

## Responsibility
Validation metrics, completeness checks, and anomaly flags for ingestion and feature streams.

## Source Map
- `src/lib.rs`: `QualityReport`, `assess_quality()`, tests

## Dependencies
- `anyhow`, `serde`, `chrono`

## Tests
- `cargo test -p data-quality`
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct QualityReport (lib.rs)`
- `fn assess_quality (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar data-quality validation and metrics
[package]
name = "data-quality"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
serde.workspace = true
chrono.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/data-quality` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p data-quality` / `cargo doc -p data-quality --no-deps`.

### Verification checklist
- [ ] `cargo check -p data-quality` passes
- [ ] `cargo doc -p data-quality` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
