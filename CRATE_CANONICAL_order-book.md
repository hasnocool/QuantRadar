# CRATE_CANONICAL_order-book.md — quantaradar-order-book

## quantaradar-order-book

### Purpose
Shared crate `quantaradar-order-book` in the QuantRadar workspace (`crates/shared/order-book`).
# crates/shared/order-book/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/order-book/codemap.md)
```text
# crates/shared/order-book/codemap

## Responsibility
Order-book models and depth analytics.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → order-book computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Level (lib.rs)`
- `fn new (lib.rs)`
- `fn value (lib.rs)`
- `struct BookSide (lib.rs)`
- `fn new (lib.rs)`
- `fn update (lib.rs)`
- `fn best_price (lib.rs)`
- `fn best_level (lib.rs)`
- `fn get (lib.rs)`
- `fn top_n (lib.rs)`
- `fn all_levels (lib.rs)`
- `fn total_volume (lib.rs)`
- `fn total_value (lib.rs)`
- `fn len (lib.rs)`
- `fn is_empty (lib.rs)`
- `fn volume_within_bps (lib.rs)`
- `fn volume_at_usd_levels (lib.rs)`
- `struct OrderBook (lib.rs)`
- `fn new (lib.rs)`
- `fn best_bid (lib.rs)`
- `fn best_ask (lib.rs)`
- `fn spread (lib.rs)`
- `fn mid_price (lib.rs)`
- `fn spread_bps (lib.rs)`
- `fn apply_delta (lib.rs)`
- `fn apply_snapshot (lib.rs)`
- `fn reconstruct_from_deltas (lib.rs)`
- `fn depth_at_usd (lib.rs)`
- `fn imbalance (lib.rs)`
- `fn weighted_mid (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-order-book"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
anyhow = { workspace = true }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
tokio = { workspace = true, features = ["sync", "time"] }
tracing = { workspace = true }
uuid = { workspace = true, features = ["v4", "serde"] }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `anyhow`, `chrono`, `serde`, `tokio`, `tracing`, `uuid`, `tempfile`

### Integration points
- Workspace member: `crates/shared/order-book` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-order-book` / `cargo doc -p quantaradar-order-book --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-order-book` passes
- [ ] `cargo doc -p quantaradar-order-book` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
