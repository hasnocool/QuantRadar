# QuantRadar Phased Plan — Master Index

Based on PLAN.md (2401 lines, 36 sections). Each phase describes the minimum working version of that category.

## Phase Overview

| Phase | File | Category | Key Deliverable |
|---|---|---|---|
| 1 | `phase-01-data-foundation.md` | Persistent data, replay, typed domain | `Direction` enum, replay engine, sequence validation, observation schema (ARCHITECTURE.md:42) |
| 2 | `phase-02-market-intelligence.md` | Features, regime, microstructure, ranking | `FeatureRow`, regime confidence, PCA component, feature store |
| 3 | `phase-03-screener-signal.md` | Screeners, signal ensemble, pipeline | 7 screener families, composite score, DSL, signal contract (ARCHITECTURE.md:44-46) |
| 4 | `phase-04-backtest-validation.md` | Backtest, WFO, robustness, champion/challenger | Realistic execution, expanding WFO, promotion gate |
| 5 | `phase-05-portfolio-risk-paper.md` | Portfolio optimizer, risk, paper trading | Liquidity-adjusted weights, regime scaling, paper state machine |
| 6 | `phase-06-production-monitoring.md` | Experiment registry, monitoring, autonomous loop | Registry, lineage, monitoring, continuous loop |

## Priority Order (from PLAN.md)
1. Persistent data + replay (Phase 1)
2. Production-grade multi-symbol WebSocket engine (Phase 1 / 2)
3. Strong typed domain/event model (Phase 1)
4. Real feature store (Phase 2)
5. Full cross-sectional intelligence/factor engine (Phase 2)
6. Realistic portfolio-aware backtester (Phase 4)
7. Experiment registry + dataset lineage (Phase 6)
8. Proper walk-forward/anti-overfitting framework (Phase 4)
9. Portfolio optimizer + advanced risk (Phase 5)
10. Signal ensemble + expected-return model (Phase 3)
11. Paper-trading state machine + reconciliation (Phase 5)
12. Dashboard/monitoring (Phase 6)
13. Freqtrade integration (future)
14. Isolated live-execution adapter (Phase 5, disabled)

## Working Version Definition
Each phase file defines:
- The minimum schema / enum / data structure.
- The minimum computation / pipeline flow.
- The verification checklist (what must pass for phase complete).
- No speculative abstractions, no future-proofing beyond the working version.

## Architecture Reminder (from PLAN.md + ARCHITECTURE.md)
Exchange Connectors → Real-Time Event Bus → Immutable Market Data + Replay
  ↓
Feature Store → Intelligence Layer (regime / factors / breadth / RS / microstructure)
  ↓
Screener Engine → Signal Ensemble → Expected Return / Risk Model
  ↓
Strategy Lab (generate + optimize) → Validation Engine (WFO / OOS / MC / PBO)
  ↓
Model Registry (Champion / Challenger) → Portfolio Optimizer → Risk Engine
  ↓
Paper Execution → Monitoring → Strategy Decay Detection → Research Memory

Design goals (ARCHITECTURE.md:49-57) apply across all phases:
1. Dynamic asset discovery instead of hard-coded universes.
2. Bounded asynchronous concurrency.
3. No data leakage.
4. Explicit trading costs.
5. Regime-conditional research.
6. Failed experiments retained for future learning.
7. Liquidity as a hard constraint.
8. Live execution isolated from research until promotion gates pass.

Observation fields (ARCHITECTURE.md:42) and signal contract (ARCHITECTURE.md:44-46) are the canonical schemas for Phase 1 and Phase 3 respectively.

Replay / reproducibility / lifecycle (from C mapping): replay verification checklist (input manifest = output manifest + state hash equality), reproducibility dependency list (9 fields), lifecycle stage gates (raw immutable → normalized reproducible → derived versioned).

## Next Step
Read each phase file. Delegate implementation via `task()` with the 6-section prompt structure. Verify with `lsp_diagnostics`, build, tests, and manual file review. Update `.omo/plans/` checkboxes as tasks complete.