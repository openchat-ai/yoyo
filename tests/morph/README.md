# tests/morph — posture / Morph-Lock fixtures

> **Status**: **NON-CONFORMING** placeholders (v3.3.1 Prompt-opt Round).
> Presence of this directory does **NOT** green **M-posture** / Appendix F.5.
> Real cases: see PROMPT-v3.md Part E.19.8 + Appendix F.5 (P00–P06).

## Layout

```
tests/morph/
  README.md                 # this file
  cases/                    # Appendix F.5 case stubs
  posture.energy-extreme.lock.stub
  posture.perf-extreme.lock.stub
```

## Commands (honest red)

```text
# Expected: exit ≠ 0 until real morph/posture tooling lands
node scripts/check-foundations.mjs
node scripts/check-plans.mjs
# CLI (Rust): yoyo link --posture=energy-extreme ... → fail-closed NON-CONFORMING
```
