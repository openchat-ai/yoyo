# parallel-batch-52 SPAWN · scratch-only (post body-extend-057)

> Continuous queue handoff from body-extend-057.
> Pin after Relock: `0643c8f550fbb85d6e85eac409cf7ac90a26d7fece1b33bcfe04af260a9f2d5a` (abbrev `0643c8f5…`).
> Handlers = 396 (H_00..H_389). Last selectors: 0x184..0x18B = H_382..H_389 (`40 184`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-51-log.md` / batch-51 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-057 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x18B are `40 18C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-057: **none** (all 8 batch-51 PASSes consolidated).

## Task: parallel-batch-52 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-52-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_382..H_389).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_389.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-058 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-058 consolidation Task (same chain protocol), passing pin `0643c8f550fbb85d…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-058-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_381 as prior + H_382 SUB-IMM 52 F8 · H_383 LDB 50 60 100 · H_384 LDB 51 60 100 · H_385 LDB 52 60 100 · H_386 ADD-IMM 50 100 · H_387 ADD-IMM 51 100 · H_388 ADD-IMM 52 100 · H_389 SUB-IMM 50 100

(Full H_48..H_381 list: see `docs/auxdocs/parallel-batch-51-SPAWN.md` §Already locked; treat that list plus H_382..H_389 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=51/52 imm=100 — finish 100 SUB triad after H_389
- LDB oo=next after 100 (e.g. 108) triad (dd=50/51/52 ss=60)
- ADD-IMM / SUB-IMM fresh imm=108 triad (slots 50/51/52)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
