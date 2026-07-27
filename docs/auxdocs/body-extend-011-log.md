# body-extend-011 Log · H_44 0x80 LDB oo=0x10 imm8 byte 0x10=16

> Tag: `body-extend-011-EXPERIMENTAL-LDB-OFF16-HANDLER` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `cf2324f2fdcc732f…` → `ce0ceff131dc4e07…`.
> **handler count: 51 → 52** (+1 at selector 0x32).

## 1. Pick rationale

Default from body-extend-010 §4: H_44 `0x80 LDB oo=0x10` exercises a fresh imm8 byte (0x10 = 16 decimal) at a fresh selector (0x32). Symmetric to H_43 (oo=0x40), H_42 (oo=0x50), and H_41 (oo=0x7f); extends the LDB imm8-path byte-coverage matrix to include 0x10. Positive imm8 path (16 ∈ [-128, 127]) — both peers stay on the imm8 path; neither escalates to imm32.

**Why it's "next default"**: positive imm8 byte (0x10=16) at fresh selector; both peers stay on imm8 path (4B `48 83 c0 10`); symmetric to H_43's oo=0x40 byte-position coverage; extends LDB imm8-path byte-coverage matrix.

Peer emit paths confirmed (dispatch contract forbids editing emit.rs / encode-x64.js):
- **JS** `encode-x64.js:104-110` — `if (op === 0x80) { const out = [...loadState(a(1), 0, 0)]; if (a(2)) out.push(...addImmRax(a(2))); out.push(0x48, 0x0f, 0xb6, 0x00); out.push(...storeState(a(0), 0, 0)); return out; }` → `addImmRax(0x10)` → `0x10 >= -128 && 0x10 <= 127` → `[0x48, 0x83, 0xc0, 0x10 & 0xff]` = `[0x48, 0x83, 0xc0, 0x10]` (4B imm8 path). ✓
- **Rust** `emit.rs:130-141` — `fn emit_ldb(dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> { let mut out = load_state(ss, Reg::Rax)?; if oo != 0 { out.extend(assembler::add_imm(Reg::Rax, oo as u64)?); } out.extend_from_slice(&[0x48, 0x0F, 0xB6, 0x00]); out.extend(store_state(dd, Reg::Rax)?); Ok(out) }` → `add_imm(Rax, 0x10)` → `0x10 as i64 = 16`, `16 >= -128 && 16 <= 127` → `vec![0x48, 0x83, 0xC0, 0x10]` (4B imm8 path). ✓

Hand-derived 23B expected bytes for `80 50 60 10`:
```
load_state(0x60, rax):   49 8b 87 00 03 00 00   (7B, disp32=0x300 LE)
add_imm(rax, 0x10=16):   48 83 c0 10             (4B, imm8 path: 16 ∈ [-128, 127])
movzx rax, byte [rax]:  48 0f b6 00              (4B)
store_state(0x50, rax):  49 89 87 80 02 00 00    (7B, disp32=0x280 LE)
ret:                     c3                       (1B)
Total:                   23B = 498b87000300004883c010480fb60049898780020000c3
```

**Critical imm8 boundary assertion**: encoder MUST stay on imm8 path (4B `48 83 c0 10`), NOT escalate to imm32 (would be `48 81 c0` 7B). Both peers' signed threshold (`imm >= -128 && imm <= 127`) classifies 0x10 (= 16) as imm8. The signed-imm8 LEFT-edge signed-token semantic (-128 → imm8 byte 0x80) remains covered by the JS-only `checkLDBoffm128` probe (Rust ty_parser lacks signed-hex support; emit.rs treats oo as unsigned u16 per dispatch contract).

## 2. Execution record

- Hand-authored H_44 in `yoyo/projects/yoyo.ty` (appended to end of file at line range ~661-682, before H_43 mirror block): `40 32 / 80 50 60 10 / FF`; not RAW_BYTE. Mirrors H_43 template.
- Added handler fixture `selfhost_min_ldb_off16_handler.ty` (selector 0x32, opcode 80 50 60 10) and independent 23B pin `498b87000300004883c010480fb60049898780020000c3`.
- JS `checkLDBOFF16HANDLER()` verifies exact bytes, 23B length, imm8 signature (`48 83 c0 10`), and absence of imm32 opcode `48 81 c0`.
- Rust `ldb_off16_handler_slot_check()` and `check_selfhost_min_ldb_off16_handler()` verify the same selector and bytes.
- JS-actual verified directly via `scripts/_probe/js-ty2text.mjs` (23B exact match); Rust-actual verified via the new `ldb_off16_handler_slot_check` (23B exact match).
- JS golden: **35/35 PASS** (was 34/34, +1 for LDB-OFF16-HANDLER).
- Rust self-test: **PASS**; Rust golden: **42/42 PASS** (was 41/41, +1 for G-SM-LDB-OFF16-HANDLER).
- Full canonical emit: JS=Rust=**1250B** code (was 1227B; +23B for H_44); byte-equal **Y** (M1.exe == M_rust.exe, sha256 `7b64c990a7d7a9898be16d87e589482230b2bf8c00145f64c28f5caf4e6b6c53`).
- 2-chain DDC: **EQUAL**, 1536B compared, hash `24f6b1bbe2c81d3eed41b72fb8686a1883cd016fb0b7cdd24ef342953a6d584d`.
- Lock verification: PASS at `ce0ceff131dc4e07483919f0f3744698d2a09d2516d4d2ca1452930672b79a2f`; previous pin chained to `cf2324f2fdcc732fce980b05398ccdb53ce088e8922a44b52c1de64e7038a9ed`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: H_44 LDB oo=0x10 at selector 0x32.
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: canonical handler appended (H_43 template mirrored).
4. Selftest: exact 23B and imm8-path assertions PASS (JS LDB-OFF16-HANDLER + Rust ldb_off16_handler_slot_check + check_selfhost_min_ldb_off16_handler).
5. Goldens: JS 35/35 and Rust 42/42 PASS.
6. Lock: `verify-yoyo-ty.mjs` PASS at `ce0ceff131dc4e07…`.
7. DDC: `verify-selfhost.ps1` EQUAL on 1536B.
8. Commit: none.

## 4. Next default

Queued body-extend-012: H_45 — pick the next LDB or other 3-arg coverage point; the symmetric imm8 path coverage at oo=0x10 closes another natural byte-side extension after H_37 (oo=0, 19B), H_40 (oo=8, 23B), H_41 (oo=127, 23B), H_42 (oo=0x50, 23B), H_43 (oo=0x40, 23B), H_44 (oo=0x10, 23B). The remaining imm8 byte-side coverage options at this selector density include: oo=0x20 (32), oo=0x60 (96), oo=0x70 (112), oo=1, oo=2, oo=4, oo=32, etc. Next default should examine another positive imm8 byte (e.g., oo=0x20=32) at a fresh selector to continue extending the LDB imm8-path byte-coverage matrix symmetrically.