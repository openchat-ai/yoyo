# parallel-batch-46 SPAWN · scratch-only (post body-extend-051)

> Continuous queue handoff from body-extend-051.
> Pin after Relock: `ee5b881e34301f79f6c647181243709ea5ccfdbf03a2088c7d44b1de98d91b4f` (abbrev `ee5b881e…`).
> Handlers = 348 (H_00..H_341). Last selectors: 0x154..0x15B = H_334..H_341 (`40 154`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-45-log.md` / batch-45 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-051 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x15B are `40 15C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-051: **none** (all 8 batch-45 PASSes consolidated).

## Task: parallel-batch-46 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-46-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_334..H_341).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_341.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-052 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-052 consolidation Task (same chain protocol), passing pin `ee5b881e34301f79…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-052-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_333 as prior + H_334 SUB-IMM 52 C8 · H_335 ADD-IMM 50 D0 · H_336 ADD-IMM 51 D0 · H_337 ADD-IMM 52 D0 · H_338 SUB-IMM 50 D0 · H_339 SUB-IMM 51 D0 · H_340 SUB-IMM 52 D0 · H_341 LDB 50 60 E0

(Full H_48..H_333 list: see `docs/auxdocs/parallel-batch-45-SPAWN.md` §Already locked; treat that list plus H_334..H_341 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=51/52 ss=60 oo=E0 (finish E0 triad after H_341; expect imm32 26B)
- ADD-IMM imm=D8 triad (slots 50/51/52) — fresh imm after D0; expect imm32 22B
- SUB-IMM imm=D8 triad (slots 50/51/52) — complements ADD-IMM * D8
- LDB oo=E8 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after E0
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
