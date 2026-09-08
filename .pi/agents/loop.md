name: loop-agent
command: /loop
description: Starts recursive self-improving loop — agent: loop-runner, a tireless recursive agent that improves scores and terminates cleanly.
script: .agents/loop-cmd.sh
mode: recursive
registry: .agents/registry.json
termination: score>=0.9 or depth>8 or STOP file