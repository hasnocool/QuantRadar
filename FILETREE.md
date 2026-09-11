# QuantRadar — Complete File Tree

**Generated:** 2026-09-11 20:05:23 UTC

---

## Root

```
QuantRadar/
├── .ignore
├── .omo/
│   ├── boulder.json
│   ├── drafts/
│   │   ├── install-parallel-worker-plugins.md
│   │   ├── next-2-3-milestones.md
│   │   ├── phased-plan-mapping.md
│   │   └── test-suite-regression.md
│   ├── evidence/
│   │   └── phased-plan-mapping/
│   │       ├── task-1-phased-plan-mapping.md
│   │       ├── task-2-phased-plan-mapping.md
│   │       ├── task-3-phased-plan-mapping.md
│   │       └── task-4-phased-plan-mapping.md
│   ├── notepads/
│   │   ├── next-2-3-milestones/
│   │   │   ├── decisions.md
│   │   │   ├── issues.md
│   │   │   ├── learnings.md
│   │   │   └── problems.md
│   │   └── phased/
│   │       ├── decisions.md
│   │       ├── issues.md
│   │       ├── learnings.md
│   │       └── problems.md
│   ├── plans/
│   │   ├── install-parallel-worker-plugins.md
│   │   ├── next-2-3-milestones.md
│   │   ├── phased-plan-mapping.md
│   │   └── test-suite-regression.md
│   ├── run-continuation/
│   │   └── (50+ session continuation JSON files)
│   └── start-work/
│       └── ledger.jsonl
├── .pi/
│   └── agents/
│       └── loop.md
├── .agents/
│   ├── loop-agent.sh
│   ├── loop-cmd.sh
│   ├── recursive-agent.sh
│   ├── self-improving-agent.sh
│   └── registry.json
├── .opencode/
│   └── agent/
│       └── quantaradar.md
├── AGENTS.md
├── AGENT_INSTRUCTIONS.md
├── ARCHITECTURE.md
├── ARCHITECTURE_CANONICAL.md
├── CLI_COVERAGE.md
├── CLI_EXAMPLES.md
├── CLI_REFERENCE.md
├── Cargo.lock
├── Cargo.toml
├── MASTERLIST.md
├── PLAN.md
├── PLAN_PHASE_2.md
├── PROJECT_ROOT_AUDIT_REPORT.md
├── README.md
├── README.CLI.md
├── ROOT_GAP_AUDIT.md
├── TASKS.md
├── TODO.md
├── VALIDATION.md
├── audit_coverage.py
├── codemap.md
├── generate_codemaps.py
├── configs/
│   ├── codemap.md
│   └── default.yaml
├── crates/
│   ├── codemap.md
│   ├── shared/
│   │   ├── archives/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── backtest/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── backtest_engine/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── cli/
│   │   │   ├── Cargo.toml
│   │   │   ├── Cargo.toml.new
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   ├── lib.rs
│   │   │   │   ├── main.rs
│   │   │   │   ├── main_clean.rs
│   │   │   │   ├── main_implemented.rs
│   │   │   │   └── main_minimal.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── core/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       ├── basic.rs
│   │   │       └── unit.rs
│   │   ├── dashboard/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── data-model/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── domain-model/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── exchange-kraken/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   ├── lib.rs
│   │   │   │   ├── sequence.rs
│   │   │   │   └── ws.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── execution/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── expected-return/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── experiment/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── features/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── ingestion/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   ├── collector.rs
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── microstructure/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── news_ingestion/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── alerting.rs
│   │   │   │   ├── bin/
│   │   │   │   │   └── run_ingest.rs
│   │   │   │   ├── codemap.md
│   │   │   │   ├── google_news.rs
│   │   │   │   ├── lib.rs
│   │   │   │   ├── news_api.rs
│   │   │   │   ├── onchain.rs
│   │   │   │   ├── reddit_json.rs
│   │   │   │   ├── reddit_rss.rs
│   │   │   │   └── storage.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── order-book/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── ranking/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── regime/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── reporting/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── research/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   ├── factor_engine.rs
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── screeners/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   ├── signal-ensemble/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   ├── storage/
│   │   │   ├── Cargo.toml
│   │   │   ├── codemap.md
│   │   │   ├── src/
│   │   │   │   ├── codemap.md
│   │   │   │   └── lib.rs
│   │   │   └── tests/
│   │   │       └── basic.rs
│   │   └── websocket/
│   │       ├── Cargo.toml
│   │       ├── codemap.md
│   │       ├── src/
│   │       │   ├── codemap.md
│   │       │   └── lib.rs
│   │       └── tests/
│   │           └── basic.rs
│   └── standalone/
│       ├── data-quality/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── derivatives/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── ensemble/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── event_bus/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── events/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── exchange-binance/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── exchange-coinbase/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── expected_return/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── feature-engine/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── feature-store/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   ├── lib.rs
│       │   │   └── lib.rs.bak
│       │   └── tests/
│       │       └── basic.rs
│       ├── freqtrade_integration/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── hold-period/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── lineage/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── live-exec/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── logging/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── lib.rs
│       │   │   └── main.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── model_registry/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── monitoring/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── multi_exchange/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── optimization/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── orderbook/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── paper-trading/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── paper/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── pca/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── persistent-data/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   ├── test_persist.json
│       │   └── tests/
│       │       └── basic.rs
│       ├── pipeline/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── portfolio/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── portfolio_risk/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── promotion_gate/
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   │   └── lib.rs
│       ├── rate-limiter/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── regime-detector/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── registry/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── replay/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── report-gen/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   └── src/
│       │       └── main.rs
│       ├── risk/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── scheduler/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── sentiment/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── signals/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── strategies/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── strategy_dsl/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── test-scale/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   ├── lib.rs
│       │   │   └── property_tests.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── types/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── universe/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── universe_history/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       ├── validation/
│       │   ├── Cargo.toml
│       │   ├── codemap.md
│       │   ├── src/
│       │   │   ├── codemap.md
│       │   │   └── lib.rs
│       │   └── tests/
│       │       └── basic.rs
│       └── websocket/
│           ├── Cargo.toml
│           ├── codemap.md
│           ├── src/
│           │   ├── codemap.md
│           │   └── lib.rs
│           └── tests/
│               └── basic.rs
├── data/
│   ├── kraken.json
│   ├── news/
│   │   ├── alerts.json
│   │   ├── dashboard.json
│   │   ├── news_1788902210.json
│   │   ├── news_1788902260.json
│   │   ├── news_1788903331.json
│   │   ├── news_1788903951.json
│   │   ├── news_1788904479.json
│   │   ├── news_1788904660.json
│   │   ├── news_1788904749.json
│   │   ├── news_1788904863.json
│   │   ├── news_1788904970.json
│   │   ├── onchain_1788902210.json
│   │   ├── onchain_1788902260.json
│   │   ├── onchain_1788903331.json
│   │   ├── onchain_1788903951.json
│   │   ├── onchain_1788904479.json
│   │   ├── onchain_1788904660.json
│   │   ├── onchain_1788904749.json
│   │   ├── onchain_1788904863.json
│   │   └── onchain_1788904970.json
│   └── raw/
│       ├── BTC_USD.json
│       └── ETH_USD.json
├── docs/
│   ├── architecture/
│   │   └── ARCHITECTURE_CANONICAL.md
│   ├── cli/
│   │   ├── CLI_COVERAGE.md
│   │   ├── CLI_EXAMPLES.md
│   │   └── CLI_REFERENCE.md
│   ├── control-plan/
│   │   ├── AGENTS.md
│   │   ├── MASTERLIST.md
│   │   ├── PLAN.md
│   │   ├── TASKS.md
│   │   └── TODO.md
│   └── crates/
│       ├── CRATE_CANONICAL_archives.md
│       ├── CRATE_CANONICAL_backtest.md
│       ├── CRATE_CANONICAL_backtest_engine.md
│       ├── CRATE_CANONICAL_cli.md
│       ├── CRATE_CANONICAL_core.md
│       ├── CRATE_CANONICAL_dashboard.md
│       ├── CRATE_CANONICAL_data-model.md
│       ├── CRATE_CANONICAL_data-quality.md
│       ├── CRATE_CANONICAL_derivatives.md
│       ├── CRATE_CANONICAL_domain-model.md
│       ├── CRATE_CANONICAL_ensemble.md
│       ├── CRATE_CANONICAL_event_bus.md
│       ├── CRATE_CANONICAL_events.md
│       ├── CRATE_CANONICAL_exchange-binance.md
│       ├── CRATE_CANONICAL_exchange-coinbase.md
│       ├── CRATE_CANONICAL_exchange-kraken.md
│       ├── CRATE_CANONICAL_execution.md
│       ├── CRATE_CANONICAL_expected-return.md
│       ├── CRATE_CANONICAL_expected_return.md
│       ├── CRATE_CANONICAL_experiment.md
│       ├── CRATE_CANONICAL_feature-engine.md
│       ├── CRATE_CANONICAL_feature-store.md
│       ├── CRATE_CANONICAL_features.md
│       ├── CRATE_CANONICAL_freqtrade_integration.md
│       ├── CRATE_CANONICAL_hold-period.md
│       ├── CRATE_CANONICAL_ingestion.md
│       ├── CRATE_CANONICAL_lineage.md
│       ├── CRATE_CANONICAL_live-exec.md
│       ├── CRATE_CANONICAL_logging.md
│       ├── CRATE_CANONICAL_microstructure.md
│       ├── CRATE_CANONICAL_model_registry.md
│       ├── CRATE_CANONICAL_monitoring.md
│       ├── CRATE_CANONICAL_multi_exchange.md
│       ├── CRATE_CANONICAL_news_ingestion.md
│       ├── CRATE_CANONICAL_optimization.md
│       ├── CRATE_CANONICAL_order-book.md
│       ├── CRATE_CANONICAL_orderbook.md
│       ├── CRATE_CANONICAL_paper-trading.md
│       ├── CRATE_CANONICAL_paper.md
│       ├── CRATE_CANONICAL_pca.md
│       ├── CRATE_CANONICAL_persistent-data.md
│       ├── CRATE_CANONICAL_pipeline.md
│       ├── CRATE_CANONICAL_portfolio.md
│       ├── CRATE_CANONICAL_portfolio_risk.md
│       ├── CRATE_CANONICAL_ranking.md
│       ├── CRATE_CANONICAL_rate-limiter.md
│       ├── CRATE_CANONICAL_regime-detector.md
│       ├── CRATE_CANONICAL_regime.md
│       ├── CRATE_CANONICAL_registry.md
│       ├── CRATE_CANONICAL_replay.md
│       ├── CRATE_CANONICAL_report-gen.md
│       ├── CRATE_CANONICAL_reporting.md
│       ├── CRATE_CANONICAL_research.md
│       ├── CRATE_CANONICAL_risk.md
│       ├── CRATE_CANONICAL_scheduler.md
│       ├── CRATE_CANONICAL_screeners.md
│       ├── CRATE_CANONICAL_sentiment.md
│       ├── CRATE_CANONICAL_signal-ensemble.md
│       ├── CRATE_CANONICAL_signals.md
│       ├── CRATE_CANONICAL_storage.md
│       ├── CRATE_CANONICAL_strategies.md
│       ├── CRATE_CANONICAL_strategy_dsl.md
│       ├── CRATE_CANONICAL_test-scale.md
│       ├── CRATE_CANONICAL_types.md
│       ├── CRATE_CANONICAL_universe.md
│       ├── CRATE_CANONICAL_universe_history.md
│       ├── CRATE_CANONICAL_validation.md
│       └── CRATE_CANONICAL_websocket.md
├── generate_codemaps.py
├── plans/
│   ├── phased/
│   │   ├── index.md
│   │   ├── phase-01-data-foundation.md
│   │   ├── phase-02-market-intelligence.md
│   │   ├── phase-03-screener-signal.md
│   │   ├── phase-04-backtest-validation.md
│   │   ├── phase-05-portfolio-risk-paper.md
│   │   └── phase-06-production-monitoring.md
│   └── phased/
│       └── index.md
├── python/
│   ├── __init__.py
│   ├── codemap.md
│   ├── feature_store/
│   │   ├── codemap.md
│   │   └── store.py
│   ├── feature_store/
│   │   └── codemap.md
│   ├── pyproject.toml
│   ├── quantaradar/
│   │   ├── __init__.py
│   │   ├── codemap.md
│   │   ├── log_config.py
│   │   ├── ml/
│   │   │   ├── anomaly.py
│   │   │   └── codemap.md
│   │   ├── robustness.py
│   │   ├── test_robustness.py
│   │   └── walk_forward/
│   │       ├── codemap.md
│   │       ├── evaluator.py
│   │       └── test_evaluator.py
│   ├── tests/
│   │   └── test_walk_forward_robustness.py
│   └── uv.lock
├── tests/
│   ├── fixtures/
│   │   ├── bar_generator.rs
│   │   └── feature_row_generator.rs
│   └── strategies/
│       └── mod.rs
└── uv.lock
```

---

*This file tree was auto-generated on 2026-09-11 20:05:23 UTC. Re-run `generate_codemaps.py` or the file tree generator to refresh.*