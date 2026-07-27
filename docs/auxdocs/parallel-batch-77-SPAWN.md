# parallel-batch-77 SPAWN · scratch-only (post body-extend-082)

> Continuous queue handoff from body-extend-082.
> Pin after Relock: `05a3a9c6693fa65c20f47a3eab1bc536c5e5fe0a168381faf0cf72330ca58056` (abbrev `05a3a9c6…`).
> Handlers = 595 (H_00..H_588). Last selectors: 0x24B..0x252 = H_581..H_588 (`40 24B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-76-log.md` / batch-76 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-082 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x252 are `40 253`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-082 / batch-76: **SUB-IMM slot=52 imm=1B0** (finish 1B0 SUB triad); **LDB 50/51/52 60 1B8** (start 1B8 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1B8** (start 1B8 ADD/SUB triads).

## Task: parallel-batch-77 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-77-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_581..H_588).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_588.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-083 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-083 consolidation Task (same chain protocol), passing pin `05a3a9c6…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-083-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_580 as prior + H_581 LDB 50 60 1B0 · H_582 LDB 51 60 1B0 · H_583 LDB 52 60 1B0 · H_584 ADD-IMM 50 1B0 · H_585 ADD-IMM 51 1B0 · H_586 ADD-IMM 52 1B0 · H_587 SUB-IMM 50 1B0 · H_588 SUB-IMM 51 1B0

(Full H_48..H_580 list: see `docs/auxdocs/parallel-batch-76-SPAWN.md` §Already locked plus H_573..H_580; treat that list plus H_581..H_588 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=52 imm=1B0 (finish 1B0 SUB triad; imm32 22B)
- **Start deferred:** LDB dd=50/51/52 ss=60 oo=1B8 (start 1B8 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1B8 (start 1B8 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1B8 (start 1B8 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
