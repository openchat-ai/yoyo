# body-extend-012 Log · H_45 0x80 LDB oo=0x20 imm8 byte 0x20=32

> Tag: `body-extend-012-EXPERIMENTAL-LDB-OFF32-HANDLER` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `ce0ceff131dc4e07…` → `4342f2fb916214cc…`.
> **handler count: 52 → 53** (+1 at selector 0x33).

## 1. Pick rationale

Default from body-extend-011 §4: H_45 `0x80 LDB oo=0x20` exercises a fresh imm8 byte (0x20 = 32 decimal) at a fresh selector (0x33). Positive imm8 path (32 ∈ [-128, 127]) — both peers stay on the imm8 path; neither escalates to imm32.

**Why it's "next default"**: positive imm8 byte (0x20=32) at fresh selector; both peers stay on imm8 path (4B `48 83 c0 20`); symmetric to H_44's oo=0x10 byte-position coverage; per body-extend-011 auto-roll cue.

Peer emit paths confirmed (dispatch contract forbids editing emit.rs / encode-x64.js):
- **JS** `encode-x64.js:104-110` — `if (op === 0x80) { const out = [...loadState(a(1), 0, 0)]; if (a(2)) out.push(...addImmRax(a(2))); out.push(0x48, 0x0f, 0xb6, 0x00); out.push(...storeState(a(0), 0, 0)); return out; }` → `addImmRax(0x20)` → `0x20 >= -128 && 0x20 <= 127` → `[0x48, 0x83, 0xc0, 0x20 & 0xff]` = `[0x48, 0x83, 0xc0, 0x20]` (4B imm8 path). ✓
- **Rust** `emit.rs:130-141` — `fn emit_ldb(dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> { let mut out = load_state(ss, Reg::Rax)?; if oo != 0 { out.extend(assembler::add_imm(Reg::Rax, oo as u64)?); } out.extend_from_slice(&[0x48, 0x0F, 0xB6, 0x00]); out.extend(store_state(dd, Reg::Rax)?); Ok(out) }` → `add_imm(Rax, 0x20)` → `0x20 as i64 = 32`, `32 >= -128 && 32 <= 127` → `vec![0x48, 0x83, 0xC0, 0x20]` (4B imm8 path). ✓

Hand-derived 23B expected bytes for `80 50 60 20`:
```
load_state(0x60, rax):   49 8b 87 00 03 00 00   (7B, disp32=0x300 LE)
add_imm(rax, 0x20=32):   48 83 c0 20             (4B, imm8 path: 32 ∈ [-128, 127])
movzx rax, byte [rax]:  48 0f b6 00              (4B)
store_state(0x50, rax):  49 89 87 80 02 00 00    (7B, disp32=0x280 LE)
ret:                     c3                       (1B)
Total:                   23B = 498b87000300004883c020480fb60049898780020000c3
```

**Critical imm8 boundary assertion**: encoder MUST stay on imm8 path (4B `48 83 c0 20`), NOT escalate to imm32 (would be `48 81 c0` 7B). Both peers' signed threshold (`imm >= -128 && imm <= 127`) classifies 0x20 (= 32) as imm8. The signed-imm8 LEFT-edge signed-token semantic (-128 → imm8 byte 0x80) remains covered by the JS-only `checkLDBoffm128` probe (Rust ty_parser lacks signed-hex support; emit.rs treats oo as unsigned u16 per dispatch contract).

## 2. Execution record

- Hand-authored H_45 in `yoyo/projects/yoyo.ty` (appended to end of file): `40 33 / 80 50 60 20 / FF`; not RAW_BYTE. Mirrors H_44 template.
- Added handler fixture `selfhost_min_ldb_off32_handler.ty` (selector 0x33, opcode 80 50 60 20) and independent 23B pin `498b87000300004883c020480fb60049898780020000c3`.
- JS `checkLDBOFF32HANDLER()` verifies exact bytes, 23B length, imm8 signature (`48 83 c0 20`), and absence of imm32 opcode `48 81 c0`.
- Rust `ldb_off32_handler_slot_check()` and `check_selfhost_min_ldb_off32_handler()` verify the same selector and bytes.
- JS-actual verified directly via `scripts/_probe/js-ty2text.mjs` (23B exact match); Rust-actual verified via the new `ldb_off32_handler_slot_check` (23B exact match).
- JS golden: **36/36 PASS** (was 35/35, +1 for LDB-OFF32-HANDLER).
- Rust self-test: **PASS**; Rust golden: **43/43 PASS** (was 42/42, +1 for G-SM-LDB-OFF32-HANDLER).
- Full canonical emit: JS=Rust=**1273B** code (was 1250B; +23B for H_45); byte-equal **Y** (M1.exe == M_rust.exe, sha256 `017a911989ec1e79994933f5141df798c7a0f3e3ea7923de479da8ddf16139f7`).
- 2-chain DDC: **EQUAL**, 1536B compared, hash `017a911989ec1e79994933f5141df798c7a0f3e3ea7923de479da8ddf16139f7`.
- Lock verification: PASS at `4342f2fb916214cc662040e590a2b75aa97622c45e5cc016e5bd18208098e5b1`; previous pin chained to `ce0ceff131dc4e07483919f0f3744698d2a09d2516d4d2ca1452930672b79a2f`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: H_45 LDB oo=0x20 at selector 0x33.
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: canonical handler appended (H_44 template mirrored).
4. Selftest: exact 23B and imm8-path assertions PASS (JS LDB-OFF32-HANDLER + Rust ldb_off32_handler_slot_check + check_selfhost_min_ldb_off32_handler).
5. Goldens: JS 36/36 and Rust 43/43 PASS.
6. Lock: `verify-yoyo-ty.mjs` PASS at `4342f2fb916214cc…`.
7. DDC: `verify-selfhost.ps1` EQUAL on 1536B.
8. Commit: none.

## 4. Next default

Queued body-extend-013: H_46 — pick the next LDB or other 3-arg coverage point; the symmetric imm8 path coverage at oo=0x20 closes another natural byte-side extension after H_37 (oo=0, 19B), H_40 (oo=8, 23B), H_41 (oo=127, 23B), H_42 (oo=0x50, 23B), H_43 (oo=0x40, 23B), H_44 (oo=0x10, 23B), H_45 (oo=0x20, 23B). The remaining imm8 byte-side coverage options at this selector density include: oo=0x60 (96), oo=0x70 (112), oo=1, oo=2, oo=4, etc. Next default should examine another positive imm8 byte (e.g., oo=0x60=96) at a fresh selector to continue extending the LDB imm8-path byte-coverage matrix symmetrically.