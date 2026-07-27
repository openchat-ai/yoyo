# parallel-batch-83 SPAWN · scratch-only (post body-extend-088)

> Continuous queue handoff from body-extend-088.
> Pin after Relock: `697ad7847ba15e825ee7a2663be37eb71de542256a38f42ed2e7dc16ddca549c` (abbrev `697ad784…`).
> Handlers = 643 (H_00..H_636). Last selectors: 0x27B..0x282 = H_629..H_636 (`40 27B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-82-log.md` / batch-82 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-088 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x282 are `40 283`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-088 / batch-82: **LDB dd=52 ss=60 oo=1E0** (finish 1E0 LDB triad); **ADD-IMM / SUB-IMM slot=50/51/52 imm=1E0** (start 1E0 ADD/SUB triads).

## Task: parallel-batch-83 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-83-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_629..H_636).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_636.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-089 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-089 consolidation Task (same chain protocol), passing pin `697ad784…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-089-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_628 as prior + H_629 ADD-IMM 50 1D8 · H_630 ADD-IMM 51 1D8 · H_631 ADD-IMM 52 1D8 · H_632 SUB-IMM 50 1D8 · H_633 SUB-IMM 51 1D8 · H_634 SUB-IMM 52 1D8 · H_635 LDB 50 60 1E0 · H_636 LDB 51 60 1E0

(Full H_48..H_628 list: see `docs/auxdocs/parallel-batch-82-SPAWN.md` §Already locked plus H_621..H_628; treat that list plus H_629..H_636 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=52 ss=60 oo=1E0 (finish 1E0 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1E0 (start 1E0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1E0 (start 1E0 SUB triad; imm32 22B)
- **Next ladder:** LDB/ADD-IMM/SUB-IMM imm=1E8… if triad space allows
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
