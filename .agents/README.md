# `.agents/` — QuantRadar runtime

Contract: `docs/control-plane/AGENT.md`. Spec: `docs/control-plane/MASTERLIST.md`.
Roadmap: `docs/control-plane/PLAN.md`. Live state: `docs/control-plane/TODO.md`.

## Scripts

| Script | Role (§) | Invoke |
|--------|----------|--------|
| `loop-cmd.sh` | Gateway (§15/§16) | `bash .agents/loop-cmd.sh [loop\|recursive\|self-improving]` (also: `./loop`) |
| `loop-agent.sh` | Async scheduler (P3) | `bash .agents/loop-agent.sh "<cmd>" [MAX]` |
| `recursive-agent.sh` | Delegation, depth ≤ 5 (§9/§10) | `bash .agents/recursive-agent.sh [depth] [name]` |
| `self-improving-agent.sh` | Supervision/recovery (§26/§28) | `bash .agents/self-improving-agent.sh [depth] [name]` |
| `pi-agent.sh` | pi-framework bridge (§4) | `bash .agents/pi-agent.sh [name] [mode]` |

## Rules

- Non-blocking, no `sleep` polling. Every loop/recursion has a bound
  (MAX / depth ≤ 5 / score ≥ 0.9 / STOP file / `.agents/DONE` break).
- Registry (`.agents/registry.json`) entries:
  `{agent, role, workspace, score, depth, weight, ts}`. Readers tolerate
  missing keys (role→`worker`, weight→1); corrupt file → warn + skip update.
- Write `.agents/STOP` to stop the next pass; write `.agents/DONE` to break
  `loop-agent.sh` early. Never wipe `registry.json` (history is metrics).
- Missing `AGENT.md` → warn + minimal behavior, never crash.
- Continue agent/subagent tasks; close inactive tasks and retry (AGENTS.md §Task Continuation & Retry).
<!-- ponytail: one page. Behavior lives in the scripts; don't duplicate it here. -->
