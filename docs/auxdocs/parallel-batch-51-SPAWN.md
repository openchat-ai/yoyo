# parallel-batch-51 SPAWN · scratch-only (post body-extend-056)

> Continuous queue handoff from body-extend-056.
> Pin after Relock: `824207c608fe5d03e4bd1c3bca1f33aec844dd62f4bc66ca4a6877364538314b` (abbrev `824207c6…`).
> Handlers = 388 (H_00..H_381). Last selectors: 0x17C..0x183 = H_374..H_381 (`40 17C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-50-log.md` / batch-50 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-056 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x183 are `40 184`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-056: **none** (all 8 batch-50 PASSes consolidated).

## Task: parallel-batch-51 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-51-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_374..H_381).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_381.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-057 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-057 consolidation Task (same chain protocol), passing pin `824207c608fe5d03…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-057-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_373 as prior + H_374 LDB 50 60 F8 · H_375 LDB 51 60 F8 · H_376 LDB 52 60 F8 · H_377 ADD-IMM 50 F8 · H_378 ADD-IMM 51 F8 · H_379 ADD-IMM 52 F8 · H_380 SUB-IMM 50 F8 · H_381 SUB-IMM 51 F8

(Full H_48..H_373 list: see `docs/auxdocs/parallel-batch-50-SPAWN.md` §Already locked; treat that list plus H_374..H_381 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=52 imm=F8 — finish F8 SUB triad after H_380/H_381
- LDB oo=next after F8 (if available) triad (dd=50/51/52 ss=60)
- ADD-IMM / SUB-IMM fresh imm beyond F8 (slots 50/51/52)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
