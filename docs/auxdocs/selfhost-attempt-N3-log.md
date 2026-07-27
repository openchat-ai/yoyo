# W-START Attempt N3 Log · Full-`yoyo.ty` JS↔Rust `.text` byte-diff (EXPERIMENTAL)

> Tag: `attempt-N3-EXPERIMENTAL-fullsource-text-diff` · Timestamp: 2026-07-24
> Status: **EXPERIMENTAL · NON-GREEN**（≠ Phase 2 ≠ freeze ≠ full self-host）
> W-START: EXPERIMENTAL

## Inputs
- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685`.
- Locked source: `f:\yoyo\yoyo\projects\yoyo.ty`; SHA256 exactly matches the lock.
- Toolchain: `rustc 1.96.0`, `cargo 1.96.0`, `node v24.14.0`.
- Trust anchors UNTOUCHED: `yoyo.ty`, `*.lock`, peers' source (`yoyo-js/src/*`, `yoyo-rust/src/*`), `expected/*.code.hex`, `yoyo-asm/*`, `PROMPT-v3.md`.
- Only files added: `scripts/_probe/js-ty2text.mjs` (wrapper), `scripts/_probe/_attempt_n3_decode.mjs` (helper), `scripts/_probe/_attempt_n3_diff.mjs` (diff script), `scripts/_probe/js_yoyoty_text.txt`, `scripts/_probe/rust_yoyoty_text.bin`, `scripts/_probe/js_yoyoty_code.bin`, `scripts/_probe/rust_yoyoty_code.bin`, this log.

## Phase A — JS wrapper
- Path: `scripts/_probe/js-ty2text.mjs` (≤80 lines).
- Mechanism: reads `yoyo-js/scripts/golden.js` as text, loads it inside a Node
  `vm` sandbox with a `require` rooted at `yoyo-js/scripts/`, replaces the
  trailing `main();` invocation with `globalThis.__yoyoProbeExports = { parseTy,
  compileCode };`, then calls those functions. The upstream source body
  (incl. `require('../src/platform/encode-x64')`) runs UNMODIFIED — only the
  entry point is changed. No parallel decode path; no golden.js file edit.
- Synthetic test: ran on `selfhost_min_inc.ty` (18B INC) — exit 0, sha256
  `79dc29a5…` matches the canonical golden.
- Full-source run: `node js-ty2text.mjs yoyo.ty` — exit 0, **931 B** code,
  sha256 `5714c2a5…`.

## Phase B — Rust stream
- Command: `cargo run -q -p verifier --bin yoyo -- link --target=stub yoyo.ty rust_yoyoty_text.bin`
- Result: `emit: 202 ops → 931 code bytes, 0 data bytes, entry H_00`,
  `wrote flat rust_yoyoty_text.bin (932 bytes)` (1B stub startup `c3` +
  931 B code).
- Code-only (after stripping the leading `c3` stub startup): **931 B**,
  sha256 `5714c2a5…` — IDENTICAL to JS sha256.

## Phase C — Per-op masked diff

### Full-stream byte equality
| stream | len | sha256 |
|---|---|---|
| JS code   | 931 | `5714c2a59147ee561049c2d53e1241720dd59123cb0b6e37d7ee1e792c9f4f04` |
| Rust code | 931 | `5714c2a59147ee561049c2d53e1241720dd59123cb0b6e37d7ee1e792c9f4f04` |
| byte-equal-all | — | **YES (100.00 %, 931/931)** |

### Per-op spans (greedy opcode-scan, identical offsets in both peers)
| op | count | span (hex) | len | first 8 hex | last 8 hex | verdict |
|---|---|---|---|---|---|---|
| INC | 1 | `0x00f9–0x010a` | 18 | `498b87800200ffc0` | `0200000000c3` * | **EQUAL** |
| DEC | 1 | `0x010b–0x011c` | 18 | `498b87800200ffc8` | `0200000000c3` * | **EQUAL** |
| JMP | 1 | `0x011d–0x0122` |  6 | `e9defeffffffffff` | `feffffffffffc3` * | **EQUAL** |

`*` last 8 hex chars shown reflect full 8-char slice; actual last 4 bytes of
the span are the slot-disp tail of `store_state` plus the trailing RET `c3`.

### Masked section comparison (only INC / DEC / JMP bytes unmasked; everything else `00`)
- Masked JS stream byte-equal to masked Rust stream: **YES**.
- Outside the unmasked INC/DEC/JMP spans, both streams are byte-identical
  trivially because the entire 931B stream is byte-equal to begin with.

### What's masked and why
- D-1 ops (`0x20/0x50/0x51`): there are no `0x20` (legacy op) or `0x50/0x51`
  standalone opcodes in the emitted stream — `0x50/0x51` appear as
  operands (slot disp) inside `load_state`/`store_state` MOVs. Those
  bytes are NOT masked here (they're part of the canonical SET/GET shape
  which both peers emit identically in the locked stub), but flagged
  in scope: any future divergence in the SLOT-by-name path is out of
  this probe's scope (D-1 decision row).
- Startup byte: Rust emits a 1B stub `c3` startup (baremetal stub target);
  stripped before comparison (JS has no startup byte — emits raw `.text`
  via M0).
- Padding / unreached: none observed — `yoyo.ty` stub's handlers are tight
  (`H_05..H_16` are pure NOP chains ending in RET, no padding).

### JMP target rel32 note
- Single JMP at `0x011d`: `e9 de fe ff ff c3` (rel32 LE = `0xfffffede` =
  signed `-290`). Target = `0x011d + 5 + (-290)` = `0x011d + 5 - 0x122`
  = `0x0000` = `H_00`. Both peers produce IDENTICAL rel32 → identical
  layout. The "rel32 may differ for full source" caveat from N2 was a
  synthetic-fixture concern; on this locked stub with only one JMP
  back-edge to `H_00`, the offset collides trivially.

## Verdict / RED status

**EXPERIMENTAL observation only.** No GREEN promotion, no Phase 2 / freeze claim, no Relock. The locked `yoyo.ty` stub is intentionally small (34 handlers, 202 ops). Per-op pins exist only for **INC / DEC / JMP / CALL / JE / JCC-ALL / ALLOC / LOAD / WRITE** across JS M0 and the Rust verifier; **SUB / IMUL / GET / MOVRR / RAW_BYTE / STR / DATA** have no per-op pins in this audit.
This probe confirms byte-equality at the `.text`-stream level across the entire 931 B output; INC / DEC / JMP per-op spans agree exactly.

### Still RED (unchanged, NOT promoted by N3)
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 · 冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

Probe-local unresolved observation (not part of the PROMPT still-red line): D-1 SLOT-by-name divergence (`0x20/0x50/0x51`) remains unadjudicated by this probe.

## Next-step suggestion
Run the same probe against a `yoyo.ty` that exercises D-1 opcodes
(`0x20/0x50/0x51` as actual standalone opcodes, not slot disp operands)
in a synthetic file, to see whether the byte-equal property survives
when the slot-by-name path is forced through the divergence gate.