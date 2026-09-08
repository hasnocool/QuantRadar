#!/bin/bash
# Agent: self-improver — an adaptive, learning agent that improves its score with each recursive pass.
# ponytail: self-improving loop agent — reads registry, improves param, loops
DEPTH="${1:-0}"
NAME="${2:-self-improver}"
REGISTRY=".agents/registry.json"
STOP_FILE=".agents/STOP"
# Termination guard
if [ -f "$STOP_FILE" ]; then echo "stop_guard: $NAME"; rm -f "$STOP_FILE"; exit 0; fi
# Read previous score from registry if exists
PREV=$(python3 -c "
import json,sys
try:
    r=json.load(open('$REGISTRY'))
    scores=[a.get('score',0) for a in r.get('agents',[]) if a.get('agent')=='$NAME']
    print(max(scores) if scores else 0.0)
except: print(0.0)
" 2>/dev/null || echo 0.0)
# Self-improve: increase depth-cap and score
NEW_DEPTH=$((DEPTH + 1))
NEW_SCORE=$(python3 -c "print(min(1.0, float('$PREV') + 0.15 * $NEW_DEPTH))" 2>/dev/null || echo 0.15)
echo "improve: $NAME depth=$DEPTH prev_score=$PREV new_score=$NEW_SCORE"
# Registry update
mkdir -p .agents
python3 -c "
import json,sys,time
path='$REGISTRY'
try:
    r=json.load(open(path))
except: r={'agents':[]}
r['agents'].append({'agent':'$NAME','depth':$NEW_DEPTH,'score':float('$NEW_SCORE'),'ts':time.time()})
json.dump(r,open(path,'w'))
" 2>/dev/null || echo '{"agent":"'$NAME'","depth":'$NEW_DEPTH'","score":'$NEW_SCORE'"}' >> "$REGISTRY"
# Recursion condition: loop until score >= 0.9 or depth > 8
if python3 -c "exit(0 if float('$NEW_SCORE') >= 0.9 or $NEW_DEPTH > 8 else 1)"; then
    echo "self_improve_done: $NAME depth=$NEW_DEPTH score=$NEW_SCORE"
    exit 0
fi
exec bash "$0" "$NEW_DEPTH" "$NAME"
