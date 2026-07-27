# parallel-batch-38 SPAWN · scratch-only (post body-extend-043)

> Continuous queue handoff from body-extend-043.
> Pin after Relock: `113decd0cbfa7a1106ae3f17f82ba7b6a135c8ad6a3b579b7c30978ffb96d7a0` (abbrev `113decd0…`).
> Handlers = 284 (H_00..H_277). Last selectors: 0x114..0x11B = H_270..H_277 (`40 114`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-37-log.md` / batch-37 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-043 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x11B are `40 11C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-043: **none** (all 8 batch-37 PASSes consolidated).

## Task: parallel-batch-38 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-38-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_270..H_277).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_277.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-044 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-044 consolidation Task (same chain protocol), passing pin `113decd0cbfa7a11…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-044-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_269 as prior + H_270 ADD-IMM 51 90 · H_271 ADD-IMM 52 90 · H_272 SUB-IMM 50 90 · H_273 SUB-IMM 51 90 · H_274 SUB-IMM 52 90 · H_275 LDB 50 60 B0 · H_276 LDB 51 60 B0 · H_277 LDB 52 60 B0

(Full H_48..H_269 list: see `docs/auxdocs/parallel-batch-37-SPAWN.md` §Already locked; treat that list plus H_270..H_277 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM imm=98 triad (slots 50/51/52) — fresh imm after 90; expect imm32 22B
- SUB-IMM imm=98 triad (slots 50/51/52) — complements ADD-IMM * 98
- LDB oo=B8 triad (dd=50/51/52 ss=60; expect imm32 26B)
- ADD-IMM / SUB-IMM fresh imm=A0 triad if 98 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
