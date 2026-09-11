# CRATE_CANONICAL_rate-limiter.md — quantaradar-rate-limiter

## quantaradar-rate-limiter

### Purpose
Standalone crate `quantaradar-rate-limiter` in the QuantRadar workspace (`crates/standalone/rate-limiter`).
# rate-limiter

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/rate-limiter/codemap.md)
```text
# rate-limiter

## Responsibility
Rate limiting for ingestion and API access.

## Source Map
- src/lib.rs
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct RateLimiterConfig (lib.rs)`
- `struct TokenBucket (lib.rs)`
- `fn new (lib.rs)`
- `struct AdaptiveRateLimiter (lib.rs)`
- `struct RateLimiterStats (lib.rs)`
- `fn new (lib.rs)`
- `struct ConcurrencyLimiter (lib.rs)`
- `fn new (lib.rs)`
- `fn try_acquire (lib.rs)`
- `fn available_permits (lib.rs)`
- `fn active_count (lib.rs)`
- `struct ConcurrencyPermit (lib.rs)`
- `struct MessageRateLimiter (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-rate-limiter"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow = { workspace = true }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
tokio = { workspace = true, features = ["sync", "time"] }
tracing = { workspace = true }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `anyhow`, `chrono`, `serde`, `tokio`, `tracing`, `tempfile`

### Integration points
- Workspace member: `crates/standalone/rate-limiter` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-rate-limiter` / `cargo doc -p quantaradar-rate-limiter --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-rate-limiter` passes
- [ ] `cargo doc -p quantaradar-rate-limiter` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
