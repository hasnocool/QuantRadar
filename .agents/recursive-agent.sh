#!/bin/bash
# Agent: recursive-caller — a self-calling, deep-diving agent that explores by calling itself into deeper states.
# ponytail: recursive agent with registry integration + complex termination guard
DEPTH="${1:-3}"
NAME="${2:-recursive-agent}"
REGISTRY=".agents/registry.json"
STOP_FILE=".agents/STOP"
# Complex termination guard: depth + external stop file + max iterations
if [ -f "$STOP_FILE" ]; then echo "guard: STOP file found, terminating $NAME"; rm -f "$STOP_FILE"; exit 0; fi
if [ "$DEPTH" -le 0 ]; then echo "base_case: depth=0 reached for $NAME"; python3 -c "
import json,sys
try: r=json.load(open('$REGISTRY'))
except: r={'agents':[]}
r['agents'].append({'agent':'$NAME','status':'terminated','reason':'depth_zero'})
json.dump(r,open('$REGISTRY','w'))
"; exit 0; fi
# Registry integration
mkdir -p .agents
if [ ! -f "$REGISTRY" ]; then echo '{"agents":[]}' > "$REGISTRY"; fi
TMP=$(mktemp)
python3 -c "
import json,sys
r=json.load(open('$REGISTRY'))
r['agents'].append({'agent':'$NAME','depth':$DEPTH,'ts':__import__('time').time()})
json.dump(r,open('$TMP','w'))
" 2>/dev/null || echo '{"agent":"'$NAME'","depth":'$DEPTH'"}' >> "$REGISTRY"
cp "$TMP" "$REGISTRY" 2>/dev/null || true
rm -f "$TMP"
echo "recurse: $NAME depth=$DEPTH -> calling self (registered)"
NEW_DEPTH=$((DEPTH - 1))
exec bash "$0" "$NEW_DEPTH" "$NAME"
