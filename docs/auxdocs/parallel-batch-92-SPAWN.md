# parallel-batch-92 SPAWN · scratch-only (post body-extend-097)

> Continuous queue handoff from body-extend-097.
> Pin after Relock: `e6ba7d6cfcbb11da0a3a63dab93cde597a265934cf95064968d97697c85cd68a` (abbrev `e6ba7d6c…`).
> Handlers = 715 (H_00..H_708). Last selectors: 0x2C3..0x2CA = H_701..H_708 (`40 2C3`..`40 2CA` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-91-log.md` / batch-91 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-097 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2CA are `40 2CB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-097 / batch-91: **LDB dd=52 ss=60 oo=220** (finish 220 LDB triad); **ADD-IMM slot=50/51/52 imm=220** (start 220 ADD triad); **SUB-IMM slot=50/51/52 imm=220** (start 220 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-92 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-92-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_701..H_708).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_708.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-098 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-098 consolidation Task (same chain protocol), passing pin `e6ba7d6c…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-098-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_700 as prior + H_701 ADD-IMM 50 218 · H_702 ADD-IMM 51 218 · H_703 ADD-IMM 52 218 · H_704 SUB-IMM 50 218 · H_705 SUB-IMM 51 218 · H_706 SUB-IMM 52 218 · H_707 LDB 50 60 220 · H_708 LDB 51 60 220

(Full H_48..H_700 list: see `docs/auxdocs/parallel-batch-91-SPAWN.md` §Already locked plus H_693..H_700; treat that list plus H_701..H_708 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=52 ss=60 oo=220 (finish 220 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=220 (start 220 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=220 (start 220 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 220) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
