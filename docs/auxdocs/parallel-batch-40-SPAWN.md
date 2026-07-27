# parallel-batch-40 SPAWN · scratch-only (post body-extend-045)

> Continuous queue handoff from body-extend-045.
> Pin after Relock: `8c80a6fa783440b2ef724beb1860f295c81cde46c53f35d0cdcc40ff8798519c` (abbrev `8c80a6fa…`).
> Handlers = 300 (H_00..H_293). Last selectors: 0x124..0x12B = H_286..H_293 (`40 124`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-39-log.md` / batch-39 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-045 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x12B are `40 12C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-045: **none** (all 8 batch-39 PASSes consolidated).

## Task: parallel-batch-40 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-40-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_286..H_293).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_293.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-046 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-046 consolidation Task (same chain protocol), passing pin `8c80a6fa783440b2…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-046-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_285 as prior + H_286 LDB 52 60 B8 · H_287 ADD-IMM 50 A0 · H_288 ADD-IMM 51 A0 · H_289 ADD-IMM 52 A0 · H_290 SUB-IMM 50 A0 · H_291 SUB-IMM 51 A0 · H_292 SUB-IMM 52 A0 · H_293 LDB 50 60 C0

(Full H_48..H_285 list: see `docs/auxdocs/parallel-batch-39-SPAWN.md` §Already locked; treat that list plus H_286..H_293 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB 51 60 C0 · LDB 52 60 C0 — finish C0 triad (H_293=50; expect imm32 26B)
- ADD-IMM imm=A8 triad (slots 50/51/52) — fresh imm after A0; expect imm32 22B
- SUB-IMM imm=A8 triad (slots 50/51/52) — complements ADD-IMM * A8
- LDB oo=C8 triad (dd=50/51/52 ss=60; expect imm32 26B) if C0 lands
- ADD-IMM / SUB-IMM fresh imm=B0 triad if A8 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
