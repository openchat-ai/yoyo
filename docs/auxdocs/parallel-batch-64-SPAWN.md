# parallel-batch-64 SPAWN · scratch-only (post body-extend-069)

> Continuous queue handoff from body-extend-069.
> Pin after Relock: `f9afff3e953337091fdaa161a919f6d92488d72c1f70687907395922a811ec42` (abbrev `f9afff3e…`).
> Handlers = 491 (H_00..H_484). Last selectors: 0x1E3..0x1EA = H_477..H_484 (`40 1E3`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-63-log.md` / batch-63 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-069 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1EA are `40 1EB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-069: **none** (all 8 batch-63 PASSes consolidated).

## Task: parallel-batch-64 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-64-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_477..H_484).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_484.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-070 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-070 consolidation Task (same chain protocol), passing pin `f9afff3e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-070-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_476 as prior + H_477 ADD-IMM 51 150 · H_478 ADD-IMM 52 150 · H_479 SUB-IMM 50 150 · H_480 SUB-IMM 51 150 · H_481 SUB-IMM 52 150 · H_482 LDB 50 60 158 · H_483 LDB 51 60 158 · H_484 LDB 52 60 158

(Full H_48..H_476 list: see `docs/auxdocs/parallel-batch-63-SPAWN.md` §Already locked plus H_469..H_476; treat that list plus H_477..H_484 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=50/51/52 imm=158 — start 158 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=158 — start 158 SUB triad (imm32 22B)
- LDB dd=50/51/52 ss=60 oo=160 — start 160 LDB triad (imm32 26B)
- ADD-IMM / SUB-IMM imm=160 triad starts
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
