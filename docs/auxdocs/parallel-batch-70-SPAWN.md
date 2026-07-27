# parallel-batch-70 SPAWN · scratch-only (post body-extend-075)

> Continuous queue handoff from body-extend-075.
> Pin after Relock: `69f1bb2f223e28673dfb97de72b1305d451313a4865d02e766ed947748a10cf9` (abbrev `69f1bb2f…`).
> Handlers = 539 (H_00..H_532). Last selectors: 0x213..0x21A = H_525..H_532 (`40 213`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-69-log.md` / batch-69 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-075 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x21A are `40 21B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-075 / batch-69: **SUB-IMM slot=50/51/52 imm=180** (start 180 SUB triad).

## Task: parallel-batch-70 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-70-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_525..H_532).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_532.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-076 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-076 consolidation Task (same chain protocol), passing pin `69f1bb2f…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-076-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_524 as prior + H_525 SUB-IMM 51 178 · H_526 SUB-IMM 52 178 · H_527 LDB 50 60 180 · H_528 LDB 51 60 180 · H_529 LDB 52 60 180 · H_530 ADD-IMM 50 180 · H_531 ADD-IMM 51 180 · H_532 ADD-IMM 52 180

(Full H_48..H_524 list: see `docs/auxdocs/parallel-batch-69-SPAWN.md` §Already locked plus H_517..H_524; treat that list plus H_525..H_532 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=50/51/52 imm=180 (start 180 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=188 — start 188 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=188 — start 188 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=188 — start 188 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
