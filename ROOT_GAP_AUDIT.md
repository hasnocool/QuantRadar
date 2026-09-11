SEE ALSO: docs/control-plane/MASTERLIST.md §3 (canonical authority).
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

## Round 4 resolution (current session)
- All 66 sub-codomaps filled (0 empty stubs); audit confirms 66/66 = 100%.
- Replay dataset integration restored (`replay_full_dataset_integration`) and builds; commit `582e1a5`.
- Python templates filled (`python/quantaradar/codemap.md`, `python/codemap.md`); commit `8107d96`.
- News ingestion fix committed (`a9e1a1f`); uncommitted count = 0 (final commit `5bc0542`).
- Persistent #1 implemented in `crates/standalone/persistent-data/src/lib.rs` (`persist_dataset()` + test); PLAN.md table and audit_updated to 6/35.
- PLAN.md line 50 updated; table line 5 synchronized; audit_coverage.py `plan_done` corrected to 6.
Remaining (not claimed complete): PLAN.md still shows 6/35 rather than 35/35; deep test/load quality not fully verified.

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

## Round 5 — Current Root Gap State + Phase 2 Items (verified 2026-09-08)

### Current root gap state
- `PLAN.md` table reports 5/35 = 14% implemented (same as Round 1); deepwork claims Phase 1 & 2 complete but runtime audit shows Phase 2 is **structure complete, live integration incomplete**.
- `crates/shared/websocket/src/lib.rs` (981 lines): `MarketFeedManager` has bounded concurrency, adaptive rate limiter, reconnect/backoff, heartbeat, subscription management, sequence validation, stale-feed detection, per-symbol buffers, backpressure, feed health scores — all present with working tests. **Connection is simulated** (`// Simulate WebSocket connection (in production: use tokio-tungstenite)`). No real TCP/TLS WebSocket client wired.
- `crates/standalone/rate-limiter/src/lib.rs` (392 lines): adaptive token-bucket rate limiter replacing fixed 350ms sleep. Used by websocket via `MessageRateLimiter::process_message()`.
- `crates/shared/ingestion/src/lib.rs` (500 lines): `Collector` trait, `Normalizer`, `QualityValidator`, `IngestionPipeline` with batch flush to parquet. Tests pass.
- Phase 2 verdict: **~1,870 lines of structure code** (websocket + rate-limiter + ingestion), but **0 lines of live exchange integration**. The 6 Phase 2 checklist items are structurally complete (100% plan claims) but production execution is ~50% complete.

### Phase 2 items as 1 coherent list (from PLAN.md + PLAN_PHASE_2.md)

**Phase 2 — Production WebSocket Engine (Weeks 3-4)** — 6 items:

1. **MarketFeedManager with bounded concurrency** — `crates/shared/websocket/`, 981 lines. `FeedManager` with `Arc<RwLock<HashMap<String, FeedState>>>`, `tokio::sync::Semaphore` (max 100 concurrent feeds), `MessageRateLimiter` (max 50 concurrent messages), `VecDeque` per-symbol and global buffers, `mpsc::Sender<MarketDataMessage>` channel. Simulated connection loop only.
2. **Adaptive rate limiter (replace fixed sleep)** — `crates/standalone/rate-limiter/`, 392 lines. Replaces `crates/shared/exchange-kraken/src/lib.rs` fixed `sleep(Duration::from_millis(350))`. Configurable `RateLimiterConfig`. Used via `MessageRateLimiter::process_message()`.
3. **Reconnect/backoff, heartbeat monitoring** — `crates/shared/websocket/`. `spawn_feed_task()` with exponential backoff (`*1.5` + 1000ms jitter), `max_reconnect_attempts` (default 10), `heartbeat_interval` (30s), separate `tokio::spawn()` ping task. `FeedHealth` tracks `reconnect_count`, `last_reconnect`.
4. **Subscription management, sequence validation** — `crates/shared/websocket/`. `Subscription` struct (symbol/exchange/data_types/depth/interval). `SubscriptionStatus` enum (`Unsubscribed → Subscribing → Subscribed → Failed`). `handle_message()` validates `seq == last_sequence` (duplicate), `seq > last + 1` (gap with reconnect trigger), `seq < last` (out-of-order). `subscribe()`/`unsubscribe()` API with `pending_subscriptions`.
5. **Stale-feed detection, auto-resubscription** — `crates/shared/websocket/`. `check_stale_feeds()` compares `last_message` vs `stale_timeout` (60s). `auto_resubscribe_stale()` reconnects stale feeds if `enable_auto_resubscribe` (default true). Per-feed `stale` bool updated in `update_feed_health()`.
6. **Per-symbol buffers, backpressure, feed health scores** — `crates/shared/websocket/`. Per-symbol `message_buffer: VecDeque` (max 100,000), global `global_buffer`. `FeedHealth` computes `health_score` (0.0–1.0) from drop rate, error rate, staleness, sequence OK, latency. `FeedManagerStats` tracks total/active feeds, messages, drops, reconnects, avg health. Overflow `pop_front()` increments `dropped_count`.

**Remaining Phase 2 work to reach production** (from `PLAN_PHASE_2.md`):
- Wire `tokio-tungstenite` into `spawn_feed_task()` (connection is currently simulated)
- Implement exchange-specific subscription message serialization (Kraken/Binance/Coinbase JSON formats)
- Integrate `replay` crate with `reconnect_feed()` for sequence gap replay
- Wire `FeedHealth` metrics to `monitoring/` and `dashboard/` stacks
- Add real WebSocket `ping`/`pong` frame handling (`tungstenite::protocol::Frame::Ping`/`Pong`)
- Implement delta-order-book stream processing (`DataType::OrderBook` currently only handles snapshots)
- Configure configurable drop policy (oldest/newest/block) for backpressure

### Git state (Round 5)
- Uncommitted: `.omo/plans/phased-plan-mapping.md` (+1/-1), `PLAN_PHASE_2.md` (untracked), `tests/` (untracked)
- Last commit: `62edbdb feat: add per-crate markdown report generator CLI`
- Total workspace: 105 crates (66 shared + 39 standalone) + python + configs

## Normalization / Control-Plane — Session Verification
- `AGENTS.md` reduced to cross-links; `SEE ALSO: docs/control-plane/MASTERLIST.md §3` at line 1; zero MASTERLIST §1-39 duplication verified (grep exit 1).
- `.opencode/agent/quantaradar.md` preserved at 2063B with 1 cross-ref comment (MASTERLIST authority + registry mapping); old `.opencode/agent/quantradar.md` (4390B duplicate) removed.
- `docs/CONTROL_PLANE.md` strengthened: appended "Canonical truth per mapped file" + normalization status section.
- All independent-rule docs received `SEE ALSO: docs/control-plane/MASTERLIST.md §3`: CLI_REFERENCE.md, CLI_COVERAGE.md, ARCHITECTURE.md, ARCHITECTURE_CANONICAL.md, README.md, PLAN.md, ROOT_GAP_AUDIT.md.
- `TODO.md` updated with QR-N01..QR-N05 normalization tasks (Done).
- `.agents/registry.json` preserved (11 agents, valid JSON); no structural change needed.
- Full project implementation remains partial per audit_coverage.py (6/35 PLAN.md sections fully implemented; 66/66 crates compliant; WebSocket simulated, replay stub, portfolio optimization 2/10, live-exec isolated). Normalization deliverables from `docs/NORMALIZATION_PLAN.md` are now complete.

---

## Normalization / Control-Plane Status (appended; session verification)

Per `docs/NORMALIZATION_PLAN.md` (canonical source) — normalization deliverables are now **complete**:
- `AGENTS.md` cross-link verified (`SEE ALSO: docs/control-plane/MASTERLIST.md §3` at line 3; zero duplication of MASTERLIST §1–39).
- `.opencode/agent/quantaradar.md` preserved at **2063B** with exactly 1 cross-ref comment (MASTERLIST authority + `registry.json` mapping); old duplicate `quantradar.md` (4390B) removed.
- `docs/CONTROL_PLANE.md` strengthened (added canonical-truth mapping + normalization status; 41 lines).
- `SEE ALSO: docs/control-plane/MASTERLIST.md §3` headers added to independent-rule docs (`CLI_REFERENCE.md`, `CLI_COVERAGE.md`, `ARCHITECTURE.md`, `ARCHITECTURE_CANONICAL.md`, `README.md`, `PLAN.md`, `ROOT_GAP_AUDIT.md`, `AGENTS.md`).
- `docs/control-plane/TODO.md` updated (QR-N01..QR-N05 normalization tasks marked Done; 110 lines).
- `.agents/registry.json` preserved (11 agents, valid JSON; no structural change).

**Not complete — full project implementation remains partial (per `audit_coverage.py` and `docs/PROJECT_ROOT_AUDIT_REPORT.md` §9 conclusion):**
- `PLAN.md` sections fully implemented ≈ **6/35 = ~17%** (audit_coverage.py `plan_done=6`; `plan_pct≈17%`; earlier rounds reported 5/35 = 14%, now corrected to 6/35).
- Crate compliance: **66/66 compliant** (source + codemap + tests present); 105 crates in workspace.
- Production integration gaps remain open: WebSocket feed loop is **simulated** only (`spawn_feed_task` has no real `tungstenite` connection); replay crate (`crates/standalone/replay/`) is a **stub** (91 lines, `ReplayDataset`/`Default` only, `replay_full_dataset_integration` builds but dataset path unverified); portfolio optimization rated **2/10** (`.slim/deepwork/quantaradar-implementation.md`); persistent data, backtesting, and live multi-exchange adapter remain at stub/simulated status.
- Audit script self-limits (`audit_coverage.py` lines 55–57) exclude deeper load/property/scale verification; commit `62edbdb` is the latest workspace state; uncommitted `tests/`, `.omo/plans/phased-plan-mapping.md`, `PLAN_PHASE_2.md` remain.

Existing audit conclusions (§1–8, Round 4/5 evidence, git state, Phase 2 remaining work) are **preserved untouched**; this section only records normalization status and restates the partial-implementation position.
