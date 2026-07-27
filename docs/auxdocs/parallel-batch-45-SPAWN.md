# parallel-batch-45 SPAWN · scratch-only (post body-extend-050)

> Continuous queue handoff from body-extend-050.
> Pin after Relock: `1566906f85667e97cb5701b0d3ba8fdd806e893b1982fa3ad11a1138efb8adfe` (abbrev `1566906f…`).
> Handlers = 340 (H_00..H_333). Last selectors: 0x14C..0x153 = H_326..H_333 (`40 14C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-44-log.md` / batch-44 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-050 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x153 are `40 154`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-050: **none** (all 8 batch-44 PASSes consolidated).

## Task: parallel-batch-45 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-45-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_326..H_333).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_333.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-051 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-051 consolidation Task (same chain protocol), passing pin `1566906f85667e97…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-051-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_325 as prior + H_326 LDB 50 60 D8 · H_327 LDB 51 60 D8 · H_328 LDB 52 60 D8 · H_329 ADD-IMM 50 C8 · H_330 ADD-IMM 51 C8 · H_331 ADD-IMM 52 C8 · H_332 SUB-IMM 50 C8 · H_333 SUB-IMM 51 C8

(Full H_48..H_325 list: see `docs/auxdocs/parallel-batch-44-SPAWN.md` §Already locked; treat that list plus H_326..H_333 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=52 imm=C8 (finish C8 SUB triad; expect imm32 22B) — complements H_332/H_333
- ADD-IMM imm=D0 triad (slots 50/51/52) — fresh imm after C8; expect imm32 22B
- SUB-IMM imm=D0 triad (slots 50/51/52) — complements ADD-IMM * D0
- LDB oo=E0 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after D8 triad complete
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
