# parallel-batch-53 SPAWN · scratch-only (post body-extend-058)

> Continuous queue handoff from body-extend-058.
> Pin after Relock: `c258ff3271396e1822dba5baf34c98aae7003f19c10a916a0aa3967142f5c2dc` (abbrev `c258ff32…`).
> Handlers = 404 (H_00..H_397). Last selectors: 0x18C..0x193 = H_390..H_397 (`40 18C`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-52-log.md` / batch-52 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-058 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x193 are `40 194`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-058: **none** (all 8 batch-52 PASSes consolidated).

## Task: parallel-batch-53 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-53-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_390..H_397).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_397.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-059 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-059 consolidation Task (same chain protocol), passing pin `c258ff3271396e18…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-059-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_389 as prior + H_390 SUB-IMM 51 100 · H_391 SUB-IMM 52 100 · H_392 LDB 50 60 108 · H_393 LDB 51 60 108 · H_394 LDB 52 60 108 · H_395 ADD-IMM 50 108 · H_396 ADD-IMM 51 108 · H_397 ADD-IMM 52 108

(Full H_48..H_389 list: see `docs/auxdocs/parallel-batch-52-SPAWN.md` §Already locked; treat that list plus H_390..H_397 as occupied.)

## Suggested fresh pick directions (non-binding)

- SUB-IMM slot=50/51/52 imm=108 — finish 108 SUB triad after H_395..H_397 ADD
- LDB oo=next after 108 (e.g. 110) triad (dd=50/51/52 ss=60)
- ADD-IMM / SUB-IMM fresh imm=110 triad (slots 50/51/52)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
