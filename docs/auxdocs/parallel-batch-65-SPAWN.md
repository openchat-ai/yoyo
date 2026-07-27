# parallel-batch-65 SPAWN · scratch-only (post body-extend-070)

> Continuous queue handoff from body-extend-070.
> Pin after Relock: `192ba67ac8bb814df865a108032dd1e9301c93c4e3fc89f44c8c4edfaf84791f` (abbrev `192ba67a…`).
> Handlers = 499 (H_00..H_492). Last selectors: 0x1EB..0x1F2 = H_485..H_492 (`40 1EB`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-64-log.md` / batch-64 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-070 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1F2 are `40 1F3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-070: **none** (all 8 batch-64 PASSes consolidated).

## Task: parallel-batch-65 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-65-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_485..H_492).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_492.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-071 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-071 consolidation Task (same chain protocol), passing pin `192ba67a…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-071-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_484 as prior + H_485 ADD-IMM 50 158 · H_486 ADD-IMM 51 158 · H_487 ADD-IMM 52 158 · H_488 SUB-IMM 50 158 · H_489 SUB-IMM 51 158 · H_490 SUB-IMM 52 158 · H_491 LDB 50 60 160 · H_492 LDB 51 60 160

(Full H_48..H_484 list: see `docs/auxdocs/parallel-batch-64-SPAWN.md` §Already locked plus H_477..H_484; treat that list plus H_485..H_492 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=52 ss=60 oo=160 — finish 160 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=160 — start 160 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=160 — start 160 SUB triad (imm32 22B)
- LDB oo=168 triad starts
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
