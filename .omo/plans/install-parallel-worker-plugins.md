# Work Plan: install-parallel-worker-plugins

- **Slug**: install-parallel-worker-plugins
- **Status**: approved
- **Intent**: clear
- **Review Required**: false

## Goal
Clean up duplicate plugin entries in `/home/hyperion/.config/opencode/opencode.jsonc` (removing redundant RTK, ponytail, and oh-my-openagent variants) and add `@hueyexe/opencode-ensemble@0.16.0` and `opencode-async-agent`.

## Scope
- Modify `/home/hyperion/.config/opencode/opencode.jsonc`

## Must-NOT-Have
- Do not remove non-duplicate required plugins.
- Do not modify project-level `opencode.json` or other local files.

## Todos

- [ ] 1. /home/hyperion/.config/opencode/opencode.jsonc: Deduplicate plugin array (remove duplicate rtk, ponytail, and oh-my-opencode entries) and add @hueyexe/opencode-ensemble@0.16.0 and opencode-async-agent - expect clean, non-redundant plugin list in valid JSONC format

## Final verification wave

- [ ] F1. Check /home/hyperion/.config/opencode/opencode.jsonc content - expect deduplicated plugin list with @hueyexe/opencode-ensemble@0.16.0 and opencode-async-agent present
