# parallel-batch-84 SPAWN · scratch-only (post body-extend-089)

> Continuous queue handoff from body-extend-089.
> Pin after Relock: `e8500277650750c55bc94ec1a9c5e0277367daa257b09371e33f569a8d46c129` (abbrev `e8500277…`).
> Handlers = 651 (H_00..H_644). Last selectors: 0x283..0x28A = H_637..H_644 (`40 283`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-83-log.md` / batch-83 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-089 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x28A are `40 28B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-089 / batch-83: **LDB dd=51/52 ss=60 oo=1E8** (finish 1E8 LDB triad); **ADD-IMM / SUB-IMM slot=50/51/52 imm=1E8** (start 1E8 ADD/SUB triads).

## Task: parallel-batch-84 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-84-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_637..H_644).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_644.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-090 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-090 consolidation Task (same chain protocol), passing pin `e8500277…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-090-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_636 as prior + H_637 LDB 52 60 1E0 · H_638 ADD-IMM 50 1E0 · H_639 ADD-IMM 51 1E0 · H_640 ADD-IMM 52 1E0 · H_641 SUB-IMM 50 1E0 · H_642 SUB-IMM 51 1E0 · H_643 SUB-IMM 52 1E0 · H_644 LDB 50 60 1E8

(Full H_48..H_636 list: see `docs/auxdocs/parallel-batch-83-SPAWN.md` §Already locked plus H_629..H_636; treat that list plus H_637..H_644 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=51/52 ss=60 oo=1E8 (finish 1E8 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1E8 (start 1E8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1E8 (start 1E8 SUB triad; imm32 22B)
- **Next ladder:** LDB/ADD-IMM/SUB-IMM imm=1F0… if triad space allows
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
