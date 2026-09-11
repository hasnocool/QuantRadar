# CRATE_CANONICAL_news_ingestion.md — news_ingestion

## news_ingestion

### Purpose
Shared crate `news_ingestion` in the QuantRadar workspace (`crates/shared/news_ingestion`).
# news_ingestion

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/news_ingestion/codemap.md)
```text
# news_ingestion

## Responsibility
News ingestion: Reddit RSS, Google News, on-chain, alerting.

## Source Map
- src/lib.rs, src/reddit_rss.rs, src/reddit_json.rs, src/google_news.rs, src/onchain.rs, src/storage.rs, src/alerting.rs
```

### Source layout (`src/`)
- `src/alerting.rs`
- `src/bin/run_ingest.rs`
- `src/google_news.rs`
- `src/lib.rs`
- `src/news_api.rs`
- `src/onchain.rs`
- `src/reddit_json.rs`
- `src/reddit_rss.rs`
- `src/storage.rs`

### Public API surface (sampled from source)
- `struct AlertConfig (alerting.rs)`
- `struct Alerter (alerting.rs)`
- `fn new (alerting.rs)`
- `fn check_news (alerting.rs)`
- `fn check_onchain (alerting.rs)`
- `struct GoogleNewsCollector (google_news.rs)`
- `fn new (google_news.rs)`
- `struct NewsItem (lib.rs)`
- `enum NewsSource (lib.rs)`
- `struct OnChainEvent (lib.rs)`
- `mod reddit_rss (lib.rs)`
- `mod reddit_json (lib.rs)`
- `mod google_news (lib.rs)`
- `mod news_api (lib.rs)`
- `mod onchain (lib.rs)`
- `mod storage (lib.rs)`
- `mod alerting (lib.rs)`
- `struct IngestionConfig (lib.rs)`
- `struct NewsApiCollector (news_api.rs)`
- `fn new (news_api.rs)`
- `struct OnChainRpcClient (onchain.rs)`
- `fn new (onchain.rs)`
- `struct RedditJsonCollector (reddit_json.rs)`
- `fn new (reddit_json.rs)`
- `struct RedditRssCollector (reddit_rss.rs)`
- `fn new (reddit_rss.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "news_ingestion"
version = "0.2.0"
edition = "2024"

[dependencies]
anyhow = { workspace = true }
chrono = { workspace = true }
reqwest = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
arrow = { workspace = true }
parquet = { workspace = true }
urlencoding = "2"
regex = "1"
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `version`, `edition`, `anyhow`, `chrono`, `reqwest`, `serde`, `serde_json`, `tokio`, `arrow`, `parquet`, `urlencoding`, `regex`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/news_ingestion` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p news_ingestion` / `cargo doc -p news_ingestion --no-deps`.

### Verification checklist
- [ ] `cargo check -p news_ingestion` passes
- [ ] `cargo doc -p news_ingestion` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
