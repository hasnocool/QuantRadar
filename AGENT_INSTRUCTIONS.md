# QuantRadar Agent Instructions

## Session State
- **Last action**: Fixed factor_engine.rs compilation errors + rank logic; all tests pass
- **Current status**: Research crate (`quantaradar-research`) builds and tests clean — 9/9 tests pass, 0 warnings
- **Next step**: M10-M12 tasks (definitions not yet persisted — see "Milestone Progress" note)

## Compilation Status
### ✅ lib build: `cargo build -p quantaradar-research` PASS, 0 warnings
### ✅ lib test: `cargo test -p quantaradar-research` PASS — 9/9 (8 lib + 1 integration)

## What Was Actually Fixed (2026-09-10 session)

### Root cause 1: `gen` is a reserved keyword in edition 2024
The workspace uses `edition = "2024"`. `rand 0.8`'s `rng.gen::<f64>()` fails with
`expected identifier, found reserved keyword gen`. Fixed by escaping: `rng.r#gen::<f64>()`
(5 occurrences in test code) plus importing the trait: `use rand::{thread_rng, Rng};`.

### Root cause 2: 0-based vs 1-based ranks
`rank_assets()` emitted 0-based ranks (from `enumerate()`); tests assert 1-based
(rank 1 = best). Fixed with `rank + 1`; percentile corrected to `rank_1based / n`
(previously always `1.0`, which silently disabled the percentile term in `estimate_uncertainty`).

### Warning cleanup (14 → 0)
- Removed unused imports `FeatureRow`, `Regime`; removed 6 redundant `mut`s; removed unused `equal_weight`
- `cargo fix` cleared 8 redundant `mut`s in `process()`
- Made `ScoredAsset` `pub` (was flagged `private_interfaces`)
- Removed never-read `factor_stats` field + `FactorStats` struct

## How to Continue This Session

1. ~~Fix test compilation~~ DONE — all tests pass
2. Update `docs/control-plan/TODO.md` / `MASTERLIST.md` if user requests board updates
3. Continue M10-M12 when milestone definitions are confirmed

## Milestone Progress

> ⚠️ NOTE (2026-09-10): The M1-M12 milestone ladder is referenced in this file but was
> never persisted to any doc. `docs/control-plan/TODO.md` (the live board, formerly
> referenced as `docs/control-plane/TODO.md`) contains QR-001..QR-008 and QR-N01..N05 only.
> M10-M12 definitions must be recovered or re-derived before they can be executed.

- M1-M8: DONE (status carried from prior session; definitions not persisted)
- M9 feature store: VERIFIED (tests PASS)
- M9 factor engine: VERIFIED (build + tests PASS, 0 warnings — 2026-09-10)
- M10-M12: PENDING (definitions missing)

## Key Files
- `crates/shared/research/src/factor_engine.rs` - Factor engine (compiles clean, tests pass)
- `docs/control-plan/TODO.md` - Live task board (QR-001..QR-008; no M-milestones)
- `docs/control-plan/PLAN.md` - P0-P17 roadmap (master+PLAN section mappings in `.omo/plans/phased-plan-mapping.md`)
- `MASTERLIST.md`, `AGENTS.md` - Root cross-links