# parallel-batch-61 SPAWN · scratch-only (post body-extend-066)

> Continuous queue handoff from body-extend-066.
> Pin after Relock: `d52ed6373d5b085118d5a601ac8f25b8a529e7c16b36b6dd3bce2115d73ec080` (abbrev `d52ed637…`).
> Handlers = 467 (H_00..H_460). Last selectors: 0x1CB..0x1D2 = H_453..H_460 (`40 1CB`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-60-log.md` / batch-60 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-066 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1D2 are `40 1D3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-066: **none** (all 8 batch-60 PASSes consolidated).

## Task: parallel-batch-61 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-61-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_453..H_460).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_460.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-067 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-067 consolidation Task (same chain protocol), passing pin `d52ed637…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-067-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_452 as prior + H_453 SUB-IMM 51 138 · H_454 SUB-IMM 52 138 · H_455 LDB 50 60 140 · H_456 LDB 51 60 140 · H_457 LDB 52 60 140 · H_458 ADD-IMM 50 140 · H_459 ADD-IMM 51 140 · H_460 ADD-IMM 52 140

(Full H_48..H_452 list: see `docs/auxdocs/parallel-batch-60-SPAWN.md` §Already locked plus H_445..H_452; treat that list plus H_453..H_460 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=50/51/52 imm=140 — start 140 SUB triad (imm32 22B)
- LDB dd=50/51/52 ss=60 oo=148 — start 148 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=148 — start 148 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=148 — start 148 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
