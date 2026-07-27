# parallel-batch-68 SPAWN · scratch-only (post body-extend-073)

> Continuous queue handoff from body-extend-073.
> Pin after Relock: `1a6cb44aa28367d25d6727eec5206e5895c3c948be080a60dcadb7d853bc8bac` (abbrev `1a6cb44a…`).
> Handlers = 523 (H_00..H_516). Last selectors: 0x203..0x20A = H_509..H_516 (`40 203`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-67-log.md` / batch-67 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-073 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x20A are `40 20B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-073 / batch-67: **SUB-IMM slot=52 imm=170** (finish 170 SUB triad).

## Task: parallel-batch-68 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-68-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_509..H_516).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_516.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-074 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-074 consolidation Task (same chain protocol), passing pin `1a6cb44a…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-074-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_508 as prior + H_509 LDB 50 60 170 · H_510 LDB 51 60 170 · H_511 LDB 52 60 170 · H_512 ADD-IMM 50 170 · H_513 ADD-IMM 51 170 · H_514 ADD-IMM 52 170 · H_515 SUB-IMM 50 170 · H_516 SUB-IMM 51 170

(Full H_48..H_508 list: see `docs/auxdocs/parallel-batch-67-SPAWN.md` §Already locked plus H_501..H_508; treat that list plus H_509..H_516 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=52 imm=170 (complete 170 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=178 — start 178 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=178 — start 178 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=178 — start 178 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
