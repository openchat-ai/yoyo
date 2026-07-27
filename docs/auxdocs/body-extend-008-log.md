# body-extend-008 Log · H_41 0x80 LDB oo=127 imm8 right edge

> Tag: `body-extend-008-EXPERIMENTAL-LDB-OFF127` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `0b4affcdbe7a2b22…` → `6c3e6e3bcaed158a…`.
> **handler count: 48 → 49** (+1 at selector 0x2F).

## 1. Pick rationale

Default from body-extend-007 §2.11: H_41 `0x80 LDB oo=127` exercises the signed-imm8 RIGHT edge; 127 is the largest value that must remain on `48 83 c0 7f`.

## 2. Execution record

- Hand-authored H_41 in `yoyo/projects/yoyo.ty`: `40 2F / 80 50 60 7F / FF`; not RAW_BYTE.
- Added handler fixture and independent 23B pin `498b87000300004883c07f480fb60049898780020000c3`.
- JS `checkLDBOFF127HANDLER()` verifies exact bytes, 23B length, imm8 signature, and absence of `48 81 c0 7f 00 00 00`.
- Rust `ldb_off127_handler_slot_check()` and `check_selfhost_min_ldb_off127_handler()` verify the same selector and bytes.
- JS golden: **32/32 PASS**.
- Rust self-test: **PASS**; Rust golden: **39/39 PASS**.
- Full canonical emit: JS=Rust=**1181B** code; byte-equal **Y**.
- 2-chain DDC: **EQUAL**, 1536B compared, hash `b05f9af6e6d1c97acdc1ef308ec6de6aaad0e534ce10752b01b85bebbcf6562a`.
- Lock verification: PASS at `6c3e6e3bcaed158afa4e8aeadd244b187a9050a5ee670718f306ca26964ce271`; previous pin chained to `0b4affcdbe7a2b22f809e70db2076af4b473eeef718bb34984a955899f1b01d5`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: H_41 LDB oo=127 at selector 0x2F.
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: canonical handler appended.
4. Selftest: exact 23B and imm8-boundary assertions PASS.
5. Goldens: JS 32/32 and Rust 39/39 PASS.
6. Lock: `verify-yoyo-ty.mjs` PASS.
7. DDC: `verify-selfhost.ps1` EQUAL on 1536B.
8. Commit: none.

## 4. Next default

Queued body-extend-009: H_42 `0x80 LDB oo=-128` at selector 0x30, the signed-imm8 LEFT edge, mirroring existing `selfhost_min_ldb_offm128.ty`.
