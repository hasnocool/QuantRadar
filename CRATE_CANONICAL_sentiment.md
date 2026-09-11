# CRATE_CANONICAL_sentiment.md — quantaradar-sentiment

## quantaradar-sentiment

### Purpose
Standalone crate `quantaradar-sentiment` in the QuantRadar workspace (`crates/standalone/sentiment`).
# crates/standalone/sentiment/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/sentiment/codemap.md)
```text
# crates/standalone/sentiment/codemap

## Responsibility
Sentiment analysis pipeline.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → sentiment computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Sentiment (lib.rs)`
- `fn score_text (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-sentiment"
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
- Workspace member: `crates/standalone/sentiment` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-sentiment` / `cargo doc -p quantaradar-sentiment --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-sentiment` passes
- [ ] `cargo doc -p quantaradar-sentiment` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
