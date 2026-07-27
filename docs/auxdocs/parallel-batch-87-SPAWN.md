# parallel-batch-87 SPAWN · scratch-only (post body-extend-092)

> Continuous queue handoff from body-extend-092.
> Pin after Relock: `1991af8484d67ec19980bf14771d523d332f85c9974e1da09d45496baf46ebb5` (abbrev `1991af84…`).
> Handlers = 675 (H_00..H_668). Last selectors: 0x29B..0x2A2 = H_661..H_668 (`40 29B`..`40 2A2` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-86-log.md` / batch-86 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-092 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2A2 are `40 2A3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-092 / batch-86: **SUB-IMM slot=51/52 imm=1F8** (finish 1F8 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-87 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-87-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_661..H_668).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_668.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-093 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-093 consolidation Task (same chain protocol), passing pin `1991af84…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-093-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_660 as prior + H_661 SUB-IMM 52 1F0 · H_662 LDB 50 60 1F8 · H_663 LDB 51 60 1F8 · H_664 LDB 52 60 1F8 · H_665 ADD-IMM 50 1F8 · H_666 ADD-IMM 51 1F8 · H_667 ADD-IMM 52 1F8 · H_668 SUB-IMM 50 1F8

(Full H_48..H_660 list: see `docs/auxdocs/parallel-batch-86-SPAWN.md` §Already locked plus H_653..H_660; treat that list plus H_661..H_668 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=51/52 imm=1F8 (finish 1F8 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB/ADD-IMM/SUB-IMM imm beyond 1F8) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
