# parallel-batch-76 SPAWN · scratch-only (post body-extend-081)

> Continuous queue handoff from body-extend-081.
> Pin after Relock: `267c611dbb648db15251e6e6ee8a52287434680892e9f2ad290fd161eb2b916c` (abbrev `267c611d…`).
> Handlers = 587 (H_00..H_580). Last selectors: 0x243..0x24A = H_573..H_580 (`40 243`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-75-log.md` / batch-75 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-081 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x24A are `40 24B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-081 / batch-75: **LDB 50/51/52 60 1B0** (start 1B0 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1B0** (start 1B0 ADD/SUB triads).

## Task: parallel-batch-76 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-76-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_573..H_580).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_580.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-082 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-082 consolidation Task (same chain protocol), passing pin `267c611d…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-082-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_572 as prior + H_573 LDB 51 60 1A8 · H_574 LDB 52 60 1A8 · H_575 ADD-IMM 50 1A8 · H_576 ADD-IMM 51 1A8 · H_577 ADD-IMM 52 1A8 · H_578 SUB-IMM 50 1A8 · H_579 SUB-IMM 51 1A8 · H_580 SUB-IMM 52 1A8

(Full H_48..H_572 list: see `docs/auxdocs/parallel-batch-75-SPAWN.md` §Already locked plus H_565..H_572; treat that list plus H_573..H_580 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred:** LDB dd=50/51/52 ss=60 oo=1B0 (start 1B0 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1B0 (start 1B0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1B0 (start 1B0 SUB triad; imm32 22B)
- LDB dd=50/51/52 ss=60 oo=1B8 — start 1B8 LDB triad (imm32 26B)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1B8 — start 1B8 ADD/SUB triads
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
