# parallel-batch-90 SPAWN · scratch-only (post body-extend-095)

> Continuous queue handoff from body-extend-095.
> Pin after Relock: `aef6d89f98ceb7c8d9770950da9a584d7165f7e0d6713fc30c1d3f14c92552ee` (abbrev `aef6d89f…`).
> Handlers = 699 (H_00..H_692). Last selectors: 0x2B3..0x2BA = H_685..H_692 (`40 2B3`..`40 2BA` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-89-log.md` / batch-89 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-095 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2BA are `40 2BB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-095 / batch-89: **ADD-IMM slot=51/52 imm=210** (finish 210 ADD triad); **SUB-IMM slot=50/51/52 imm=210** (start 210 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-90 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-90-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_685..H_692).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_692.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-096 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-096 consolidation Task (same chain protocol), passing pin `aef6d89f…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-096-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_684 as prior + H_685 ADD-IMM 52 208 · H_686 SUB-IMM 50 208 · H_687 SUB-IMM 51 208 · H_688 SUB-IMM 52 208 · H_689 LDB 50 60 210 · H_690 LDB 51 60 210 · H_691 LDB 52 60 210 · H_692 ADD-IMM 50 210

(Full H_48..H_684 list: see `docs/auxdocs/parallel-batch-89-SPAWN.md` §Already locked plus H_677..H_684; treat that list plus H_685..H_692 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** ADD-IMM slot=51/52 imm=210 (finish 210 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=210 (start 210 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 210) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
