# parallel-batch-49 SPAWN · scratch-only (post body-extend-054)

> Continuous queue handoff from body-extend-054.
> Pin after Relock: `13cb91ab1e1cc24d3f4b6d9a151a2e9a8d487556099cc030a189d6ac30554d9b` (abbrev `13cb91ab…`).
> Handlers = 372 (H_00..H_365). Last selectors: 0x16C..0x173 = H_358..H_365 (`40 16C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-48-log.md` / batch-48 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-054 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x173 are `40 174`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-054: **none** (all 8 batch-48 PASSes consolidated).

## Task: parallel-batch-49 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-49-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_358..H_365).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_365.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-055 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-055 consolidation Task (same chain protocol), passing pin `13cb91ab1e1cc24d…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-055-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_357 as prior + H_358 SUB-IMM 52 E0 · H_359 ADD-IMM 50 E8 · H_360 ADD-IMM 51 E8 · H_361 ADD-IMM 52 E8 · H_362 SUB-IMM 50 E8 · H_363 SUB-IMM 51 E8 · H_364 SUB-IMM 52 E8 · H_365 LDB 50 60 F0

(Full H_48..H_357 list: see `docs/auxdocs/parallel-batch-48-SPAWN.md` §Already locked; treat that list plus H_358..H_365 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=51/52 ss=60 oo=F0 (finish F0 triad after H_365) — expect imm32 26B
- ADD-IMM imm=F0 triad (slots 50/51/52) — fresh imm after E8; expect imm32 22B
- SUB-IMM imm=F0 triad (slots 50/51/52) — complements ADD-IMM * F0
- LDB oo=F8 triad (dd=50/51/52 ss=60) — next oo after F0; expect imm32 26B
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
