# parallel-batch-35 SPAWN · scratch-only (post body-extend-040)

> Continuous queue handoff from body-extend-040.
> Pin after Relock: `a58ead289233c42ba1c6e9a84aedb6218176aad27ecd5cbdd0d4659a2e5bc187` (abbrev `a58ead28…`).
> Handlers = 260 (H_00..H_253). Last selectors: 0xFC..0xFF = H_246..H_249; 0x100..0x103 = H_250..H_253 (`40 100`.. via label-width A).
> Source protocol: mirror `docs/auxdocs/parallel-batch-34-log.md` / batch-34 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-040 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x103 are `40 104`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-040: **none** (all 8 batch-34 PASSes consolidated; label-width A landed).

## Task: parallel-batch-35 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-35-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_246..H_253).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_253.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-041 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-041 consolidation Task (same chain protocol), passing pin `a58ead289233c42b…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-041-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_245 as prior + H_246 LDB 51 60 98 · H_247 LDB 52 60 98 · H_248 SUB-IMM 50 78 · H_249 SUB-IMM 51 78 · H_250 SUB-IMM 52 78 · H_251 ADD-IMM 50 80 · H_252 ADD-IMM 51 80 · H_253 ADD-IMM 52 80

(Full H_48..H_245 list: see `docs/auxdocs/parallel-batch-34-SPAWN.md` §Already locked; treat that list plus H_246..H_253 as occupied.)

## Suggested fresh pick directions (non-binding)

- LDB oo=A0 triad (dd=50/51/52 ss=60; expect imm32 26B)
- SUB-IMM imm=80 triad (slots 50/51/52) — complements locked ADD-IMM * 80
- ADD-IMM / SUB-IMM fresh imm=88 triad (slots 50/51/52)
- LDB oo=A8 next rung if A0 lands
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
