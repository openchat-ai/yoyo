# body-extend-003 Log · H_30 0x64 MOVRR (EXPERIMENTAL · NOT GREEN)

> Tag: `body-extend-003-EXPERIMENTAL-MOVRR-H_30` · 2026-07-25 (UTC+8)
> W-START: **EXPERIMENTAL · NON-GREEN** · follows body-extend-002.
> Status: **EXPERIMENTAL · single hand-extension beat · forward progress only**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠ gen1≡gen2)

## 1. Handler chosen
H_30 = `0x64 MOVRR dst src`, the default next beat. PROMPT §1 ISA table records D-2: both peers currently alias MOVRR to GET. Peer paths exist: JS `encodeOp(0x64)` is `loadState(src)+storeState(dst)`; Rust `TirOp::Movrr` routes to `emit_get(dst,src)`. No divergence and no PROMPT normative change.

## 2. Independent derivation
JS `encodeOp(0x64, [0x50,0x51])` emits `49 8b 87 88 02 00 00` (load S[0x51]) + `49 89 87 80 02 00 00` (store S[0x50]) + `c3` = `498b878802000049898780020000c3` (15B). Rust `emit_get(0x50,0x51)+ret` emits the identical pin. Independent JS check confirmed byte equality.

## 3. Files touched
`yoyo/projects/yoyo.ty` appended H_30 at lines 456–463; `selfhost_min_movrr.ty`; `expected/selfhost_min_movrr.code.hex`; JS `golden.js` `checkMOVRR` + case; Rust `self_test.rs` `movrr_slot_check`; Rust `main.rs` golden entry; `yoyo.ty.lock`; this log. No PROMPT edit, no version bump, no commit.

## 4. Results
- JS `node yoyo-js/scripts/golden.js`: **21/21 PASS**; MOVRR pin exact.
- Rust `cargo test -p verifier --bin yoyo self_test_passes`: **PASS**.
- Rust `cargo run -p verifier --bin yoyo -- test golden`: **28/28 PASS**; G-SM-MOVRR exact.
- Full emit: JS and Rust both **984B code / 231424B PE**, byte-equal.
- 2-chain DDC: **EQUAL**, compared 1024B, hash `ba69996340cc0ed9d44ff2c50d71971d5a8b0642bddf7a4c873f861e41a3fa30`.

## 5. Lock Protocol / Relock (8 steps)
1. Pick: H_30 `0x64 MOVRR dst src`, D-2 alias verified in ISA table.
2. Encoder: no fix; JS and Rust peer paths already existed.
3. Hand-author: appended H_30; existing H_2F label retained at H_2F.
4. Selftest: `movrr_slot_check` PASS.
5. Goldens: JS 21/21, Rust 28/28, fixture and peer bytes equal.
6. `verify-yoyo-ty.mjs`: PASS after pin update.
7. `verify-selfhost.ps1`: 2-chain DDC EQUAL.
8. Git commit: none (W-START convention).

## 6. Pin chain
old: `b0219ed108093ebe3046dd28bce4fb3465e0988e99cfe5fe2a9a0983661d0355`
new: `1cbbdb4650b4babdaf9160b8259867143f3ec47a1148403ea27d0319b6d68603`

## 7. Scope and honesty
Handler count: **37/410 → 38/412**. W-START remains EXPERIMENTAL; no item promoted to GREEN. Still RED: full compiler self-host, 3-chain section-ddc, G06, Phase 2 exit, freeze compiler, M-morph, Phase 4c libyoyo, gen1≡gen2, CI. D-2 semantic cleanup remains deferred to Phase 2; no invented semantic split.

## 8. Next default beat
Next default = next simple non-aliased 1-arg or 2-arg handler per ISA; do not select another D-2 alias unless it is the clear queue default.
