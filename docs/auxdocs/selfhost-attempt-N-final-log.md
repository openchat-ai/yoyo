# W-START Attempt N-final Log · N-series closeout (NON-NORMATIVE · EXPERIMENTAL)

> Timestamp: 2026-07-24 (UTC+8) · W-START: `EXPERIMENTAL` · Ref: `docs/auxdocs/selfhost-start-node.md`
> Status: **EXPERIMENTAL · NON-GREEN**（≠ 自举 GREEN ≠ freeze ≠ full self-host）

## Context
N1–N4 covered attempt-level dispatch/harness, the INC/DEC/JMP subset, full locked `yoyo.ty` JS↔Rust `.text`, and a D-1 slot-by-name synth; all results are observations, not GREEN promotion.

## Audit findings and fixes

| finding | log | fix |
|---|---|---|
| N3 opcode-scope overstatement | `selfhost-attempt-N3-log.md` | Limited the claim to the nine pinned opcodes and explicitly listed seven opcode families without per-op pins. |
| N1 仍红 shortlist and duplication | `selfhost-attempt-N1-log.md` | Replaced the shortlist with PROMPT-v3.md’s exact nine-item still-red line; removed the duplicate full-body item. |
| N3/N4 wrapper-path conflict | `selfhost-attempt-N3-log.md`, `selfhost-attempt-N4-log.md` | Aligned both logs to the only on-disk wrapper: `scripts/_probe/js-ty2text.mjs`. |

## N1–N4 byte-equal outcomes

| attempt | scope | outcome | status |
|---|---|---|---|
| N1 | dispatch and harness baseline | JS 18/18 + asm 2/2 + Rust 25/25; 2-chain DDC **EQUAL** | attempt-level observation |
| N2 | synthetic INC/DEC/JMP `.text` subset | compared stream **exactly equal** | observation only |
| N3 | full locked `yoyo.ty`, JS↔Rust `.text` | **931/931 bytes equal**, same SHA256; INC/DEC/JMP spans equal | observation only |
| N4 | synthetic D-1 `0x20/0x50/0x51` slot-by-name path | **90/90 bytes equal**; D-1 remains unadjudicated | observation only |

## Why stop N-series

**byte-compare saturation confirmed by N4; future attempts need a different template (asm peer / runtime canary / M-N invariant).** The equal byte streams do not establish full self-host, 3-chain section parity, runtime behavior, morph/posture, freeze, or GREEN status.

## Still RED（verbatim from `PROMPT-v3.md`）

full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 · 冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

## Promotion and trust boundary

`PROMPT-v3.md` **NOT touched**; nothing is promoted to GREEN. Trust roots, lock material, peers, goldens, asm sources, and `js-ty2text.mjs` were not changed. No commit.

## Suggested next directions（not bound to byte-compare）

- Build an asm-peer and 3-chain section execution harness with explicit failure-closed gates.
- Add runtime canaries for observable handler behavior across stub and real platform backends.
- Define and test M-N invariants for morph/posture, gen1≡gen2, and the eventual freeze gate.
