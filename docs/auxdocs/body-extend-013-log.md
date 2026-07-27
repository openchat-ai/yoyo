# body-extend-013 Log · H_46 0x80 LDB oo=0x60 imm8 byte 0x60=96

> Tag: `body-extend-013-EXPERIMENTAL-LDB-OFF96-HANDLER` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `4342f2fb916214cc…` → `21a81fe1c8b52875…`.
> **handler count: 53 → 54** (+1 at selector 0x34).

## 1. Pick rationale

Default from body-extend-012 §4: H_46 `0x80 LDB oo=0x60` exercises a fresh positive imm8 byte (0x60 = 96 decimal) at a fresh selector (0x34). Both peers stay on the imm8 path; neither escalates to imm32.

**Why it's "next default"**: Positive imm8 byte (0x60=96) at fresh selector; both peers stay on imm8 path; per body-extend-012 auto-roll cue.

Peer emit paths confirmed without modifying trusted encoders:
- **JS** `encode-x64.js:104-110` — `encodeOp(0x80, [0x50, 0x60, 0x60])` calls `addImmRax(0x60)` and emits `48 83 c0 60`.
- **Rust** `emit.rs:130-141` — `TirOp::Ldb → emit_ldb(0x50, 0x60, 0x60)` calls `add_imm(Rax, 0x60)` and emits `48 83 c0 60`.

Hand-derived 23B expected bytes for `80 50 60 60`:

```
load_state(0x60, rax):   49 8b 87 00 03 00 00   (7B, disp32=0x300 LE)
add_imm(rax, 0x60=96):   48 83 c0 60             (4B, imm8 path: 96 ∈ [-128, 127])
movzx rax, byte [rax]:  48 0f b6 00              (4B)
store_state(0x50, rax):  49 89 87 80 02 00 00    (7B, disp32=0x280 LE)
ret:                     c3                       (1B)
Total:                   23B = 498b87000300004883c060480fb60049898780020000c3
```

**Critical imm8 boundary assertion**: encoder stayed on the 4B `48 83 c0 60` path and did not emit the 7B imm32 form `48 81 c0 60 00 00 00`.

## 2. Execution record

- Hand-authored H_46 at `yoyo/projects/yoyo.ty:717-730`: `40 34 / 80 50 60 60 / FF`; not RAW_BYTE; mirrors H_45.
- Added `selfhost_min_ldb_off96_handler.ty` and independent 23B pin `498b87000300004883c060480fb60049898780020000c3`.
- JS-actual fixture probe via `scripts/_probe/js-ty2text.mjs`: exact 23B match.
- JS `checkLDBOFF96HANDLER()` verifies exact bytes, length, `48 83 c0 60`, and absence of `48 81 c0`.
- Rust `ldb_off96_handler_slot_check()` and `check_selfhost_min_ldb_off96_handler()` verify the same selector and bytes.
- JS golden: **37/37 PASS**.
- Rust self-test: **PASS**; Rust golden: **44/44 PASS**.
- Full canonical emit: JS=Rust=**1296B** code (was 1273B; +23B); byte-equal **Y**. Raw code sha256: `61acead92abcc31d57578a86a8583d5dd939bff0a717d5d6adc9031dce5211b0`.
- 2-chain DDC: **EQUAL**, 1536B compared, hash `d1e8b221cc6e6313cf8eb222f9d313f3150ec48f49d6450c8a3a1ce833959f12`.
- Lock verification: PASS at `21a81fe1c8b52875af8d6e73668cd3d01ac332eb5fa51405fcb62c7da9102c98`; previous pin chained to `4342f2fb916214cc662040e590a2b75aa97622c45e5cc016e5bd18208098e5b1`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: H_46 LDB oo=0x60 at selector 0x34.
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: canonical handler appended at lines 717-730.
4. Selftest: exact 23B and imm8-path assertions PASS.
5. Goldens: JS 37/37 and Rust 44/44 PASS; full emit byte-equal Y at 1296B.
6. Lock: `verify-yoyo-ty.mjs` PASS at `21a81fe1c8b52875…`.
7. DDC: `verify-selfhost.ps1` EQUAL on 1536B.
8. Commit: none.

## 4. Next default

Queued body-extend-014: H_47 — `0x80 LDB dd=0x50 ss=0x60 oo=0x70` at selector 0x35. The fresh positive imm8 byte 0x70 (=112) remains within [-128, 127], so both peers should emit identical `48 83 c0 70` and continue the LDB imm8-path byte-coverage matrix without encoder changes.
