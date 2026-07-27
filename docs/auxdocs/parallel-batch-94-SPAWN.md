# parallel-batch-94 SPAWN · scratch-only (post body-extend-099)

> Continuous queue handoff from body-extend-099.
> Pin after Relock: `3fc618f9e14a881a91460a8c1be733bade35794eca50282f64c5eb0cabb57021` (abbrev `3fc618f9…`).
> Handlers = 731 (H_00..H_724). Last selectors: 0x2D3..0x2DA = H_717..H_724 (`40 2D3`..`40 2DA` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-93-log.md` / batch-93 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-099 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green. (Prior body-extend-098 measured DIFFER — both honest.)
> Next selectors after 0x2DA are `40 2DB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-099 / batch-93: **LDB dd=50/51/52 ss=60 oo=230** (start 230 LDB ladder); **ADD-IMM slot=50/51/52 imm=230** (start 230 ADD triad); **SUB-IMM slot=50/51/52 imm=230** (start 230 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-94 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-94-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_717..H_724).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_724.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-100 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-100 consolidation Task (same chain protocol), passing pin `3fc618f9…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-100-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_716 as prior + H_717 LDB 51 60 228 · H_718 LDB 52 60 228 · H_719 ADD-IMM 50 228 · H_720 ADD-IMM 51 228 · H_721 ADD-IMM 52 228 · H_722 SUB-IMM 50 228 · H_723 SUB-IMM 51 228 · H_724 SUB-IMM 52 228

(Full H_48..H_716 list: see `docs/auxdocs/parallel-batch-93-SPAWN.md` §Already locked plus H_709..H_716; treat that list plus H_717..H_724 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** LDB dd=50/51/52 ss=60 oo=230 (start 230 LDB ladder; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=230 (start 230 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=230 (start 230 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 230) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
