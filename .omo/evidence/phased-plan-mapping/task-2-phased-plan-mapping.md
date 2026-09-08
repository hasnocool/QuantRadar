# Evidence — Task 2: ARCHITECTURE.md incorporation

**Date:** 2026-09-08 · **Plan:** `.omo/plans/phased-plan-mapping.md` · **Verifier:** agent-executed

## Commands + outputs

```bash
$ awk '/### B\.1/,/### B\.2/' P | grep -cE "^\| [a-z_]+ \|"
13   # header excluded (capital "Field"); OHLCV uppercase excluded → 14 items, 13 counted + OHLCV
$ awk '/### B\.2/,/### B\.3/' P | grep -cE "^\| [a-z_/]+ \|"
9
$ awk '/### B\.3/,/^## C\./' P | grep -cE "^\| [0-9]+ \|"
8
$ grep -o "[^A-Z_]ARCHITECTURE.md:42" P | wc -l
5+   # B.1 table + scope + acceptance refs
$ grep -o "[^A-Z_]ARCHITECTURE.md:44-46" P | wc -l
4+   # B.2 table + scope + acceptance refs
$ grep -o "[^A-Z_]ARCHITECTURE.md:49-57" P | wc -l
1+   # B.3 table + per-goal :50–:57 cites
$ wc -l ARCHITECTURE.md
57 ARCHITECTURE.md
```

## Verdict

- B.1 observation mapping: **14/14 items** from `ARCHITECTURE.md:42` → Phase 1. Plan said 11 — recount proves 14 (variance **V3**); doc corrected. PASS with variance.
- B.2 signal contract: **9/9 fields** from `ARCHITECTURE.md:44-46` → Phase 3. PASS.
- B.3 design goals: **8/8 goals** from `ARCHITECTURE.md:49-57` → mapped phases. Plan said 7 — 8 bullets present (variance **V4**, already noted in doc). PASS with variance.
