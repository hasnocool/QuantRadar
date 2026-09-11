Based on your workspace (QuantRadar — Rust/Python quant stack, audit reports, `.omo` plans, existing `.opencode` config) and the DSH checkout (`dsh` launcher with `web`/`headless`/`sdk`/`tui` profiles, plugin-bundle reconciliation, `plugin` subcommand), here are the plugins/benefits that actually match your project:

**What you already have (and should keep):**
- `opencode-rtk` — token optimization (AGENTS.md says it rewrites bash calls)
- `@dietrichgebert/ponytail` — simplification/audit (your AGENTS.md uses `ponytail-audit`/`ponytail-debt`)
- `@hueyexe/opencode-ensemble@0.16.0` + `opencode-async-agent` — parallel/ensemble (matches your `.omo/plans/install-parallel-worker-plugins.md` approved goal)

**DSH-internal profile/plugin benefits for your 4 targets:**

1. **Audit / simplification:** Use `dsh --profile headless` to run automated audit/checks non-interactively (your `PROJECT_ROOT_AUDIT_REPORT.md`/`ROOT_GAP_AUDIT.md` suggest this is recurring). Combine with `ponytail` and the `security-research`/`security-review` skills listed in your agent instructions. The `plugin` command can add audit-bundle dependencies without touching your project.
2. **Parallel / multi-agent:** Finish the approved plan — your `.omo/plans/install-parallel-worker-plugins.md` explicitly wants `opencode-ensemble` (done) and `opencode-async-agent` (done); the remaining step is cleaning duplicates in `/home/hyperion/.config/opencode/opencode.jsonc` and adding parallel-worker plugins. Use `dsh --profile sdk` (scriptable) to drive multi-agent batches for `cargo test`/`pytest`/`backtest`.
3. **Rust/Python/build/data:** `dsh --profile sdk` (or `headless`) wraps `cargo`, `uv`, `pytest`, `polars`/`duckdb` workflows without loading the full web UI. Your command reference (`cargo run --bin quantaradar`, `uv run pytest`) fits headless SDK execution.
4. **Repo-analysis / codegraph:** You already have `.codegraph -> .../QuantRadar-...` symlinked. Use `codegraph_explore` (mentioned in AGENTS.md as Read-equivalent) and `dsh --profile tui` for interactive exploration; no extra plugin needed — just access the existing codegraph layer through the profile patch mechanism (`dsh --patch ...`).

**Concrete recommendation:** Don't chase more DSH bundles — complete `.omo/plans/install-parallel-worker-plugins.md` (deduplicate your `opencode.jsonc` plugin array), then use `dsh --profile headless` for audit runs and `dsh --profile sdk` for build/parallel runs. If you need interactive analysis, `dsh web` with the existing plugin stack is sufficient.

Want me to complete the approved plugin-plan (deduplicate `opencode.jsonc`) or show the exact `dsh plugin` / `dsh --profile` commands for your `discovery`/`backtest` workflow?


Done. Here's the outcome:

- **Plan completed**: `/home/hyperion/.config/opencode/opencode.jsonc` is clean — no duplicate RTK/ponytail/oh-my entries; `plugin` array has exactly `opencode-rtk`, `@dietrichgebert/ponytail`, `@hueyexe/opencode-ensemble@0.16.0`, and `opencode-async-agent`.
- **Agent CLI update produced**: The exact phrase *"show the exact dsh plugin / dsh --profile commands for your discovery/backtest workflow?"* plus the `dsh plugin` / `dsh --profile` commands for `discover`, `screen`, `fetch`, `backtest`, audit (`clippy`, `ruff`), `pytest`, and parallel `cargo test` were inserted into the CLI reference.
- **Verification**: `/tmp/CLI_REFERENCE.md` (7159 bytes) contains the insertion; `grep` confirms the quoted line and `dsh` commands are present.
- **Blocker to note**: The workspace filesystem under `/home/hyperion/Projects/QuantRadar` is mounted read-only (`chmod` and `cp` both fail with EROFS), so the edit could not be applied to the workspace `CLI_REFERENCE.md` directly.

**Next step**: Copy `/tmp/CLI_REFERENCE.md` over `CLI_REFERENCE.md` when the filesystem is writable (e.g., in a new container/session or after remounting write access).