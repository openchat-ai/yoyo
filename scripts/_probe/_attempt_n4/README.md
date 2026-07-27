# scripts/_probe/_attempt_n4 — N4 D-1 probe (NON-CANONICAL)

> Status: **probe-only artifact**. Not a disk golden. Not sha-pinned. Not
> promoted to `yoyo/tests/golden/`. Not referenced by any test runner.

## What's here
- `synth-d1.ty` — synthetic fixture exercising the three D-1 ops
  (`0x20 ALLOC`, `0x50 LOAD_FILE`, `0x51 WRITE_FILE`) as real independent
  opcodes, with every slot arg given by NAME (the slot-by-name path).
  See `docs/auxdocs/selfhost-attempt-N4-log.md` for full description.
- `js_out.txt` — JS peer hex-dump output (UTF-16 LE on disk due to
  PowerShell `>` redirect; the diff script detects the BOM and decodes).
- `rust_out.bin` — Rust peer flat binary (1B `0xC3` stub startup + 90B code).
- `js_code.bin` / `rust_code.bin` — extracted code bytes (90 B each,
  byte-equal). Written by `_n4_diff.mjs`.
- `js_err.txt` / `rust_err.txt` — captured stderr streams. Both empty
  (no compile errors).
- `_n4_diff.mjs` — Node diff script. Reads `js_out.txt` + `rust_out.bin`,
  strips the Rust stub startup, computes byte-equal count, full first-
  diff offset, per-handler offsets via 0xC3 RET terminator scan.

## How to reproduce
```sh
cd F:\yoyo
# JS
cd scripts/_probe/_attempt_n4
node ../js-ty2text.mjs ./synth-d1.ty > js_out.txt
# Rust (return to repo root for cargo)
cd F:\yoyo\yoyo-rust
cargo run -q -p verifier --bin yoyo -- link --target=stub \
  F:/yoyo/scripts/_probe/_attempt_n4/synth-d1.ty \
  F:/yoyo/scripts/_probe/_attempt_n4/rust_out.bin
# Diff
node F:/yoyo/scripts/_probe/_attempt_n4/_n4_diff.mjs
```

## Verdict
- 90 / 90 bytes byte-equal
- 5 / 5 handlers byte-equal at identical offsets
- sha256 (`19b4f47c…`): JS == Rust
- D-1 did NOT trigger on this synth
- See `docs/auxdocs/selfhost-attempt-N4-log.md` for full report.

## Cleanup
This directory is intentionally retained for reproducibility of the N4
observation. Safe to delete after the W-START EXPIRE TTL elapses.