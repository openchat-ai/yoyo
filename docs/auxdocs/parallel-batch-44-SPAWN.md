# parallel-batch-44 SPAWN · scratch-only (post body-extend-049)

> Continuous queue handoff from body-extend-049.
> Pin after Relock: `69adc5a0b11c8f176687deff6753b2fa51b6611c3cd1193c79bf1143b7b4c957` (abbrev `69adc5a0…`).
> Handlers = 332 (H_00..H_325). Last selectors: 0x144..0x14B = H_318..H_325 (`40 144`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-43-log.md` / batch-43 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-049 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x14B are `40 14C`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-049: **none** (all 8 batch-43 PASSes consolidated).

## Task: parallel-batch-44 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-44-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_318..H_325).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_325.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-050 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-050 consolidation Task (same chain protocol), passing pin `69adc5a0b11c8f17…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-050-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_317 as prior + H_318 LDB 51 60 D0 · H_319 LDB 52 60 D0 · H_320 ADD-IMM 50 C0 · H_321 ADD-IMM 51 C0 · H_322 ADD-IMM 52 C0 · H_323 SUB-IMM 50 C0 · H_324 SUB-IMM 51 C0 · H_325 SUB-IMM 52 C0

(Full H_48..H_317 list: see `docs/auxdocs/parallel-batch-43-SPAWN.md` §Already locked; treat that list plus H_318..H_325 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB oo=D8 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after D0 triad complete
- ADD-IMM imm=C8 triad (slots 50/51/52) — fresh imm after C0; expect imm32 22B
- SUB-IMM imm=C8 triad (slots 50/51/52) — complements ADD-IMM * C8
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
