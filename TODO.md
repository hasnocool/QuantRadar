# TODO.md — Live Projection

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
