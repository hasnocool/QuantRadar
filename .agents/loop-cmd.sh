#!/bin/bash
# Contract: AGENT.md §11; MASTERLIST §§14-16/P7.
# Gateway (/loop): routes invocations to the right runtime agent, no polling.
# Reads AGENT.md presence (warn if missing); forwards exit codes.
# Usage: ./loop-cmd.sh [loop|recursive|self-improving] [args...]
# Default (no args): legacy self-improving pass, depth 0, loop-agent.
[ -f AGENT.md ] || echo "warn: AGENT.md missing, minimal mode" >&2
MODE="${1:-self-improving}"
case "$MODE" in
  loop) shift; bash .agents/loop-agent.sh "${@:-echo loop-tick}" ;;
  recursive) shift; bash .agents/recursive-agent.sh "${@:-2}" ;;
  self-improving) shift; bash .agents/self-improving-agent.sh ${1:-0} ${2:-loop-agent} ;;
  *) echo "loop-cmd: unknown mode '$MODE' (loop/recursive/self-improving)" >&2; exit 1 ;;
esac