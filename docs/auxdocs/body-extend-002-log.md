# body-extend-002 Log · H_2F 0x61 SUB slot imm (EXPERIMENTAL · NOT GREEN)

> Tag: `body-extend-002-EXPERIMENTAL-SUB-IMM-H_2F` · 2026-07-25 (UTC+8)
> W-START: **EXPERIMENTAL · NON-GREEN** · Ref: `selfhost-start-node.md`,
> N1–N5b logs, body-extend-001 log.
> Status: **EXPERIMENTAL · single hand-extension beat · forward progress only**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠ gen1≡gen2)

## 1. Handler chosen — 0x61 SUB slot imm at H_2F
2-arg opcode, imm8 path (3 ∈ [-128, 127]), smallest clean surface; no
D-1/D-2/D-3/D-4 aliasing; both peers already implement `emit_sub_imm`
(Rust) / `encodeOp(0x61, ...)` (JS) via the same x86-64 primitives
(`loadState` + `subImmRax` + `storeState`). Rejected: 0x64 MOVRR
(D-2 alias); 0x63 IMUL / 0x6A SUBV (multi-slot, larger surface);
0x62 ADD imm (already H_2E body-extend-001, would be duplicate).

## 2. Derivation of expected bytes (independent, two paths)
**JS** (`encode-x64.js` `encodeOp(0x61, [0x50, 3])` + `0xC3`):
`loadState(0x50,rax)`→`49 8b 87 80 02 00 00`(7B) +
`subImmRax(3)`→`48 83 e8 03`(4B, imm8) +
`storeState(0x50,rax)`→`49 89 87 80 02 00 00`(7B) + `c3` = **19B**.
**Rust** (`assembler.rs` `emit_sub_imm(0x50, 3) + ret`): same primitives
→ same 19B. Pin: `498b87800200004883e80349898780020000c3`.
Differs from H_2E (ADD imm8: `48 83 c0 03`) ONLY at ModRM: `/5` (0xe8)
for SUB vs `/0` (0xc0) for ADD — same REX + 0x83 opcode.
Empirically: JS `derive_sub_imm.js` and Rust `derive_sub_imm.rs` both
emit the same 19B (Rust helper ≡ manual ≡ JS path). H_23 JS↔Rust
byte-equal confirmed via `checkSUBIMM` + `check_selfhost_min_sub_imm` ✓.

## 3. Files touched (only the lock-respected surface + new artifacts)
| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +21 lines (H_2F + comment); no existing line modified; **36→37 handlers, 427→448 lines** |
| `yoyo/tests/golden/selfhost_min_sub_imm.ty` | new fixture (1-handler probe) |
| `yoyo/tests/golden/expected/selfhost_min_sub_imm.code.hex` | new 19B pin |
| `yoyo/tests/yoyo.ty.lock` | pin updated; `previous_sha256` added (anti-rewrite chain) |
| `yoyo-js/scripts/golden.js` | +`checkSUBIMM()` (mirror of `checkADDIMM`); +1 entry in `cases` |
| `yoyo-rust/verifier/src/self_test.rs` | +`sub_imm_slot_check()` in `run_self_test` |
| `yoyo-rust/verifier/src/main.rs` | +`check_selfhost_min_sub_imm()` + runner entry (dispatch only; no encoder/emit source touched) |
| `scripts/_probe/derive_sub_imm.{js,rs}` | new probes (audit-only) |
| `docs/auxdocs/body-extend-002-log.md` | this log |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no `yoyo.asm`, no peer
trusted source outside the lock-respected surface, no existing
`expected/*.code.hex` modified.

## 4. JS + Rust reflector results
| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **20/20 PASS** (was 19/19) — `SUB-IMM PASS slot=0x50 imm=3 code=498b87800200004883e80349898780020000c3` |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** — `sub_imm_slot_check` (19B length, sig `48 83 e8 03`, `load_state` start, `store_state` before `ret`) |
| `cargo run -p verifier --bin yoyo -- test golden` | **27/27 PASS** (was 26/26) — `G-SM-SUB-IMM PASS: M_rust read yoyo.ty H_23 and emitted 498b87800200004883e80349898780020000c3 via opcode sub_imm+FF` |
| H_23 JS↔Rust byte-equal | `498b87800200004883e80349898780020000c3` ✓ (19B) |
| full `yoyo.ty` JS↔Rust byte-equal | **969B code section / 231424B PE32+**, SHA-256 `85a4d7fc45002d86c8657102ddbe95b7adb756276919580ca8ac3774944a8a0c` — 2-chain DDC EQUAL |
| 2-chain DDC text compare (verify-selfhost.ps1) | `hash_a=hash_b=bcc5231bdd115afa9528083ffea4cd311833444d5faaad20807fab6c9c7d9715` (1024B compared) |

## 5. Lock Protocol result (8-step trace, all PASS)
| step | outcome |
|---|---|
| 1. Pick | 0x61 SUB slot imm at H_2F (§1) |
| 2. Encoder | no fix; both peers' emit_sub_imm / subImmRax already in tree (PROMPT §4.3.2 + §4S.3.1 encoder rule) |
| 3. Hand-author | H_2F added at yoyo.ty line 446–448 (mirror of H_2E template) |
| 4. selftest | `sub_imm_slot_check` PASS (19B shape, sig `48 83 e8 03`) |
| 5. Goldens | Rust 27/27 + JS 20/20 + H_23 byte-equal + full yoyo.ty byte-equal (969B) |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`b0219ed108093ebe…`) |
| 7. `verify-selfhost.ps1` | 2-chain DDC EQUAL (`bcc5231b…`, 1024B compared) |
| 8. git commit | none (W-START convention) |

## 6. New pin
```
old: ca9ed6199169317605b0826d66ea4ad9d8422ad46134f3bd98523dff1963a309
new: b0219ed108093ebe3046dd28bce4fb3465e0988e99cfe5fe2a9a0983661d0355
```
Recorded in `yoyo/tests/yoyo.ty.lock` as `sha256`; old as
`previous_sha256` (anti-rewrite). W-START row in `PROMPT-v3.md` line 73
stays **EXPERIMENTAL**; pin change ≠ attempt promoted to GREEN.

## 7. EXPERIMENTAL scope reaffirmation
Single hand-extension beat (1 handler, 1 golden, 2 reflectors, 1 Relock).
**Forward progress only** — not a self-host GREEN claim. W-START row
unchanged; W-START red-list unchanged; no 3-chain DDC, no full-body, no
G06, no Phase 2, no freeze, no gen1≡gen2; stub still uses RAW_BYTE for
H_05..H_10 chain handlers.

## 8. What's still RED (verbatim from `PROMPT-v3.md`)
```
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI
```
No item promoted to GREEN. **handler count: 36/408 → 37/410** (W-START
framing: 36→37 handlers; file grew 408→410 lines weighted by W-START
method, or 427→448 actual lines including H_2F comment per file growth).

## 9. Honesty override checks
- Peer JS/Rust divergence at this handler: **NONE** (same 19B).
- Lock Protocol step 1 (compile) failure: **NONE**.
- No "self-host closer" / "self-host GREEN" claim — single hand-extension,
  not a step change in self-host coverage.
- No PROMPT edit. No version bump. No Week axis row added.

## 10. Next-step suggestion
**H_30 `0x64 MOVRR dst src`** (D-2: forces explicit semantic or 0x60/0x64
split; or defer to Phase 2 cleanup). Or any other 2-arg imm/2-slot
arithmetic that mirrors 001+002 templates cleanly. Each = 1 Relock
following this template. Relock overhead ~30s.
