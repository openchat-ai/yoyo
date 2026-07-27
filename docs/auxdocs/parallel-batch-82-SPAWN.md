# parallel-batch-82 SPAWN · scratch-only (post body-extend-087)

> Continuous queue handoff from body-extend-087.
> Pin after Relock: `db550629db78a974cd83bec8db879fec415cd6fe37c94b35f57ce10a6917010d` (abbrev `db550629…`).
> Handlers = 635 (H_00..H_628). Last selectors: 0x273..0x27A = H_621..H_628 (`40 273`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-81-log.md` / batch-81 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-087 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x27A are `40 27B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-087 / batch-81: **ADD-IMM slot=50/51/52 imm=1D8** (start 1D8 ADD triad); **SUB-IMM slot=50/51/52 imm=1D8** (start 1D8 SUB triad); **next imm ladder 1E0…** (LDB/ADD-IMM/SUB-IMM).

## Task: parallel-batch-82 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-82-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_621..H_628).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_628.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-088 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-088 consolidation Task (same chain protocol), passing pin `db550629…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-088-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_620 as prior + H_621 ADD-IMM 51 1D0 · H_622 ADD-IMM 52 1D0 · H_623 SUB-IMM 50 1D0 · H_624 SUB-IMM 51 1D0 · H_625 SUB-IMM 52 1D0 · H_626 LDB 50 60 1D8 · H_627 LDB 51 60 1D8 · H_628 LDB 52 60 1D8

(Full H_48..H_620 list: see `docs/auxdocs/parallel-batch-81-SPAWN.md` §Already locked plus H_613..H_620; treat that list plus H_621..H_628 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** ADD-IMM slot=50/51/52 imm=1D8 (start 1D8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1D8 (start 1D8 SUB triad; imm32 22B)
- **Next ladder:** LDB dd=50/51/52 ss=60 oo=1E0 (start 1E0 LDB triad; imm32 26B)
- **Next ladder:** ADD-IMM/SUB-IMM slot=50/51/52 imm=1E0 (start 1E0 ADD/SUB triads; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
