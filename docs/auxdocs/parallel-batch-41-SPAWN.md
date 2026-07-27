# parallel-batch-41 SPAWN · scratch-only (post body-extend-046)

> Continuous queue handoff from body-extend-046.
> Pin after Relock: `422c843275989ac30c1ba7406c7ff47076310df79ef0c3193903bca15460afde` (abbrev `422c8432…`).
> Handlers = 308 (H_00..H_301). Last selectors: 0x12C..0x133 = H_294..H_301 (`40 12C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-40-log.md` / batch-40 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-046 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x133 are `40 134`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-046: **none** (all 8 batch-40 PASSes consolidated).

## Task: parallel-batch-41 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-41-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_294..H_301).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_301.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-047 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-047 consolidation Task (same chain protocol), passing pin `422c843275989ac3…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-047-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_293 as prior + H_294 LDB 51 60 C0 · H_295 LDB 52 60 C0 · H_296 ADD-IMM 50 A8 · H_297 ADD-IMM 51 A8 · H_298 ADD-IMM 52 A8 · H_299 SUB-IMM 50 A8 · H_300 SUB-IMM 51 A8 · H_301 SUB-IMM 52 A8

(Full H_48..H_293 list: see `docs/auxdocs/parallel-batch-40-SPAWN.md` §Already locked; treat that list plus H_294..H_301 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB oo=C8 triad (dd=50/51/52 ss=60; expect imm32 26B) — next oo after C0 triad complete
- ADD-IMM imm=B0 triad (slots 50/51/52) — fresh imm after A8; expect imm32 22B
- SUB-IMM imm=B0 triad (slots 50/51/52) — complements ADD-IMM * B0
- LDB oo=D0 triad if C8 lands
- ADD-IMM / SUB-IMM fresh imm=B8 triad if B0 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
