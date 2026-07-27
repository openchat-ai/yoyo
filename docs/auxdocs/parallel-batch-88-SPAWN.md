# parallel-batch-88 SPAWN · scratch-only (post body-extend-093)

> Continuous queue handoff from body-extend-093.
> Pin after Relock: `04656bbbbb152b5402bd76daa324a51a7f68477df3b3615827ef88aa2907978b` (abbrev `04656bbb…`).
> Handlers = 683 (H_00..H_676). Last selectors: 0x2A3..0x2AA = H_669..H_676 (`40 2A3`..`40 2AA` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-87-log.md` / batch-87 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-093 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x2AA are `40 2AB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-093 / batch-87: **SUB-IMM slot=50/51/52 imm=200** (start/finish 200 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-88 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-88-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_669..H_676).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_676.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-094 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-094 consolidation Task (same chain protocol), passing pin `04656bbb…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-094-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_668 as prior + H_669 SUB-IMM 51 1F8 · H_670 SUB-IMM 52 1F8 · H_671 LDB 50 60 200 · H_672 LDB 51 60 200 · H_673 LDB 52 60 200 · H_674 ADD-IMM 50 200 · H_675 ADD-IMM 51 200 · H_676 ADD-IMM 52 200

(Full H_48..H_668 list: see `docs/auxdocs/parallel-batch-87-SPAWN.md` §Already locked plus H_661..H_668; treat that list plus H_669..H_676 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=50/51/52 imm=200 (start/finish 200 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB/ADD-IMM beyond 200) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
