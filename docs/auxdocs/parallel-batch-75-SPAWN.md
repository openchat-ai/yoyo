# parallel-batch-75 SPAWN · scratch-only (post body-extend-080)

> Continuous queue handoff from body-extend-080.
> Pin after Relock: `e255cd93a26ec455cc4def0ceb38c1cfc93bcb1ec7476f9e57ecd062d1be065a` (abbrev `e255cd93…`).
> Handlers = 579 (H_00..H_572). Last selectors: 0x23B..0x242 = H_565..H_572 (`40 23B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-74-log.md` / batch-74 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-080 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x242 are `40 243`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-080 / batch-74: **LDB 51 60 1A8 / LDB 52 60 1A8** (finish 1A8 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1A8** (start 1A8 ADD/SUB triads).

## Task: parallel-batch-75 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-75-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_565..H_572).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_572.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-081 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-081 consolidation Task (same chain protocol), passing pin `e255cd93…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-081-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_564 as prior + H_565 LDB 52 60 1A0 · H_566 ADD-IMM 50 1A0 · H_567 ADD-IMM 51 1A0 · H_568 ADD-IMM 52 1A0 · H_569 SUB-IMM 50 1A0 · H_570 SUB-IMM 51 1A0 · H_571 SUB-IMM 52 1A0 · H_572 LDB 50 60 1A8

(Full H_48..H_564 list: see `docs/auxdocs/parallel-batch-74-SPAWN.md` §Already locked plus H_557..H_564; treat that list plus H_565..H_572 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=51 ss=60 oo=1A8 / LDB dd=52 ss=60 oo=1A8 (finish 1A8 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1A8 (start 1A8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1A8 (start 1A8 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=1B0 — start 1B0 LDB triad (imm32 26B)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1B0 — start 1B0 ADD/SUB triads
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
