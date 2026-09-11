# AGENT.md — QuantRadar Operating Contract & Agent Configuration

## Agent Definition
---
description: QuantRadar project agent with RTK-first conventions
mode: subagent
model: anthropic/claude-sonnet-4-6
---

This agent inherits global instructions from `~/.config/opencode/AGENTS.md` (RTK-first, skills, token etiquette).

## Project Context

QuantRadar — Rust + Python quantitative research platform.

**Structure:**
```
QuantRadar/
├── crates/          # Rust workspace (core, data, strategy, execution)
├── python/          # Python research/analysis scripts
├── configs/         # Strategy configs, environment configs
├── plans/           # Work plans (.omo/plans/)
├── reports/         # Generated reports
└── target/          # Build artifacts
```

## Control Plane Integration

**Control-plane:** authority `MASTERLIST.md`, roadmap `PLAN.md`, operating contract `AGENT.md`, live state `TODO.md`, runtime `.agents/` (`registry.json` + scripts). Index: `CONTROL_PLANE.md`.

SEE ALSO: `docs/control-plane/MASTERLIST.md §3`

## Task Continuation & Retry

- Continue each agent task and each subagent task.
- If a task cannot proceed, close it as inactive and retry.
- Verify retry/close state via `.agents/registry.json` + `TODO.md`.

## Repository Map

A full codemap is available at `codemap.md` in the project root.

Before working on any task, read `codemap.md` to understand:
- Project architecture and entry points
- Directory responsibilities and design patterns
- Data flow and integration points between modules

For deep work on a specific folder, also read that folder's `codemap.md`.

---

## §2.6 / §24 Operating Contract

### Agent Runtime Commands
- `cargo run --bin quantaradar discover` — discover markets (1391 found)
- `cargo run --bin quantaradar scan` — scan for signals
- `cargo run --bin quantaradar fetch <PAIR>` — fetch OHLC for pair
- `cargo run --bin quantaradar screen <PAIR>` — screen a pair
- `cargo run --bin quantaradar backtest <FILE>` — backtest from data file
- `cargo run -p quantaradar -- discover`
- `cargo run -p quantaradar -- screen --pair BTC/USD --interval 1440`
- `cargo run -p quantaradar -- fetch BTC/USD --interval 1440`
- `cargo run -p quantaradar -- backtest data/raw/BTC_USD.json`

### Python Research Pipeline
```bash
python3 -m venv .venv && . .venv/bin/activate
pip install -e '.[dev,ml]'
```

### RTK Commands (Token Optimization)
Core RTK commands covered: ls, tree, read, find, wc, git, gh, glab, gt, test, err, jest, vitest, ctest, pytest, cargo, npm, npx, pnpm, bun, bunx, pip, uv, deno, go, gradlew, mvn, mvnd, sbt, dotnet, next, php, tsc, lint, prettier, format, ruff, mypy, phpstan, ecs, pint, phpcs, golangci-lint, aws, psql, json, deps, env, grep, rg, diff, log, summary, smart, wget, curl, docker, kubectl, oc, prisma, config, init, gain, cc-economics, discover, session, telemetry, learn, trust, untrust, verify, hook-audit, rewrite, run, proxy, pipe, hook.

**RTK Usage:**
- The rtk plugin auto-rewrites `bash` tool calls (`git status` → `rtk git status`).
- Prefer commands with RTK coverage; prefix `rtk` explicitly for raw/piped shells.
- Unfiltered: `rtk proxy <cmd>` (tracked) or `rtk run <cmd>` (raw).
- Meta: `rtk gain` (savings), `rtk discover` (missed opportunities).
- Never pipe rtk output back through rtk.

### Skills Available
- **ponytail** (lite|full|ultra) — laziest working solution; YAGNI → stdlib → native → one-line → minimum
- **ponytail-review** — over-engineering review on diffs
- **ponytail-audit** — whole-repo over-engineering audit
- **ponytail-debt** — harvest `ponytail:` comments into ledger
- **ponytail-gain** — benchmark impact scoreboard
- **ponytail-help** — quick reference
- **security-research / security-review** — vulnerability audits (3 hunters + 2 PoC engineers)
- **customize-opencode** — opencode config/agent/skill/plugin work only
- **Builtin command skills** (load on demand): programming, frontend, visual-qa, playwright, git-master, debugging, review-work, remove-ai-slops, refactor, lsp-setup, ast-grep, data-scientist, ulw-plan, start-work, handoff

### Token Etiquette
- Prefer rtk-filtered output over raw dumps; never cat whole files when grep/codegraph answers
- `codegraph_explore` is Read-equivalent for indexed code — call it before Read/grep
- Stop searching once you have enough context

### Project-Specific Conventions
- **Rust** — strict types, `thiserror`/`anyhow`, `sqlx` for DB, `tokio` async, `clap` CLI
- **Python** — `uv` + `ruff` + `basedpyright`, `pydantic` v2, `polars`/`duckdb` for data
- **Config** — TOML for configs, environment variables for secrets
- **Testing** — `cargo test` (Rust), `pytest` (Python), integration tests in `tests/`
- **Git** — conventional commits, `main` branch, PRs for changes

## QuantRadar-Specific Rules

### CLI Commands
```bash
# Build
cargo build --workspace
uv sync

# Test
cargo test --workspace
uv run pytest

# CLI (quantaradar)
cargo run --bin quantaradar discover          # discover markets (1391 found)
cargo run --bin quantaradar scan              # scan for signals
cargo run --bin quantaradar fetch <PAIR>      # fetch OHLC for pair
cargo run --bin quantaradar screen <PAIR>     # screen a pair
cargo run --bin quantaradar backtest <FILE>   # backtest from data file

# Alternative CLI (package run)
cargo run -p quantaradar -- discover
cargo run -p quantaradar -- screen --pair BTC/USD --interval 1440
cargo run -p quantaradar -- fetch BTC/USD --interval 1440
cargo run -p quantaradar -- backtest data/raw/BTC_USD.json

# Python research
python3 -m venv .venv && . .venv/bin/activate
pip install -e '.[dev,ml]'

# Lint/Format
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
uv run ruff check .
uv run basedpyright
```

### Agent Runtime Updates
- **.agents/loop-agent.sh** — contracts added referencing AGENT.md; non-blocking enforcement; termination guard: MAX required (default 10); breaks early if CMD writes .agents/DONE; missing AGENT.md -> warn, continue minimal (safe degrade §24)
- **.agents/loop-cmd.sh** — gateway (/loop): routes invocations to the right runtime agent, no polling; reads AGENT.md presence (warn if missing); forwards exit codes
- **.agents/recursive-agent.sh** — depth guard max 5 (§10); STOP file tolerance; registry append on each descent; new registry schema tolerated
- **.agents/self-improving-agent.sh** — failure recovery (§26): plateau (no gain over prev two reads) -> reset depth to 0, alert, exit 0 instead of spinning; score >= 0.8 stop condition; depth > 8 stop condition
- **.agents/pi-agent.sh** — bridge: connects pi framework to QuantRadar runtime agents and .opencode/agent/quantaradar.md; pass-through only: never wipes registry.json, never duplicates runtime logic, forwards exit codes

### .agents/Registry.json v1.1 Structure
```json
{
  "agents": [
    {"agent": "program-director", "depth": 0, "role": "program-director", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "plans/"},
    {"agent": "architect", "depth": 0, "role": "architect", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "crates/"},
    {"agent": "planner", "depth": 0, "role": "planner", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "plans/"},
    {"agent": "lead-implementer", "depth": 0, "role": "lead-implementer", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "crates/"},
    {"agent": "reviewer", "depth": 0, "role": "reviewer", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "crates/"},
    {"agent": "tester", "depth": 0, "role": "tester", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "tests/"},
    {"agent": "integrator", "depth": 0, "role": "integrator", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "."},
    {"agent": "researcher", "depth": 0, "role": "researcher", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": "python/quantaradar/"},
    {"agent": "loop-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "recursive-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "self-improving-agent", "depth": 0, "role": "supervisor", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "pi-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".opencode/agent/"}
  ]
}
```

### Cross-Link References
- `.opencode/agent/quantaradar.md` — cross-references AGENT.md / MASTERLIST.md / TODO.md / .agents/registry.json
- `MASTERLIST.md` — stable spec: authority hierarchy (§3), mission (§1), architecture (§4)
- `TODO.md` — live projection with milestones mapped to P0–P7

## §24 Safe Degrade
If AGENT.md is missing, agent runtimes continue in minimal mode with warnings:
- loop-agent.sh: "warn: AGENT.md missing, minimal mode" >&2
- loop-cmd.sh: "warn: AGENT.md missing, minimal mode" >&2
- recursive-agent.sh: tolerates new registry schema
- self-improving-agent.sh: score defaults to 0.0 on unreadable registry
- pi-agent.sh: "warn: AGENT.md missing, minimal mode" >&2
