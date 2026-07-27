# body-extend-001 Log · H_2E 0x62 ADD slot imm (EXPERIMENTAL · NOT GREEN)

> Tag: `body-extend-001-EXPERIMENTAL-ADD-IMM-H_2E` · 2026-07-24 (UTC+8)
> W-START: **EXPERIMENTAL · NON-GREEN** · Ref: `selfhost-start-node.md`, N1–N5b logs.
> Status: **EXPERIMENTAL · single hand-extension beat · forward progress only**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠ gen1≡gen2)

## 1. Handler chosen — 0x62 ADD slot imm at H_2E
2-arg opcode, imm8 path (3 ∈ [-128, 127]), smallest clean surface; no
D-1/D-2/D-3/D-4 aliasing; both peers already implement `emit_add_imm`
(Rust) / `encodeOp(0x62, ...)` (JS) via the same x86-64 primitives
(`loadState` + `addImmRax` + `storeState`). Rejected: 0x61 SUB (next
beat); 0x64 MOVRR (D-2 alias); 0x63 IMUL / 0x6A SUBV / 0x68 ADDV /
0x69 ORV (multi-slot, larger surface).

## 2. Derivation of expected bytes (independent, two paths)
**JS** (`encode-x64.js` `encodeOp(0x62, [0x50, 3])`):
`loadState(0x50,rax)`→`49 8b 87 80 02 00 00`(7B) +
`addImmRax(3)`→`48 83 c0 03`(4B, imm8) +
`storeState(0x50,rax)`→`49 89 87 80 02 00 00`(7B) + `c3` = **19B**.
**Rust** (`assembler.rs` `emit_add_imm(0x50, 3) + ret`): same primitives
→ same 19B. Pin: `498b87800200004883c00349898780020000c3`.
Empirically: JS `js-ty2text.mjs` and Rust `link --target=stub` both
emit the same 19B. H_22 JS↔Rust byte-equal confirmed via
`scripts/_probe/body_ext_001_h22.mjs` ✓.

## 3. Files touched (only the lock-respected surface + new artifacts)
| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +21 lines (H_2E + comment); no existing line modified; **34→35 handlers, 408→427 lines** |
| `yoyo/tests/golden/selfhost_min_add_imm.ty` | new fixture (1-handler probe) |
| `yoyo/tests/golden/expected/selfhost_min_add_imm.code.hex` | new 19B pin |
| `yoyo/tests/yoyo.ty.lock` | pin updated; `previous_sha256` added (anti-rewrite chain) |
| `yoyo-js/scripts/golden.js` | +`checkADDIMM()` (mirror of `checkINC`); +1 entry in `cases` |
| `yoyo-rust/verifier/src/self_test.rs` | +`add_imm_slot_check()` in `run_self_test` |
| `yoyo-rust/verifier/src/main.rs` | +`check_selfhost_min_add_imm()` + runner entry (dispatch only; no encoder/emit source touched) |
| `scripts/_probe/body_ext_001_{ddc,h22}.mjs`, `derive_add_imm.js` | new probes (audit-only) |
| `docs/auxdocs/body-extend-001-log.md` | this log |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no `yoyo.asm`, no peer
trusted source outside the lock-respected surface, no existing
`expected/*.code.hex` modified.

## 4. JS + Rust reflector results
| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **19/19 PASS** (was 18/18) — `ADD-IMM PASS slot=0x50 imm=3 code=498b87800200004883c00349898780020000c3` |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** — `add_imm_slot_check` (19B length, sig `48 83 c0 03`, `load_state` start, `store_state` before `ret`) |
| `cargo run -p verifier --bin yoyo -- test golden` | **26/26 PASS** (was 25/25) — `G-SM-ADD-IMM PASS: M_rust read yoyo.ty H_22 and emitted 498b87800200004883c00349898780020000c3 via opcode add_imm+FF` |
| H_22 JS↔Rust byte-equal | `498b87800200004883c00349898780020000c3` ✓ (19B) |
| full `yoyo.ty` JS↔Rust byte-equal | **950B**, SHA-256 `9f538521ab5535ed920dc938e1730d7f18ad7470678c0bb510c6ec5a9cdd319b` — 2-chain DDC EQUAL |

## 5. Lock Protocol result (8-step trace, all PASS)
| step | outcome |
|---|---|
| 1. Pick | 0x62 ADD slot imm at H_2E (§1) |
| 2. Encoder | no fix; both peers' emit_add_imm / addImmRax already in tree |
| 3. Hand-author | H_2E added at yoyo.ty line 425–428 |
| 4. selftest | `add_imm_slot_check` PASS (19B shape) |
| 5. Goldens | Rust 26/26 + JS 19/19 + H_22 byte-equal + full yoyo.ty byte-equal |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`ca9ed61991693176…`) |
| 7. `verify-selfhost.ps1` | DDC EQUAL (hash_a=hash_b=`ac31ae14…`, 1024B compared) |
| 8. git commit | none (W-START convention) |

## 6. New pin
```
old: b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685
new: ca9ed6199169317605b0826d66ea4ad9d8422ad46134f3bd98523dff1963a309
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
No item promoted to GREEN. **handler count: 35/406 → 36/408** (per W-START
framing: 35→36 handlers; file grew 408→427 lines including H_2E comment).

## 9. Honesty override checks
- Peer JS/Rust divergence at this handler: **NONE** (same 19B).
- Lock Protocol step 1 (compile) failure: **NONE**.
- No "self-host closer" / "self-host GREEN" claim — single hand-extension,
  not a step change in self-host coverage.

## 10. Next-step suggestion
**H_2F `0x61 SUB slot imm`** (mirror of H_2E with `sub_imm` primitive;
smallest next beat). Then **H_30 `0x64 MOVRR dst src`** (forces D-2
explicit semantic or 0x60/0x64 split). Each = 1 Relock following
this template. Relock overhead ~30s; cost is the careful hand-extension.
