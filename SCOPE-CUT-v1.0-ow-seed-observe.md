# OW-SEED observe — current facts (post-v1.0 path 2 · 关洞)

> **Status:** OBSERVE only · **disposition = CUT**  
> **Date:** 2026-09-29  
> **Gate:** `scripts/stage9-pure-m4.ps1` (Stage 9-C H_00 pure M4)  
> **Honest:** Rust `yoyo.exe` **still** emits seed — **OW-SEED remains CUT**.  
> **Not CLOSED.** Pinning emitter/seed hash ≠ non-Rust emit path.

## Purpose

Machine-verifiable snapshot of the seed/emitter surface as of 2026-09-29.
This is **detection bar evidence**, not a CLOSED claim. OW-SEED CLOSED
requires a non-Rust emit path (YOYO compiler self-bootstrap) — see
`SCOPE-CUT-v1.0-hole-inventory.md`.

## Stage 9-C result (this run)

```text
H_00 chain gen1→gen4: GREEN
gen4 ≡ gen3_direct (.text DDC): EQUAL
bootstrap --selfhost: NOT USED
Stage 9-C: may check [x]
```

| Step | Result | Bytes |
|------|--------|-------|
| seed gen1 via `yoyo link` | GREEN | 251392 |
| gen1→gen2 (H_00 zero-arg) | GREEN | 251392 |
| gen2→gen3 (H_00 zero-arg) | GREEN | 251392 |
| gen3→gen4 (H_00 zero-arg) | GREEN | 251392 |
| gen3_direct via `bootstrap` (no `--selfhost`) | GREEN | 251392 |
| gen4 vs gen3_direct `.text` DDC | EQUAL | 20992 compared |
| gen3 vs gen4 `.text` DDC | EQUAL | 20992 compared |

## OW-SEED pin (fail-closed observe)

| Field | Value |
|-------|-------|
| **emitter** | `yoyo-rust/target/release/yoyo.exe` |
| **emitter bytes** | `22292992` |
| **emitter sha256_prefix** | `52f0a813b354a0f5` |
| **emitter sha256 full** | `52f0a813b354a0f50d7581bbfde35a43af44ddf65ba9771821028adc85f2633e` |
| **emitter LastWriteTime** | 2026-09-14 07:59:05 |
| **seed** | `scripts/_stage9-pure-m4/gen1.exe` |
| **seed bytes** | `251392` |
| **seed sha256_prefix** | `b0a8dbb0d3133e2a` |
| **seed sha256 full** | `b0a8dbb0d3133e2a7f763fe79af19975bb2f2f505b0fed1d625d0e8b8f793a74` |
| **SEED_HOST path** | `h00` |
| **SEED_HOST observe** | `SEED_HOST cmd=link target=win32 path=h00 bytes=251392 dll_embed=- sha256_prefix=b0a8dbb0d3133e2a` |
| **SEED_HOST ≡ on-disk seed** | YES (bytes + sha256_prefix match) |
| **gen4 .text sha256_prefix** | `1ec3766f` |
| **file sha256_prefix (gen4 ≡ gen3_direct)** | `b0a8dbb0` |
| **rustc** | `1.98.1 (48a229cea 2026-09-01)` |
| **bootstrap --selfhost** | NOT USED |

## What this proves

1. **H_00 pure M4 algebra works** — gen1→gen4 without host `--selfhost`.
2. **gen4 ≡ gen3_direct** on `.text` (DDC EQUAL) — M4 algebra inside YOYO PEs matches the seed/link host path.
3. **Emitter + seed are machine-pinned** — size + sha256_prefix fail-closed (stage13/15 inventory contract).

## What this does **not** prove (honest CUT)

1. Seed is still emitted by **Rust** `yoyo.exe` → **OW-SEED CUT**.
2. Each genN still embeds **Rust** `yoyo_runtime.dll` → **OW-RT CUT**.
3. `yoyo.ty` (3326 lines) is IR/handler orchestration — **not** a full YOYO-written compiler source.
4. Closing OW-SEED requires a **non-Rust emit path** (YOYO compiler self-bootstrap) — multi-month long pole; not this observe tick.

## Fixed-point extension (2026-09-30)

YOYO-built PE as emitter (not Rust `yoyo.exe`):

```text
gen4 (H_00 YOYO PE) → gen5
gen5 ≡ gen4  (.text DDC EQUAL + full-file sha256 EQUAL)
disposition = CUT  (gen1 seed still from Rust yoyo.exe)
```

| Field | Value |
|-------|-------|
| **emitter this step** | `scripts/_stage9-pure-m4/gen4.exe` (YOYO PE · not Rust) |
| **output** | `scripts/_stage9-pure-m4/gen5.exe` |
| **gen5 bytes** | `251392` |
| **gen4 sha256_prefix** | `b0a8dbb0d3133e2a` |
| **gen5 sha256_prefix** | `b0a8dbb0d3133e2a` |
| **full-file EQUAL** | YES |
| **`.text` DDC** | EQUAL · compared_bytes=`20992` · hash=`1ec3766f…` |

**Proves:** once past the Rust seed, the YOYO H_00 PE is a **stable fixed-point emitter** (genN→genN+1 ≡ genN).

**Does not prove:** OW-SEED CLOSED — the *first* seed (`gen1`) is still `yoyo link` from Rust `yoyo.exe`. Fixed-point after seed ≠ replacing the seed emitter.

### Repro (fixed-point)

```powershell
cd F:\yoyo\scripts\_stage9-pure-m4
# after stage9-pure-m4.ps1 left gen4.exe + input.tyb/input.ky
& .\gen4.exe
Copy-Item -Force output.exe gen5.exe
& F:\yoyo\yoyo-rust\target\release\yoyo.exe diff gen5.exe gen4.exe
# expect DDC: EQUAL; file hashes equal
```

## Repro

```powershell
cd F:\yoyo
& .\scripts\stage9-pure-m4.ps1
# Then pin:
$Yoyo = "F:\yoyo\yoyo-rust\target\release\yoyo.exe"
$Seed = "F:\yoyo\scripts\_stage9-pure-m4\gen1.exe"
(Get-Item $Yoyo).Length
(Get-FileHash $Yoyo -Algorithm SHA256).Hash.ToLowerInvariant()
(Get-Item $Seed).Length
(Get-FileHash $Seed -Algorithm SHA256).Hash.ToLowerInvariant()
```

## Related

- `SCOPE-CUT-v1.0-hole-inventory.md` — OW-SEED CLOSED criteria
- `scripts/stage13-link-host.ps1` — seed/link host contract + OW-SEED pin
- `scripts/stage9-pure-m4.ps1` — H_00 pure M4 gate (this run)
- `POST-1.0-HOLE-CHECKLIST.md` — path 2 关洞看板

---

*Post-v1.0 path 2 · OW-SEED observe only · disposition=CUT · 2026-09-29*
