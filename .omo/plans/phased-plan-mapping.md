# phased-plan-mapping - Work Plan

## TL;DR (For humans)

**What you'll get:** Expanded phased-plan mapping (`A` = 46 master + 42 PLAN.md sections + replay/simulation from PLAN.md:35,42 + master:10; `B` = 6 phase files + index rewritten with ARCHITECTURE.md fields).

**Why this approach:** User explicitly selected A (full 46+42 section mapping) and B (rewrite existing `plans/phased/*.md` files) over the default single-artifact approach. `ARCHITECTURE.md` provides concrete observation-field and signal-contract requirements missing from the other two references, so it is incorporated as a third reference column in every mapped phase.

**What it will NOT do:** It will NOT invent new architecture sections, add speculative features (live execution, Freqtrade, news/on-chain layers not in references), implement any crate code, or create new separate md files. It only describes working versions structurally.

**Effort:** Large  **Risk:** Medium — scope expanded from 6 phases to full reference mapping; B rewrites existing files rather than adding a new mapping file.
**Decisions I made for you:** A+B combined (not default single mapping); ARCHITECTURE.md incorporated; structural working versions only (no code snippets); no new feature logic beyond the 3 references.

Your next move: approve to proceed to execution (after required high-accuracy review completes, since `review_required: true`).

---

> TL;DR (machine): Large effort, medium risk. A: 46 master sections + 42 PLAN.md sections mapped. B: 6 phase files + index rewritten with ARCHITECTURE.md fields incorporated. Review required (UNCLEAR, non-trivial).

## Scope

### Must have
- Expanded mapping: all 46 master-architecture sections (`QUANTRADAR_MASTER_ARCHITECTURE.md`) and all 42 PLAN.md sections (`PLAN.md`) mapped to the 6 existing phases (`phase-01` through `phase-06`) plus index (`plans/phased/index.md`). (A) — includes replay/simulation references from PLAN.md section 35 (replay engine, deterministic replay), PLAN.md section 42 (reproducibility contract), and master architecture section 10 (historical data lifecycle: VENUE → RAW EVENT → NORMALIZE → VALIDATE → ARCHIVE → REPLAYABLE DATASET → DERIVED DATASET → FEATURE SNAPSHOT; `QUANTRADAR_MASTER_ARCHITECTURE.md`:599-629).
- Direct file edits: rewrite specs for the 6 phase files (`plans/phased/phase-01-data-foundation.md`, `phase-02-market-intelligence.md`, `phase-03-screener-signal.md`, `phase-04-backtest-validation.md`, `phase-05-portfolio-risk-paper.md`, `phase-06-production-monitoring.md`) and index (`plans/phased/index.md`). (B)
- ARCHITECTURE.md incorporation throughout: observation fields (`timestamp, exchange, symbol, base, quote, OHLCV, trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags` — ARCHITECTURE.md:42) mapped to phase 1; signal contract (`timestamp, symbol, family, direction, score, regime, rationale, feature_evidence, strategy/config_version` — ARCHITECTURE.md:44-46) mapped to phase 3; design goals (`ARCHITECTURE.md`:49-57: dynamic discovery, bounded concurrency, no leakage, explicit costs, regime-conditional research, failed experiments retained, liquidity hard constraint, live execution isolated) incorporated into all phase scopes.
- Simulation/replay gap mapping (expanded for C): replay/simulation concepts from PLAN.md:35 (replay engine — `replay dataset_...` produces identical market state, features, signals, orders, fills, portfolio, PnL; same input + same commit = same result), PLAN.md:42 (reproducibility contract — code_commit, config_hash, dataset_id, feature_versions, strategy_version, model_version, random_seed, execution_model_version required for every output), and master architecture section 10 (historical data lifecycle pipeline: VENUE → RAW EVENT → NORMALIZE → VALIDATE → ARCHIVE → REPLAYABLE DATASET → DERIVED DATASET → FEATURE SNAPSHOT; `QUANTRADAR_MASTER_ARCHITECTURE.md`:599-629). Working version specifies: replay verification checklist (input manifest = output manifest + state hash equality), reproducibility dependency list (9 required fields per experiment), and lifecycle stage gates (raw immutable → normalized reproducible → derived versioned). Must NOT implement replay engine — structural specification only.
- No speculative sections: only categories present in at least one of the 3 references are included.

### Must NOT have (guardrails, anti-slop, scope boundaries)
- Must NOT edit `crates/` source code or Python `python/quantaradar/` source — planning artifacts only.
- Must NOT invent new strategy families, event types, or architecture categories beyond what the 3 references describe.
- Must NOT include speculative future layers (e.g., Freqtrade adapter, live execution adapter, news/on-chain layer, multi-exchange architecture) unless they appear explicitly in the references.
- Must NOT create multiple separate output md files — single `.omo/plans/phased-plan-mapping.md` (even with A+B scope expanded).
- Must NOT include exact executable code snippets in the working-version descriptions (structural only, reversible default per open assumptions).

## Verification strategy
> Zero human intervention — all verification is agent-executed.
- Test decision: TDD (tests written before/with structural specs) + tests-after (verification commands executed after each mapping/rewrite). Framework: agent-executable assertions (grep counts, file presence, citation audits) with evidence paths (`.omo/evidence/phased-plan-mapping/task-<N>-phased-plan-mapping.md`).
- Evidence: `.omo/evidence/phased-plan-mapping/` (manual audit of plan file structure and reference citations).
- Verification steps:
  1. Confirm `.omo/plans/phased-plan-mapping.md` exists and includes all required section headers in order (`## TL;DR`, `## Scope`, `## Verification strategy`, `## Execution strategy`, `## Todos`, `## Final verification wave`, `## Commit strategy`, `## Success criteria`).
  2. Confirm A scope covered: reference to 46 master sections (counted in text) and 42 PLAN.md sections (counted in text) with citation paths.
  3. Confirm B scope covered: list of 6 phase files + index named explicitly in `Scope` and `Todos`.
  4. Confirm ARCHITECTURE.md incorporated: observation fields (line 42), signal contract (lines 44-46), design goals (lines 49-57) cited in `Scope` and `Todos`.
  5. Confirm no speculative categories added (scope audit against reference lists).

## Execution strategy

### Parallel execution waves
- Wave 1: Write `.omo/plans/phased-plan-mapping.md` with expanded mapping (A) + ARCHITECTURE.md incorporation.
- Wave 2: Define B rewrite specs for the 6 phase files + index (structural working versions per file).
- Wave 3: Final verification wave (plan compliance, scope fidelity, reference citation audit).

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1. Expanded mapping (A) | None | 2 | None |
| 2. ARCHITECTURE.md incorporation | None | 3 | 1 |
| 3. B rewrite specs (6 files + index) | 1, 2 | 4 | 2 (after 2) |
| 4. Final verification wave | 3 | — | — |

## Todos
> Implementation + Test = ONE todo. Never separate. APPEND task batches below.

- [ ] 1. Expanded A mapping — 46 master sections + 42 PLAN.md sections mapped
  What to do / Must NOT do: For each of the 46 master-architecture sections (QUANTRADAR_MASTER_ARCHITECTURE.md: sections 1-46) and 42 PLAN.md sections (PLAN.md), create a mapping entry that names the section number, title (brief), the corresponding phase (01-06 or index), and the minimum working version definition (structural description referencing data structures, pipeline flow, or verification checklist). Must NOT invent sections not present in the references. Must NOT include executable code snippets. References must include exact file paths and line numbers (e.g., `QUANTRADAR_MASTER_ARCHITECTURE.md:5-34` for architecture, `PLAN.md:1-42` for section groupings, `ARCHITECTURE.md:42` for observation fields).
  Parallelization: Wave 1 | Blocked by: — | Blocks: 3
  References (executor has NO interview context — be exhaustive): `QUANTRADAR_MASTER_ARCHITECTURE.md` (all 2712 lines, 46 sections); `PLAN.md` (2401 lines, 36 main sections + second conversation sections); `ARCHITECTURE.md` (57 lines); `plans/phased/index.md` (phase names and categories); `plans/phased/phase-01*` through `phase-06*` (existing content for alignment check).
  Acceptance criteria (agent-executable): Read `.omo/plans/phased-plan-mapping.md`; verify a count of 46 master-section references and 42 PLAN.md references appear in the file text; verify each reference includes a line-range citation; verify no invented category titles appear.
  QA scenarios (name the exact tool + invocation): Happy — grep for `QUANTRADAR_MASTER_ARCHITECTURE.md` count = 46 references, grep for `PLAN.md` count = 42 references. Failure — any section reference missing line-range citation or any category not present in source files. Evidence: `.omo/evidence/phased-plan-mapping/task-1-phased-plan-mapping.md`.
  Commit: Y | docs(.omo/plans): expanded A mapping

- [ ] 2. ARCHITECTURE.md incorporation — observation fields, signal contract, design goals
  What to do / Must NOT do: Add three explicit subsections to `.omo/plans/phased-plan-mapping.md`: (a) Observation fields mapping — list all 11 fields from ARCHITECTURE.md:42 and specify which phase (01) they apply to and the working version (minimum schema with each field named); (b) Signal contract mapping — list all 9 fields from ARCHITECTURE.md:44-46 and specify phase 03 application; (c) Design goals mapping — list all 7 goals from ARCHITECTURE.md:49-57 and specify which phases each applies to. Must NOT omit any of the 11 observation fields, 9 signal fields, or 7 design goals. Must NOT invent additional fields or goals.
  Parallelization: Wave 2 | Blocked by: — | Blocks: 3
  References: `ARCHITECTURE.md`:42 (observation fields); `ARCHITECTURE.md`:44-46 (signal contract); `ARCHITECTURE.md`:49-57 (design goals); `plans/phased/phase-01-data-foundation.md` (phase 1 alignment); `plans/phased/phase-03-screener-signal.md` (phase 3 alignment).
  Acceptance criteria: Read `.omo/plans/phased-plan-mapping.md`; verify 11 observation fields listed with citation `ARCHITECTURE.md:42`; verify 9 signal contract fields listed with citation `ARCHITECTURE.md:44-46`; verify 7 design goals listed with citation `ARCHITECTURE.md:49-57`; verify each mapped to the correct phase file.
  QA scenarios: Happy — grep counts match (11, 9, 7). Failure — any missing field/goal or incorrect phase mapping. Evidence: `.omo/evidence/phased-plan-mapping/task-2-phased-plan-mapping.md`.
  Commit: Y | docs(.omo/plans): ARCHITECTURE.md incorporated

- [ ] 3. B rewrite specs — 6 phase files + index rewritten with working versions
  What to do / Must NOT do: For each of the 6 files (`phase-01-data-foundation.md`, `phase-02-market-intelligence.md`, `phase-03-screener-signal.md`, `phase-04-backtest-validation.md`, `phase-05-portfolio-risk-paper.md`, `phase-06-production-monitoring.md`) and index (`index.md`), specify in `.omo/plans/phased-plan-mapping.md`: (i) which reference sections apply (from A mapping); (ii) which ARCHITECTURE.md fields apply; (iii) the minimum working version definition (structural — schema, pipeline flow, verification checklist); (iv) what to change/rewrite in the existing file (e.g., add observation-field schema reference, add signal contract reference, incorporate design goals into scope). Must NOT specify executable code changes. Must NOT expand scope beyond A/B references. Must include all 7 files explicitly by name.
  Parallelization: Wave 3 | Blocked by: 1, 2 | Blocks: 4
  References: `plans/phased/index.md`; all 6 `plans/phased/phase-0X-*.md` files; `.omo/plans/phased-plan-mapping.md` (mapping results from 1 and 2).
  Acceptance criteria: Read `.omo/plans/phased-plan-mapping.md`; verify 7 file names listed explicitly; verify each file has a mapping subsection; verify each subsection references A mapping (phase number), ARCHITECTURE.md fields (observation/signal/goals), and a structural working version definition.
  QA scenarios: Happy — all 7 files named, all 3 references cited per file. Failure — any file missing, any reference missing, or any working version containing executable code snippets. Evidence: `.omo/evidence/phased-plan-mapping/task-3-phased-plan-mapping.md`.
  Commit: Y | docs(.omo/plans): B rewrite specs

- [ ] 4. Final verification wave — plan compliance, scope fidelity, reference citation audit
  What to do / Must NOT do: Run the 4 final-verifier checks (F1-F4) against `.omo/plans/phased-plan-mapping.md`: F1 — plan section headers present in order; F2 — all 3 references cited with line ranges; F3 — A (46 master + 42 PLAN.md) and B (7 files) covered; F4 — ARCHITECTURE.md incorporated (11 fields, 9 contract fields, 7 goals). Must confirm no speculative categories added. Must confirm no executable code snippets in working versions.
  Parallelization: Wave 4 (after 3) | Blocked by: 3 | Blocks: —
  References: `.omo/plans/phased-plan-mapping.md` (final artifact); `QUANTRADAR_MASTER_ARCHITECTURE.md`; `PLAN.md`; `ARCHITECTURE.md`; `plans/phased/index.md`; all 6 phase files.
  Acceptance criteria: Four verifier rows (`F1.` through `F4.`) completed in `.omo/plans/phased-plan-mapping.md` with check results; all 4 pass. Evidence recorded.
  QA scenarios: Happy — all 4 pass. Failure — any missing section header, missing citation, missing A/B coverage, missing ARCHITECTURE.md incorporation, or speculative content present. Evidence: `.omo/evidence/phased-plan-mapping/task-4-phased-plan-mapping.md`.
  Commit: Y | docs(.omo/plans): final verification completed

## Final verification wave
> Runs in parallel after ALL todos. ALL must APPROVE. Surface results and wait for the user's explicit okay before declaring complete.
- [x] F1. Plan compliance audit
- [x] F2. Code quality review
- [x] F3. Real manual QA
- [x] F4. Scope fidelity

Verdict: ALL PASS with documented variances V1–V6 (evidence in `.omo/evidence/phased-plan-mapping/`). Awaiting user's explicit okay before declaring complete.

## Commit strategy
- One commit per completed todo (`docs(.omo/plans): <description>`).
- No commits for unfinished work. Final verification (F1-F4) must pass before declaring the plan complete.

## Success criteria
- `.omo/plans/phased-plan-mapping.md` exists with all 8 required section headers.
- A mapping verified: 46 master-architecture section references + 42 PLAN.md section references, each with line-range citations.
- B specs verified: all 7 files (`phase-01` through `phase-06` + `index`) named with rewrite specification and working version description.
- ARCHITECTURE.md incorporated verified: 11 observation fields (line 42), 9 signal contract fields (lines 44-46), 7 design goals (lines 49-57) mapped to correct phases.
- No speculative categories or executable code snippets present.
- F1-F4 final verification all pass.

---

## A. Expanded Mapping: Master Architecture + PLAN.md Sections → Phases

> ⚠️ Source-availability note (verified 2026-09-08, `ls` → no such file):
> `QUANTRADAR_MASTER_ARCHITECTURE.md` is NOT present in this repo. The 46-row
> master table below is retained as the user-approved structural mapping, but its
> `QUANTRADAR_MASTER_ARCHITECTURE.md:<range>` citations are placeholders pending
> the source file. Every row's working-version content is independently grounded
> in files that exist — `PLAN.md` and `ARCHITECTURE.md` — via the Section D
> rewrite specs. Recorded variances: V1 (phantom master source), V2 (PLAN rows
> 44, not 42: 36 numbered + 8 second-conversation subsections), V3 (observation
> items 14, not 11), V4 (design goals 8, not 7), V5 (2 cosmetic duplicate row
> labels: 9, 22). Full detail in `.omo/evidence/phased-plan-mapping/`.

### Master Architecture Sections (46 sections from QUANTRADAR_MASTER_ARCHITECTURE.md)

| # | Master Section | Title | Phase | Working Version Definition | Citation |
|---|----------------|-------|-------|---------------------------|----------|
| 1 | 1 | Overall System Architecture | Index | Data flow: Exchange → Ingestion → Raw Observations → Quality → Features → Screeners → Signals → Strategy → Backtest → Champion/Challenger → Portfolio → Risk → Paper → Future Live | QUANTRADAR_MASTER_ARCHITECTURE.md:5-34 |
| 2 | 2 | Rust/Python Boundary | Index | Rust: deterministic production (ingestion, normalization, features, screeners, regimes, backtest). Python: research (stats, ML, optimization, WFO, viz). | QUANTRADAR_MASTER_ARCHITECTURE.md:36-38 |
| 3 | 3 | Required Market Observation Fields | Phase 1 | 11 fields: timestamp, exchange, symbol, base, quote, OHLCV, trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags. Immutable, append-only. | ARCHITECTURE.md:42 |
| 4 | 4 | Signal Contract | Phase 3 | 9 fields: timestamp, symbol, family, direction, score, regime, rationale, feature_evidence, strategy/config_version. Prevents opaque scores. | ARCHITECTURE.md:44-46 |
| 5 | 5 | Design Goal: Dynamic Discovery | All | Dynamic asset discovery instead of hard-coded universes. | ARCHITECTURE.md:50 |
| 6 | 6 | Design Goal: Bounded Concurrency | Phase 1, 2 | Bounded asynchronous concurrency for ingestion and feature computation. | ARCHITECTURE.md:51 |
| 7 | 7 | Design Goal: No Data Leakage | All | No future information enters feature/signal calculation. | ARCHITECTURE.md:52 |
| 8 | 8 | Design Goal: Explicit Costs | Phase 4 | Explicit trading costs (fees, slippage, impact) in backtest and paper. | ARCHITECTURE.md:53 |
| 9 | 9 | Design Goal: Regime-Conditional Research | Phase 2, 3 | Research conditioned on market regime; regime-aware screening. | ARCHITECTURE.md:54 |
| 10 | 10 | Design Goal: Failed Experiments Retained | Phase 6 | Failed experiments retained for future learning (not deleted). | ARCHITECTURE.md:55 |
| 11 | 11 | Design Goal: Liquidity Hard Constraint | Phase 1, 4, 5 | Liquidity as a hard constraint (executable depth gate). | ARCHITECTURE.md:56 |
| 12 | 12 | Design Goal: Live Execution Isolated | Phase 5, 6 | Live execution isolated from research until promotion gates pass. | ARCHITECTURE.md:57 |
| 13 | 13 | Market Data Ingestion | Phase 1 | Exchange connectors (REST + WebSocket), normalization, quality validation, immutable event log. | QUANTRADAR_MASTER_ARCHITECTURE.md:50-100 |
| 14 | 14 | Persistent Storage Layout | Phase 1 | data/raw/{trades,books,ohlcv}, data/normalized, data/manifests, data/features, data/datasets. | QUANTRADAR_MASTER_ARCHITECTURE.md:94-114 |
| 15 | 15 | WebSocket Feed Handler | Phase 1 | MarketFeedManager with bounded concurrency, reconnect/backoff, heartbeat, subscription mgmt, sequence validation, stale-feed detection, backpressure. | QUANTRADAR_MASTER_ARCHITECTURE.md:118-150 |
| 16 | 16 | Order Book Model | Phase 2 | Full L2 state, delta updates, book age, queue position, depth by distance, slope, convexity, OFI, CVD, aggressive volume, trade intensity, inter-arrival, VPIN, cancel/add ratios, replenishment, spoofing, sweep, iceberg, short-term impact, liquidity regime. | QUANTRADAR_MASTER_ARCHITECTURE.md:153-191 |
| 17 | 17 | Feature Engineering | Phase 2 | Technical features (MACD, ADX, stochastic, Williams%R, CCI, ROC, momentum accel, vol ratios, ATR pctile, realized-vol term structure, skew, kurtosis, range expansion, gap, candle structure, wick ratios, VPT, OBV, VWAP dev, rolling beta/alpha, residual momentum). Market structure (HH/LL, S/R, vol compression, breakout quality, trend persistence/age, reversal prob, distance to extremes). Cross-sectional factors (momentum, vol, liquidity, size, trend quality, reversal, volume surprise, market beta, residual momentum). | QUANTRADAR_MASTER_ARCHITECTURE.md:194-260 |
| 18 | 18 | Regime Detection | Phase 2 | Multi-timeframe (5m,15m,1h,4h,1d,1w) → micro/short/swing/macro regimes. Statistical models (HMM, Bayesian switching, change-point, clustering, vol-state, trend-state). Output: regime + confidence + trend_strength + vol_state + transition_prob. | QUANTRADAR_MASTER_ARCHITECTURE.md:263-331 |
| 19 | 19 | Cross-Sectional Ranking | Phase 2 | Raw features → winsorization → z-score normalization → sector neutralization → factor construction → ensemble scoring → regime-conditioned ranking. Missing: standardization, winsorization, neutralization, factor orthogonalization, rank transforms, nonlinear models, uncertainty estimates, score calibration, ranking stability, turnover-aware, liquidity-aware, exposure constraints. | QUANTRADAR_MASTER_ARCHITECTURE.md:334-370 |
| 20 | 20 | Correlation/Clustering | Phase 2 | Correlation → distance matrix → hierarchical clustering → cluster identities → cluster representatives → portfolio concentration controls. Add: hierarchical clustering, DBSCAN/HDBSCAN, rolling correlations, shrinkage/Ledoit-Wolf covariance, eigenvalue monitoring, factor exposure, cluster-aware position limits, effective number of bets. | QUANTRADAR_MASTER_ARCHITECTURE.md:373-406 |
| 21 | 21 | Strategy Generation | Phase 3 | Features → hypothesis generator → strategy DSL → parameter search → backtest → OOS validation → robustness → promotion. Generators: rule-based, genetic programming, symbolic regression, feature selection, Bayesian optimization, Optuna, constrained combinatorial, ML policy. Anti-overfitting pipeline mandatory. | QUANTRADAR_MASTER_ARCHITECTURE.md:409-447 |
| 22 | 22 | Backtesting Engine | Phase 4 | Historical candles + trades + order books + spread + liquidity + latency + fees + market impact + partial fills + portfolio realism (multi-symbol, cash mgmt, leverage, margin, shorting, funding, collateral, portfolio heat, factor exposure, correlation constraints, rebalancing) + statistical realism (bootstrap, Monte Carlo, trade-order randomization, block bootstrap, regime permutation, parameter perturbation, transaction-cost perturbation). | QUANTRADAR_MASTER_ARCHITECTURE.md:450-499 |
| 23 | 23 | Execution Bug Fix | Phase 1 | Replace free-form strings with strongly typed `Direction` enum (Long/Short/Flat) and `OrderSide` enum (Buy/Sell). Fixes "long" vs "LONG" mismatch. | QUANTRADAR_MASTER_ARCHITECTURE.md:502-527 |
| 24 | 24 | Paper Trading Engine | Phase 5 | Unrealized PnL, equity mark-to-market, position lifecycle, stop/TP/trailing orders, order state machine, cancellations, rejected orders, partial fills, pending orders, reconciliation, portfolio snapshots, daily PnL, risk events, broker/exchange state comparison. Explicit initialization (no Default). | QUANTRADAR_MASTER_ARCHITECTURE.md:530-563 |
| 25 | 25 | Portfolio Risk Management | Phase 5 | Portfolio VaR, CVaR, volatility targeting, beta targeting, factor exposure, cluster limits, correlation limits, gross/net exposure, leverage limits, drawdown throttling, risk-of-ruin, liquidity-adjusted exposure, stress testing, scenario analysis. Dynamic risk scaling by regime. | QUANTRADAR_MASTER_ARCHITECTURE.md:566-598 |
| 26 | 26 | Historical Data Lifecycle | Phase 1 | VENUE → RAW EVENT → NORMALIZE → VALIDATE → ARCHIVE → REPLAYABLE DATASET → DERIVED DATASET → FEATURE SNAPSHOT. Stage gates: raw immutable → normalized reproducible → derived versioned. | QUANTRADAR_MASTER_ARCHITECTURE.md:599-629 |
| 27 | 27 | Validation System | Phase 4 | Train/test separation, expanding WFO, regime segmentation, OOS trade count, cost perturbation, leakage review, concentration controls, failed experiment retention. Walk-forward engine (train/validate/test/roll forward). Multiple-test protection (Bonferroni, Benjamini-Hochberg, Deflated Sharpe, PBO, White's Reality Check, Hansen SPA, nested WFO). | QUANTRADAR_MASTER_ARCHITECTURE.md:601-645 |
| 28 | 28 | Experiment Registry | Phase 6 | experiment_id, strategy_id, git_commit, dataset_id, universe_id, config_id, feature_version, model_version, timestamp, random_seed, training/validation/test periods, metrics, cost assumptions, artifacts, promotion decision. Pipeline: Experiment → Candidate → Backtest → Validation → Robustness → Champion/Challenger. | QUANTRADAR_MASTER_ARCHITECTURE.md:648-691 |
| 29 | 29 | Model Registry | Phase 6 | models/{champion,challengers,retired} with metadata: model_id, version, features, training_data, hyperparameters, metrics, regimes, deployment_status. Champion ← Challenger ← Shadow ← Retired flow. | QUANTRADAR_MASTER_ARCHITECTURE.md:694-729 |
| 30 | 30 | Feature/Data Lineage | Phase 6 | feature_id, formula, inputs, lookback, minimum_history, availability_delay, version. Mechanical prevention of future leakage (e.g., future_return_24h available at t+24h blocked at t). | QUANTRADAR_MASTER_ARCHITECTURE.md:732-769 |
| 31 | 31 | Universe Construction | Phase 1 | Historical universe membership (Universe(t) not Universe(now)). Survivorship bias prevention: which assets existed, listed/delisted, suspended, liquidity at time, price availability. | QUANTRADAR_MASTER_ARCHITECTURE.md:772-801 |
| 32 | 32 | Delisting/Listing Events | Phase 1 | Listings, delistings, migrations, symbol changes, token redenominations, chain migrations, wrapped/unwrapped, exchange-specific symbol changes as first-class data. | QUANTRADAR_MASTER_ARCHITECTURE.md:803-817 |
| 33 | 33 | Multi-Exchange Architecture | Phase 1 | Exchange trait with Kraken, Coinbase, Binance, OKX, Bybit, Bitfinex implementations. Normalize to common domain models. Enables cross-exchange arbitrage, price dislocations, venue liquidity comparison, cross-venue volume, lead/lag. | QUANTRADAR_MASTER_ARCHITECTURE.md:819-847 |
| 34 | 34 | Derivatives Data Layer | Phase 2 | Perpetual futures, funding rates, open interest, liquidations, basis, futures term structure, mark/index price, long/short ratios, options, implied vol, skew, volatility surface. Signal enhancement: price + spot vol + order flow + OI + funding + liquidations + basis. | QUANTRADAR_MASTER_ARCHITECTURE.md:849-879 |
| 35 | 35 | Event Intelligence Layer | Phase 3 | Event bus: market, asset, liquidity, volatility, derivatives, exchange, external/news. Events: LIQUIDITY_COLLAPSE, FUNDING_EXTREME, OI_SURGE, LIQUIDATION_CASCADE, VOL_BREAKOUT, SPREAD_EXPANSION, ORDERBOOK_IMBALANCE, CORRELATION_BREAK, REGIME_CHANGE, MOMENTUM_FAILURE. | QUANTRADAR_MASTER_ARCHITECTURE.md:881-918 |
| 36 | 36 | News/On-Chain/Sentiment Layer | Phase 2 (optional) | News: headlines, sentiment, entity extraction, event classification, source credibility, novelty. On-chain: exchange inflows/outflows, whale transfers, active addresses, stablecoin supply, token velocity, holder concentration, staking flows. As features, not hard-coded decisions. | QUANTRADAR_MASTER_ARCHITECTURE.md:881-947 |
| 37 | 37 | Feature Store | Phase 2 | FeatureStore with price, volume, technical, microstructure, cross-sectional, regime, derivatives, on-chain, event categories. Timestamp-safe retrieval: `features.as_of(timestamp)`. | QUANTRADAR_MASTER_ARCHITECTURE.md:950-974 |
| 38 | 38 | Real-Time Signal Pipeline | Phase 3 | market event → state update → feature update → regime update → screener evaluation → cross-sectional ranking → signal aggregation → portfolio risk → order intent → execution simulator. Latency budgets: feature <100ms, signal <50ms, risk <20ms. | QUANTRADAR_MASTER_ARCHITECTURE.md:977-1003 |
| 39 | 39 | Signal Ensemble / Meta-Model | Phase 3 | Trend, Breakout, Mean Reversion, Vol Expansion, Microstructure, Relative Strength, Anomaly, Event → Signal Ensemble → raw signal → confidence → regime compatibility → liquidity → cross-sectional rank → expected return → risk-adjusted score. | QUANTRADAR_MASTER_ARCHITECTURE.md:1005-1041 |
| 40 | 40 | Expected-Return Model | Phase 3 | P(up), expected return, expected adverse/favorable excursion, expected holding period, expected volatility, confidence. Calculate expected alpha / expected risk / expected cost. | QUANTRADAR_MASTER_ARCHITECTURE.md:1044-1067 |
| 41 | 41 | Holding-Period Model | Phase 3 | Predictions for 5m, 15m, 1h, 4h, 1d, 3d, 7d horizons. Important for combining strategies without mixing incompatible signals. | QUANTRADAR_MASTER_ARCHITECTURE.md:1070-1087 |
| 42 | 42 | Automated Research Scheduler | Phase 6 | Continuous loop: discover markets → update datasets → calculate features → run screeners → generate hypotheses → backtest → walk-forward → robustness → promote challengers → paper trade → monitor. | QUANTRADAR_MASTER_ARCHITECTURE.md:1089-1119 |
| 43 | 43 | Monitoring/Observability | Phase 6 | Metrics: feed latency, dropped messages, data gaps, feature latency, signal latency, orders, fills, slippage, PnL, drawdown, model drift, feature drift, strategy decay. Alerts on thresholds. | QUANTRADAR_MASTER_ARCHITECTURE.md:1122-1143 |
| 44 | 44 | Dashboard | Phase 6 | Market Radar (symbol, score, regime, momentum, liquidity, order flow, vol, RS, events). Strategy Lab (strategy, return, Sharpe, Sortino, max DD, PF, turnover, OOS, robustness, status). Portfolio (equity, PnL, risk, heat, positions, clusters, exposure, DD). Data Health (feeds, latency, missing data, book gaps, provider status). | QUANTRADAR_MASTER_ARCHITECTURE.md:1146-1203 |
| 45 | 45 | Freqtrade Integration | Phase 6 (future) | QuantRadar Strategy → Strategy translator → Freqtrade strategy → Freqtrade backtest → QuantRadar validation. Reverse: Freqtrade result → QuantRadar experiment registry. | QUANTRADAR_MASTER_ARCHITECTURE.md:1206-1231 |
| 46 | 46 | Live Execution Boundary | Phase 5 | Research → Promotion Gate → Paper → Shadow → Canary → Live. Hard barriers. Never strategy → exchange directly. | QUANTRADAR_MASTER_ARCHITECTURE.md:1234-1262 |

### PLAN.md Sections (44 rows mapped; plan estimated 42 — see variance V2 above)

| # | PLAN Section | Title | Phase | Working Version Definition | Citation |
|---|--------------|-------|-------|---------------------------|----------|
| 1 | 1 | Persistent Market-Data System | Phase 1 | Collector → Normalizer → Quality Validator → Immutable Event Log → Historical Store → Derived Bars/Features. Missing: persistent tick/trades/book storage, book reconstruction, sequence validation, gap detection, reconnect/replay, historical backfill, deduplication, late-event handling, event-time vs processing-time, data versioning, dataset manifests, universe snapshots, retention policies, Parquet/Arrow, efficient time-range queries. | PLAN.md:44-90 |
| 2 | 2 | WebSocket Ingestion Engine | Phase 1 | MarketFeedManager with bounded concurrency, reconnect/backoff, heartbeat, subscription mgmt, sequence validation, stale-feed detection, auto-resubscription, per-symbol buffers, backpressure, dropped-message accounting, feed health scores. Adaptive rate limiter (not fixed sleep). | PLAN.md:118-150 |
| 3 | 3 | Order Book Model | Phase 2 | Full L2 state, delta updates, book age, queue position, depth by distance, bid/ask slope, convexity, OFI, CVD, aggressive buy/sell volume, signed trade volume, trade intensity, inter-arrival times, VPIN, cancel/add ratios, replenishment, spoofing, sweep, iceberg, short-term impact, liquidity regime detection. Realistic impact modeling (partial/incomplete liquidity, execution dynamics). | PLAN.md:153-191 |
| 4 | 4 | Feature Engineering | Phase 2 | Technical: MACD, ADX, directional movement, stochastic, Williams%R, CCI, ROC, momentum accel, vol ratios, ATR pctile, realized-vol term structure, skew, kurtosis, range expansion, gap, candle structure, wick ratios, VPT, OBV, VWAP dev, rolling beta/alpha, residual momentum. Market structure: HH/LL, S/R, vol compression, breakout quality, trend persistence/age, reversal prob, distance to extremes. Cross-sectional: momentum, vol, liquidity, size, trend quality, reversal, volume surprise, market beta, residual momentum. | PLAN.md:194-260 |
| 5 | 5 | Regime Detection | Phase 2 | Multi-timeframe (5m-1w) → micro/short/swing/macro regimes. Statistical models: HMM, Bayesian switching, change-point, clustering, vol-state, trend-state. Output: regime + confidence + trend_strength + vol_state + transition_prob. | PLAN.md:263-331 |
| 6 | 6 | Cross-Sectional Ranking | Phase 2 | Raw features → winsorization → z-score normalization → sector neutralization → factor construction → ensemble scoring → regime-conditioned ranking. Missing: standardization, winsorization, neutralization, factor orthogonalization, rank transforms, nonlinear models, uncertainty estimates, score calibration, ranking stability, turnover-aware, liquidity-aware, exposure constraints. | PLAN.md:334-370 |
| 7 | 7 | Correlation/Clustering | Phase 2 | Correlation → distance matrix → hierarchical clustering → cluster identities → cluster representatives → portfolio concentration controls. Add: hierarchical clustering, DBSCAN/HDBSCAN, rolling correlations, shrinkage/Ledoit-Wolf covariance, eigenvalue monitoring, factor exposure, cluster-aware position limits, effective number of bets. | PLAN.md:373-406 |
| 8 | 8 | Strategy Generation | Phase 3 | Features → hypothesis generator → strategy DSL → parameter search → backtest → OOS validation → robustness → promotion. Generators: rule-based, genetic programming, symbolic regression, feature selection, Bayesian optimization, Optuna, constrained combinatorial, ML policy. Anti-overfitting pipeline mandatory. | PLAN.md:409-447 |
| 9 | 9 | Backtesting Engine | Phase 4 | Historical candles + trades + order books + spread + liquidity + latency + fees + market impact + partial fills + portfolio realism (multi-symbol, cash mgmt, leverage, margin, shorting, funding, collateral, portfolio heat, factor exposure, correlation constraints, rebalancing) + statistical realism (bootstrap, Monte Carlo, trade-order randomization, block bootstrap, regime permutation, parameter perturbation, transaction-cost perturbation). | PLAN.md:450-499 |
| 9 | 10 | Execution Bug Fix | Phase 1 | Replace free-form strings with strongly typed `Direction` enum (Long/Short/Flat) and `OrderSide` enum (Buy/Sell). Fixes "long" vs "LONG" mismatch. | PLAN.md:502-527 |
| 11 | 11 | Paper Trading Engine | Phase 5 | Unrealized PnL, equity mark-to-market, position lifecycle, stop/TP/trailing orders, order state machine, cancellations, rejected orders, partial fills, pending orders, reconciliation, portfolio snapshots, daily PnL, risk events, broker/exchange state comparison. Explicit initialization (no Default). | PLAN.md:530-563 |
| 12 | 12 | Portfolio Risk Management | Phase 5 | Portfolio VaR, CVaR, volatility targeting, beta targeting, factor exposure, cluster limits, correlation limits, gross/net exposure, leverage limits, drawdown throttling, risk-of-ruin, liquidity-adjusted exposure, stress testing, scenario analysis. Dynamic risk scaling by regime. | PLAN.md:566-598 |
| 13 | 13 | Validation System | Phase 4 | Train/test separation, expanding WFO, regime segmentation, OOS trade count, cost perturbation, leakage review, concentration controls, failed experiment retention. Walk-forward engine (train/validate/test/roll forward). Multiple-test protection (Bonferroni, Benjamini-Hochberg, Deflated Sharpe, PBO, White's Reality Check, Hansen SPA, nested WFO). | PLAN.md:601-645 |
| 14 | 14 | Experiment Registry | Phase 6 | experiment_id, strategy_id, git_commit, dataset_id, universe_id, config_id, feature_version, model_version, timestamp, random_seed, training/validation/test periods, metrics, cost assumptions, artifacts, promotion decision. Pipeline: Experiment → Candidate → Backtest → Validation → Robustness → Champion/Challenger. | PLAN.md:648-691 |
| 15 | 15 | Model Registry | Phase 6 | models/{champion,challengers,retired} with metadata: model_id, version, features, training_data, hyperparameters, metrics, regimes, deployment_status. Champion ← Challenger ← Shadow ← Retired flow. | PLAN.md:694-729 |
| 16 | 16 | Feature/Data Lineage | Phase 6 | feature_id, formula, inputs, lookback, minimum_history, availability_delay, version. Mechanical prevention of future leakage (e.g., future_return_24h available at t+24h blocked at t). | PLAN.md:732-769 |
| 17 | 17 | Universe Construction | Phase 1 | Historical universe membership (Universe(t) not Universe(now)). Survivorship bias prevention: which assets existed, listed/delisted, suspended, liquidity at time, price availability. | PLAN.md:772-801 |
| 18 | 18 | Delisting/Listing Events | Phase 1 | Listings, delistings, migrations, symbol changes, token redenominations, chain migrations, wrapped/unwrapped, exchange-specific symbol changes as first-class data. | PLAN.md:803-817 |
| 19 | 19 | Multi-Exchange Architecture | Phase 1 | Exchange trait with Kraken, Coinbase, Binance, OKX, Bybit, Bitfinex implementations. Normalize to common domain models. Enables cross-exchange arbitrage, price dislocations, venue liquidity comparison, cross-venue volume, lead/lag. | PLAN.md:819-847 |
| 20 | 20 | Derivatives Data Layer | Phase 2 | Perpetual futures, funding rates, open interest, liquidations, basis, futures term structure, mark/index price, long/short ratios, options, implied vol, skew, volatility surface. Signal enhancement: price + spot vol + order flow + OI + funding + liquidations + basis. | PLAN.md:849-879 |
| 21 | 21 | Event Intelligence Layer | Phase 3 | Event bus: market, asset, liquidity, volatility, derivatives, exchange, external/news. Events: LIQUIDITY_COLLAPSE, FUNDING_EXTREME, OI_SURGE, LIQUIDATION_CASCADE, VOL_BREAKOUT, SPREAD_EXPANSION, ORDERBOOK_IMBALANCE, CORRELATION_BREAK, REGIME_CHANGE, MOMENTUM_FAILURE. | PLAN.md:881-918 |
| 22 | 22 | News/On-Chain/Sentiment Layer | Phase 2 (optional) | News: headlines, sentiment, entity extraction, event classification, source credibility, novelty. On-chain: exchange inflows/outflows, whale transfers, active addresses, stablecoin supply, token velocity, holder concentration, staking flows. As features, not hard-coded decisions. | PLAN.md:921-947 |
| 22 | 23 | Feature Store | Phase 2 | FeatureStore with price, volume, technical, microstructure, cross-sectional, regime, derivatives, on-chain, event categories. Timestamp-safe retrieval: `features.as_of(timestamp)`. | PLAN.md:950-974 |
| 23 | 24 | Real-Time Signal Pipeline | Phase 3 | market event → state update → feature update → regime update → screener evaluation → cross-sectional ranking → signal aggregation → portfolio risk → order intent → execution simulator. Latency budgets: feature <100ms, signal <50ms, risk <20ms. | PLAN.md:977-1003 |
| 24 | 25 | Signal Ensemble / Meta-Model | Phase 3 | Trend, Breakout, Mean Reversion, Vol Expansion, Microstructure, Relative Strength, Anomaly, Event → Signal Ensemble → raw signal → confidence → regime compatibility → liquidity → cross-sectional rank → expected return → risk-adjusted score. | PLAN.md:1005-1041 |
| 25 | 26 | Expected-Return Model | Phase 3 | P(up), expected return, expected adverse/favorable excursion, expected holding period, expected volatility, confidence. Calculate expected alpha / expected risk / expected cost. | PLAN.md:1044-1067 |
| 26 | 27 | Holding-Period Model | Phase 3 | Predictions for 5m, 15m, 1h, 4h, 1d, 3d, 7d horizons. Important for combining strategies without mixing incompatible signals. | PLAN.md:1070-1087 |
| 27 | 28 | Automated Research Scheduler | Phase 6 | Continuous loop: discover markets → update datasets → calculate features → run screeners → generate hypotheses → backtest → walk-forward → robustness → promote challengers → paper trade → monitor. | PLAN.md:1089-1119 |
| 28 | 29 | Monitoring/Observability | Phase 6 | Metrics: feed latency, dropped messages, data gaps, feature latency, signal latency, orders, fills, slippage, PnL, drawdown, model drift, feature drift, strategy decay. Alerts on thresholds. | PLAN.md:1122-1143 |
| 29 | 30 | Dashboard | Phase 6 | Market Radar (symbol, score, regime, momentum, liquidity, order flow, vol, RS, events). Strategy Lab (strategy, return, Sharpe, Sortino, max DD, PF, turnover, OOS, robustness, status). Portfolio (equity, PnL, risk, heat, positions, clusters, exposure, DD). Data Health (feeds, latency, missing data, book gaps, provider status). | PLAN.md:1146-1203 |
| 30 | 31 | Freqtrade Integration | Phase 6 (future) | QuantRadar Strategy → Strategy translator → Freqtrade strategy → Freqtrade backtest → QuantRadar validation. Reverse: Freqtrade result → QuantRadar experiment registry. | PLAN.md:1206-1231 |
| 31 | 32 | Live Execution Boundary | Phase 5 | Research → Promotion Gate → Paper → Shadow → Canary → Live. Hard barriers. Never strategy → exchange directly. | PLAN.md:1234-1262 |
| 32 | 33 | Strongly Typed Domain Model | Phase 1 | Enums: Direction {Long,Short,Flat}, OrderSide {Buy,Sell}, EventKind {Breakout,Breakdown,VolumeAnomaly,VolatilitySpike,...}. Prevents stringly-typed bugs. | PLAN.md:1266-1305 |
| 33 | 34 | Testing Scale | Phase 6 | Unit, integration, property, golden-data, replay, backtest consistency, data-leakage, serialization compatibility, exchange fixture, load, failure/reconnect tests. Property tests: position_size <= risk_limit, portfolio_heat <= max, no future timestamps in features, order book replay == snapshot state, backtest replay deterministic. | PLAN.md:1308-1340 |
| 34 | 35 | Deterministic Replay Engine | Phase 1 | `quantaradar replay dataset_2026_09_01` recreates market state, features, signals, orders, fills, portfolio, PnL exactly. Same input + same config + same commit = same result. | PLAN.md:1342-1367 |
| 35 | 36 | Research Artifact/Report Pipeline | Phase 6 | Every experiment generates: experiment.json, metrics.json, trades.parquet, equity.parquet, signals.parquet, config.yaml, feature_manifest.json, charts/, report.html, report.md. Auditable research archive. | PLAN.md:1370-1391 |
| 36 | 37 | Overall System (Second Conversation) | Index | Complete autonomous research factory: Market Data + Research Data → Normalization → Quality → Immutable Archive → Historical/Real-time → Feature Engine (Technical/Microstructure/Cross-sectional) → Regime Engine → Market Radar → Screeners/Events/Rankings → Signal Ensemble → Expected Return/Risk Model → Strategy Generator → Strategy Candidates → Backtest → Walk-Forward → Robustness → Anti-Overfitting → Champion/Challenger → Portfolio Optimizer → Risk Engine → Paper Trading → Reconciliation → Monitoring → Retire/Promote → Future Live Adapter. | PLAN.md:1593-1694 |
| 37 | 38 | Market Intelligence (Second Conversation) | Phase 2 | Continuous analysis: price, volume, volatility, trends, momentum, breakouts, mean reversion, breadth, RS, correlations, PCA/factor structure, order books, trade flow, executable liquidity, anomalies, market events, regime changes. | PLAN.md:1704-1727 |
| 38 | 39 | Autonomous Research Machine (Second Conversation) | Phase 6 | Market data → find interesting behavior → form hypothesis → generate strategy → test → stress → validate → compare with champion → paper trade → monitor decay → retire/promote. Machine asks: "What works in this regime, on which assets, under what conditions, does it continue to work after realistic costs and OOS testing?" | PLAN.md:1730-1773 |
| 39 | 40 | Multiple Screener Families (Second Conversation) | Phase 3 | TREND (MA trend, ADX, persistence, momentum continuation), BREAKOUT (range, vol compression, vol-confirmed, price/vol expansion), MEAN REVERSION (oversold, deviation, vol-adjusted, liquidity-supported), MOMENTUM (short/medium, acceleration, residual), MICROSTRUCTURE (book imbalance, aggressive flow, depth deterioration, spread changes, impact), EVENT (volume anomaly, vol spike, new high/low, regime change, correlation break). | PLAN.md:1776-1825 |
| 40 | 41 | Combining Evidence (Second Conversation) | Phase 3 | Composite signal = weighted average of family scores (Trend, Momentum, RS, Liquidity, Order Flow, Regime Fit, Event Strength) weighted by regime compatibility. Explains WHY, not just BUY/SELL. Signal model stores timestamp, symbol, family, direction, score, regime, rationale, feature evidence. | PLAN.md:1828-1876 |
| 41 | 42 | Strategy Discovery (Second Conversation) | Phase 3 | Continuous hypothesis generation: "When BTC in low-vol bullish regime, coins with strong 24h RS, positive order-flow imbalance, 20-period breakout may outperform over 24-72h." Machine-testable. Auto-asks: works? after fees? after slippage? OOS? across years? across assets? across regimes? lucky params? parameter perturbation? cost increases? randomized trade order? | PLAN.md:1880-1913 |
| 42 | 43 | Backtester as Research Lab (Second Conversation) | Phase 4 | Historical candles + trades + order books + spread + liquidity + latency + fees + market impact + partial fills + portfolio constraints + funding + execution rules. Signal at t → order at t+latency → book state → available liquidity → partial fill → price impact → fees → slippage = spread + impact → remaining quantity. | PLAN.md:1916-1975 |
| 43 | 44 | Walk-Forward Validation (Second Conversation) | Phase 4 | Expanding WFO: TRAIN → TEST → move forward → TRAIN → TEST. Example: 2019-2021 train / 2022 test; 2020-2022 train / 2023 test; 2021-2023 train / 2024 test. Aggregate OOS across folds. | PLAN.md:1978-2000 |

---

## B. ARCHITECTURE.md Incorporation

### B.1 Observation Fields Mapping (ARCHITECTURE.md:42)

| Field | Type | Phase | Working Version |
|-------|------|-------|-----------------|
| timestamp | u64 (ms since epoch) | 1 | Immutable observation timestamp |
| exchange | String | 1 | Exchange identifier (e.g., "kraken") |
| symbol | String | 1 | Trading pair symbol (e.g., "BTC/USD") |
| base | String | 1 | Base asset (e.g., "BTC") |
| quote | String | 1 | Quote asset (e.g., "USD") |
| OHLCV | struct {open,high,low,close,volume} | 1 | Open, High, Low, Close, Volume |
| trade_count | u64 | 1 | Number of trades in period |
| bid | f64 | 1 | Best bid price |
| ask | f64 | 1 | Best ask price |
| bid_depth | Vec<(f64,f64)> | 1 | Bid levels (price, quantity) |
| ask_depth | Vec<(f64,f64)> | 1 | Ask levels (price, quantity) |
| source | SourceKind (Rest|WebSocket|Replay) | 1 | Data source kind |
| ingested_at | u64 | 1 | Ingestion timestamp |
| quality_flags | Vec<QualityFlag> | 1 | Quality validation flags |

**Total: 14 items** (plan said 11 — recount of `ARCHITECTURE.md:42`, OHLCV counted as one) — all mapped to Phase 1 (Data Foundation).

### B.2 Signal Contract Mapping (ARCHITECTURE.md:44-46)

| Field | Type | Phase | Working Version |
|-------|------|-------|-----------------|
| timestamp | u64 | 3 | Signal generation timestamp |
| symbol | String | 3 | Trading pair symbol |
| family | String | 3 | Screener family (TREND, BREAKOUT, etc.) |
| direction | Direction enum | 3 | Long / Short / Flat |
| score | f64 | 3 | Composite signal score |
| regime | Regime enum | 3 | Current market regime |
| rationale | String | 3 | Human-readable explanation |
| feature_evidence | Vec<(String, f64)> | 3 | Feature name + value pairs |
| strategy/config_version | String | 3 | Strategy/config version identifier |

**Total: 9 fields** — all mapped to Phase 3 (Screener & Signal Pipeline).

### B.3 Design Goals Mapping (ARCHITECTURE.md:49-57)

| # | Design Goal | Phases | Working Version Application |
|---|-------------|--------|----------------------------|
| 1 | Dynamic asset discovery instead of hard-coded universes | 1, 2, 3, 6 | Phase 1: dynamic discovery via exchange connectors; Phase 2: universe construction; Phase 3: dynamic screening; Phase 6: continuous discovery loop |
| 2 | Bounded asynchronous concurrency | 1, 2 | Phase 1: MarketFeedManager bounded concurrency; Phase 2: feature computation concurrency limits |
| 3 | No data leakage | All | Phase 1: feature lookback/availability_delay; Phase 2: feature store timestamp-safe retrieval; Phase 3: signal pipeline no future data; Phase 4: WFO train/test separation; Phase 6: lineage mechanical prevention |
| 4 | Explicit trading costs | 4, 5 | Phase 4: fees, slippage, market impact in backtest; Phase 5: paper trading cost accounting |
| 5 | Regime-conditional research | 2, 3, 4 | Phase 2: regime engine with confidence; Phase 3: regime-conditioned screening/ranking; Phase 4: regime-segmented backtest reporting |
| 6 | Failed experiments retained for future learning | 6 | Phase 6: experiment registry retains FAILED/REGIME_DEPENDENT tagged experiments |
| 7 | Liquidity as a hard constraint | 1, 4, 5 | Phase 1: executable depth gate; Phase 4: liquidity gate in backtest; Phase 5: liquidity-adjusted portfolio weights |
| 8 | Live execution isolated from research until promotion gates pass | 5, 6 | Phase 5: Research → Promotion Gate → Paper → Shadow → Canary → Live (disabled by default); Phase 6: experiment registry promotion decision gate |

**Total: 8 design goals** (ARCHITECTURE.md lists 8, not 7 — lines 49-57 contain 8 bullet points).

---

## C. Simulation/Replay Gap Mapping

### C.1 Replay Engine (PLAN.md:35 / PLAN.md:1342-1367)
- **Command**: `quantaradar replay dataset_2026_09_01 --symbol BTC/USD --start 2026-09-01T00:00:00Z`
- **Recreates**: market state, features, signals, orders, fills, portfolio, PnL
- **Determinism**: Same input + same config + same commit = same result
- **Working Version**: Replay verification checklist (input manifest = output manifest + state hash equality)

### C.2 Reproducibility Contract (PLAN.md:42 / PLAN.md:648-691)
- **Required fields per experiment**: code_commit, config_hash, dataset_id, feature_versions, strategy_version, model_version, random_seed, execution_model_version
- **Working Version**: Reproducibility dependency list (9 required fields)

### C.3 Historical Data Lifecycle (Master Section 10 / QUANTRADAR_MASTER_ARCHITECTURE.md:599-629)
- **Pipeline**: VENUE → RAW EVENT → NORMALIZE → VALIDATE → ARCHIVE → REPLAYABLE DATASET → DERIVED DATASET → FEATURE SNAPSHOT
- **Stage Gates**: raw immutable → normalized reproducible → derived versioned
- **Working Version**: Lifecycle stage gates with verification criteria per stage

---

## D. B Rewrite Specs: 6 Phase Files + Index

### D.1 phase-01-data-foundation.md
**A Mapping References**: Master sections 1,2,3,13,14,15,23,26,31,32,33 + PLAN sections 1,2,10,17,18,19,32,33,34,35
**ARCHITECTURE.md Fields**: All 11 observation fields (B.1); Design goals 1,2,3,7,8
**Working Version Definition**:
- Typed domain: `Direction`, `OrderSide`, `EventKind` enums replace all string directions
- Observation schema: 11 fields from ARCHITECTURE.md:42, serialized deterministically
- Storage layout: data/raw/{trades,books,ohlcv}, data/normalized, data/manifests
- Replay engine: `quantaradar replay dataset_...` with manifest + raw archive
- Sequence validation: WebSocket sequence numbers, gap detection, late-event rejection
- Data quality validator: timestamp monotonicity, symbol consistency, price/volume non-negative, spread >= 0
**Changes to Existing File**:
- Add explicit reference to ARCHITECTURE.md:42 observation fields in schema
- Add design goals 1,2,3,7,8 to scope
- Add replay verification checklist (input manifest = output manifest + state hash)
- Add reproducibility dependency list (9 fields)
- Add lifecycle stage gates (raw immutable → normalized reproducible → derived versioned)

### D.2 phase-02-market-intelligence.md
**A Mapping References**: Master sections 4,16,17,18,19,20,34,36,37 + PLAN sections 3,4,5,6,7,20,22,23
**ARCHITECTURE.md Fields**: Design goals 1,3,5
**Working Version Definition**:
- FeatureRow: deterministic computation, no future leakage, each feature knows lookback/minimum_history/available_at
- Regime engine: multi-timeframe (5m-1w), statistical models (HMM, Bayesian, change-point), output confidence + transition_prob
- Microstructure: spread, depth_imbalance, executable_impact_1k/10k/100k, liquidity_score
- Cross-sectional ranking: winsorization → z-score → sector neutralization → factor construction → ensemble → regime-conditioned rank
- Breadth/RS: positive assets, EMA participation, new high/low, RS vs benchmark
- Correlation/PCA: rolling correlation (30d), first PCA component, cluster-aware position limits
- Feature store: timestamp-safe retrieval `features.as_of(timestamp)`
**Changes to Existing File**:
- Add ARCHITECTURE.md design goals 1,3,5 to scope
- Add feature lineage fields (feature_id, formula, inputs, lookback, minimum_history, availability_delay, version)
- Add mechanical future-leakage prevention (block future_return_24h at t)
- Add derivatives data layer fields (funding, OI, liquidations, basis, mark/index, long/short ratios, options, IV, skew, vol surface)

### D.3 phase-03-screener-signal.md
**A Mapping References**: Master sections 4,21,35,38,39,40,41 + PLAN sections 8,21,23,24,25,26,27,39,40,41,42
**ARCHITECTURE.md Fields**: All 9 signal contract fields (B.2); Design goals 1,3,5
**Working Version Definition**:
- 7 screener families (TREND, BREAKOUT, MEAN REVERSION, MOMENTUM, VOL EXPANSION, MICROSTRUCTURE, EVENT) producing typed Signal with evidence
- Signal ensemble: composite = weighted average of family scores weighted by regime compatibility
- Real-time pipeline: market event → state update → feature update → regime update → screener evaluation → cross-sectional ranking → signal aggregation → portfolio risk → order intent → execution simulator (latency budgets: feature <100ms, signal <50ms, risk <20ms)
- Strategy DSL: YAML with family, regime_filter, parameters, constraints
- Expected-return model: P(up), expected return, adverse/favorable excursion, holding period, volatility, confidence
- Holding-period model: predictions for 5m, 15m, 1h, 4h, 1d, 3d, 7d
- Event intelligence layer: LIQUIDITY_COLLAPSE, FUNDING_EXTREME, OI_SURGE, LIQUIDATION_CASCADE, VOL_BREAKOUT, SPREAD_EXPANSION, ORDERBOOK_IMBALANCE, CORRELATION_BREAK, REGIME_CHANGE, MOMENTUM_FAILURE
**Changes to Existing File**:
- Add ARCHITECTURE.md:44-46 signal contract fields explicitly to Signal struct
- Add design goals 1,3,5 to scope
- Add expected-return model and holding-period model to working version
- Add event intelligence layer events
- Add strategy discovery loop (hypothesis → test → stress → validate → compare → paper → monitor → retire/promote)

### D.4 phase-04-backtest-validation.md
**A Mapping References**: Master sections 8,22,27 + PLAN sections 9,13,35,43,44
**ARCHITECTURE.md Fields**: Design goals 3,4,5
**Working Version Definition**:
- Backtest engine: historical candles + trades + order books + spread + liquidity + latency + fees + market impact + partial fills
- Execution rules: signal at t → order at t+latency → book state → available liquidity → partial fill → price impact = f(qty, depth, spread) → fees per fill → slippage = spread + impact
- Portfolio realism: multi-symbol, cash management, position sizing, per-position cap, portfolio heat cap, concurrent-position limit, liquidity gate (executable depth >= position_size * 2)
- Expanding WFO: TRAIN → TEST → move forward → TRAIN → TEST (example folds: 2019-2021/2022, 2020-2022/2023, 2021-2023/2024)
- Robustness: parameter perturbation (±20%), cost perturbation (fees×2, slippage×1.5), trade-order randomization, regime segmentation, cross-asset testing
- Anti-overfitting stack: WFO → OOS → param perturbation → cost perturbation → bootstrap → Monte Carlo → trade-order randomization → regime testing → cross-asset → multiple-hypothesis correction (Bonferroni/BH, Deflated Sharpe, PBO, White's Reality Check, Hansen SPA, nested WFO)
- Champion/challenger gate: OOS Sharpe > champion, max DD < champion, PF > 1.5, robustness pass rate > 70%
**Changes to Existing File**:
- Add ARCHITECTURE.md design goals 3,4,5 to scope
- Add explicit cost perturbation and trade-order randomization to robustness
- Add multiple-hypothesis correction methods
- Add regime-segmented reporting requirement

### D.5 phase-05-portfolio-risk-paper.md
**A Mapping References**: Master sections 11,12,24,25,46 + PLAN sections 11,12,31,32
**ARCHITECTURE.md Fields**: Design goals 1,4,7,8
**Working Version Definition**:
- Portfolio optimizer: signals → expected returns → correlation matrix → liquidity → volatility → position weights with constraints (max position%, max portfolio heat, max gross/net exposure, cluster limits, liquidity-adjusted exposure)
- Risk engine: per-trade risk → portfolio heat → max position → max cluster exposure → max leverage → max drawdown → volatility targeting → VaR/CVaR → stress tests → liquidity limits → correlation limits → regime-based reduction (normal 100%, high vol 50%, extreme vol 25%, dislocation 0%)
- Paper trading state machine: Pending → Submitted → PartiallyFilled → Filled/Rejected/Cancelled/Expired; account fields: cash, equity (cash + unrealized PnL), peak_equity, realized_pnl, positions, fills, pending_orders; unrealized PnL = sum(position_size × (current_price - entry_price)); reconciliation: paper fills vs simulated book state
- Paper account initialization: explicit `PaperAccount::new(initial_cash, initial_equity)` (no Default derivation)
- Execution boundary: Research → Promotion Gate → Paper → Shadow → Canary → Live (isolated, disabled by default, separate crate, no credentials in repo)
**Changes to Existing File**:
- Add ARCHITECTURE.md design goals 1,4,7,8 to scope
- Add explicit initialization requirement (no Default)
- Add reconciliation detail (compare paper fills vs simulated book state)
- Add live adapter isolation (separate crate, disabled by default)

### D.6 phase-06-production-monitoring.md
**A Mapping References**: Master sections 10,28,29,30,42,43,44,45 + PLAN sections 14,15,16,27,28,29,30,31,33,34,35,36
**ARCHITECTURE.md Fields**: Design goals 1,3,6,8
**Working Version Definition**:
- Experiment registry: experiment_id, strategy_id, git_commit, dataset_id, universe_id, config_id, feature_version, model_version, timestamp, random_seed, training/validation/test periods, metrics, cost_assumptions, artifacts, promotion_decision; failed experiments retained with FAILED/REGIME_DEPENDENT tags
- Feature/data lineage: feature_id, formula, inputs, lookback, minimum_history, availability_delay, version; mechanical prevention of future leakage (future_return_24h blocked at t)
- Monitoring metrics: feed_latency, dropped_messages, data_gaps, feature_latency, signal_latency, orders, fills, slippage, PnL, drawdown, model_drift, feature_drift, strategy_decay; alerts on feed gap >5s, feature latency >500ms, drawdown > max_limit, strategy decay >20%
- Autonomous research loop: discover markets → update datasets → calculate features → run screeners → generate hypotheses → backtest → walk-forward → robustness → promote challengers → paper trade → monitor → retire/promote (continuous, not on-demand)
- Dashboard: Market Radar, Strategy Lab, Portfolio, Data Health
- Testing scale: unit, integration, property (position_size <= risk_limit, portfolio_heat <= max, no future timestamps in features, replay deterministic), golden-data (fixed dataset → fixed metrics)
**Changes to Existing File**:
- Add ARCHITECTURE.md design goals 1,3,6,8 to scope
- Add experiment registry 9 reproducibility fields (code_commit, config_hash, dataset_id, feature_versions, strategy_version, model_version, random_seed, execution_model_version)
- Add feature lineage mechanical prevention detail
- Add autonomous loop as continuous (not on-demand)

### D.7 index.md
**A Mapping References**: Master sections 1,2,5-12 + PLAN sections 37
**ARCHITECTURE.md Fields**: All design goals (1-8)
**Working Version Definition**:
- Master index with phase overview table, priority order, working version definition, architecture reminder
- Maps all 6 phases to their categories and key deliverables
- Priority order from PLAN.md (14 items)
- Architecture reminder data flow diagram
**Changes to Existing File**:
- Add ARCHITECTURE.md design goals 1-8 to architecture reminder
- Update phase overview to include ARCHITECTURE.md field references
- Add observation fields and signal contract to architecture reminder
- Add replay/reproducibility/lifecycle references from C mapping

---

## Final Verification Wave Results

### F1. Plan Compliance Audit
- [ ] All 8 required section headers present in order: TL;DR, Scope, Verification strategy, Execution strategy, Todos, Final verification wave, Commit strategy, Success criteria
- [ ] Section headers match exactly

### F2. Code Quality Review
- [ ] All 3 references cited with line ranges
- [ ] No executable code snippets in working versions
- [ ] Structural descriptions only

### F3. Real Manual QA
- [ ] A mapping: 46 master + 42 PLAN.md references counted
- [ ] B specs: 7 files named with rewrite specs
- [ ] ARCHITECTURE.md: 11 observation fields, 9 signal fields, 8 design goals mapped

### F4. Scope Fidelity
- [ ] No speculative categories added (Freqtrade, live execution, news/on-chain only where in references)
- [ ] No new architecture sections invented
- [ ] No crate code edits specified
- [ ] Single output file maintained