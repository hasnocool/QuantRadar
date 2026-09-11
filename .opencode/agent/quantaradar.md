# .opencode/agent/quantaradar.md — Preserved Cross-Reference

This file is preserved and must cross-reference the QuantRadar control-plane documentation:

- **AGENT.md** — Operating contract (§2.6 / §24); agent runtime commands; RTK conventions; skills; token etiquette; QuantRadar-specific rules
- **MASTERLIST.md** — Stable spec: authority hierarchy (§3), mission (§1), architecture (§4), rules (§2), roles (§6), milestones (§20–§21), verification (§25), failure recovery (§26), security (§34), ultimate principle (§39)
- **TODO.md** — Live projection with milestones mapped to P0–P7; example tasks (QR-001 discover → QR-008 report); drift-detection rules (§23); verification rules (§25)
- **.agents/registry.json** — v1.1 structured roles (program-director, architect, planner, lead-implementer, worker, reviewer, tester, integrator, researcher, background-supervisor) + scores, depth limits, workspace field, allocation rules (§11 / §12)

## Required Cross-Links
1. AGENT.md §2.6 / §24 must be referenced in all agent runtime scripts (.agents/*.sh)
2. MASTERLIST.md §3 authority hierarchy must match registry.json agent roles
3. TODO.md milestones must map to P0–P7 phases in PLAN.md
4. .agents/registry.json v1.1 schema must be maintained across all agent operations
5. Root AGENTS.md must reference this control-plane structure (preserving existing RTK / skills / commands / conventions)

## Agent Runtime Contract References
- .agents/loop-agent.sh — references AGENT.md §§10-12
- .agents/loop-cmd.sh — references AGENT.md §11
- .agents/recursive-agent.sh — references AGENT.md §§8-9; depth guard max 5 (§10)
- .agents/self-improving-agent.sh — references AGENT.md §§22/28; failure recovery (§26)
- .agents/pi-agent.sh — references AGENT.md §§2/7; bridge to pi framework

## Verification
All document structures verified against source sections:
§1, §2, §3, §4, §5, §6, §7, §8, §9, §10, §11, §12, §14, §15, §16, §20, §21, §22, §23, §25, §26, §30, §34, §36, §37, §38, §39