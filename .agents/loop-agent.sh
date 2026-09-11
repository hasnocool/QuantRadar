#!/bin/bash
# Contract: AGENT.md §§10-12; MASTERLIST §§2.1/P3.
# Non-blocking async scheduler: runs CMD up to MAX times, never holds locks,
# never sleep-polls, one iteration's failure never aborts the loop.
# Termination guard: MAX required (default 10); breaks early if CMD writes
# .agents/DONE. Missing AGENT.md -> warn, continue minimal (safe degrade §24).
# Usage: ./loop-agent.sh <command> [max_iterations]
[ -f AGENT.md ] || echo "warn: AGENT.md missing, minimal mode" >&2
CMD="${1:-echo loop}"
MAX="${2:-10}"
case "$MAX" in ''|*[!0-9]*) echo "loop-agent: MAX must be a positive int" >&2; exit 1;; esac
ITER=0
while [ $ITER -lt $MAX ]; do
  [ -f .agents/DONE ] && { echo "loop break: DONE at iter $ITER"; rm -f .agents/DONE; break; }
  echo "loop[$ITER]: $CMD"
  eval "$CMD" || echo "loop[$ITER] exit $?"
  ITER=$((ITER+1))
done
echo "loop complete: $ITER iterations"