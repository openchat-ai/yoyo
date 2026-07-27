# parallel-batch-47 SPAWN · scratch-only (post body-extend-052)

> Continuous queue handoff from body-extend-052.
> Pin after Relock: `edee584aa21a26569fe08e60d5089daf8d823c9df4c829c62b788b10815f4a51` (abbrev `edee584a…`).
> Handlers = 356 (H_00..H_349). Last selectors: 0x15C..0x163 = H_342..H_349 (`40 15C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-46-log.md` / batch-46 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-052 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x163 are `40 164`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-052: **none** (all 8 batch-46 PASSes consolidated).

## Task: parallel-batch-47 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-47-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_342..H_349).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_349.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-053 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-053 consolidation Task (same chain protocol), passing pin `edee584aa21a2656…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-053-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_341 as prior + H_342 LDB 51 60 E0 · H_343 LDB 52 60 E0 · H_344 ADD-IMM 50 D8 · H_345 ADD-IMM 51 D8 · H_346 ADD-IMM 52 D8 · H_347 SUB-IMM 50 D8 · H_348 SUB-IMM 51 D8 · H_349 SUB-IMM 52 D8

(Full H_48..H_341 list: see `docs/auxdocs/parallel-batch-46-SPAWN.md` §Already locked; treat that list plus H_342..H_349 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB oo=E8 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after E0
- ADD-IMM imm=E0 triad (slots 50/51/52) — fresh imm after D8; expect imm32 22B
- SUB-IMM imm=E0 triad (slots 50/51/52) — complements ADD-IMM * E0
- LDB oo=F0 triad (dd=50/51/52 ss=60) if E8 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
