#!/bin/bash
# Agent: pi-agent-wrapper — a multi-mode bridge agent that connects pi framework to loop, recursive, and self-improving agents.
# ponytail: pi agent wrapper — minimal integration with pi agent ecosystem
NAME="${1:-pi-agent}"
MODE="${2:-loop}"
case "$MODE" in
  loop) bash .agents/loop-agent.sh "echo pi-$NAME-tick" 3 ;;
  recursive) bash .agents/recursive-agent.sh 2 "pi-$NAME" ;;
  self-improving) rm -f .agents/registry.json; echo '{"agents":[]}' > .agents/registry.json; bash .agents/self-improving-agent.sh 0 "pi-$NAME" ;;
  *) echo "pi-agent $NAME mode unknown: $MODE (use loop/recursive/self-improving)"; exit 1 ;;
esac
