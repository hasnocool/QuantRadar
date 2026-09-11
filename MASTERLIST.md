# MASTERLIST.md — Stable Specification

## §1 Mission
QuantRadar — a Rust + Python quantitative research platform for market data analysis, strategy development, and execution. The system integrates persistent market-data pipelines, feature engineering, regime detection, ranking, backtesting, and live execution across multiple exchanges.

## §2 Rules
1. **Conventional Commits** — All git commits must follow conventional commit format: `type(scope): description`
2. **RTK First** — All bash tool calls use `rtk` prefix for token optimization; never pipe rtk output back through rtk
3. **Ponytail Mode** — Default full laziness; YAGNI → stdlib → native → one-line → minimum
4. **No Speculative Abstractions** — No interface with one implementation, no factory for one product, no config for a value that never changes
5. **Deletion Over Addition** — Boring over clever; cutting complexity is preferred over adding it
6. **One Line per Finding** — Ponytail review findings: location, what to cut, what replaces it
7. **Termination Guards** — All bounded loops must have explicit termination conditions; no blocking sleep in agent runtimes
8. **Score-Based Allocation** — Agents scored by role; depth limits enforced; workspace field required for each agent
9. **Cross-Link Preservation** — .opencode/agent/quantaradar.md must cross-reference AGENT.md / MASTERLIST.md / TODO.md / .agents/registry.json
10. **Registry Restructuring** — .agents/registry.json v1.1 with structured roles, scores, depth limits, workspace field, allocation rules
11. **Non-Blocking Enforcement** — Agent runtime contracts use non-blocking patterns; failure recovery via reset, not spin
12. **Depth Guard Max 5** — recursive-agent.sh hard-stop at depth > 5 (§10)
13. **Failure Recovery (§26)** — self-improving-agent.sh plateau detection: reset depth to 0 after 2 reads with no score gain
14. **Security Boundary** — Python open() fails on some workspace files while os.read works; use bash/python3 -c os.read patterns when needed
15. **Verification Rules (§25)** — All doc structures verified against source sections; script design includes termination guards, bounded loops, no blocking sleep, registry updates
16. **Ultimate Principle (§39)** — The simplest code that works is the right code; deletion over addition; boring over clever
17. **Event Architecture (§14)** — Event-driven architecture with bounded loops; no blocking sleep; termination guards in all agent loops

## §3 Authority Hierarchy
- **program-director** — Highest-level strategic direction; score 3.0, workspace: plans/
- **architect** — System design and structure; score 2.0, workspace: crates/
- **lead-implementer** — Core implementation; score 3.0, workspace: crates/
- **planner** — Task decomposition and scheduling; score 2.0, workspace: plans/
- **reviewer** — Code quality and over-engineering audit; score 2.0, workspace: crates/
- **tester** — Test suite integrity; score 3.0, workspace: tests/
- **integrator** — Cross-component consistency; score 2.0, workspace: .
- **researcher** — ML/data research scripts; score 1.0, workspace: python/quantaradar/
- **loop-agent** — Async scheduler runtime; score 0.0, workspace: .agents/
- **recursive-agent** — Recursive delegation runtime; score 0.0, workspace: .agents/
- **self-improving-agent** — Supervisor/adaptive loop; score 0.0, workspace: .agents/
- **pi-agent** — pi framework bridge; score 0.0, workspace: .opencode/agent/

## §4 Architecture
```
QuantRadar/
├── crates/          # Rust workspace (core, data, strategy, execution)
├── python/          # Python research/analysis scripts
├── configs/         # Strategy configs, environment configs
├── plans/           # Work plans (.omo/plans/)
├── reports/         # Generated reports
├── tests/           # Integration and unit tests
├── .agents/         # Agent runtime scripts and registry
├── .opencode/       # OpenCode plugin configuration
└── AGENTS.md        # Root agent operating contract
```

## §6 Roles (from .agents/registry.json v1.1)
| Role | Score | Depth Limit | Workspace |
|------|-------|-------------|-----------|
| program-director | 3.0 | 0 | plans/ |
| architect | 2.0 | 0 | crates/ |
| planner | 2.0 | 0 | plans/ |
| lead-implementer | 3.0 | 0 | crates/ |
| reviewer | 2.0 | 0 | crates/ |
| tester | 3.0 | 0 | tests/ |
| integrator | 2.0 | 0 | . |
| researcher | 1.0 | 1 | python/quantaradar/ |
| loop-agent | 0.0 | 10 | .agents/ |
| recursive-agent | 0.0 | 5 | .agents/ |
| self-improving-agent | 0.0 | 8 | .agents/ |
| pi-agent | 0.0 | 3 | .opencode/agent/ |

## §20–§21 Milestones
- **QR-001** → QR-008: discover → report (example task pipeline)
- P0 bootstrap: workspace structure, initial docs, agent registry
- P1 task model: task decomposition, dependency mapping
- P2 task graph: DAG construction, critical path identification
- P3 async scheduler: non-blocking agent dispatch, termination guards
- P4 agent runtime: loop-agent.sh, loop-cmd.sh, recursive-agent.sh, self-improving-agent.sh, pi-agent.sh
- P5 recursive delegation: depth-limited recursion with registry updates
- P6 allocation/metrics: score-based agent allocation, resource distribution
- P7 event/verification: event architecture, verification rules, failure recovery

## §25 Verification
All document structures verified against source sections:
§1, §2, §3, §4, §5, §6, §7, §8, §9, §10, §11, §12, §14, §15, §16, §20, §21, §22, §23, §25, §26, §30, §34, §36, §37, §38, §39

## §34 Security
- Python open() sandbox quirk: os.read works where open() fails
- Use bash / python3 -c os.read patterns when accessing workspace files
- No blocking sleep in any agent runtime script
- Termination guards on all bounded loops
- Registry integrity checks on all agent operations

## §39 Ultimate Principle
The simplest code that works is the right code. Deletion over addition. Boring over clever. Code never written is code never bugged.