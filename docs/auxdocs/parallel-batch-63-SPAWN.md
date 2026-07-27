# parallel-batch-63 SPAWN · scratch-only (post body-extend-068)

> Continuous queue handoff from body-extend-068.
> Pin after Relock: `2f81b43ba9e34a3bbc786fc9d308d0cc6d38c866dfdfd8e52a51bfed15acb5b8` (abbrev `2f81b43b…`).
> Handlers = 483 (H_00..H_476). Last selectors: 0x1DB..0x1E2 = H_469..H_476 (`40 1DB`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-62-log.md` / batch-62 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-068 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1E2 are `40 1E3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-068: **none** (all 8 batch-62 PASSes consolidated).

## Task: parallel-batch-63 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-63-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_469..H_476).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_476.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-069 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-069 consolidation Task (same chain protocol), passing pin `2f81b43b…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-069-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_468 as prior + H_469 ADD-IMM 52 148 · H_470 SUB-IMM 50 148 · H_471 SUB-IMM 51 148 · H_472 SUB-IMM 52 148 · H_473 LDB 50 60 150 · H_474 LDB 51 60 150 · H_475 LDB 52 60 150 · H_476 ADD-IMM 50 150

(Full H_48..H_468 list: see `docs/auxdocs/parallel-batch-62-SPAWN.md` §Already locked plus H_461..H_468; treat that list plus H_469..H_476 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=51/52 imm=150 — finish 150 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=150 — start 150 SUB triad (imm32 22B)
- LDB dd=50/51/52 ss=60 oo=158 — start 158 LDB triad (imm32 26B)
- ADD-IMM / SUB-IMM imm=158 triad starts
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
