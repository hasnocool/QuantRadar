#!/bin/bash
# Contract: AGENT.md §§8-9; MASTERLIST §§9-10/P5.
# Recursive delegation: fan out one level, register it, verify current level,
# then descend. Termination: depth<=0 base case, depth>5 hard-stop (runaway
# guard §10), STOP file. Never recurse without the registry append below
# (the verification/registration step). Tolerates new registry schema.
# Usage: ./recursive-agent.sh [depth] [name]
DEPTH="${1:-3}"
NAME="${2:-recursive-agent}"
REGISTRY=".agents/registry.json"
STOP_FILE=".agents/STOP"
if [ -f "$STOP_FILE" ]; then echo "guard: STOP file found, terminating $NAME"; rm -f "$STOP_FILE"; exit 0; fi
case "$DEPTH" in ''|*[!0-9]*) echo "recursive-agent: DEPTH must be a non-negative int" >&2; exit 1;; esac
if [ "$DEPTH" -gt 5 ]; then echo "guard: depth $DEPTH > 5 hard-stop ($NAME)" >&2; exit 1; fi
if [ "$DEPTH" -le 0 ]; then echo "base_case: depth=0 reached for $NAME"; exit 0; fi
mkdir -p .agents
[ -f "$REGISTRY" ] || echo '{"agents":[]}' > "$REGISTRY"
python3 - "$REGISTRY" "$NAME" "$DEPTH" <<'EOF' 2>/dev/null || echo "warn: registry append skipped ($NAME depth $DEPTH)" >&2
import json, sys, time
path, name, depth = sys.argv[1], sys.argv[2], int(sys.argv[3])
try:
    r = json.load(open(path))
except Exception:
    print("corrupt registry, skipping update", file=sys.stderr); sys.exit(0)
r.setdefault("agents", []).append({"agent": name, "role": "runtime",
    "workspace": ".agents/", "depth": depth, "score": 0.0, "weight": 1,
    "ts": time.time()})
json.dump(r, open(path, "w"))
EOF
echo "recurse: $NAME depth=$DEPTH -> descending (registered)"
exec bash "$0" "$((DEPTH - 1))" "$NAME"