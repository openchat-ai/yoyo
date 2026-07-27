# parallel-batch-74 SPAWN · scratch-only (post body-extend-079)

> Continuous queue handoff from body-extend-079.
> Pin after Relock: `0e5b612c7e4882a1de87b39c35cafe0e6ccdfdc174e4f378dcd28b799de58c73` (abbrev `0e5b612c…`).
> Handlers = 571 (H_00..H_564). Last selectors: 0x233..0x23A = H_557..H_564 (`40 233`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-73-log.md` / batch-73 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-079 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x23A are `40 23B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-079 / batch-73: **LDB 52 60 1A0** (finish 1A0 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1A0** (start 1A0 ADD/SUB triads).

## Task: parallel-batch-74 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-74-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_557..H_564).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_564.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-080 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-080 consolidation Task (same chain protocol), passing pin `0e5b612c…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-080-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_556 as prior + H_557 ADD-IMM 50 198 · H_558 ADD-IMM 51 198 · H_559 ADD-IMM 52 198 · H_560 SUB-IMM 50 198 · H_561 SUB-IMM 51 198 · H_562 SUB-IMM 52 198 · H_563 LDB 50 60 1A0 · H_564 LDB 51 60 1A0

(Full H_48..H_556 list: see `docs/auxdocs/parallel-batch-73-SPAWN.md` §Already locked plus H_549..H_556; treat that list plus H_557..H_564 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=52 ss=60 oo=1A0 (finish 1A0 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1A0 (start 1A0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1A0 (start 1A0 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=1A8 — start 1A8 LDB triad (imm32 26B)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1A8 — start 1A8 ADD/SUB triads
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
