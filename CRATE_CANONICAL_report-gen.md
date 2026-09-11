# CRATE_CANONICAL_report-gen.md — quantaradar-report-gen

## quantaradar-report-gen

### Purpose
Standalone crate `quantaradar-report-gen` in the QuantRadar workspace (`crates/standalone/report-gen`).
Provides focused functionality within the QuantRadar quantitative research platform.

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/report-gen/codemap.md)
```text
(no codemap.md content — see src/ listing below)
```

### Source layout (`src/`)
- `src/main.rs`

### Public API surface (sampled from source)
- (no `pub` items sampled — see source files above; run `cargo doc -p quantaradar-report-gen` for full API)

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-report-gen"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde_json = "1"
```

### Dependencies (manifest keys)
`name`, `version`, `edition`, `clap`, `serde_json`

### Integration points
- Workspace member: `crates/standalone/report-gen` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-report-gen` / `cargo doc -p quantaradar-report-gen --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-report-gen` passes
- [ ] `cargo doc -p quantaradar-report-gen` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
