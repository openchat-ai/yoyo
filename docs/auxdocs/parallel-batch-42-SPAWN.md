# parallel-batch-42 SPAWN · scratch-only (post body-extend-047)

> Continuous queue handoff from body-extend-047.
> Pin after Relock: `000042c8ea316c07fce78e5bb05814229058adea09ac196d0d1e8a90987336f2` (abbrev `000042c8…`).
> Handlers = 316 (H_00..H_309). Last selectors: 0x134..0x13B = H_302..H_309 (`40 134`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-41-log.md` / batch-41 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-047 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x13B are `40 13C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-047: **none** (all 8 batch-41 PASSes consolidated).

## Task: parallel-batch-42 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-42-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_302..H_309).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_309.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-048 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-048 consolidation Task (same chain protocol), passing pin `000042c8ea316c07…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-048-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_301 as prior + H_302 LDB 50 60 C8 · H_303 LDB 51 60 C8 · H_304 LDB 52 60 C8 · H_305 ADD-IMM 50 B0 · H_306 ADD-IMM 51 B0 · H_307 ADD-IMM 52 B0 · H_308 SUB-IMM 50 B0 · H_309 SUB-IMM 51 B0

(Full H_48..H_301 list: see `docs/auxdocs/parallel-batch-41-SPAWN.md` §Already locked; treat that list plus H_302..H_309 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM 52 B0 (finish B0 triad; expect imm32 22B) — complements H_308/H_309
- ADD-IMM imm=B8 triad (slots 50/51/52) — fresh imm after B0; expect imm32 22B
- SUB-IMM imm=B8 triad (slots 50/51/52) — complements ADD-IMM * B8
- LDB oo=D0 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after C8 triad complete
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
