# Project Root Gap Audit — Round 1 Evidence

Objective: audit/remaining concrete gaps in project root. Source: session inspection (pwd=/home/hyperion/Projects/QuantRadar).

## Verified gaps with paths

1. Sub-codemaps missing (root codemap.md lines 17-29 references 11 folder codemaps; none present):
   crates/core/codemap.md, crates/cli/codemap.md, crates/features/codemap.md, crates/regime/codemap.md,
   crates/screeners/codemap.md, crates/microstructure/codemap.md, crates/research/codemap.md,
   crates/backtest/codemap.md, crates/reporting/codemap.md, crates/execution/codemap.md,
   crates/exchange-kraken/codemap.md — all MISSING.

2. Python templates empty (same pattern as root comment line 32):
   python/quantaradar/codemap.md — empty stub; python/codemap.md — empty stub.

3. Uncommitted workspace changes (git status):
   M crates/shared/cli/src/lib.rs (dispatch fix uncommitted)
   M crates/standalone/replay/src/lib.rs (replay stub replaced full-integration comment; only stub present)

4. Replay implementation gap (crates/standalone/replay/src/lib.rs):
   Previous line was "// Final replay/rebuild with full dataset integration"; now minimal stub with Default/ReplayDataset; no dataset-integration path verified.

5. PLAN.md implementation low: audit_coverage.py reports 5/35 = 14% (line 25-26 in script; verified by file count of "Implemented" occurrences).

6. Audit script self-limits (audit_coverage.py lines 55-57): deeper test quality, git commit coverage, property/load tests at scale, CLI variants NOT verified — these remain unclosed.

7. Data/reports: data/raw/ has 1 item; reports/markets.json valid (1391 records); reports/scan.json, screen.json, backtest.json exist but no documented schema-check in repo.

8. Root note confirms shallow atlas: "sub-maps are empty templates. Fill per-folder when deep work starts." (codemap.md line 32).

## Not gaps (verified present)
- audit_coverage.py present and runs (66/66 crate compliance reported)
- codemap.md present at root
- Cargo.toml 66 members; cargo workspace builds
- reports files present (markets valid JSON list of 1391)

## Next-round actions needed to close
- Populate at least core sub-codemaps (core, cli) or document why deferred
- Fill python/quantaradar/codemap.md (design/flow sections)
- Commit or discard uncommitted cli/replay edits; complete replay dataset integration or document stub status
- Expand audit_coverage to cover CLI variants / property tests if required by objective
- Confirm PLAN.md 5 implemented sections align with actual crate tests (gap between presence and quality)

## Round 2 additional evidence (verified in session)
- Python stub quote (python/quantaradar/codemap.md lines 3-19): every section is "<!-- Fixer: Fill ... -->" — 0 content; 19 lines total, same for python/codemap.md.
- Replay stub quantitative: crates/standalone/replay/src/lib.rs = 91 lines (was 1 line comment); provides Replay/ReplayDataset/Default but no dataset-integration call verified; previous comment "Final replay/rebuild with full dataset integration" removed.
- CLI uncommitted diff quantified: crates/shared/cli/src/lib.rs = +1/-1 (execute_command expanded from self.dispatch() to match block); not committed.
- Audit script length: audit_coverage.py = 61 lines; explicitly excludes deeper verification (lines 55-57).
- Hidden audit artifacts exist but aren't integrated: .slim/deepwork/plan-100-coverage.md (3014 bytes) and quantaradar-implementation.md (9070 bytes) — not referenced by audit script or root codemap.
- Root ignore configuration present (.gitignore excludes /data/ /reports/ /target/; .ignore references .slim/deepwork) — not a gap, but confirms generated artifacts are excluded, which matches the thin data/raw/ and reports state.
- Total uncommitted new files in root: ROOT_GAP_AUDIT.md (??) plus pre-existing M files; no new sub-codomaps added.

## Round 3 cross-check evidence (verified in session)
- Hidden deepwork artifacts read: `.slim/deepwork/plan-100-coverage.md` confirms 35 PLAN.md items with only 5 implemented; `.slim/deepwork/quantaradar-implementation.md` rates portfolio optimization 2/10, backtesting 3/10, data persistence 1/10, architecture 8/10 — aligns with audit script's 14% (5/35) and reveals lower maturity than crate-level 100% presence suggests.
- Cargo verification: `quantaradar-replay` builds (`cargo check --manifest-path crates/standalone/replay/Cargo.toml`) with 1 unused-variable warning; `quantaradar` (cli) builds (`cargo check --manifest-path crates/shared/cli/Cargo.toml`) with 3 warnings — uncommitted edits don't break compilation, confirming the gap is quality/integration, not syntax.
- No sub-codemaps created; python templates unchanged; replay still stub; audit script unchanged. Gaps remain open with deeper confirmation.
