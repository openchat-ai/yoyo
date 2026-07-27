# body-extend-005 Log · §1 Candidate List

## 1. Candidate list — pre-flight only

State confirmed from `body-extend-004-log.md`: H_31 is `0x69 ORV dst src`,
39/414, pin `c7426067edca2a2079e76e1132c4e272cab72b5a123b603c4a39d84d37db5be5`.
This record does not edit `yoyo.ty` or its lock and makes no GREEN claim.

Canonical core table: `PROMPT-v3.md` Part 4.1. Peer primitive support today:
- JS: `00,20,30,41,50,51,60,61,62,63,64,65,66,67,68,69,6A,70-7A,80,84,85,A0,A1,FF`.
- Rust: `00,10,12,13,20,30,41,50,51,60,61,62,63,64,65,66,67,68,69,6A,70-7A,80,84,85,A0,A1,FF`.
- Common primitive-supported surface is the JS list above.

Opcodes already exercised in current `yoyo.ty` bodies:
`20,30,41,50,51,60,61,62,64,65,66,67,68,69,70-7A,A0,FF`.
Excluded before ranking: D-1 `20/50/51`; D-2 `64`; D-3 `84/85`;
D-4 gen1≡gen2 / asm-parity work. Variadic `A1` is not ranked as fixed-arity.

Ranked picks (1-arg before 2-arg before 3-arg):
1. `0x00 NOP` → suggested `H_32_NOP` — 0-arg; smallest exact primitive (`90`).
2. `0xA1 RAW_BYTES` → suggested `H_33_RAW_BYTES` — 1-arg minimal case; direct byte emission.
3. `0x63 IMUL` → suggested `H_34_IMUL` — 2-arg; established load/load/ALU/store shape.
4. `0x6A SUBV` → suggested `H_35_SUBV` — 2-arg; same surface as ORV with `sub_reg`.
5. `0x65 CMP` → suggested `H_36_CMP` — 2-arg; load/load/compare, no store.
6. `0x80 LDB` → suggested `H_37_LDB` — 3-arg; both peers implement load-byte primitive path.
7. `0x30 SET` → suggested `H_38_SET` — 2-arg; already opcode-covered but retained only as a control candidate.
8. `0x60 GET` → suggested `H_39_GET` — 2-arg; already opcode-covered but retained only as a control candidate.

Honesty note: after exclusions and removing already-covered opcodes, the fixed/common fresh set
contains only `00,63,6A,80` plus variadic `A1`; therefore picks 7-8 are explicit covered
controls, not fresh coverage. No unsupported fresh opcode was invented to force eight.

## 3. Parallel sub-agent scratch results (§1 picks 2–8 minus held-back H_39)

> Tag: `body-extend-005-EXPERIMENTAL-batched-7` · 2026-07-25 (UTC+8).
> Following §2 H_32. W-START: **EXPERIMENTAL · NON-GREEN**.
> Dispatched in parallel via 7 background sub-agents, each writing its own
> scratch fixture (`.ty`), expected bytes (`.code.hex`), and a log fragment
> (`_log_hN.md`). This §3 collects the per-handler head-line results from
> those 7 sub-agent fragments (`docs/auxdocs/_log_h33.md`..`_log_h39.md`).

### §3.0 Consolidated per-handler table (from sub-agent return-dicts)

| pick | handler | name      | opcode | bytes             | byte-eq | sha256 (hand=js=rust)               | stop-if                       | result   |
|------|---------|-----------|--------|-------------------|---------|--------------------------------------|-------------------------------|----------|
| 2    | H_33    | RAW_BYTES | 0xA1   | `ccddc3` (3B)     | Y       | `995785db9410cf30…f467a71f916`       | NONE                          | PASS     |
| 3    | H_34    | IMUL      | 0x63   | `498b87…898780020000c3` (26B) | Y | `6776c1a32bb19d5f…fa3594ab`         | NONE                          | PASS     |
| 4    | H_35    | SUBV      | 0x6A   | `498b87…898780020000c3` (25B) | Y | `f50034410fe55a8e…41dedda22`         | NONE                          | PASS     |
| 5    | H_36    | CMP       | 0x65   | `498b87…48 39 c8 c3` (18B)   | Y | `db92cc2bb229a0ae…18925f3`           | NONE                          | PASS     |
| 6    | H_37    | LDB       | 0x80   | `498b87…49898780…c3` (19B)   | Y | `51ce6ddf2ccc294a…98acb95`           | NONE                          | PASS     |
| 7    | H_38    | SET       | 0x30   | `48b8…49898780…c3` (18B)     | Y | `196cd779c54c7701…746f8ae7c0`         | NONE                          | PASS     |
| 8    | H_39    | GET       | 0x60   | **REJECTED** — hand≠rust (hand=`488b87…c3` 15B, rust=`c3498b…c3`); JS driver `yoyo-js/scripts/js-ty2text.mjs` not present on disk | Y/N — N (rust+js missing) | hand=`8da78127aa0f2fa…` / rust=`0859fa62…` (different) | PRESCRIBED JS DRIVER MISSING + peer-source edit required | **REJECTED** |

### §3.1 H_33 RAW_BYTES (0xA1) — PASS

See `docs/auxdocs/_log_h33.md`. JS branch `args.map((x) => x & 0xff)`
and Rust arm `inst.args.iter().map(|v| *v as u8).collect()` both emit
literal byte args verbatim. Both peers route through the real variadic
literal-byte emit path — NOT RAW_BYTE 0xA0 filler.

### §3.2 H_34 IMUL (0x63) — PASS

See `docs/auxdocs/_log_h34.md`. JS: `loadState(0x50)+loadState(0x51,rcx)+mulRegRaxRcx+storeState(0x50)+ret`.
Rust: `emit_imul` via `0F AF`. Both produce identical 26B stream.
Pin: `498b8780020000498b8f88020000480fafc149898780020000c3`.

### §3.3 H_35 SUBV (0x6A) — PASS

See `docs/auxdocs/_log_h35.md`. JS: `loadState(0x50)+loadState(0x51,rcx)+subRegRaxRcx+storeState(0x50)+ret`.
Rust: `emit_subv` via `29 C8`. Both produce identical 25B stream.
Differs from IMUL only at byte 16: `48 29 C8` (sub rax,rcx) vs `48 0F AF C1` (imul rax,rcx).

### §3.4 H_36 CMP (0x65) — PASS

See `docs/auxdocs/_log_h36.md`. JS: `loadState(0x50)+loadState(0x51,rcx)+cmpRegRaxRcx+ret`.
Rust: `emit_cmp` via `39 C8`. Both produce identical 18B stream with NO store
(compare-only, no store — distinguish from MOVRR/ADDV/ORV/SUBV/IMUL).

### §3.5 H_37 LDB (0x80) — PASS

See `docs/auxdocs/_log_h37.md`. Both peers route through `load_state(0x60,rax)
+ movzx rax,byte[rax] + store_state(0x50,rax) + ret`. Pin 19B. Composition:
`498b8700030000480fb60049898780020000c3`.

### §3.6 H_38 SET (0x30) CONTROL — PASS

See `docs/auxdocs/_log_h38.md`. Already-opcode-covered control pick (H_00..H_04
exercise the same primitive path). The existing JS M0 / Rust emit route through
the canonical SET-shape: `movabs rax, 0x00 + store_state(S[0x50], rax) + ret` =
`48b8000000000000000049898780020000c3` (18B). No regression vs H_00..H_04.

### §3.7 H_39 GET (0x60) CONTROL — REJECTED

**H_39 is held back per dispatch rule (§5).** Two failed conditions:
- **STOP-IF #1**: Prescribed JS driver `yoyo-js/scripts/js-ty2text.mjs` was
  not present on disk in H_39's environment; the sub-agent could not obtain
  a JS-actual hex, so JS-actual is empty / unreachable.
- **STOP-IF #2**: hand-derived (`488b878802000048898780020000c3`, 15B) ≠ Rust
  emit (`c3498b878802000049898780020000c3`, 16B with leading `0xc3`).
  The leading `0xc3` in Rust output suggests a stub-emit ordering bug or a
  RET being injected before the body; this is a peer divergence that would
  require a Rust emit.rs edit to resolve — which the parent agent (this
  dispatch) is forbidden from making.

The other 6 handlers (H_33..H_38) all byte-equal across hand/JS/Rust with
no peer-source edits; they proceed to Phase C (consolidation) as §4 records.

## 4. Consolidated result (§1 picks 2–8 minus held-back H_39)

> Tag: `body-extend-005-CONSOLIDATION-6-of-8` · 2026-07-25 (UTC+8).
> Following §3 sub-agent PASS results for H_33..H_38. H_39 is held back
> per §5. W-START: **EXPERIMENTAL · NON-GREEN**.

### §4.1 Files touched (consolidated for the 6 surviving handlers)

| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +6 handlers (H_33..H_38), all at selectors `0x27..0x2C`. H_32 line preserved unchanged. Handlers H_33..H_38 mirror the existing body-extend-NNN templates. **40→46 handlers** (H_05..H_38 spanning 40 handlers; H_33..H_38 = 6 new) |
| `yoyo/tests/golden/selfhost_min_raw_bytes.ty` + `expected/.code.hex` | new fixture (3B pin `ccddc3`) |
| `yoyo/tests/golden/selfhost_min_imul.ty` + `expected/.code.hex` | new fixture (26B pin) |
| `yoyo/tests/golden/selfhost_min_subv.ty` + `expected/.code.hex` | new fixture (25B pin) |
| `yoyo/tests/golden/selfhost_min_cmp.ty` + `expected/.code.hex` | new fixture (18B pin) |
| `yoyo/tests/golden/selfhost_min_ldb_body.ty` + `expected/.code.hex` | new fixture (19B pin) |
| `yoyo/tests/golden/selfhost_min_set_control.ty` + `expected/.code.hex` | new fixture (18B pin — H_00-shape, no regression) |
| `yoyo/tests/yoyo.ty.lock` | sha256 advanced; `previous_sha256` advanced (anti-rewrite chain); §3 record left intact |
| `yoyo-js/scripts/golden.js` | +6 `checkX()` functions (`checkRAWBYTES`, `checkIMUL`, `checkSUBV`, `checkCMP`, `checkLDBBODY`, `checkSETCONTROL`); +6 entries in `cases`; +6 in summary line. **`23 → 29`** |
| `yoyo-rust/verifier/src/self_test.rs` | +6 `*_slot_check()` calls in `run_self_test` (`raw_bytes_slot_check`, `imul_slot_check`, `subv_slot_check`, `cmp_slot_check`, `ldb_body_slot_check`, `set_control_slot_check`). **`run_self_test` +6, all PASS** (`self_test_passes` unit test ok) |
| `yoyo-rust/verifier/src/main.rs` | +6 `check_selfhost_min_*()` functions + 6 dispatch entries in `cmd_test_golden`; +6 in help text and summary line. **`30 → 36`** |
| `docs/auxdocs/body-extend-005-log.md` | this §4 record + §3 records above; H_32 §2 unchanged; H_39 §5 below |
| `docs/auxdocs/_log_h33.md`..`_log_h38.md` | per-sub-agent fragments written by parallel sub-agents (read-only references for §3) |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no peer source edits,
no existing `expected/*.code.hex` modified, no handler at selectors
`0x22..0x26` modified.

### §4.2 JS + Rust reflector results (after consolidation)

| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **29/29 PASS** (was 23/23) — `RAW-BYTES`, `IMUL`, `SUBV`, `CMP`, `LDB-BODY`, `SET-CONTROL` all PASS |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** (run_self_test covers all 6 new slot checks: `raw_bytes`, `imul`, `subv`, `cmp`, `ldb_body`, `set_control`) |
| `cargo run -p verifier --bin yoyo -- test golden` | **36/36 PASS** (was 30/30) — `G-SM-RAW-BYTES`, `G-SM-IMUL`, `G-SM-SUBV`, `G-SM-CMP`, `G-SM-LDB-BODY`, `G-SM-SET-CONTROL` all PASS |
| H_33..H_38 JS↔Rust byte-equal | all 6 ✓ (3B, 26B, 25B, 18B, 19B, 18B = 109B total) |
| full `yoyo.ty` JS↔Rust byte-equal | **1536B code section / 231936B PE32+** (after +6 handler additions), SHA-256 of code `928f9853cb8e455012c7fa2b11946861ff83ce78b3d39c1e28165d6da1037074` — 2-chain DDC EQUAL (1536B compared) |
| 2-chain DDC text compare (verify-selfhost.ps1) | `hash_a=hash_b=928f9853cb8e455012c7fa2b11946861ff83ce78b3d39c1e28165d6da1037074` (1536B compared) |

### §4.3 Lock Protocol result (8-step trace, all PASS)

| step | outcome |
|---|---|
| 1. Pick | 6 picks: `0xA1 RAW_BYTES` (H_33), `0x63 IMUL` (H_34), `0x6A SUBV` (H_35), `0x65 CMP` (H_36), `0x80 LDB` (H_37), `0x30 SET-CONTROL` (H_38) per §1 picks 2–7 (picks 2–8 minus held-back pick 8 H_39 per §5) |
| 2. Encoder | no fix on 6 of 6; both peers' `encodeOp(0xA1)`, `encodeOp(0x63)`, `emit_imul`, `encodeOp(0x6A)`, `emit_subv`, `encodeOp(0x65)`, `emit_cmp`, `encodeOp(0x80)` (load+movzx+store), `encodeOp(0x30)` (movabs+store) already in tree |
| 3. Hand-author | H_33..H_38 added at `yoyo.ty` end (mirror existing body-extend-001..004 templates) |
| 4. selftest | `run_self_test` PASS (all 6 new `*_slot_check` pass; `self_test_passes` unit ok) |
| 5. Goldens | Rust 36/36 + JS 29/29 + 6 handlers byte-equal (hand=js=rust) + full canonical `yoyo.ty` byte-equal (1536B code section, 231936B PE32+) |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`35f77232f06fedc3…`) |
| 7. `verify-selfhost.ps1` | 2-chain DDC EQUAL (`928f9853…`, 1536B compared); lockdown PASS |
| 8. git commit | none (W-START convention) |

### §4.4 New pin

```
old: 4b055271a2c5eca22858b8b5725d88ea309c33c9838885811a5546d530bc9283
new: 35f77232f06fedc34ec3c8b90d3288c046a001fedfab9cc9e711687df9102ec6
```

Recorded in `yoyo/tests/yoyo.ty.lock` as `sha256`; old as
`previous_sha256` (anti-rewrite). W-START row in `PROMPT-v3.md` line 73
stays **EXPERIMENTAL**; pin change ≠ attempt promoted to GREEN.

### §4.5 EXPERIMENTAL scope reaffirmation

6 hand-extension beats (6 handlers, 6 goldens, 2 reflectors, 1 Relock).
**Forward progress only** — not a self-host GREEN claim. W-START row
unchanged; W-START red-list unchanged; no 3-chain DDC, no full-body, no
G06, no Phase 2, no freeze, no gen1≡gen2.

### §4.6 What's still RED (verbatim from `PROMPT-v3.md`)

```
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI
```

No item promoted to GREEN. **handler count: 40/416 → 46/422** (W-START
framing: +6 handlers, +12 lines weighted by W-START method).

### §4.7 Honesty override checks

- Peer JS/Rust divergence at these 6 handlers: **NONE** (same bytes).
- Peer JS/Rust divergence at H_39: **YES** (see §5); H_39 held back.
- Lock Protocol step 1 (compile) failure: **NONE** on the 6 surviving.
- No "self-host closer" / "self-host GREEN" claim — 6 hand-extensions,
  not a step change in self-host coverage.
- No PROMPT edit. No version bump. No Week axis row added.
- No `git commit` performed (W-START convention).

## 5. H_39 0x60 GET — REJECTED (peer divergence + emit missing)

> Tag: `body-extend-005-EXPERIMENTAL-H_39-REJECTED` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.

### §5.1 Handler chosen — 0x60 GET at H_39 (per §1 pick 8)

2-arg control candidate; already opcode-covered in `yoyo.ty` H_01/H_04
(`60 50 51`). This beat was supposed to confirm no regression at the new
selector `0x2D`. Both peers' `encodeOp(0x60, [dst, src])` paths were
expected to produce identical bytes via `load_state(src) + store_state(dst) + ret`.

### §5.2 Rejection conditions fired (both)

The parent (this dispatch) is fail-closed: BOTH conditions below trigger
H_39 rejection and §1 pick 8 is held back. The other 6 picks still
proceed to §4 consolidation.

**Condition #1 — prescribed JS driver missing on disk:** The H_39
sub-agent could not produce a JS-actual hex because the prescribed
driver `yoyo-js/scripts/js-ty2text.mjs` was not present in the
sub-agent's tool access. Stop-IF #1 of the dispatch contract is
"prescribed JS driver missing" → REJECTED.

**Condition #2 — hand ≠ rust (peer divergence):** H_39's hand-derived
bytes for `60 50 51` were `488b878802000048898780020000c3` (15B:
`load_state(0x51, rax) + store_state(0x50, rax) + ret`). The Rust
emit produced `c3498b878802000049898780020000c3` (16B with a leading
`0xc3` byte before the body). Stop-IF #2 is "hand ≠ rust (peer-source
edit required)" → REJECTED. The leading `0xc3` suggests a stub-emit
ordering bug — but per dispatch rules the parent agent is forbidden
from editing `yoyo-rust/verifier/src/emit.rs`; this would require
peer-source work that is OUT OF SCOPE for this consolidation.

### §5.3 Byte-level diff (for the audit trail)

```
hand_derived: 488b878802000048898780020000c3  (15B)
js_actual:    <unobtainable — driver missing>
rust_actual:  c3 498b878802000049898780020000c3  (16B — leading c3 + 15B body)
hand_sha256:  8da78127aa0f2fa7855da11f9bebc3a7a1f13f5d825a9414fdc09feb669fe2e2
rust_sha256:  0859fa620047c3ae24b57e5c351169af62650b6b654d2b6892bf18d8d696484d
```

The `c3` prefix in `rust_actual` is OUT-OF-ORDER relative to the body —
this could indicate a stub-emit (PlatformKind::Stub) emitting a RET
before the op body, OR a doubled-`c3` artifact from `lower_op_checked`
double-injection. Without the JS driver it cannot be triaged in this
batch.

### §5.4 What is NOT rejected

Note that `encode-x64.js` line 73 explicitly defines `0x60` (GET) as
`[...loadState(a(1), 0, 0), ...storeState(a(0), 0, 0)]` — a known-good
JS shape. AND `assembler::emit_get(0x50, 0x51)` returns a known-good
15B Rust pin (used in `movrr_slot_check`). Both individually pass.
The `c3`-prefixed corruption is therefore downstream (in
`emit::emit` or `Executor::compile_one_handler` or `lower_op_checked`
for selector 0x60 with state-slot args) and not in the per-opcode
primitive paths.

### §5.5 Next-step suggestion

H_39 should be retried in a future beat once:
(a) the JS driver artifact is added to disk (it doesn't currently
exist at `yoyo-js/scripts/js-ty2text.mjs`); AND
(b) the leading-`c3` Rust artifact is investigated by inspection of
`emit::emit_one` / `Executor::compile_one_handler` for selector
`0x60` with state-slot args — outside the lock-respected surface, so
this is deferred.

Until then, `yoyo.ty` stays at handler H_38 (selector 0x2C). H_39
does NOT advance the lock pin beyond §4.

---

## 2. H_32 0x00 NOP — execution record (this handler only)

> Tag: `body-extend-005-EXPERIMENTAL-NOP-H_32` · 2026-07-25 (UTC+8)
> W-START: **EXPERIMENTAL · NON-GREEN** · follows body-extend-004 H_31.
> Status: **EXPERIMENTAL · single hand-extension beat · forward progress only**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠ gen1≡gen2)

### 2.1 Handler chosen — 0x00 NOP at H_32 (per §1 pick 1)

Zero-argument, exact-primitive (`90`) control surface. Both peers already
implement the NOP compile path: `encodeOp(0x00) → [0x90]` (JS) and
`TirOp::Nop → Ok(vec![0x90])` (Rust). No new peer source required (the
peer surface was already listing `00` as primitive-supported in §1). The
fixture pin is independently derived: `90` (NOP) + `c3` (RET) = `90c3`,
2B. This is the smallest exact-primitive beat in the ranked §1 list and
intentionally precedes the variadic/1-arg/2-arg/3-arg picks so that the
later beats can be compared against an uncontaminated NOP control.

### 2.2 Independent derivation of expected bytes (2B)

**JS** (`encodeOp(0x00, [])` + `0xC3`):
- `encodeOp(0x00, [])` returns `[0x90]` (1B).
- `0xC3` → `c3` (1B).
- Total: **2B**, hex `90c3`.

**Rust** (`TirOp::Nop` + `ret()`):
- `emit_one` on `TirOp::Nop` returns `Ok(vec![0x90])` (1B).
- `assembler::ret()` returns `[0xC3]` (1B).
- Total: **2B**, same hex `90c3`.

Empirically confirmed byte-equal via:
- `node yoyo-js/scripts/golden.js` reports `NOP PASS — zero-arg code=90c3`.
- `cargo run -p verifier --bin yoyo -- test golden` reports
  `G-SM-NOP PASS — M_rust read yoyo.ty H_26 and emitted 90c3 via opcode nop+FF`.
- Both paths route through the real NOP compile path; the fixture is
  NOT a RAW_BYTE filler (H_05..H_10 are RAW_BYTE and remain in that
  category; H_32 is the first non-RAW_BYTE H_2x+ control and exercises
  the zero-arg `0x00` opcode route directly).

### 2.3 Files touched (only the lock-respected surface + new artifacts)

| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +6 lines (H_32 label def, `00` op, `FF` ret, 3-line comment); no existing line modified; **39→40 handlers** |
| `yoyo/tests/golden/selfhost_min_nop.ty` | new fixture (1-handler probe, 2 op lines) |
| `yoyo/tests/golden/expected/selfhost_min_nop.code.hex` | new 2B pin `90c3` |
| `yoyo/tests/yoyo.ty.lock` | pin updated; `previous_sha256` advanced (anti-rewrite chain) |
| `yoyo-js/scripts/golden.js` | +`checkNOP()` (mirror of `checkORV` template, zero-arg); +1 entry in `cases`; summary line updated |
| `yoyo-rust/verifier/src/self_test.rs` | +`nop_slot_check()` in `run_self_test` (pin `90c3`, length 2B) |
| `yoyo-rust/verifier/src/main.rs` | +`check_selfhost_min_nop()` + runner entry + help text + summary line (dispatch only; no encoder/emit source touched) |
| `docs/auxdocs/body-extend-005-log.md` | this §2 record (single sub-section; §1 spec left intact as the plan) |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no `yoyo.asm`, no peer
trusted source outside the lock-respected surface, no existing
`expected/*.code.hex` modified.

### 2.4 JS + Rust reflector results

| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **23/23 PASS** (was 22/22) — `NOP PASS — zero-arg code=90c3` |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** — `nop_slot_check` (`90c3` length 2B, exact `[0x90, 0xC3]` match) |
| `cargo run -p verifier --bin yoyo -- test golden` | **30/30 PASS** (was 29/29) — `G-SM-NOP PASS — M_rust read yoyo.ty H_26 and emitted 90c3 via opcode nop+FF` |
| H_32 JS↔Rust byte-equal | `90c3` ✓ (2B) |
| full `yoyo.ty` JS↔Rust byte-equal | **1011B code section / 231424B PE32+**, SHA-256 of code `e17cd5274f0e1a542ab1acd3dbb7d62b2665df313b9924631d4e8a8dfd00b747` — 2-chain DDC EQUAL (1024B compared) |
| 2-chain DDC text compare (verify-selfhost.ps1) | `hash_a=hash_b=e17cd5274f0e1a542ab1acd3dbb7d62b2665df313b9924631d4e8a8dfd00b747` (1024B compared) |

### 2.5 Lock Protocol result (8-step trace, all PASS)

| step | outcome |
|---|---|
| 1. Pick | `0x00 NOP` at H_32 (§1 pick 1) |
| 2. Encoder | no fix; both peers' `encodeOp(0x00, [])` and `TirOp::Nop` already in tree |
| 3. Hand-author | H_32 added at `yoyo.ty` end (mirrors H_31 template: label def + opcode + RET) |
| 4. selftest | `nop_slot_check` PASS (`90c3` exact 2B match) |
| 5. Goldens | Rust 30/30 + JS 23/23 + H_26 byte-equal + full `yoyo.ty` byte-equal (1011B) |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`4b055271a2c5eca2…`) |
| 7. `verify-selfhost.ps1` | 2-chain DDC EQUAL (`e17cd527…`, 1024B compared); lockdown PASS |
| 8. git commit | none (W-START convention) |

### 2.6 New pin

```
old: c7426067edca2a2079e76e1132c4e272cab72b5a123b603c4a39d84d37db5be5
new: 4b055271a2c5eca22858b8b5725d88ea309c33c9838885811a5546d530bc9283
```

Recorded in `yoyo/tests/yoyo.ty.lock` as `sha256`; old as
`previous_sha256` (anti-rewrite). W-START row in `PROMPT-v3.md` line 73
stays **EXPERIMENTAL**; pin change ≠ attempt promoted to GREEN.

### 2.7 EXPERIMENTAL scope reaffirmation

Single hand-extension beat (1 handler, 1 golden, 2 reflectors, 1 Relock).
**Forward progress only** — not a self-host GREEN claim. W-START row
unchanged; W-START red-list unchanged; no 3-chain DDC, no full-body, no
G06, no Phase 2, no freeze, no gen1≡gen2; stub still uses RAW_BYTE for
H_05..H_10 chain handlers.

### 2.8 What's still RED (verbatim from `PROMPT-v3.md`)

```
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI
```

No item promoted to GREEN. **handler count: 39/414 → 40/416** (W-START
framing: +1 handler, +2 lines weighted by W-START method).

### 2.9 Honesty override checks

- Peer JS/Rust divergence at this handler: **NONE** (same `90c3`).
- Lock Protocol step 1 (compile) failure: **NONE**.
- No "self-host closer" / "self-host GREEN" claim — single hand-extension,
  not a step change in self-host coverage.
- No PROMPT edit. No version bump. No Week axis row added.
- Audit defect: NOP is a zero-arg primitive; the existing peer surface
  already advertises `00` in both lists (§1). The fixture exercises the
  real compile path; it is not RAW_BYTE filler (H_32 uses `0x00` opcode,
  not `0xA0`).

### 2.10 Next-step suggestion

**H_33 `0xA1 RAW_BYTES`** (1-arg variadic; direct byte emission).
Already present in both peer primitive lists. After that: **H_34 `0x63
IMUL`**, then **H_35 `0x6A SUBV`**, then **H_36 `0x65 CMP`**, then
**H_37 `0x80 LDB`**, then the two explicit covered controls **H_38
`0x30 SET`** and **H_39 `0x60 GET`** (per §1 ranked picks 2–8). Each = 1
Relock following this template. Relock overhead ~30s.
