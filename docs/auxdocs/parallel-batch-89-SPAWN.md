# parallel-batch-89 SPAWN · scratch-only (post body-extend-094)

> Continuous queue handoff from body-extend-094.
> Pin after Relock: `0ef9611b50021d82d2c7870a29d1d4107164b7a3c586f41f5271a083fbdfec51` (abbrev `0ef9611b…`).
> Handlers = 691 (H_00..H_684). Last selectors: 0x2AB..0x2B2 = H_677..H_684 (`40 2AB`..`40 2B2` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-88-log.md` / batch-88 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-094 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2B2 are `40 2B3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-094 / batch-88: **ADD-IMM slot=52 imm=208** (finish 208 ADD triad); **SUB-IMM slot=50/51/52 imm=208** (start 208 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-89 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-89-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_677..H_684).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_684.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-095 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-095 consolidation Task (same chain protocol), passing pin `0ef9611b…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-095-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_676 as prior + H_677 SUB-IMM 50 200 · H_678 SUB-IMM 51 200 · H_679 SUB-IMM 52 200 · H_680 LDB 50 60 208 · H_681 LDB 51 60 208 · H_682 LDB 52 60 208 · H_683 ADD-IMM 50 208 · H_684 ADD-IMM 51 208

(Full H_48..H_676 list: see `docs/auxdocs/parallel-batch-88-SPAWN.md` §Already locked plus H_669..H_676; treat that list plus H_677..H_684 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** ADD-IMM slot=52 imm=208 (finish 208 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=208 (start 208 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 208) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
