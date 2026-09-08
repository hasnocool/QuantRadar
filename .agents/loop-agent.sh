#!/bin/bash
# Agent: loop-runner — a diligent, tireless cycle agent that repeats tasks without fatigue.
# ponytail: minimal looping agent; no termination = infinite loop — add break when needed
# Usage: ./loop-agent.sh <command> [max_iterations]
CMD="${1:-echo loop}"
MAX="${2:-10}"
ITER=0
while [ $ITER -lt $MAX ]; do
  echo "loop[$ITER]: $CMD"
  eval "$CMD" || echo "loop[$ITER] exit $?"
  ITER=$((ITER+1))
done
echo "loop complete: $ITER iterations"
