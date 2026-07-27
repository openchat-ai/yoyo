# Plan / Deduce tests

Status: **executable** via `scripts/check-plans.mjs` (Part Deduce / D-plan).

## Checks

- [x] Sample `plan.battery-15.plan.md` uses only READ_FACT|READ_PROBE|COMPARE|SELECT|EMIT
- [x] EMIT types ⊆ registered artifact set (Deduce.4)
- [x] ReplayRecord required when `claim_level=pinned` (real sha256; stubs fail)
- [x] Empty / bad plans fail-closed (exit ≠ 0)

```text
node scripts/check-plans.mjs
```

See `PROMPT-v3.md` 文首 Week 轴 / Part Deduce (D-plan).
