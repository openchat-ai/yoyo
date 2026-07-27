# parallel-batch-81 SPAWN · scratch-only (post body-extend-086)

> Continuous queue handoff from body-extend-086.
> Pin after Relock: `9546a03ee5ac5d5254a4d887560694622666ef2cfc3a6035a937c978dfd5ee67` (abbrev `9546a03e…`).
> Handlers = 627 (H_00..H_620). Last selectors: 0x26B..0x272 = H_613..H_620 (`40 26B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-80-log.md` / batch-80 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-086 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x272 are `40 273`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-086 / batch-80: **ADD-IMM slot=51/52 imm=1D0** (continue/finish 1D0 ADD triad); **SUB-IMM slot=50/51/52 imm=1D0** (start 1D0 SUB triad); **next imm ladder 1D8…** (LDB/ADD-IMM/SUB-IMM).

## Task: parallel-batch-81 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-81-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_613..H_620).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_620.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-087 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-087 consolidation Task (same chain protocol), passing pin `9546a03e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-087-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_612 as prior + H_613 ADD-IMM 52 1C8 · H_614 SUB-IMM 50 1C8 · H_615 SUB-IMM 51 1C8 · H_616 SUB-IMM 52 1C8 · H_617 LDB 50 60 1D0 · H_618 LDB 51 60 1D0 · H_619 LDB 52 60 1D0 · H_620 ADD-IMM 50 1D0

(Full H_48..H_612 list: see `docs/auxdocs/parallel-batch-80-SPAWN.md` §Already locked plus H_605..H_612; treat that list plus H_613..H_620 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Continue deferred:** ADD-IMM slot=51/52 imm=1D0 (continue/finish 1D0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1D0 (start 1D0 SUB triad; imm32 22B)
- **Next ladder:** LDB dd=50/51/52 ss=60 oo=1D8 (start 1D8 LDB triad; imm32 26B)
- **Next ladder:** ADD-IMM/SUB-IMM slot=50/51/52 imm=1D8 (start 1D8 ADD/SUB triads; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
