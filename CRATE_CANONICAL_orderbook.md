# CRATE_CANONICAL_orderbook.md — orderbook

## orderbook

### Purpose
Standalone crate `orderbook` in the QuantRadar workspace (`crates/standalone/orderbook`).
# orderbook

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/orderbook/codemap.md)
```text
# orderbook

## Responsibility
L2 order-book model (delta/rebuild deferred to #3).

## Source Map
- `src/lib.rs`: `OrderBook`, `spread()`, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct L2Depth (lib.rs)`
- `struct OrderBookL2 (lib.rs)`
- `fn new (lib.rs)`
- `fn spread (lib.rs)`
- `fn mid (lib.rs)`
- `fn depth_imbalance (lib.rs)`
- `fn apply_delta (lib.rs)`
- `fn rebuild (lib.rs)`
- `enum OrderSide (lib.rs)`

### Cargo manifest (abridged)
```toml
# L2 order-book model (delta/rebuild stub).
[package]
name = "orderbook"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
serde.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/orderbook` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p orderbook` / `cargo doc -p orderbook --no-deps`.

### Verification checklist
- [ ] `cargo check -p orderbook` passes
- [ ] `cargo doc -p orderbook` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
