# parallel-batch-96 SPAWN · scratch-only (post body-extend-101)

> Continuous queue handoff from body-extend-101.
> Pin after Relock: `514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719` (abbrev `514ff62c…`).
> Handlers = 747 (H_00..H_740). Last selectors: 0x2E3..0x2EA = H_733..H_740.
> W-START: **EXPERIMENTAL · NON-GREEN**. Do not invent-green.
>
> **STRATEGY SWITCH**: Picks from matrix priority (P0→P1), NOT random imm ladders.
> Source: `docs/auxdocs/selfhost-emit-matrix.md` YES+MISSING rows.

## Task: parallel-batch-96 (scratch-only) — **DONE**

8 picks: 2 MEMCPY stub (P0) · 2 GET multi-slot (P0) · 2 SET multi-imm (P0) · 1 ORV multi-combo (P1) · 1 CMP multi-slot (P1).

**Result: 8 PASS / 0 REJECT.** Log: `docs/auxdocs/parallel-batch-96-log.md`.

## Already locked (do not duplicate)

H_48..H_740 as prior.

## Next selectors

`40 2EB`..`40 2F2` for H_741..H_748 (body-extend-102 consolidation).

## Parent next

**body-extend-102 serialize PASSes + 1 Relock** (consolidator implements MEMCPY). Spawn: `docs/auxdocs/body-extend-102-SPAWN.md`.
