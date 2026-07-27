# parallel-batch-66 SPAWN · scratch-only (post body-extend-071)

> Continuous queue handoff from body-extend-071.
> Pin after Relock: `1f070530a91ca949696f7552fc5d3b3690f00630a191ce25662ee33951314e41` (abbrev `1f070530…`).
> Handlers = 507 (H_00..H_500). Last selectors: 0x1F3..0x1FA = H_493..H_500 (`40 1F3`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-65-log.md` / batch-65 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-071 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x1FA are `40 1FB`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-071: **none** (all 8 batch-65 PASSes consolidated).

## Task: parallel-batch-66 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-66-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_493..H_500).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_500.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-072 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-072 consolidation Task (same chain protocol), passing pin `1f070530…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-072-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_492 as prior + H_493 LDB 52 60 160 · H_494 ADD-IMM 50 160 · H_495 ADD-IMM 51 160 · H_496 ADD-IMM 52 160 · H_497 SUB-IMM 50 160 · H_498 SUB-IMM 51 160 · H_499 SUB-IMM 52 160 · H_500 LDB 50 60 168

(Full H_48..H_492 list: see `docs/auxdocs/parallel-batch-65-SPAWN.md` §Already locked plus H_485..H_492; treat that list plus H_493..H_500 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB dd=51/52 ss=60 oo=168 — finish 168 LDB triad (imm32 26B)
- ADD-IMM slot=50/51/52 imm=168 — start 168 ADD triad (imm32 22B)
- SUB-IMM slot=50/51/52 imm=168 — start 168 SUB triad (imm32 22B)
- LDB oo=170 triad starts
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
