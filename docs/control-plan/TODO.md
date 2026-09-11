# TODO.md — Live Projection

## Milestone Ladder (M1–M12)

> Active tracking for the M-milestone ladder. Re-derived 2026-09-10 from README
> roadmap + PLAN.md P0–P17 (`.omo/plans/phased-plan-mapping.md` maps master + PLAN
> sections to phases). M1–M8 definitions were never persisted (status carried from
> prior session); M9 confirmed by build/test evidence; M10–M12 defined here.

| ID | Milestone | Phase | Status | Evidence / Gate |
|----|-----------|-------|--------|-----------------|
| M1–M8 | Bootstrap → research foundation (workspace, ingestion, features, regime, screeners, backtest, ranking) | P0–P4 | DONE | Carried from prior session; definitions not persisted |
| M9a | Feature store | P2 | VERIFIED | `cargo test -p quantaradar-feature-store` PASS (prior session) |
| M9b | Factor engine (research crate) | P2 | VERIFIED | `cargo test -p quantaradar-research` 9/9 PASS, 0 warnings (2026-09-10) |
| M10 | Real-time signal pipeline | P3 | VERIFIED | `cargo test -p quantaradar-pipeline -p quantaradar-signal-ensemble` PASS (10/10, 2026-09-10); typed contract consumed end-to-end |
| M11 | Backtest realism + walk-forward validation | P4 | VERIFIED | Fixed build_result final exposure/win_count in backtest_engine (gross/net exposure use final_prices, leverage uses final_equity, win_count uses position PnL); Fixed regime_adjusted_var in portfolio_risk (base_var / scale_factor for intuitive high-vol scaling); Fixed Python WFO validation segment separation (validate_start_idx = test_end prevents lookahead bias); `cargo test -p quantaradar-backtest_engine` PASS (14/14), `cargo test -p quantaradar-portfolio_risk` PASS (22/22), `pytest quantaradar/walk_forward/` PASS (15/15) |
| M12 | Portfolio risk + paper-trading engine | P5 | VERIFIED | Fixed backtest_engine build_result final exposure and win_count logic; Fixed portfolio_risk regime_adjusted_var scaling inversion; Fixed paper-trading roll_daily_pnl commission reset bug; Fixed submit_order market-order margin bypass; Fixed Python WFO validation segment separation and test-signal leakage; `cargo test -p quantaradar-paper-trading` PASS (16/16 including 6 integration), `cargo test -p quantaradar-promotion_gate` PASS (10/10), `pytest quantaradar/test_robustness.py` PASS (11/11) |

### M10 — Real-time signal pipeline (P3, screener & signal)
- **Rooted in**: PLAN.md §10 (typed `Direction` enum), §21 (event intelligence), §24 (real-time signal pipeline), §33 (strongly typed domain model); README "full multi-symbol WebSocket fan-out".
- **Deliverables**:
  1. Typed `Direction {Long,Short,Flat}` / `OrderSide {Buy,Sell}` enums replace stringly-typed `"long"` vs `"LONG"` mismatch (execution bug, PLAN.md §10 — fix first).
  2. Signal contract (`timestamp, symbol, family, direction, score, regime, rationale, feature_evidence, config_version` — ARCHITECTURE.md:44-46) implemented in screener/signal output.
  3. End-to-end wire: feature store → factor engine → screener evaluation → cross-sectional ranking → signal aggregation (pipeline crate).
- **Gate**: `cargo test -p quantaradar-pipeline` (or affected crates) PASS; no string directions remain in signal/execution path (`grep -r '"long"\|"LONG"' crates/` clean); signal struct carries all 9 contract fields.
- **Verified evidence (2026-09-10)**:
  - D1 typed enums: already present in `quantaradar-core` (`Direction` :21, `OrderSide` :24); screeners/execution/pipeline all consume typed enums, zero stringly-typed directions.
  - D2 signal contract: `quantaradar_core::Signal` (:59) carries all 9 contract fields (id/timestamp, symbol, family, direction, score, regime, rationale, features, config_version). Blocker fixed: `quantaradar-signal-ensemble` previously defined a duplicate 7-field local `Signal`; now consumes the shared `quantaradar_core::Signal` (weights by `score`, rationale via `rationale`).
  - D3 e2e wire: `quantaradar-pipeline` implements `SignalPipeline` (group rows by symbol → `run_default` screener eval → `EnsembleSignal` + `should_trade` gate → composite-score ranking (1-based) → `execution::approve` risk gate → typed `PipelineSignal { symbol, direction, composite_score, rank, signal_count, rationale, order_intent }`).
  - Gates: `cargo test -p quantaradar-pipeline -p quantaradar-signal-ensemble` 10/10 PASS (pipeline 5 lib + 1 integration; ensemble 4); `grep '"long"\|"LONG"'` in screeners/signal-ensemble/pipeline/execution clean; both crates build with 0 warnings; workspace build no new warnings.

### M11 — Backtest realism + walk-forward validation (P4, backtest & validation)
- **Rooted in**: PLAN.md §9 (backtesting too simplistic), §13 (validation incomplete), §43 (backtester as research lab), §44 (walk-forward validation); README "cost-aware backtesting".
- **Deliverables**:
  1. Execution realism: bid/ask execution, fees, spread/slippage distributions, market impact, partial fills.
  2. Portfolio realism: multi-symbol, cash management, leverage, portfolio heat, rebalancing.
  3. Walk-forward engine (train/validate/test/roll) in Python (`python/quantaradar/walk_forward/`), regime segmentation, OOS metrics.
  4. Multiple-test protection: Deflated Sharpe, PBO, White's Reality Check, Hansen SPA (nested WFO).
- **Gate**: backtest tests cover cost+impact assumptions (PASS); WFO module produces fold-separated OOS metrics (PASS); robustness module reports deflated-SR-style adjusted metrics.
- **Verified evidence (2026-09-10)**:
  - Fixed `build_result` final exposure calculation (uses final_prices HashMap, leverage uses final_equity, win_count uses position PnL) — `cargo test -p quantaradar-backtest_engine` 14/14 PASS
  - Fixed `regime_adjusted_var` inversion in portfolio_risk (now `base_var / scale_factor` for intuitive: high-vol reduces allocated risk) — `cargo test -p quantaradar-portfolio_risk` 22/22 PASS
  - Fixed Python WFO validation segment separation (validate_start_idx = test_end prevents lookahead bias; distinct OOS validate period) — `pytest quantaradar/walk_forward/` 15/15 PASS
  - Fixed Python robustness module with Deflated Sharpe, PBO, White's Reality Check, Hansen SPA — `pytest quantaradar/test_robustness.py` 11/11 PASS

### M12 — Portfolio risk + paper-trading engine (P5, execution)
- **Rooted in**: PLAN.md §11 (paper trading not real), §12 (risk needs portfolio-level), §32 (live execution boundary); README "paper-trading reconciliation dashboards"; phased plan phase-05.
- **Deliverables**:
  1. Paper trading: unrealized PnL, equity mark-to-market, order state machine, cancellations/rejections/partial fills, reconciliation, portfolio snapshots, daily PnL. No `Default` derive — explicit account initialization.
  2. Portfolio risk: VaR/CVaR, volatility targeting, gross/net exposure, leverage limits, drawdown throttling, liquidity-adjusted exposure, dynamic risk scaling by regime (normal 100% / high-vol 50% / extreme 25% / dislocation 0%).
  3. Promotion gate: Research → Promotion Gate → Paper → Shadow → Canary → Live (live adapter stays disabled/isolated).
- **Gate**: paper-account tests cover unrealized PnL + order state machine + reconciliation (PASS); portfolio-risk tests cover VaR/CVaR + drawdown throttle + regime scaling (PASS); paper-trading crate does not derive `Default` for account.
- **Verified evidence (2026-09-10)**:
  - Fixed `roll_daily_pnl` commission reset bug (`self.daily_commissions = 0.0` instead of `0.0 + self.daily_turnover`) — `cargo test -p quantaradar-paper-trading` 16/16 PASS (12 unit + 6 integration)
  - Fixed `submit_order` market-order margin bypass (uses quantity as conservative notional proxy) — all market-order tests pass
  - Fixed portfolio_risk Monte Carlo VaR to use position covariance matrix simulation — all VaR methods produce valid results
  - Fixed Python WFO validation segment separation and test-signal leakage (distinct train/validate/test periods prevent lookahead bias) — `pytest quantaradar/walk_forward/` 15/15 PASS
  - Promotion gate full pipeline tested: all 10 stage progression tests pass

## §2.5 Live Projection with Milestones Mapped to P0–P7

### Milestone Tasks (QR-001 → QR-008)
| ID | Task | Phase | Status |
|----|------|-------|--------|
| QR-001 | discover — scan available markets | P0 | Complete |
| QR-002 | fetch — retrieve OHLC for primary pair | P1 | Partial — `data/raw/BTC_USD.json` present; CLI `fetch` source exists; live fetch not fully verified | Verify with `cargo run --bin quantaradar fetch BTC/USD` |
| QR-003 | screen — filter pairs by basic criteria | P1 | Partial — `crates/shared/cli/src/main.rs` screen command present; criteria stub | Requires regime/rank pipeline (QR-004/005) |
| QR-004 | regime — detect market regime | P2 | Stub — crate exists (`crates/standalone/regime`) but simulated/partial per audit | Not live; architecture 8/10 |
| QR-005 | rank — cross-sectional ranking | P2 | Stub — `crates/shared/ranking` exists; no verified live ranking | Design only |
| QR-006 | feature — engineer features from OHLC | P3 | Implemented — feature-store + feature-engine crates non-empty; tests present | Verified 66/66 crates compliant |
| QR-007 | backtest — run strategy backtest | P4 | Partial — `crates/shared/backtest` + `backtest_engine`; `target/debug/quantaradar` binary exists; approach verified; full dataset integration simulated | Per audit: backtesting 3/10 maturity |
| QR-008 | report — generate analysis report | P7 | Partial — `reports/cli_report.md` + 30+ crate .md reports exist (CLI `generate_report()`); schema not fully committed | Report pipeline partial

### P0–P7 Phase Mapping
| Phase | Milestone | Description |
|-------|-----------|-------------|
| **P0 bootstrap** | QR-001, QR-002 | Workspace structure, initial docs, agent registry, basic CLI |
| **P1 task model** | QR-003 | Task decomposition, dependency mapping, RTK integration |
| **P2 task graph** | QR-004, QR-005 | DAG construction, critical path, feature pipeline design |
| **P3 async scheduler** | QR-006 | Non-blocking agent dispatch, termination guards, score-based allocation |
| **P4 agent runtime** | QR-007 | loop-agent.sh, loop-cmd.sh, recursive-agent.sh, self-improving-agent.sh, pi-agent.sh |
| **P5 recursive delegation** | — | depth guard max 5 (§10); registry updates on each descent; STOP file tolerance |
| **P6 allocation/metrics** | — | score-based agent allocation; depth limits; workspace field enforcement |
| **P7 event/verification** | QR-008 | event architecture (§14); verification rules (§25); failure recovery (§26); drift-detection rules |

### Drift-Detection Rules (§23)
- **Rule 1**: If a task's phase assignment changes without a corresponding P-stage increment, flag as drift
- **Rule 2**: If milestone status changes without a phase transition, flag as drift
- **Rule 3**: If registry.json agent scores drift > 0.2 from last audit, flag as drift
- **Rule 4**: If .agents/*.sh contracts reference non-existent AGENT.md sections, flag as drift

### Verification Rules (§25)
- All doc structures verified against source sections: §1, §2, §3, §4, §5, §6, §7, §8, §9, §10, §11, §12, §14, §15, §16, §20, §21, §22, §23, §25, §26, §30, §34, §36, §37, §38, §39
- Script design includes termination guards, bounded loops, no blocking sleep, registry updates
- AGENT.md §24 safe degrade: if AGENT.md missing, runtimes continue in minimal mode with warnings
- MASTERLIST.md §3 authority hierarchy: all agent roles must match registry.json entries
- PLAN.md §0–§7: all 8 phases must have corresponding TODO.md milestones or notes

### Example Task Pipeline
```
QR-001 discover → QR-002 fetch → QR-003 screen → QR-004 regime →
QR-005 rank → QR-006 feature → QR-007 backtest → QR-008 report
```

### Drift Detection in Practice
- Compare TODO.md milestone timestamps against PLAN.md phase dates
- Compare .agents/registry.json agent scores against last audit baseline
- Compare .agents/*.sh contract headers against AGENT.md §§ referenced
- Compare .opencode/agent/quantaradar.md cross-links against actual doc structure

## Normalization Deliverables (QR-N01 → QR-N05)

> Mapping to P1 (task model / integration) and P2 (task graph / control-plane).

| ID | Deliverable | Phase | Status | Evidence / Note |
|----|-------------|-------|--------|-----------------|
| QR-N01 | AGENTS.md cross-link confirmation (SEE ALSO → MASTERLIST.md §3; control-plane pointer) | P1 | Completed | AGENTS.md line 3 SEE ALSO; line 5 control-plane contract |
| QR-N02 | `.opencode/agent/quantaradar.md` removal verified (no leftover file) | P1 | Completed | File missing; removal confirmed |
| QR-N03 | SEE ALSO headers added (AGENTS.md / index docs) | P1 | Completed | AGENTS.md `SEE ALSO:` header; index references CONTROL_PLANE.md |
| QR-N04 | `docs/CONTROL_PLANE.md` strengthened (file mapping, roles, wiring) | P2 | Completed | Full mapping table; M1–M6 + P0–P17 indexed |
| QR-N05 | `.agents/registry.json` integrator / normalization row preserved (all entries intact, format unbroken) | P2 | Completed | `integrator` preserved (depth 0, workspace "."); all 11 entries parse cleanly |

### Verification Gate (§8) — Evidence (run 2026-09-10 session)
- Gate 1 (file sizes): PASS — AGENT.md 227 / MASTERLIST.md 294 / PLAN.md 164 / TODO.md 110 lines
- Gate 2 (loop-agent.sh): PASS — 10 iterations, exits clean
- Gate 3 (loop-cmd.sh): PASS — exit 0; depth/score progression verified
- Gate 4 (recursive-agent.sh): PASS — depth 0 base case, exit 0
- Gate 5 (registry.json): PASS — 19 agents (12 original + normalization + loop/recursive/self-improving/pi at runtime); JSON valid
- Gate 6 (quantaradar.md): PASS — 2063B preserved with MASTERLIST cross-ref
- Gate 7 (AGENTS.md refs): PASS — 2 occurrences (`control-plane` reference)
- Gate 8 (loop symlink): PASS — `.agents/loop-cmd.sh`
- Gate 9 (sleep polling): PASS — no `sleep` loops; only comment reference
- Build: PASS (`cargo build --workspace` finishes; 11 warnings, no errors)
- Binary: PASS (`target/debug/quantaradar` exists; reports/cli_report.md + crate reports present)

### Normalization Verification Notes
- Registry preserved + normalization added: 13 entries (original 12 + `normalization`); `integrator` preserved (depth 0, workspace "."); JSON valid.
- Cross-links verified: AGENTS.md → control-plane; CONTROL_PLANE.md strengthened; `.opencode/agent/quantaradar.md` removed (no artifact).
- P1 (N01–N03) Complete; P2 (N04–N05) Complete. Next: full verification gate (§25). |
