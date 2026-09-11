# CRATE_CANONICAL_websocket.md — quantaradar-websocket

## quantaradar-websocket

### Purpose
Shared crate `quantaradar-websocket` in the QuantRadar workspace (`crates/shared/websocket`).
# crates/shared/websocket/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/websocket/codemap.md)
```text
# crates/shared/websocket/codemap

## Responsibility
WebSocket ingestion and event-bus delivery.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → websocket computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct FeedHealth (lib.rs)`
- `struct HealthAlert (lib.rs)`
- `enum DropPolicy (lib.rs)`
- `enum WsMode (lib.rs)`
- `struct GapFillRequest (lib.rs)`
- `type GapFillFn (lib.rs)`
- `enum SubscriptionStatus (lib.rs)`
- `struct MarketDataMessage (lib.rs)`
- `enum DataType (lib.rs)`
- `struct Subscription (lib.rs)`
- `struct FeedManagerConfig (lib.rs)`
- `struct MarketFeedManager (lib.rs)`
- `struct FeedManagerStats (lib.rs)`
- `fn exchange_ws_url (lib.rs)`
- `fn kraken_subscribe_json (lib.rs)`
- `fn binance_subscribe_json (lib.rs)`
- `fn coinbase_subscribe_json (lib.rs)`
- `fn new (lib.rs)`
- `fn subscribe_health (lib.rs)`
- `fn set_gap_fill_provider (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-websocket"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
quantaradar-rate-limiter = { path = "../../standalone/rate-limiter" }
anyhow = { workspace = true }
chrono = { workspace = true, features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
tokio = { workspace = true, features = ["full", "sync"] }
tracing = { workspace = true }
uuid = { workspace = true, features = ["v4", "serde"] }
rand = "0.8"
tokio-tungstenite = { version = "0.24", features = ["rustls-tls-webpki-roots"] }
futures-util = { version = "0.3", features = ["sink", "std"] }

[dev-dependencies]
tempfile = "3"
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-rate-limiter`, `anyhow`, `chrono`, `serde`, `serde_json`, `tokio`, `tracing`, `uuid`, `rand`, `tokio-tungstenite`, `futures-util`, `tempfile`

### Integration points
- Workspace member: `crates/shared/websocket` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-websocket` / `cargo doc -p quantaradar-websocket --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-websocket` passes
- [ ] `cargo doc -p quantaradar-websocket` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
