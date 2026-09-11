# CRATE_CANONICAL_logging.md — quantaradar-logging

## quantaradar-logging

### Purpose
Standalone crate `quantaradar-logging` in the QuantRadar workspace (`crates/standalone/logging`).
Provides focused functionality within the QuantRadar quantitative research platform.

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/logging/codemap.md)
```text
(no codemap.md content — see src/ listing below)
```

### Source layout (`src/`)
- `src/lib.rs`
- `src/main.rs`

### Public API surface (sampled from source)
- `const MAX_BYTES (lib.rs)`
- `const FLUSH_MS (lib.rs)`
- `enum Level (lib.rs)`
- `struct BatchLogger (lib.rs)`
- `fn new (lib.rs)`
- `fn log (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-logging"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["rt", "macros", "time", "sync"] }
chrono = "0.4"

[dev-dependencies]
```

### Dependencies (manifest keys)
`name`, `version`, `edition`, `tokio`, `chrono`

### Integration points
- Workspace member: `crates/standalone/logging` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-logging` / `cargo doc -p quantaradar-logging --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-logging` passes
- [ ] `cargo doc -p quantaradar-logging` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
