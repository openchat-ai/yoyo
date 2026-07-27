# parallel-batch-67 SPAWN · scratch-only (post body-extend-072)

> Continuous queue handoff from body-extend-072.
> Pin after Relock: `e1554db8dcce9946348a88383bed73939d4a835e8dc0989a2788a72a590e6a6b` (abbrev `e1554db8…`).
> Handlers = 515 (H_00..H_508). Last selectors: 0x1FB..0x202 = H_501..H_508 (`40 1FB`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-66-log.md` / batch-66 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-072 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x202 are `40 203`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-072: **none** (all 8 batch-66 PASSes consolidated).

## Task: parallel-batch-67 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-67-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_501..H_508).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_508.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-073 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-073 consolidation Task (same chain protocol), passing pin `e1554db8…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-073-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_500 as prior + H_501 LDB 51 60 168 · H_502 LDB 52 60 168 · H_503 ADD-IMM 50 168 · H_504 ADD-IMM 51 168 · H_505 ADD-IMM 52 168 · H_506 SUB-IMM 50 168 · H_507 SUB-IMM 51 168 · H_508 SUB-IMM 52 168

(Full H_48..H_500 list: see `docs/auxdocs/parallel-batch-66-SPAWN.md` §Already locked plus H_493..H_500; treat that list plus H_501..H_508 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=50/51/52 ss=60 oo=170 — start 170 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=170 — start 170 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=170 — start 170 SUB triad (imm32 22B)
- LDB oo=178 triad starts / finish any open ladder rungs
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
