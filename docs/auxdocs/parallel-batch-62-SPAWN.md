# parallel-batch-62 SPAWN · scratch-only (post body-extend-067)

> Continuous queue handoff from body-extend-067.
> Pin after Relock: `deaf40134394a58d9e81fd3a8f55c4ec9110fc93ad8d366e547f0628144dd098` (abbrev `deaf4013…`).
> Handlers = 475 (H_00..H_468). Last selectors: 0x1D3..0x1DA = H_461..H_468 (`40 1D3`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-61-log.md` / batch-61 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-067 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1DA are `40 1DB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-067: **none** (all 8 batch-61 PASSes consolidated).

## Task: parallel-batch-62 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-62-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_461..H_468).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_468.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-068 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-068 consolidation Task (same chain protocol), passing pin `deaf4013…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-068-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_460 as prior + H_461 SUB-IMM 50 140 · H_462 SUB-IMM 51 140 · H_463 SUB-IMM 52 140 · H_464 LDB 50 60 148 · H_465 LDB 51 60 148 · H_466 LDB 52 60 148 · H_467 ADD-IMM 50 148 · H_468 ADD-IMM 51 148

(Full H_48..H_460 list: see `docs/auxdocs/parallel-batch-61-SPAWN.md` §Already locked plus H_453..H_460; treat that list plus H_461..H_468 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=52 imm=148 — finish 148 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=148 — start 148 SUB triad (imm32 22B)
- LDB dd=50/51/52 ss=60 oo=150 — start 150 LDB triad (imm32 26B)
- ADD-IMM / SUB-IMM imm=150 triad starts
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
