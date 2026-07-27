# parallel-batch-54 SPAWN · scratch-only (post body-extend-059)

> Continuous queue handoff from body-extend-059.
> Pin after Relock: `bd7bad15e53fe296e790c57803a0d44930e95c7f7db99ee866685fbb5d504f12` (abbrev `bd7bad15…`).
> Handlers = 412 (H_00..H_405). Last selectors: 0x194..0x19B = H_398..H_405 (`40 194`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-53-log.md` / batch-53 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-059 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x19B are `40 19C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-059: **none** (all 8 batch-53 PASSes consolidated).

## Task: parallel-batch-54 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-54-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_398..H_405).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_405.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-060 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-060 consolidation Task (same chain protocol), passing pin `bd7bad15e53fe296…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-060-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_397 as prior + H_398 SUB-IMM 50 108 · H_399 SUB-IMM 51 108 · H_400 SUB-IMM 52 108 · H_401 LDB 50 60 110 · H_402 LDB 51 60 110 · H_403 LDB 52 60 110 · H_404 ADD-IMM 50 110 · H_405 ADD-IMM 51 110

(Full H_48..H_397 list: see `docs/auxdocs/parallel-batch-53-SPAWN.md` §Already locked; treat that list plus H_398..H_405 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=52 imm=110 — finish 110 ADD triad after H_404..H_405
- SUB-IMM slot=50/51/52 imm=110 — finish 110 SUB triad (imm32 22B)
- LDB oo=next after 110 (e.g. 118) triad (dd=50/51/52 ss=60)
- ADD-IMM / SUB-IMM fresh imm=118 triad (slots 50/51/52)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
