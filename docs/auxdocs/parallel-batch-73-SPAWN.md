# parallel-batch-73 SPAWN · scratch-only (post body-extend-078)

> Continuous queue handoff from body-extend-078.
> Pin after Relock: `4c42576df4f80a8d3f4e57074fb4fc081bc16d37c9638b9fd0659ddae86fd42b` (abbrev `4c42576d…`).
> Handlers = 563 (H_00..H_556). Last selectors: 0x22B..0x232 = H_549..H_556 (`40 22B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-72-log.md` / batch-72 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-078 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x232 are `40 233`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-078 / batch-72: **ADD-IMM/SUB-IMM slot=50/51/52 imm=198** (start 198 ADD/SUB triads).

## Task: parallel-batch-73 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-73-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_549..H_556).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_556.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-079 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-079 consolidation Task (same chain protocol), passing pin `4c42576d…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-079-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_548 as prior + H_549 ADD-IMM 51 190 · H_550 ADD-IMM 52 190 · H_551 SUB-IMM 50 190 · H_552 SUB-IMM 51 190 · H_553 SUB-IMM 52 190 · H_554 LDB 50 60 198 · H_555 LDB 51 60 198 · H_556 LDB 52 60 198

(Full H_48..H_548 list: see `docs/auxdocs/parallel-batch-72-SPAWN.md` §Already locked plus H_541..H_548; treat that list plus H_549..H_556 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** ADD-IMM slot=50/51/52 imm=198 (start 198 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=198 (start 198 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=1A0 — start 1A0 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=1A0 — start 1A0 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=1A0 — start 1A0 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
