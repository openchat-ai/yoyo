# parallel-batch-37 SPAWN · scratch-only (post body-extend-042)

> Continuous queue handoff from body-extend-042.
> Pin after Relock: `afceb388015dd4a7e7a2de16a109eb8649189bb28471d021bb4b82eeaa9d1311` (abbrev `afceb388…`).
> Handlers = 276 (H_00..H_269). Last selectors: 0x10C..0x113 = H_262..H_269 (`40 10C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-36-log.md` / batch-36 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-042 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x113 are `40 114`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-042: **none** (all 8 batch-36 PASSes consolidated).

## Task: parallel-batch-37 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-37-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_262..H_269).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_269.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-043 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-043 consolidation Task (same chain protocol), passing pin `afceb388015dd4a7…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-043-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_261 as prior + H_262 ADD-IMM 52 88 · H_263 SUB-IMM 50 88 · H_264 SUB-IMM 51 88 · H_265 SUB-IMM 52 88 · H_266 LDB 50 60 A8 · H_267 LDB 51 60 A8 · H_268 LDB 52 60 A8 · H_269 ADD-IMM 50 90

(Full H_48..H_261 list: see `docs/auxdocs/parallel-batch-36-SPAWN.md` §Already locked; treat that list plus H_262..H_269 as occupied.)

## Suggested fresh pick directions (non-binding)

- ADD-IMM slot=51 imm=90 / slot=52 imm=90 (complete ADD 90 triad with H_269; expect imm32 22B)
- SUB-IMM imm=90 triad (slots 50/51/52) — complements ADD-IMM * 90
- LDB oo=B0 triad (dd=50/51/52 ss=60; expect imm32 26B)
- ADD-IMM / SUB-IMM fresh imm=98 triad if 90 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
