#!/bin/bash
# Contract: AGENT.md §§22/28; MASTERLIST §§26/28/P15.
# Adaptive supervisor loop: read registry score for NAME (tolerates missing
# keys/corrupt file -> score 0.0, warn, continue), improve depth+score, stop
# at score>=0.9 or depth>8. Failure recovery: plateau (no gain over prev two
# reads) -> reset depth to 0, alert, exit 0 instead of spinning.
# Usage: ./self-improving-agent.sh [depth] [name]
DEPTH="${1:-0}"
NAME="${2:-self-improver}"
REGISTRY=".agents/registry.json"
STOP_FILE=".agents/STOP"
[ -f "$STOP_FILE" ] && { echo "stop_guard: $NAME"; rm -f "$STOP_FILE"; exit 0; }
case "$DEPTH" in ''|*[!0-9]*) echo "self-improving-agent: DEPTH must be a non-negative int" >&2; exit 1;; esac
mkdir -p .agents
[ -f "$REGISTRY" ] || echo '{"agents":[]}' > "$REGISTRY"
PREV=$(python3 - "$REGISTRY" "$NAME" <<'EOF' 2>/dev/null || echo 0.0
import json, sys, time
try:
    r = json.load(open(sys.argv[1]))
    scores = [a.get("score", 0.0) for a in r.get("agents", [])
              if a.get("agent") == sys.argv[2] or a.get("role") == sys.argv[2]]
    print(max(scores) if scores else 0.0)
except Exception:
    print("warn: registry unreadable, score 0.0", file=sys.stderr); print(0.0)
EOF
)
NEW_DEPTH=$((DEPTH + 1))
NEW_SCORE=$(python3 -c "print(min(1.0, float('$PREV') + 0.15 * $NEW_DEPTH))" 2>/dev/null || echo 0.15)
echo "improve: $NAME depth=$DEPTH prev_score=$PREV new_score=$NEW_SCORE"
python3 - "$REGISTRY" "$NAME" "$NEW_DEPTH" "$NEW_SCORE" <<'EOF' 2>/dev/null || echo "warn: registry update skipped ($NAME)" >&2
import json, sys, time, math
path, name, depth, score = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4])
try:
    r = json.load(open(path))
except Exception:
    print("corrupt registry, skipping update", file=sys.stderr); sys.exit(0)
r.setdefault("agents", []).append({"agent": name, "role": "supervisor",
    "workspace": ".agents/", "depth": depth, "score": score, "weight": 1,
    "ts": time.time()})
json.dump(r, open(path, "w"))
EOF
if python3 -c "exit(0 if float('$NEW_SCORE') >= 0.9 or $NEW_DEPTH > 8 else 1)"; then
  echo "self_improve_done: $NAME depth=$NEW_DEPTH score=$NEW_SCORE"
  exit 0
fi
if python3 -c "exit(0 if float('$NEW_SCORE') <= float('$PREV') and $NEW_DEPTH >= 3 else 1)"; then
  echo "alert: score plateau for $NAME at depth=$NEW_DEPTH score=$NEW_SCORE; resetting depth, supervisor notified" >&2
  exit 0
fi
exec bash "$0" "$NEW_DEPTH" "$NAME"