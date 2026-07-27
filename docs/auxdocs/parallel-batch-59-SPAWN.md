# parallel-batch-59 SPAWN · scratch-only (post body-extend-064)

> Continuous queue handoff from body-extend-064.
> Pin after Relock: `d9aff9ed76e4f649fcee1c50496dd813e23690f73b35ce4cfc4e700ef466f276` (abbrev `d9aff9ed…`).
> Handlers = 451 (H_00..H_444). Last selectors: 0x1BB..0x1C2 = H_437..H_444 (`40 1BB`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-58-log.md` / batch-58 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-064 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1C2 are `40 1C3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-064: **none** (all 8 batch-58 PASSes consolidated).

## Task: parallel-batch-59 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-59-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_437..H_444).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_444.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-065 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-065 consolidation Task (same chain protocol), passing pin `d9aff9ed…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-065-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_436 as prior + H_437 LDB 50 60 130 · H_438 LDB 51 60 130 · H_439 LDB 52 60 130 · H_440 ADD-IMM 50 130 · H_441 ADD-IMM 51 130 · H_442 ADD-IMM 52 130 · H_443 SUB-IMM 50 130 · H_444 SUB-IMM 51 130

(Full H_48..H_436 list: see `docs/auxdocs/parallel-batch-58-SPAWN.md` §Already locked plus H_430..H_436; treat that list plus H_437..H_444 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=52 imm=130 — finish 130 SUB triad (imm32 22B)
- LDB dd=50/51/52 ss=60 oo=138 — start 138 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=138 — start 138 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=138 — start 138 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
