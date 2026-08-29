# SCOPE-CUT — v1.0 hole inventory（Stage 16-A 定稿 · post-v1.0 OW-RT shrink）

> **Status:** FINAL（v1.0 终态收口 · 可机器验收）  
> **Gate:** `scripts/stage16-scope-cut-finalize.ps1`（alias `stage16-a.ps1`）  
> **Rule:** 本文件是 **诚实定稿**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL；不得把仍 CUT 的洞标成 CLOSED。

**Upstream:** `SCOPE-CUT-v0.9-hole-inventory.md`（Stage 15-A ACTIVE 枚举）仍有效；本文件把同一七项 **提升为 v1.0 FINAL disposition**（能关则 CLOSED+证据；不能关则 CUT 钉进 RELEASE）。

**Post-v1.0 path 2（关洞）· OW-RT shrink（2026-08-29）：** Win H_00 seed/link **不再 exact-embed** `yoyo_runtime.dll`；改为 cwd sidecar `yoyo_rt.dll` + `LoadLibraryA`。DLL 观测 **141312**（LTO 后）。**仍 CUT**（Rust runtime 宿主信任未灭）— **禁止**把「无 embed」单独标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-RT Linux sidecar parity（2026-08-29）：** Linux H_00 **不再 exact-embed** `libyoyo_runtime.so`；仅嵌 trampoline，cwd sidecar `./libyoyo_runtime.so` + `dlopen`。seed ELF **253952**（MAX **300000**；was ~512000）。**仍 CUT**（Rust `.so` + glibc/libdl tramp）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-STUB shrink（2026-08-29）：** H_00 stub **235→96** B：去掉 export 名字符串 + **ordinal-0** 解析（`yoyo_runtime` 钉 `yoyo_runtime_selfhost_main` 为首个 named export；不再 in-stub strcmp walk）。gen12 `90ad6d6e` / **17920** B；seed PE **248320**。**仍 CUT**（Rust-only stub；窗外）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT shrink（2026-08-29）：** H_00 宿主 IAT **去掉 GetProcAddress**（host-loader **3→2**：LoadLibraryA + ExitProcess）。LoadLibraryA 之后 in-process PE export resolve（ordinal-0；was full name walk）。**仍 CUT**（LoadLibraryA / libdl 仍在）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· deeper OW-IAT（2026-08-29）：** H_00 **再去掉 IAT/ASCII `LoadLibraryA`** — PEB→kernel32 **ROR13** 导出哈希解析后调用；ordinal-0 导出仍保留。host-loader IAT 面仅 **ExitProcess**。**仍 CUT**（仍宿主 LoadLibrary）— **禁止**因「无 LoadLibraryA 字符串」标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT Linux hybrid tramp（2026-08-29）：** `linux_h00_tramp.elf` **dynamic -lc only**（no libdl NEEDED）；sidecar via **dlopen@PLT** + sym walk（no dlsym）；**no glibc/ld disk mmap**（fixes CI IRELATIVE/TLS/RELR）。gen4≡gen3_direct **EQUAL**。**仍 CUT**（dlopen + ld.so libc + cwd sidecar `.so`）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT manual-map wire-up（2026-08-29 · PR #8）：** H_00 stub **905B** manual-map（CreateFile/Read/VirtualAlloc + `pe_manual_map`）；PEB `LoadLibraryA` **DROPPED**；JS `h00-manual-map-stub.hex` lockstep。**仍 CUT**（sidecar `yoyo_rt.dll` + kernel32 I/O）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-H00 CLOSED（JS IAT sync）：** JS `KERNEL32_IO_FUNCS` dropped stale `LoadLibraryA`（7→6，对齐 Rust `pe_link.rs`）；`tempNameRva` lea disp32 对齐。**three_peer_full=EQUAL** · **`72c27c9f`** / **18944** B。**OW-H00 CLOSED**。

**Post-v1.0 path 2（关洞）· OW-SEED observe pin（2026-08-29）：** stage13/15/16 fail-closed 钉 **emitter**（`yoyo.exe` basename + size + sha256_prefix）+ **seed**（PE size + sha256_prefix 与 `SEED_HOST` 一致）+ **path=h00**。`SEED_HOST sha256_prefix` 扩至 **16** hex。**仍 CUT**（seed 仍由 Rust `yoyo.exe` 发射）— **禁止**标 CLOSED / SEED_HOST_GONE。

---

## 为何定稿

v0.9 把洞从 lump 变成逐项 `CLOSED|CUT`。v1.0-A 要求洞从「v0.9 枚举」变成 **终态可验**：每一项要么 **CLOSED**（fail-closed 证据），要么 **CUT**（天花板 / 标记写入 1.0 SCOPE-CUT）。  
能关则关；不能关则诚实 CUT — **禁止假 EQUAL / 假关洞**。  
对 OW-RT / OW-IAT / OW-SEED（Rust runtime · LoadLibrary · Rust seed）**预期仍 CUT** — 无 YOYO-built runtime / loader / 非 Rust 发射路径证据时不得 CLOSED。

**DDC = detection bar，不是 Thompson 证明。**

---

## 洞清单 disposition（Stage 16-A · FINAL · post-v1.0 OW-RT）

| ID | 区域 | Disposition | 关闭证据（CLOSED 才需要） | CUT 钉（机器） |
|----|------|-------------|---------------------------|----------------|
| **OW-H00** | H_00 entry slot（18 B）+ manual-map stub | **CLOSED** | `three_peer_full=EQUAL` · full `.text` JS=asm=Rust **18944** B | — |
| **OW-STUB** | H_00 manual-map stub tail | **CUT** | `stub_tail_nonzero==0`（所有 peer） | `stub_tail_nonzero` ∈ **[40, 950]**；观测 **905** |
| **OW-RT** | Sidecar Rust runtime (Win DLL / Linux `.so`) | **CUT** | 无 exact embed **且** 无 Rust LoadLibrary/libdl sidecar 宿主信任 | Win DLL **≤150000** 观测 **141312**；Linux seed ELF **253952**（MAX **300000**）；sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so`；**no exact .so embed**（tramp still embedded） |
| **OW-IAT** | Host file I/O + sidecar | **CUT** | 无宿主 DLL 加载面（无 `yoyo_rt.dll`） | Win：**无** PEB LoadLibraryA；CreateFile/Read/VirtualAlloc + manual-map；`yoyo_rt.dll` 仍在。Linux tramp：**无** `dlopen`/`libdl`；syscall mmap loader；仍宿主 syscalls + cwd `.so` |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | **CUT** | seed 非 Rust host 发射 | Rust `yoyo link`；seed PE **≤270000**（观测 **248320**）；**emitter** size+sha256_prefix + **seed** sha256_prefix≡`SEED_HOST` + **path=h00** |
| **REL-FULLTEXT** | full `.text` peer compare | **CUT** | （禁止用 EQUAL 当毕业话术） | `full_text=DIFF` → inventory FINAL+CUT；意外 EQUAL → PARTIAL（OW-RT/IAT 仍 CUT） |
| **REL-STUBOS** | Plan9/FreeBSD/Haiku/Serenity I/O | **CUT** | 生产 I/O 落地（非本 Stage） | `stage13-cross-platform-parity.ps1` stub 钉仍在源门禁中 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## CLOSED 判定（fail-closed · 不得放水）

Gate **只**在下列证据同时成立时打印 `disposition=CLOSED`：

1. **OW-H00** — `full_text=EQUAL` **且** body window EQUAL  
2. **OW-STUB** — parsed `stub_tail_nonzero == 0`  
3. **OW-RT** — runtime.dll **not** exactly embedded **且** 无 `LoadLibraryA`/`yoyo_rt.dll` sidecar 面（YOYO-built / 无 Rust runtime 宿主）— **仅无 embed 不得 CLOSED**  
4. **OW-IAT** — 无宿主 DLL 加载面（无 `yoyo_rt.dll` sidecar 标记）— **仅无 IAT/ASCII `LoadLibraryA` 不得 CLOSED**（PEB 宿主 LoadLibrary 仍 CUT）  
5. **OW-SEED** — （**不自动 CLOSED**；需另立非 Rust 发射路径证据；CUT 须带 emitter/seed hash + path=h00）  
6. **REL-FULLTEXT** — **永不**标 CLOSED 作为毕业（EQUAL 时仅 PARTIAL / 观测）  
7. **REL-STUBOS** — **永不**标 CLOSED 除非 stage13 stub 门改为 production I/O

当前基线下 **OW-H00 CLOSED**（1）；其余 **CUT**（6）。`status=FINAL` 表可含 CLOSED+CUT 混排。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v1.0-A / RELEASE 毕业话术（窗仍 DIFF 时）
- ❌ 「洞已全关」若任一项仍 `disposition=CUT`
- ❌ Thompson-proof / 已消灭 Rust runtime / LoadLibrary / Rust seed
- ❌ 「无 exact embed」说成 OW-RT **CLOSED**（sidecar Rust 仍在）
- ❌ 「无 IAT/ASCII LoadLibraryA」说成 OW-IAT **CLOSED**（PEB 宿主 LoadLibrary + `yoyo_rt.dll` 仍在）
- ❌ 「已钉 emitter/seed hash」说成 OW-SEED **CLOSED**（仍 Rust `yoyo.exe` 发射）
- ❌ 把 SCOPE-CUT 写成失败或倒退；不得假 EQUAL / 假 CLOSED

允许：✅ 「v1.0 SCOPE-CUT FINAL · 七项 CLOSED|CUT 已机器钉 + selfhost-body EQUAL」；✅ 「OW-RT 已缩：无 exact embed / sidecar（仍 CUT）」；✅ 「OW-IAT 已缩：无 IAT LoadLibraryA/GetProcAddress / PEB+ordinal-0（仍 CUT · 宿主 LoadLibrary）」；✅ 「OW-SEED 已缩盲区：emitter+seed hash + path=h00 机器钉（仍 CUT）」

---

## 机器验收（exit 0）

```powershell
cd F:\yoyo
.\scripts\stage16-scope-cut-finalize.ps1
# 或已有 release 二进制：
.\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
```

Gate 必须同时：

1. **不退化 Stage 15-A** — `stage15-hole-inventory.ps1 -SkipBuild` exit 0  
2. **逐项定稿** — 每个 OW-\* / REL-\* 打印一行 `FINAL_HOLE id=… disposition=CLOSED|CUT evidence=…`  
3. **诚实 full `.text`** — DIFF → FINAL 表仍 CUT；不得把 DIFF 说成 EQUAL  
4. **禁止假 CLOSED** — 无证据不得 CLOSED（委托 stage15 fail-closed）  
5. **打印** `HOLE_INVENTORY_V10 status=FINAL …` 汇总行  
6. **本文件存在** — `SCOPE-CUT-v1.0-hole-inventory.md` 含七 ID + `Status: FINAL`

---

## 观测基线（2026-08-29 · post JS IAT sync · three-peer EQUAL）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust↔asm | **EQUAL** · **`72c27c9f`** · **18944** B |
| three_peer_full | **EQUAL** |
| H_00 entry slot (18 B) | **JMP+NOP aligned** JS=Rust=asm |
| stub_tail_nonzero (JS=Rust) | **905**（pin [40, 950]） |
| runtime.dll | **141312**（**no exact embed**；sidecar） |
| seed PE (Rust link) | **249344**（≤270000） |
| seed ELF (Linux link) | **253952**（≪300000；**no exact .so embed**；tramp **9032** B static mmap embed；no dlopen） |
| gen12 / fullbody `.text` | SHA prefix **`72c27c9f`** · compared **18944** B |
| Lock pin | `0275802d…` Decision #25（本缩面不改 `yoyo.ty`） |
| Disposition | **OW-H00 CLOSED** · **6× CUT**（closed=1 cut=6） |
| Gates | body-ddc · gen12 · stage17-ow-iat-wireup · stage10-linux GREEN |
| OW-IAT wire-up | PEB LoadLibrary **DROPPED**；manual-map **WIRED**；**仍 CUT** |

**Next tip（post-v1.0 path 2）：** **Win sidecar smoke** (cwd `yoyo_rt.dll` + manual-map H_00 run); **OW-IAT CLOSED** only when `yoyo_rt.dll` / sidecar marker absent.

---

*Stage 16-A · 打破后门魔咒：洞从「v0.9 枚举」变「1.0 FINAL SCOPE-CUT + 可脚本钉」· post-v1.0 OW-RT：Win/Linux exact embed → sidecar（仍 CUT）· post-v1.0 OW-IAT：GetProcAddress → ordinal-0 export resolve；Linux dlsym → ELF dyn walk（仍 CUT）· post-v1.0 OW-STUB：235→96B ordinal-0（仍 CUT）· post-v1.0 OW-H00：JS/asm JMP+NOP slot align（仍 CUT · stub DIFF）· post-v1.0 OW-SEED：emitter+seed hash pin（仍 CUT）*
