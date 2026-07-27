# parallel-batch-95 SPAWN · scratch-only (post body-extend-100)

> Continuous queue handoff from body-extend-100.
> Pin after Relock: `7c07906496a7af9cbaec74b5590ec3677117ced6c36241823bd69b6a4ff1ae51` (abbrev `7c079064…`).
> Handlers = 739 (H_00..H_732). Last selectors: 0x2DB..0x2E2 = H_725..H_732 (`40 2DB`..`40 2E2` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-94-log.md` / batch-94 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-100 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green. (Prior body-extend-099 also EQUAL — both honest.)
> Next selectors after 0x2E2 are `40 2E3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-100 / batch-94: **SUB-IMM slot=52 imm=230** (finish 230 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder beyond 230 if continuing.

## Task: parallel-batch-95 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-95-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_725..H_732).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_732.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-101 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-101 consolidation Task (same chain protocol), passing pin `7c079064…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-101-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_724 as prior + H_725 LDB 50 60 230 · H_726 LDB 51 60 230 · H_727 LDB 52 60 230 · H_728 ADD-IMM 50 230 · H_729 ADD-IMM 51 230 · H_730 ADD-IMM 52 230 · H_731 SUB-IMM 50 230 · H_732 SUB-IMM 51 230

(Full H_48..H_724 list: see `docs/auxdocs/parallel-batch-94-SPAWN.md` §Already locked plus H_717..H_724; treat that list plus H_725..H_732 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=52 imm=230 (finish 230 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB/ADD-IMM/SUB-IMM beyond 230) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
