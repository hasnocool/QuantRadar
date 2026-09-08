# test-suite-regression - Work Plan

## TL;DR (For humans)

**What you'll get:** A complete regression test suite covering all 10 Rust crates and the Python research layer. Currently only 5 unit tests exist; this adds ~40 test files with unit, property-based, golden-data, integration, data-leakage, and determinism tests. All verification runs automatically in CI with zero human intervention.

**Why this approach:** Property tests catch math regressions (EMA monotonicity, RSI bounds, ATR non-negative) that unit tests miss. Golden fixtures ensure deterministic replay. Data-leakage tests enforce the core QuantRadar contract: no future info in features/signals/backtest. Integration tests validate cross-crate pipelines end-to-end.

**What it will NOT do:** No live network tests (mocks only), no UI/browser tests (no UI), no load/stress/fuzz/benchmark tests (separate concerns), no mutation testing (default off), no real historical data fixtures (synthetic only unless you opt in).

**Effort:** Large
**Risk:** Medium - many test files but well-defined boundaries per crate

**Decisions I made for you:** proptest + pytest/hypothesis for property tests; synthetic fixtures in tests/fixtures/; workspace-level integration tests; CI runs cargo test + pytest on every PR. Veto any at the gate.

Your next move: approve to start, or request high-accuracy review (already required, will run automatically).

---

> TL;DR (machine): Large effort, Medium risk, delivers 40+ test files across 13 modules with property/golden/integration/leakage/determinism coverage

## Scope
### Must have
- Unit tests for all public functions in 10 Rust crates (core, features, exchange-kraken, screeners, regime, microstructure, research, backtest, execution, reporting)
- Property-based tests for 20+ numeric invariants via proptest (Rust) + hypothesis (Python)
- Golden-data tests with synthetic Bar/FeatureRow fixtures per crate
- Integration tests for 5 cross-crate pipelines
- Data leakage/accuracy tests enforcing no-future-info contract
- Backtest determinism/replay tests (bitwise equity curve equality)
- Serialization round-trip tests for all Serde types
- Python unit + property tests for robustness, walk_forward, anomaly modules
- CI configuration for `cargo test --workspace` + `pytest`

### Must NOT have (guardrails, anti-slop, scope boundaries)
- Live network tests against real Kraken API
- Browser/E2E UI tests
- Load/stress/fuzz/performance benchmarks
- Mutation testing (cargo-mutants) - separate concern
- Real historical data fixtures (synthetic only by default)
- Contract tests against external services

## Verification strategy
> Zero human intervention - all verification is agent-executed.
- Test decision: tests-after (existing code) + TDD for new test infrastructure
- Framework: Rust built-in #[cfg(test)] + proptest; Python pytest + hypothesis
- Evidence: .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-<N>-test-suite-regression.json

## Execution strategy
### Parallel execution waves
> Target 5-8 todos per wave. Fewer than 3 (except the final) means you under-split.

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1 | - | 2,3,4,5,6,7,8,9,10 | - |
| 2 | 1 | 11 | 3,4,5,6,7,8,9,10 |
| 3 | 1 | 11 | 2,4,5,6,7,8,9,10 |
| 4 | 1 | 11 | 2,3,5,6,7,8,9,10 |
| 5 | 1 | 11 | 2,3,4,6,7,8,9,10 |
| 6 | 1 | 11 | 2,3,4,5,7,8,9,10 |
| 7 | 1 | 11 | 2,3,4,5,6,8,9,10 |
| 8 | 1 | 11 | 2,3,4,5,6,7,9,10 |
| 9 | 1 | 11 | 2,3,4,5,6,7,8,10 |
| 10 | 1 | 11 | 2,3,4,5,6,7,8,9 |
| 11 | 2,3,4,5,6,7,8,9,10 | 12,13 | - |
| 12 | 11 | 14 | 13 |
| 13 | 11 | 14 | 12 |
| 14 | 12,13 | 15 | - |
| 15 | 14 | - | - |

## Todos
> Implementation + Test = ONE todo. Never separate.
<!-- APPEND TASK BATCHES BELOW THIS LINE WITH edit/apply_patch - never rewrite the headers above. -->
- [ ] 1. Create shared test fixtures and proptest strategies at workspace level
  What to do / Must NOT do: Create tests/fixtures/ with synthetic Bar/FeatureRow generators (deterministic seeds, no network). Create tests/strategies/ with proptest strategies for Bar, FeatureRow, MarketId, Signal, Regime. Must NOT use real API data. Must NOT create fixtures that require external downloads.
  Parallelization: Wave 1 | Blocked by: - | Blocks: 2,3,4,5,6,7,8,9,10
  References (executor has NO interview context - be exhaustive): crates/core/src/lib.rs:8-38 (Bar, FeatureRow, MarketId, Regime, Direction, OrderSide, Signal, OrderBookSnapshot, TradeTick), crates/features/src/lib.rs:20-39 (feature_rows)
  Acceptance criteria (agent-executable): cargo test --workspace compiles; fixtures generate valid Bar vectors with OHLCV consistency (high >= max(open,close), low <= min(open,close), volume > 0); FeatureRow from fixtures has all fields populated correctly
  QA scenarios (name the exact tool + invocation): happy: cargo test -p quantaradar-core fixtures:: -- --nocapture; failure: verify invalid Bar (high < low) is rejected by generator, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-1-test-suite-regression.json
  Commit: Y | test(fixtures): add shared synthetic Bar/FeatureRow generators and proptest strategies

- [ ] 2. Core crate: unit, property, golden, serialization tests
  What to do / Must NOT do: Add tests/unit.rs (extend existing), tests/property.rs (proptest: Bar range/typical_price, MarketId equality, Direction/OrderSide/Regime enum exhaustiveness, Signal serialization round-trip), tests/golden.rs (serialize/deserialize known JSON), tests/serialization.rs (version tolerance). Must NOT test private functions. Must NOT skip enum variant coverage.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/core/src/lib.rs:1-41
  Acceptance criteria: cargo test -p quantaradar-core passes; proptest runs 1000+ cases for each property; golden tests match committed JSON fixtures; serde round-trip preserves all fields bitwise
  QA scenarios: happy: cargo test -p quantaradar-core -- --test-threads=4; failure: inject corrupted JSON into golden test, verify descriptive error, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-2-test-suite-regression.json
  Commit: Y | test(core): add property/golden/serialization tests for all domain types

- [ ] 3. Features crate: unit, property, leakage, golden tests
  What to do / Must NOT do: Extend tests/unit.rs (existing EMA/RSI/ATR/feature_rows tests), add tests/property.rs (proptest: EMA monotonic after warmup, RSI in [0,100], ATR >= 0, realized_vol_20 >= 0, bb_width_20 >= 0, volume_z_20 finite, feature_rows lookback boundaries: first period-1 rows have None), tests/leakage.rs (assert no future data: feature_rows(i) depends only on bars[0..=i]), tests/golden.rs (deterministic feature_rows output for fixed seed bars). Must NOT allow any test to access bars[i+1] in feature computation.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/features/src/lib.rs:1-69
  Acceptance criteria: cargo test -p quantaradar-features passes; property tests verify 20+ invariants; leakage test fails if feature_rows uses future bars; golden test produces identical FeatureRow vectors across runs
  QA scenarios: happy: cargo test -p quantaradar-features property leakage; failure: modify feature_rows to peek at bars[i+1], verify leakage test catches it, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-3-test-suite-regression.json
  Commit: Y | test(features): add property/leakage/golden tests for all indicators and feature_rows

- [ ] 4. Exchange-Kraken crate: unit, integration (mocked), error handling tests
  What to do / Must NOT do: Add tests/unit.rs (KrakenClient construction, URL building, response parsing), tests/integration.rs (mockito/httpmock server: asset_pairs, ohlc, discover_spot success + rate limit 429, timeout, malformed JSON, empty response), tests/property.rs (proptest: pair symbol normalization, interval validation). Must NOT make real HTTP calls. Must NOT test private fields.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/exchange-kraken/src/lib.rs:1-17
  Acceptance criteria: cargo test -p quantaradar-exchange-kraken passes; mocked server returns canned responses; rate limit handling respects min_interval; error variants (timeout, 429, malformed) produce correct anyhow::Error kinds
  QA scenarios: happy: cargo test -p quantaradar-exchange-kraken integration; failure: mock 500 response, verify error propagation, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-4-test-suite-regression.json
  Commit: Y | test(exchange-kraken): add mocked integration tests for all REST endpoints

- [ ] 5. Screeners crate: unit, property, golden, regime-gate tests
  What to do / Must NOT do: Add tests/unit.rs (each of 7 screeners: Trend, Breakout, MeanReversion, VolatilityExpansion, VolumeSurge, MomentumDivergence, SupportResistanceBounce), tests/property.rs (proptest: score in [0,1], direction matches rationale, rationale non-empty when signal emitted), tests/regime_gate.rs (verify screeners respect Regime input - e.g. Trend only fires in bull regimes), tests/golden.rs (fixed FeatureRow input → expected Signal output). Must NOT test private make() helper directly.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/screeners/src/lib.rs:1-33
  Acceptance criteria: cargo test -p quantaradar-screeners passes; each screener tested with bullish/bearish/sideways FeatureRow vectors; regime gate tests verify correct filtering; golden tests produce deterministic Signal structs
  QA scenarios: happy: cargo test -p quantaradar-screeners regime_gate; failure: feed bearish data to TrendScreener, verify empty Vec, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-5-test-suite-regression.json
  Commit: Y | test(screeners): add unit/property/regime-gate/golden tests for all 7 screener families

- [ ] 6. Regime crate: unit, property, confidence, transition tests
  What to do / Must NOT do: Add tests/unit.rs (classify, classify_with_confidence, multi_timeframe_regime), tests/property.rs (proptest: regime transitions only on boundary crosses, confidence High/Medium/Low matches trend_aligned+vol_aligned), tests/golden.rs (known FeatureRow → expected RegimeState), tests/transition.rs (regime_transition detects changes, returns None on same). Must NOT hardcode threshold values in tests (use RegimeThresholds::default()).
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/regime/src/lib.rs:1-56
  Acceptance criteria: cargo test -p quantaradar-regime passes; property tests cover all 11 Regime variants; confidence logic matches truth table in classify_with_confidence; transition detection fires exactly once per change
  QA scenarios: happy: cargo test -p quantaradar-regime property; failure: flip trend_aligned without vol change, verify Medium confidence, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-6-test-suite-regression.json
  Commit: Y | test(regime): add property/confidence/transition/golden tests for regime classifier

- [ ] 7. Microstructure crate: unit, property, edge-case, golden tests
  What to do / Must NOT do: Add tests/unit.rs (analyze with normal book/trades), tests/property.rs (proptest: spread_bps >= 0, bid_depth_usd >= 0, depth_imbalance in [-1,1], trade_buy_ratio in [0,1], impact_bps_* >= 0 or INFINITY, liquidity_score > 0), tests/edge_cases.rs (empty book, empty trades, zero mid price, infinite spread, zero depth), tests/golden.rs (fixed OrderBookSnapshot+TradeTick → expected MicrostructureFeatures). Must NOT assume book always has bids/asks.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/microstructure/src/lib.rs:1-50
  Acceptance criteria: cargo test -p quantaradar-microstructure passes; property tests verify all 11 field invariants; edge cases return finite defaults (not panic); golden test matches committed output
  QA scenarios: happy: cargo test -p quantaradar-microstructure edge_cases; failure: empty OrderBookSnapshot, verify spread_bps=INFINITY, liquidity_score=0, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-7-test-suite-regression.json
  Commit: Y | test(microstructure): add property/edge-case/golden tests for analyze()

- [ ] 8. Research crate: unit, property, golden, PCA tests
  What to do / Must NOT do: Add tests/unit.rs (breadth, relative_strength, rank, correlation_matrix, pca_first_component, events, generate_strategies), tests/property.rs (proptest: breadth ratios in [0,1], relative_strength percentile in [0,1], rank scores sorted descending, correlation_matrix symmetric with 1.0 diagonal, pca_first_component norm=1, events severity in [0,1]), tests/golden.rs (fixed AssetSnapshot vectors → expected outputs), tests/pca.rs (pca_first_component converges, handles n=1). Must NOT test private helpers.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/research/src/lib.rs:1-29
  Acceptance criteria: cargo test -p quantaradar-research passes; property tests verify 15+ invariants; PCA test converges within iterations; rank() output deterministic for fixed input
  QA scenarios: happy: cargo test -p quantaradar-research pca; failure: identical return vectors in correlation_matrix, verify 1.0 diagonal, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-8-test-suite-regression.json
  Commit: Y | test(research): add property/golden/PCA tests for all research analytics

- [ ] 9. Backtest crate: unit, property, determinism, replay, golden tests
  What to do / Must NOT do: Add tests/unit.rs (BacktestConfig default, TradeRecord fields), tests/property.rs (proptest: equity curve never NaN, max_drawdown in [0,1], sharpe finite, profit_factor >= 0 or INFINITY, total_return = final_cash/initial_cash - 1), tests/determinism.rs (run() twice with same bars+config → bitwise identical BacktestResult equity vector), tests/replay.rs (serialize bars+config, deserialize, run → identical result), tests/golden.rs (fixed bars → expected BacktestResult), tests/liquidity_gate.rs (min_liquidity_score filters trades), tests/partial_fills.rs (partial_fill_probability produces correct fill ratios). Must NOT allow non-determinism in simulate_partial_fill (uses hash of qty).
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/backtest/src/lib.rs:1-196
  Acceptance criteria: cargo test -p quantaradar-backtest passes; determinism test passes 100 consecutive runs; replay test survives serde round-trip; liquidity gate test verifies zero trades when score < threshold; partial fill test verifies fill_ratio in [0.3, 1.0]
  QA scenarios: happy: cargo test -p quantaradar-backtest determinism replay; failure: modify simulate_partial_fill to use thread_rng, verify determinism test fails, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-9-test-suite-regression.json
  Commit: Y | test(backtest): add property/determinism/replay/liquidity/partial-fill/golden tests

- [ ] 10. Execution crate: unit, property, risk-limit, golden tests
  What to do / Must NOT do: Add tests/unit.rs (size_position, approve, paper_fill, update_account), tests/property.rs (proptest: size_position <= max_position_pct*equity/entry, size_position <= risk_dollars/risk_per_unit, approve returns None when direction!=Long or score<=0 or liquidity<min or positions>=max), tests/risk_limits.rs (max_portfolio_heat caps total risk, max_concurrent_positions enforced, max_drawdown_pct not tested here but in backtest), tests/golden.rs (fixed Signal+book+limits → expected OrderIntent/Fill/PaperAccount). Must NOT test private fields.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/execution/src/lib.rs:1-39
  Acceptance criteria: cargo test -p quantaradar-execution passes; property tests verify all 4 risk limit invariants; approve() correctly rejects Short signals (catches Direction::Long vs "long" bug); paper_fill handles bid/ask correctly
  QA scenarios: happy: cargo test -p quantaradar-execution risk_limits; failure: signal.direction = Short, verify approve returns None, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-10-test-suite-regression.json
  Commit: Y | test(execution): add property/risk-limit/golden tests for paper trading engine

- [ ] 11. Reporting crate: unit, property, golden tests
  What to do / Must NOT do: Add tests/unit.rs (write_json creates parent dirs, writes valid JSON), tests/property.rs (proptest: write_json + read = original value for all Serde types from core/features), tests/golden.rs (serialize known structs → expected JSON). Must NOT test filesystem permissions.
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 11
  References: crates/reporting/src/lib.rs:1-5
  Acceptance criteria: cargo test -p quantaradar-reporting passes; property test covers FeatureRow, Signal, BacktestResult, RegimeState round-trip
  QA scenarios: happy: cargo test -p quantaradar-reporting; failure: write to read-only dir, verify error, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-11-test-suite-regression.json
  Commit: Y | test(reporting): add property/golden tests for write_json

- [ ] 12. Python robustness: unit, property, boundary tests
  What to do / Must NOT do: Create tests/test_robustness.py (pytest: perturb_parameters scales, robustness_test pass_rate calculation, should_promote thresholds), tests/property.rs (hypothesis: params dict with float values, min_sharpe/max_dd/min_pass_rate boundaries, empty variants list, NaN handling). Must NOT use real market data.
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: 14
  References: python/quantaradar/robustness.py:1-46
  Acceptance criteria: pytest tests/robustness/ -v passes; property tests verify should_promote true/false at exact threshold boundaries (0.10 sharpe diff, drawdown equality, profit_factor equality)
  QA scenarios: happy: pytest tests/test_robustness.py; failure: challenger.sharpe = champion.sharpe + 0.099, verify should_promote=False, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-12-test-suite-regression.json
  Commit: Y | test(python-robustness): add unit/property/boundary tests for promotion logic

- [ ] 13. Python walk-forward: unit, property, geometry tests
  What to do / Must NOT do: Create tests/test_walk_forward.py (pytest: expanding_folds valid/invalid geometry, summarize_returns with known returns), tests/property.rs (hypothesis: n, initial_train, test_size, step combinations; invalid args raise ValueError; folds cover full range without overlap/gaps). Must NOT test private functions.
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: 14
  References: python/quantaradar/walk_forward/evaluator.py:1-20
  Acceptance criteria: pytest tests/walk_forward/ -v passes; property tests verify fold geometry invariants (train_end strictly increasing, test_start == prev train_end, no index out of bounds)
  QA scenarios: happy: pytest tests/test_walk_forward.py; failure: initial_train + test_size > n, verify ValueError, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-13-test-suite-regression.json
  Commit: Y | test(python-walkforward): add unit/property/geometry tests for expanding folds

- [ ] 14. Python anomaly: unit, property, edge-case tests
  What to do / Must NOT do: Create tests/test_anomaly.py (pytest: isolation_scores with inf/-inf/NaN, empty DataFrame, single row, all-same values), tests/property.rs (hypothesis: random DataFrames with numeric columns, output Series same index, scores >= 0). Must NOT fit model on test data (use fixed random_state).
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: 14
  References: python/quantaradar/ml/anomaly.py:1-10
  Acceptance criteria: pytest tests/anomaly/ -v passes; inf/-inf replaced with NaN then dropped; output index aligns with input; scores non-negative
  QA scenarios: happy: pytest tests/test_anomaly.py; failure: DataFrame with all NaN column, verify output all zeros, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-14-test-suite-regression.json
  Commit: Y | test(python-anomaly): add unit/property/edge-case tests for isolation forest scoring

- [ ] 15. Cross-crate integration tests: 5 end-to-end pipelines
  What to do / Must NOT do: Create tests/integration/ with 5 test files: test_discover_screen.rs (KrakenClient.discover_spot → feature_rows → regime → screeners), test_fetch_backtest.rs (KrakenClient.ohlc → feature_rows → backtest.run), test_screen_execute.rs (screeners → signals → execution.approve → paper_fill → update_account), test_microstructure_backtest.rs (microstructure.analyze → liquidity_gate → backtest), test_walkforward_promote.rs (walk_forward → robustness → should_promote). Use mocked Kraken responses. Must NOT make real network calls. Must NOT skip any pipeline stage.
  Parallelization: Wave 4 | Blocked by: 12,13 | Blocks: 16
  References: crates/exchange-kraken/src/lib.rs, crates/features/src/lib.rs, crates/regime/src/lib.rs, crates/screeners/src/lib.rs, crates/backtest/src/lib.rs, crates/execution/src/lib.rs, crates/microstructure/src/lib.rs, python/quantaradar/walk_forward/evaluator.py, python/quantaradar/robustness.py
  Acceptance criteria: cargo test --workspace integration passes; pytest tests/integration/ passes; each pipeline produces valid output types; no stage panics
  QA scenarios: happy: cargo test --workspace integration; failure: mock Kraken 429 on discover, verify graceful degradation, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-15-test-suite-regression.json
  Commit: Y | test(integration): add 5 cross-crate end-to-end pipeline tests

- [ ] 16. CI configuration: GitHub Actions workflow for cargo test + pytest
  What to do / Must NOT do: Create .github/workflows/test.yml with: cargo check --workspace, cargo test --workspace --all-targets, cargo test --workspace --features=property-tests (if feature-gated), pytest -xvs (with python deps cached). Run on push/PR. Cache cargo target/ and ~/.cargo/registry. Must NOT run slow tests (property tests with high iterations) on every PR - gate behind label or schedule.
  Parallelization: Wave 5 | Blocked by: 14 | Blocks: -
  References: Cargo.toml workspace, pyproject.toml
  Acceptance criteria: workflow YAML valid; runs on push; cargo test --workspace passes in CI; pytest passes in CI; caches restore correctly
  QA scenarios: happy: push to branch, verify workflow runs green; failure: introduce failing test, verify CI catches it, Evidence .omo/evidence/ulw/<session>/<goalId>/a<attempt>/task-16-test-suite-regression.json
  Commit: Y | ci: add GitHub Actions workflow for comprehensive test suite

## Final verification wave
> Runs in parallel after ALL todos. ALL must APPROVE. Surface results and wait for the user's explicit okay before declaring complete.
- [ ] F1. Plan compliance audit
- [ ] F2. Code quality review
- [ ] F3. Real manual QA
- [ ] F4. Scope fidelity

## Commit strategy
Each todo = one atomic commit with conventional commit message (type(scope): summary). Wave commits can be squashed locally before PR. No WIP commits in main branch.

## Success criteria
- `cargo test --workspace` passes with 0 failures (all 10 crates)
- `pytest` passes with 0 failures (all 3 Python modules)
- Property tests execute 1000+ cases per invariant (proptest/hypothesis default)
- Determinism test: 100 consecutive backtest runs produce bitwise-identical equity curves
- Leakage test: any future-data access in feature_rows/backtest causes test failure
- Integration tests: 5 pipelines complete with valid output types
- CI workflow runs green on PR
- Total test count: 200+ individual test cases across suite