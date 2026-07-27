# parallel-batch-71 SPAWN · scratch-only (post body-extend-076)

> Continuous queue handoff from body-extend-076.
> Pin after Relock: `ebbc6d765fcc0fcdc045848e93a3839d47ffdf287646adb781170a66d80690be` (abbrev `ebbc6d76…`).
> Handlers = 547 (H_00..H_540). Last selectors: 0x21B..0x222 = H_533..H_540 (`40 21B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-70-log.md` / batch-70 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-076 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x222 are `40 223`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-076 / batch-70: **ADD-IMM slot=52 imm=188** (finish 188 ADD triad); **SUB-IMM slot=50/51/52 imm=188** (start 188 SUB triad).

## Task: parallel-batch-71 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-71-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_533..H_540).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_540.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-077 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-077 consolidation Task (same chain protocol), passing pin `ebbc6d76…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-077-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_532 as prior + H_533 SUB-IMM 50 180 · H_534 SUB-IMM 51 180 · H_535 SUB-IMM 52 180 · H_536 LDB 50 60 188 · H_537 LDB 51 60 188 · H_538 LDB 52 60 188 · H_539 ADD-IMM 50 188 · H_540 ADD-IMM 51 188

(Full H_48..H_532 list: see `docs/auxdocs/parallel-batch-70-SPAWN.md` §Already locked plus H_525..H_532; treat that list plus H_533..H_540 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** ADD-IMM slot=52 imm=188 (finish 188 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=188 (start 188 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=190 — start 190 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=190 — start 190 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=190 — start 190 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
