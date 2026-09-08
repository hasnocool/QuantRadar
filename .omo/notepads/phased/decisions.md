# Decisions — phased plan

- Combined A+B mapping for phased-plan-mapping (full 46+42 section mapping + ARCHITECTURE.md incorporation) over default single-artifact approach
- ARCHITECTURE.md incorporated as third reference column in every mapped phase (observation fields, signal contract, design goals)
- No speculative sections: only categories present in at least one of the 3 references (QUANTRADAR_MASTER_ARCHITECTURE.md, PLAN.md, ARCHITECTURE.md)
- Single output file maintained: `.omo/plans/phased-plan-mapping.md` (even with A+B scope expanded)
- No executable code snippets in working version descriptions (structural only, reversible default per open assumptions)
- Replay engine: structural specification only — no implementation; verification checklist + reproducibility dependency list + lifecycle stage gates
- 11 observation fields from ARCHITECTURE.md:42 mapped to Phase 1; 9 signal contract fields from ARCHITECTURE.md:44-46 mapped to Phase 3; 8 design goals from ARCHITECTURE.md:49-57 mapped across phases
- Variances documented: V1 (phantom master source), V2 (PLAN rows 44≠42), V3 (obs. items 14≠11), V4 (goals 8≠7), V5 (cosmetic row-label dups 9,22), V6 (assignment text retains original estimates)
- No new architecture sections or architecture categories beyond references
- No live execution, Freqtrade, news/on-chain layers unless explicitly in references
- Single `.md` output file (no multiple separate files)
- Phase priority order from PLAN.md: 1=data foundation, 2=market intelligence, 3=screener/signal, 4=backtest/validation, 5=portfolio/risk/paper, 6=experiment/registry/monitoring
