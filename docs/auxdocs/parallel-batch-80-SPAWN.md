# parallel-batch-80 SPAWN · scratch-only (post body-extend-085)

> Continuous queue handoff from body-extend-085.
> Pin after Relock: `58b9ca6ef16f3ee48e22fae95f20dd6f6fa3492705659dfe181ec7857e9cf231` (abbrev `58b9ca6e…`).
> Handlers = 619 (H_00..H_612). Last selectors: 0x263..0x26A = H_605..H_612 (`40 263`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-79-log.md` / batch-79 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-085 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x26A are `40 26B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-085 / batch-79: **ADD-IMM slot=52 imm=1C8** (finish 1C8 ADD triad); **SUB-IMM slot=50/51/52 imm=1C8** (start 1C8 SUB triad); **next imm ladder 1D0…** (LDB/ADD-IMM/SUB-IMM).

## Task: parallel-batch-80 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-80-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_605..H_612).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_612.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-086 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-086 consolidation Task (same chain protocol), passing pin `58b9ca6e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-086-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_604 as prior + H_605 SUB-IMM 50 1C0 · H_606 SUB-IMM 51 1C0 · H_607 SUB-IMM 52 1C0 · H_608 LDB 50 60 1C8 · H_609 LDB 51 60 1C8 · H_610 LDB 52 60 1C8 · H_611 ADD-IMM 50 1C8 · H_612 ADD-IMM 51 1C8

(Full H_48..H_604 list: see `docs/auxdocs/parallel-batch-79-SPAWN.md` §Already locked plus H_597..H_604; treat that list plus H_605..H_612 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** ADD-IMM slot=52 imm=1C8 (finish 1C8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1C8 (start 1C8 SUB triad; imm32 22B)
- **Next ladder:** LDB dd=50/51/52 ss=60 oo=1D0 (start 1D0 LDB triad; imm32 26B)
- **Next ladder:** ADD-IMM/SUB-IMM slot=50/51/52 imm=1D0 (start 1D0 ADD/SUB triads; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
