# parallel-batch-69 SPAWN · scratch-only (post body-extend-074)

> Continuous queue handoff from body-extend-074.
> Pin after Relock: `9243965c886555e99575615e4637331b6c2a49573d50ec183fb616c3ae3d2d98` (abbrev `9243965c…`).
> Handlers = 531 (H_00..H_524). Last selectors: 0x20B..0x212 = H_517..H_524 (`40 20B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-68-log.md` / batch-68 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-074 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x212 are `40 213`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-074 / batch-68: **SUB-IMM slot=51/52 imm=178** (finish 178 SUB triad).

## Task: parallel-batch-69 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-69-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_517..H_524).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_524.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-075 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-075 consolidation Task (same chain protocol), passing pin `9243965c…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-075-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_516 as prior + H_517 SUB-IMM 52 170 · H_518 LDB 50 60 178 · H_519 LDB 51 60 178 · H_520 LDB 52 60 178 · H_521 ADD-IMM 50 178 · H_522 ADD-IMM 51 178 · H_523 ADD-IMM 52 178 · H_524 SUB-IMM 50 178

(Full H_48..H_516 list: see `docs/auxdocs/parallel-batch-68-SPAWN.md` §Already locked plus H_509..H_516; treat that list plus H_517..H_524 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=51/52 imm=178 (complete 178 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=180 — start 180 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=180 — start 180 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=180 — start 180 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
