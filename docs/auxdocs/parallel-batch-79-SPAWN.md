# parallel-batch-79 SPAWN · scratch-only (post body-extend-084)

> Continuous queue handoff from body-extend-084.
> Pin after Relock: `9eafc9ce0376d389043b0e77ec2c1ff2bc44dda11b4fb8f6449cc4ea811798ac` (abbrev `9eafc9ce…`).
> Handlers = 611 (H_00..H_604). Last selectors: 0x25B..0x262 = H_597..H_604 (`40 25B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-78-log.md` / batch-78 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-084 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x262 are `40 263`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-084 / batch-78: **SUB-IMM slot=50/51/52 imm=1C0** (start 1C0 SUB triad); **LDB 50/51/52 60 1C8** (start 1C8 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1C8** (start 1C8 ADD/SUB triads).

## Task: parallel-batch-79 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-79-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_597..H_604).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_604.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-085 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-085 consolidation Task (same chain protocol), passing pin `9eafc9ce…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-085-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_596 as prior + H_597 SUB-IMM 51 1B8 · H_598 SUB-IMM 52 1B8 · H_599 LDB 50 60 1C0 · H_600 LDB 51 60 1C0 · H_601 LDB 52 60 1C0 · H_602 ADD-IMM 50 1C0 · H_603 ADD-IMM 51 1C0 · H_604 ADD-IMM 52 1C0

(Full H_48..H_596 list: see `docs/auxdocs/parallel-batch-78-SPAWN.md` §Already locked plus H_589..H_596; treat that list plus H_597..H_604 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** SUB-IMM slot=50/51/52 imm=1C0 (start 1C0 SUB triad; imm32 22B)
- **Start deferred:** LDB dd=50/51/52 ss=60 oo=1C8 (start 1C8 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1C8 (start 1C8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1C8 (start 1C8 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
