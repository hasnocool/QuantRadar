#!/bin/bash
# Contract: AGENT.md §§2/7; MASTERLIST §4/P4.
# Bridge: connects the pi framework to the QuantRadar runtime agents and the
# project contract (.opencode/agent/quantaradar.md). Pass-through only: never
# wipes registry.json, never duplicates runtime logic, forwards exit codes.
# Usage: ./pi-agent.sh [name] [loop|recursive|self-improving]
NAME="${1:-pi-agent}"
MODE="${2:-loop}"
[ -f .opencode/agent/quantaradar.md ] || echo "warn: quantradar.md missing, minimal mode" >&2
[ -f AGENT.md ] || echo "warn: AGENT.md missing, minimal mode" >&2
case "$MODE" in
  loop) bash .agents/loop-agent.sh "echo pi-$NAME-tick" 3 ;;
  recursive) bash .agents/recursive-agent.sh 2 "pi-$NAME" ;;
  self-improving) bash .agents/self-improving-agent.sh 0 "pi-$NAME" ;;
  *) echo "pi-agent $NAME mode unknown: $MODE (use loop/recursive/self-improving)" >&2; exit 1 ;;
esac