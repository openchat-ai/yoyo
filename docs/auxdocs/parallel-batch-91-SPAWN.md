# parallel-batch-91 SPAWN · scratch-only (post body-extend-096)

> Continuous queue handoff from body-extend-096.
> Pin after Relock: `0a02f49ed0c94a2df5078022a7737c92d4021cab62c41dcbbfc5bb728f32f29c` (abbrev `0a02f49e…`).
> Handlers = 707 (H_00..H_700). Last selectors: 0x2BB..0x2C2 = H_693..H_700 (`40 2BB`..`40 2C2` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-90-log.md` / batch-90 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-096 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2C2 are `40 2C3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-096 / batch-90: **ADD-IMM slot=50/51/52 imm=218** (start 218 ADD triad); **SUB-IMM slot=50/51/52 imm=218** (start 218 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-91 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-91-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_693..H_700).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_700.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-097 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-097 consolidation Task (same chain protocol), passing pin `0a02f49e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-097-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_692 as prior + H_693 ADD-IMM 51 210 · H_694 ADD-IMM 52 210 · H_695 SUB-IMM 50 210 · H_696 SUB-IMM 51 210 · H_697 SUB-IMM 52 210 · H_698 LDB 50 60 218 · H_699 LDB 51 60 218 · H_700 LDB 52 60 218

(Full H_48..H_692 list: see `docs/auxdocs/parallel-batch-90-SPAWN.md` §Already locked plus H_685..H_692; treat that list plus H_693..H_700 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** ADD-IMM slot=50/51/52 imm=218 (start 218 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=218 (start 218 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 218) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
