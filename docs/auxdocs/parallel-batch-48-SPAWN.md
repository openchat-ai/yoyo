# parallel-batch-48 SPAWN · scratch-only (post body-extend-053)

> Continuous queue handoff from body-extend-053.
> Pin after Relock: `86485f4822e891c4f11dbc5f181c43dc3f23d7ed779b61831f2426f2329e504d` (abbrev `86485f48…`).
> Handlers = 364 (H_00..H_357). Last selectors: 0x164..0x16B = H_350..H_357 (`40 164`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-47-log.md` / batch-47 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-053 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x16B are `40 16C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-053: **none** (all 8 batch-47 PASSes consolidated).

## Task: parallel-batch-48 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-48-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_350..H_357).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_357.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-054 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-054 consolidation Task (same chain protocol), passing pin `86485f4822e891c4…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-054-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_349 as prior + H_350 LDB 50 60 E8 · H_351 LDB 51 60 E8 · H_352 LDB 52 60 E8 · H_353 ADD-IMM 50 E0 · H_354 ADD-IMM 51 E0 · H_355 ADD-IMM 52 E0 · H_356 SUB-IMM 50 E0 · H_357 SUB-IMM 51 E0

(Full H_48..H_349 list: see `docs/auxdocs/parallel-batch-47-SPAWN.md` §Already locked; treat that list plus H_350..H_357 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=52 imm=E0 (finish E0 triad after H_356/H_357) — expect imm32 22B
- ADD-IMM imm=E8 triad (slots 50/51/52) — fresh imm after E0; expect imm32 22B
- SUB-IMM imm=E8 triad (slots 50/51/52) — complements ADD-IMM * E8
- LDB oo=F0 triad (dd=50/51/52 ss=60) — next oo after E8; expect imm32 26B
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
