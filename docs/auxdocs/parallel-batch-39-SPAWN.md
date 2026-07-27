# parallel-batch-39 SPAWN · scratch-only (post body-extend-044)

> Continuous queue handoff from body-extend-044.
> Pin after Relock: `3514c8c6558f7028fdc93ea64a26dc007fe2df25592035494342ab5fbe6e102c` (abbrev `3514c8c6…`).
> Handlers = 292 (H_00..H_285). Last selectors: 0x11C..0x123 = H_278..H_285 (`40 11C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-38-log.md` / batch-38 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-044 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x123 are `40 124`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-044: **none** (all 8 batch-38 PASSes consolidated).

## Task: parallel-batch-39 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-39-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_278..H_285).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_285.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-045 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-045 consolidation Task (same chain protocol), passing pin `3514c8c6558f7028…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-045-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_277 as prior + H_278 ADD-IMM 50 98 · H_279 ADD-IMM 51 98 · H_280 ADD-IMM 52 98 · H_281 SUB-IMM 50 98 · H_282 SUB-IMM 51 98 · H_283 SUB-IMM 52 98 · H_284 LDB 50 60 B8 · H_285 LDB 51 60 B8

(Full H_48..H_277 list: see `docs/auxdocs/parallel-batch-38-SPAWN.md` §Already locked; treat that list plus H_278..H_285 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB 52 60 B8 — finish B8 triad (H_284/H_285=50/51; expect imm32 26B)
- ADD-IMM imm=A0 triad (slots 50/51/52) — fresh imm after 98; expect imm32 22B
- SUB-IMM imm=A0 triad (slots 50/51/52) — complements ADD-IMM * A0
- LDB oo=C0 triad (dd=50/51/52 ss=60; expect imm32 26B)
- ADD-IMM / SUB-IMM fresh imm=A8 triad if A0 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
