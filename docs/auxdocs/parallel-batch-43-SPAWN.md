# parallel-batch-43 SPAWN · scratch-only (post body-extend-048)

> Continuous queue handoff from body-extend-048.
> Pin after Relock: `9c2f924a2780d64647f590c707d39330fa4bff0e69a2c243c0550956ec2d41a2` (abbrev `9c2f924a…`).
> Handlers = 324 (H_00..H_317). Last selectors: 0x13C..0x143 = H_310..H_317 (`40 13C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-42-log.md` / batch-42 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-048 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x143 are `40 144`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-048: **none** (all 8 batch-42 PASSes consolidated).

## Task: parallel-batch-43 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-43-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_310..H_317).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_317.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-049 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-049 consolidation Task (same chain protocol), passing pin `9c2f924a2780d646…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-049-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_309 as prior + H_310 SUB-IMM 52 B0 · H_311 ADD-IMM 50 B8 · H_312 ADD-IMM 51 B8 · H_313 ADD-IMM 52 B8 · H_314 SUB-IMM 50 B8 · H_315 SUB-IMM 51 B8 · H_316 SUB-IMM 52 B8 · H_317 LDB 50 60 D0

(Full H_48..H_309 list: see `docs/auxdocs/parallel-batch-42-SPAWN.md` §Already locked; treat that list plus H_310..H_317 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB 51 60 D0 / LDB 52 60 D0 (finish D0 triad after H_317; expect imm32 26B)
- ADD-IMM imm=C0 triad (slots 50/51/52) — fresh imm after B8; expect imm32 22B
- SUB-IMM imm=C0 triad (slots 50/51/52) — complements ADD-IMM * C0
- LDB oo=D8 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after D0
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
