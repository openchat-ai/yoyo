# parallel-batch-58 SPAWN · scratch-only (post body-extend-063)

> Continuous queue handoff from body-extend-063.
> Pin after Relock: `f4fa77a59520fda01683d3ceffe44de6886ba77752450ffbb0947e0ba15f0d96` (abbrev `f4fa77a5…`).
> Handlers = 443 (H_00..H_436). Last selectors: 0x1B4..0x1BA = H_430..H_436 (`40 1B4`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-57-log.md` / batch-57 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-063 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1BA are `40 1BB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-063: **none** (all 7 batch-57 PASSes consolidated).

## Task: parallel-batch-58 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-58-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_430..H_436).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_436.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-064 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-064 consolidation Task (same chain protocol), passing pin `f4fa77a5…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-064-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_429 as prior + H_430 LDB 52 60 128 · H_431 ADD-IMM 50 128 · H_432 ADD-IMM 51 128 · H_433 ADD-IMM 52 128 · H_434 SUB-IMM 50 128 · H_435 SUB-IMM 51 128 · H_436 SUB-IMM 52 128

(Full H_48..H_429 list: see `docs/auxdocs/parallel-batch-57-SPAWN.md` §Already locked plus H_422..H_429; treat that list plus H_430..H_436 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=50/51/52 ss=60 oo=130 — start 130 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=130 — start 130 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=130 — start 130 SUB triad (imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
