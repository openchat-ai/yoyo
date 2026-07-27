# parallel-batch-36 SPAWN · scratch-only (post body-extend-041)

> Continuous queue handoff from body-extend-041.
> Pin after Relock: `4cb656812b03c0fdb229b2d0d9278c479ab83b33d6cc7782e75f2397b0e165db` (abbrev `4cb65681…`).
> Handlers = 268 (H_00..H_261). Last selectors: 0x104..0x10B = H_254..H_261 (`40 104`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-35-log.md` / batch-35 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-041 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x10B are `40 10C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-041: **none** (all 8 batch-35 PASSes consolidated).

## Task: parallel-batch-36 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-36-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_254..H_261).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_261.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-042 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-042 consolidation Task (same chain protocol), passing pin `4cb656812b03c0fd…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-042-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_253 as prior + H_254 LDB 50 60 A0 · H_255 LDB 51 60 A0 · H_256 LDB 52 60 A0 · H_257 SUB-IMM 50 80 · H_258 SUB-IMM 51 80 · H_259 SUB-IMM 52 80 · H_260 ADD-IMM 50 88 · H_261 ADD-IMM 51 88

(Full H_48..H_253 list: see `docs/auxdocs/parallel-batch-35-SPAWN.md` §Already locked; treat that list plus H_254..H_261 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=52 imm=88 (complete ADD 88 triad; expect imm32 22B)
- SUB-IMM imm=88 triad (slots 50/51/52) — complements ADD-IMM * 88
- LDB oo=A8 triad (dd=50/51/52 ss=60; expect imm32 26B)
- ADD-IMM / SUB-IMM fresh imm=90 triad if 88 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
