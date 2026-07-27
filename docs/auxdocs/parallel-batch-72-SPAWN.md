# parallel-batch-72 SPAWN · scratch-only (post body-extend-077)

> Continuous queue handoff from body-extend-077.
> Pin after Relock: `97ce84a29adb8c400408d7fec9d2d58a820766a61c18068b1b61eac59946e2b0` (abbrev `97ce84a2…`).
> Handlers = 555 (H_00..H_548). Last selectors: 0x223..0x22A = H_541..H_548 (`40 223`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-71-log.md` / batch-71 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-077 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x22A are `40 22B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-077 / batch-71: **ADD-IMM slot=51/52 imm=190** (finish 190 ADD triad); **SUB-IMM slot=50/51/52 imm=190** (start 190 SUB triad).

## Task: parallel-batch-72 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-72-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_541..H_548).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_548.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-078 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-078 consolidation Task (same chain protocol), passing pin `97ce84a2…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-078-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_540 as prior + H_541 ADD-IMM 52 188 · H_542 SUB-IMM 50 188 · H_543 SUB-IMM 51 188 · H_544 SUB-IMM 52 188 · H_545 LDB 50 60 190 · H_546 LDB 51 60 190 · H_547 LDB 52 60 190 · H_548 ADD-IMM 50 190

(Full H_48..H_540 list: see `docs/auxdocs/parallel-batch-71-SPAWN.md` §Already locked plus H_533..H_540; treat that list plus H_541..H_548 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** ADD-IMM slot=51/52 imm=190 (finish 190 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=190 (start 190 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=198 — start 198 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=198 — start 198 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=198 — start 198 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
