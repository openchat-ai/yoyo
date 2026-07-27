# parallel-batch-93 SPAWN · scratch-only (post body-extend-098)

> Continuous queue handoff from body-extend-098.
> Pin after Relock: `8d4277255b098dc108295590e42155afd50ffca67fbab34ea1430ef615405136` (abbrev `8d427725…`).
> Handlers = 723 (H_00..H_716). Last selectors: 0x2CB..0x2D2 = H_709..H_716 (`40 2CB`..`40 2D2` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-92-log.md` / batch-92 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-098 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2D2 are `40 2D3`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-098 / batch-92: **LDB dd=51/52 ss=60 oo=228** (finish 228 LDB triad); **ADD-IMM slot=50/51/52 imm=228** (start 228 ADD triad); **SUB-IMM slot=50/51/52 imm=228** (start 228 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-93 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-93-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_709..H_716).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_716.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-099 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-099 consolidation Task (same chain protocol), passing pin `8d427725…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-099-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_708 as prior + H_709 LDB 52 60 220 · H_710 ADD-IMM 50 220 · H_711 ADD-IMM 51 220 · H_712 ADD-IMM 52 220 · H_713 SUB-IMM 50 220 · H_714 SUB-IMM 51 220 · H_715 SUB-IMM 52 220 · H_716 LDB 50 60 228

(Full H_48..H_708 list: see `docs/auxdocs/parallel-batch-92-SPAWN.md` §Already locked plus H_701..H_708; treat that list plus H_709..H_716 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** LDB dd=51/52 ss=60 oo=228 (finish 228 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=228 (start 228 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=228 (start 228 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB beyond 228) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
