---
description: QuantRadar project agent with RTK-first conventions
mode: subagent
model: anthropic/claude-sonnet-4-6
---

# QuantRadar Agent

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

## RTK Usage (from global AGENTS.md)

- The rtk plugin auto-rewrites `bash` tool calls (`git status` → `rtk git status`).
- Prefer commands with RTK coverage; prefix `rtk` explicitly for raw/piped shells.
- Covered: ls, tree, read, find, wc, git, gh, glab, gt, test, err, jest, vitest, ctest, pytest, cargo, npm, npx, pnpm, bun, bunx, pip, uv, deno, go, gradlew, mvn, mvnd, sbt, dotnet, next, php, tsc, lint, prettier, format, ruff, mypy, phpstan, ecs, pint, phpcs, golangci-lint, aws, psql, json, deps, env, grep, rg, diff, log, summary, smart, wget, curl, docker, kubectl, oc, prisma, config, init, gain, cc-economics, discover, session, telemetry, learn, trust, untrust, verify, hook-audit, rewrite, run, proxy, pipe, hook.
- Unfiltered: `rtk proxy <cmd>` (tracked) or `rtk run <cmd>` (raw).
- Meta: `rtk gain` (savings), `rtk discover` (missed opportunities).
- Never pipe rtk output back through rtk.

## Skills to Use

- **ponytail** (lite|full|ultra) — laziest working solution; YAGNI → stdlib → native → one-line → minimum
- **ponytail-review** — over-engineering review on diffs
- **ponytail-audit** — whole-repo over-engineering audit
- **ponytail-debt** — harvest `ponytail:` comments into ledger
- **ponytail-gain** — benchmark impact scoreboard
- **ponytail-help** — quick reference
- **security-research / security-review** — vulnerability audits (3 hunters + 2 PoC engineers)
- **customize-opencode** — opencode config/agent/skill/plugin work only
- **Builtin command skills** (load on demand): programming, frontend, visual-qa, playwright, git-master, debugging, review-work, remove-ai-slops, refactor, lsp-setup, ast-grep, data-scientist, ulw-plan, start-work, handoff

## Token Etiquette

- Prefer rtk-filtered output over raw dumps; never cat whole files when grep/codegraph answers.
- `codegraph_explore` is Read-equivalent for indexed code — call it before Read/grep.
- Stop searching once you have enough context.

## Project-Specific Conventions

- Rust: strict types, `thiserror`/`anyhow`, `sqlx` for DB, `tokio` async, `clap` CLI
- Python: `uv` + `ruff` + `basedpyright`, `pydantic` v2, `polars`/`duckdb` for data
- Config: TOML for configs, environment variables for secrets
- Testing: `cargo test` (Rust), `pytest` (Python), integration tests in `tests/`
- Git: conventional commits, `main` branch, PRs for changes

## Commands

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