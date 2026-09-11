SEE ALSO: docs/control-plane/MASTERLIST.md §3 (canonical authority).
# CLI Coverage — CLI_REFERENCE.md × implementation

> Reference: `CLI_REFERENCE.md` (311 lines, copied from `/tmp/CLI_REFERENCE.md`).
> Binary: `crates/shared/cli/src/main.rs` (`[[bin]] quantaradar`, 11 flat subcommands).
> Drafts (NOT compiled, no Cargo.toml ref): `main_new.rs` (7-group tree),
> `main_implemented.rs`, `main_clean.rs`, `main_minimal.rs`.
> Legend: ✅ shipped · 🟡 draft-only · ❌ missing everywhere · ➕ shipped but undocumented.

## Data (3/6 shipped)

| Reference | Binary | Draft | Notes |
|-----------|--------|-------|-------|
| `data import` | ✅ DataImport | 🟡 | binary takes `--source --format`, writes `data/<source>.json`; ref's `--output` flag missing |
| `data export` | ❌ | 🟡 | |
| `data ingest` | ❌ | 🟡 | |
| `data list` | ✅ DataList | 🟡 | binary ignores ref's `--filter` |
| `data validate` | ✅ DataValidate | 🟡 | binary is a stub print; ref's `--checks` ignored |
| `data clean` | ❌ | 🟡 | |

## Analyze (7/7 shipped — command parity, flag gaps)

| Reference | Binary | Draft | Notes |
|-----------|--------|-------|-------|
| `analyze discover` | ✅ | 🟡 | binary flagless stub; ref has `--exchange --output` |
| `analyze scan` | ✅ | 🟡 | stub print; ref has `--interval --limit --quote --output` |
| `analyze screen` | ✅ | 🟡 | `--symbol` ok; ref's `--interval --output` missing |
| `analyze features` | ✅ | 🟡 | stub; ref takes symbols + `--output` |
| `analyze regime` | ✅ | 🟡 | `--symbol` ok; ref's `--interval` missing |
| `analyze rank` | ✅ | 🟡 | stub; ref has `--universe --interval --top-n` |
| `analyze pca` | ✅ | 🟡 | stub; ref has `--symbols --interval --components` |

## Research (0/5 shipped)

| Reference | Binary | Draft | Notes |
|-----------|--------|-------|-------|
| `research backtest` | ❌ | 🟡 | draft wires backtest crate; AGENTS.md advertises `backtest <FILE>` — neither form ships |
| `research walkforward` | ❌ | 🟡 | |
| `research generate` | ❌ | 🟡 | |
| `research optimize` | ❌ | 🟡 | |
| `research compare` | ❌ | 🟡 | |

## Strategy (0/5) · Execute (0/5) · Ops (0/6) · Utils (0/5)

All 🟡 draft-only, ❌ in binary: `strategy list/create/compile/test/deploy`,
`execute paper/live/account/order/cancel`, `ops monitor/metrics/health/logs/schedule/jobs`,
`utils fetch/replay/report/config/import-config`. Note AGENTS.md advertises
`fetch <PAIR>` — only exists as draft `utils fetch`.

## Completions (0/1)

| Reference | Binary | Draft |
|-----------|--------|-------|
| `completions bash/zsh/fish` | ❌ | ❌ (no clap_complete wiring anywhere) |

## Undocumented shipped command

| Binary | Reference |
|--------|-----------|
| ➕ `generate-report [--crate --all --tests]` | absent from CLI_REFERENCE.md entirely |

## Score

- Reference → binary: **10/40 unique commands** (25%; analyze 7/7, data 3/6, rest 0).
- Reference → draft tree: **39/40** (all but `completions`).
- Binary → reference: **10/11** (`generate-report` undocumented).

## To reach 100% (two options)

A. **Promote drafts (recommended):** replace `main.rs` with the `main_new.rs`
   7-group tree (or `main_implemented.rs` where behavior exists), add
   `completions` via `clap_complete`, document `generate-report` (or drop it).
   Closes 29/30 gaps in one move; conflicts with AGENTS.md's flat
   `discover/scan/fetch/screen/backtest` form — pick one CLI shape first.
B. **Trim reference:** mark research/strategy/execute/ops/utils/completions as
   planned (they are Stubs per root PLAN.md themes 8–12) so the doc matches the
   11-command binary. Smaller diff, but cements the flat-vs-grouped fork.
<!-- ponytail: matrix only. Re-run after any CLI change; Implementation Order: decide shape (A vs B) → implement → re-verify this table. -->
