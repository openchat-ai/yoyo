# parallel-batch-50 SPAWN · scratch-only (post body-extend-055)

> Continuous queue handoff from body-extend-055.
> Pin after Relock: `fba1f97e01a9ef7e6285451fe34b6b52a972caf99ae81f93518563d7eb1ec442` (abbrev `fba1f97e…`).
> Handlers = 380 (H_00..H_373). Last selectors: 0x174..0x17B = H_366..H_373 (`40 174`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-49-log.md` / batch-49 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-055 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x17B are `40 17C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-055: **none** (all 8 batch-49 PASSes consolidated).

## Task: parallel-batch-50 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-50-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_366..H_373).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_373.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-056 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-056 consolidation Task (same chain protocol), passing pin `fba1f97e01a9ef7e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-056-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_365 as prior + H_366 LDB 51 60 F0 · H_367 LDB 52 60 F0 · H_368 ADD-IMM 50 F0 · H_369 ADD-IMM 51 F0 · H_370 ADD-IMM 52 F0 · H_371 SUB-IMM 50 F0 · H_372 SUB-IMM 51 F0 · H_373 SUB-IMM 52 F0

(Full H_48..H_365 list: see `docs/auxdocs/parallel-batch-49-SPAWN.md` §Already locked; treat that list plus H_366..H_373 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB oo=F8 triad (dd=50/51/52 ss=60) — next oo after F0; expect imm32 26B
- ADD-IMM imm=F8 triad (slots 50/51/52) — fresh imm after F0; expect imm32 22B
- SUB-IMM imm=F8 triad (slots 50/51/52) — complements ADD-IMM * F8
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
