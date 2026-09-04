# SCOPE-CUT — v1.0 hole inventory（Stage 16-A 定稿 · post-v1.0 OW-RT shrink）

> **Status:** FINAL（v1.0 终态收口 · 可机器验收）  
> **Gate:** `scripts/stage16-scope-cut-finalize.ps1`（alias `stage16-a.ps1`）  
> **Rule:** 本文件是 **诚实定稿**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL；不得把仍 CUT 的洞标成 CLOSED。

**Upstream:** `SCOPE-CUT-v0.9-hole-inventory.md`（Stage 15-A ACTIVE 枚举）仍有效；本文件把同一七项 **提升为 v1.0 FINAL disposition**（能关则 CLOSED+证据；不能关则 CUT 钉进 RELEASE）。

**Post-v1.0 path 2（关洞）· OW-RT shrink（2026-08-29）：** Win H_00 seed/link **不再 exact-embed** `yoyo_runtime.dll`；改为 cwd sidecar `yoyo_rt.dll` + `LoadLibraryA`。DLL 观测 **158720**（LTO · Gate C 2026-09-03；was 141312）。**仍 CUT**（Rust runtime 宿主信任未灭）— **禁止**把「无 embed」单独标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-RT Linux sidecar parity（2026-08-29）：** Linux H_00 **不再 exact-embed** `libyoyo_runtime.so`；仅嵌 trampoline，cwd sidecar `./libyoyo_runtime.so` + `dlopen`。seed ELF **253952**（MAX **300000**；was ~512000）。**仍 CUT**（Rust `.so` + glibc/libdl tramp）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-STUB shrink（2026-08-29）：** H_00 stub **235→96** B：去掉 export 名字符串 + **ordinal-0** 解析（`yoyo_runtime` 钉 `yoyo_runtime_selfhost_main` 为首个 named export；不再 in-stub strcmp walk）。gen12 `90ad6d6e` / **17920** B；seed PE **248320**。**仍 CUT**（Rust-only stub；窗外）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT shrink（2026-08-29）：** H_00 宿主 IAT **去掉 GetProcAddress**（host-loader **3→2**：LoadLibraryA + ExitProcess）。LoadLibraryA 之后 in-process PE export resolve（ordinal-0；was full name walk）。**仍 CUT**（LoadLibraryA / libdl 仍在）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· deeper OW-IAT（2026-08-29）：** H_00 **再去掉 IAT/ASCII `LoadLibraryA`** — PEB→kernel32 **ROR13** 导出哈希解析后调用；ordinal-0 导出仍保留。host-loader IAT 面仅 **ExitProcess**。**仍 CUT**（仍宿主 LoadLibrary）— **禁止**因「无 LoadLibraryA 字符串」标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT Linux hybrid tramp（2026-08-29）：** `linux_h00_tramp.elf` **dynamic -lc only**（no libdl NEEDED）；sidecar via **dlopen@PLT** + sym walk（no dlsym）；**no glibc/ld disk mmap**（fixes CI IRELATIVE/TLS/RELR）。gen4≡gen3_direct **EQUAL**。**仍 CUT**（dlopen + ld.so libc + cwd sidecar `.so`）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT manual-map wire-up（2026-08-29 · PR #8）：** H_00 stub **905B** manual-map（CreateFile/Read/VirtualAlloc + `pe_manual_map`）；PEB `LoadLibraryA` **DROPPED**；JS `h00-manual-map-stub.hex` lockstep。**仍 CUT**（sidecar `yoyo_rt.dll` + kernel32 I/O）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-H00 曾 CLOSED（JS IAT sync · 历史）：** 当时 `three_peer_full=EQUAL` · **`72c27c9f`** / **18944** B。**Gate C（2026-09-03）重测：full `.text` DIFF → OW-H00 回 CUT** — **禁止**沿用 CLOSED。

**Post-v1.0 path 2（关洞）· OW-SEED observe pin（2026-08-29）：** stage13/15/16 fail-closed 钉 **emitter**（`yoyo.exe` basename + size + sha256_prefix）+ **seed**（PE size + sha256_prefix 与 `SEED_HOST` 一致）+ **path=h00**。`SEED_HOST sha256_prefix` 扩至 **16** hex。**仍 CUT**（seed 仍由 Rust `yoyo.exe` 发射）— **禁止**标 CLOSED / SEED_HOST_GONE。

**Post-v1.0 path 2（关洞）· Gate A+B GREEN（2026-09-02 · PR #26 · master `8f9d98c`）：** Win `stage17-ow-iat-wireup.ps1` with-sidecar **GREEN**（no-sidecar fail-closed exit=2）；Linux `stage10-linux-pure-m4.sh` **GREEN**。**OW-IAT 仍 CUT**（sidecar + kernel32 I/O / dlopen）— **GREEN ≠ CLOSED**。**OW-H00：Gate C 重测 full `.text` DIFF → CUT**（slot 仍对齐；**禁止**沿用旧 CLOSED 话术）。

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
| **OW-H00** | H_00 entry slot（18 B）+ manual-map stub | **CUT** | `three_peer_full=EQUAL` · full `.text` JS=asm=Rust（Gate A 后 **DIFF** · 观测 **20992** B） | `full_text=DIFF`；H_00 slot 18B JMP+NOP 仍对齐；stub 仍窗外 |
| **OW-STUB** | H_00 manual-map stub tail | **CUT** | `stub_tail_nonzero==0`（所有 peer） | `stub_tail_nonzero` ∈ **[40, 3000]**；观测 **2673** |
| **OW-RT** | Sidecar Rust runtime (Win DLL / Linux `.so`) | **CUT** | 无 exact embed **且** 无 Rust LoadLibrary/libdl sidecar 宿主信任 | Win DLL **≤170000** 观测 **158720**；Linux seed ELF **253952**（MAX **300000**）；sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so`；**no exact .so embed**（tramp still embedded） |
| **OW-IAT** | Host file I/O + sidecar | **CUT** | 无宿主 DLL 加载面（无 `yoyo_rt.dll`） | Win：**无** PEB/ASCII LoadLibraryA；CreateFile/Read/VirtualAlloc + manual-map；`yoyo_rt.dll` cwd sidecar。Linux tramp：**dlopen@PLT**（dynamic `-lc` only；**no libdl NEEDED**）；no dlsym；cwd `./libyoyo_runtime.so` |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | **CUT** | seed 非 Rust host 发射 | Rust `yoyo link`；seed PE **≤270000**（观测 **251392**）；**emitter** size+sha256_prefix + **seed** sha256_prefix≡`SEED_HOST` + **path=h00** |
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

当前基线下 **OW-H00 CUT**（Gate A 后 full `.text` JS↔Rust **DIFF** · 禁止假 CLOSED）；其余 **CUT**（6）。合计 **closed=0 cut=7**。`status=FINAL` 表可含 CLOSED+CUT 混排（现无 CLOSED）。

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

## 观测基线（2026-09-03 · Gate C 重测 · tip `8f9d98c`）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust↔asm | **DIFF** · compared **20992** B（was EQUAL `72c27c9f` / 18944 · Gate A 后 lockstep 漂移） |
| three_peer_full | **DIFF** |
| H_00 entry slot (18 B) | **JMP+NOP aligned** JS=Rust（slot 对齐 ≠ CLOSED） |
| stub_tail_nonzero (JS=Rust) | **2673**（pin [40, 3000]；was 905/[40,950]） |
| runtime.dll | **158720**（**no exact embed**；sidecar；MAX **170000**） |
| seed PE (Rust link) | **251392**（≤270000） |
| seed ELF (Linux link) | **253952**（≪300000；**no exact .so embed**；tramp **9032** B static mmap embed；no dlopen） |
| gen12 / fullbody `.text` | compared **20992** B（窗扩大；勿用旧 `72c27c9f`/18944 话术） |
| Lock pin | `0275802d…` Decision #25（本缩面不改 `yoyo.ty`） |
| Disposition | **7× CUT**（closed=0 cut=7；OW-H00 因 full DIFF 回 CUT） |
| Gates | body-ddc · gen12 · stage17-ow-iat-wireup (Win smoke GREEN) · stage10-linux GREEN |
| OW-IAT wire-up | PEB LoadLibrary **DROPPED**；manual-map **WIRED**；Win smoke **GREEN**（PR #26 · tip `8f9d98c`）；**仍 CUT**（sidecar `yoyo_rt.dll` + kernel32 I/O） |

**Post-v1.0 path 2 · Gate C honest sync（2026-09-03）：** A+B GREEN 后机器重测 — Win OW-IAT smoke **GREEN ≠ CLOSED**；OW-H00 **CUT**（full `.text` DIFF · **禁止**假 CLOSED）；stub **2673** / DLL **158720**；`closed=0 cut=7`；**禁止**假 CLOSED。

**Next tip（post-v1.0 path 2 · 整仓竣工）：** Gate **F** landed YOYO-built R→C→W effect（yoyo_built=EFFECT · yoyo_built_effect=PRESENT · 仍 CUT）。**G** = drop Rust sidecar；**OW-RT CLOSED** only then；**OW-IAT CLOSED** only when yoyo_rt.dll marker absent。

---

*Stage 16-A · 打破后门魔咒：洞从「v0.9 枚举」变「1.0 FINAL SCOPE-CUT + 可脚本钉」· post-v1.0 OW-RT：Win/Linux exact embed → sidecar（仍 CUT）· post-v1.0 OW-IAT：GetProcAddress → ordinal-0 export resolve；Linux dlsym → ELF dyn walk（仍 CUT）· post-v1.0 OW-STUB：235→96B ordinal-0（仍 CUT）· post-v1.0 OW-H00：JS/asm JMP+NOP slot align（仍 CUT · stub DIFF）· post-v1.0 OW-SEED：emitter+seed hash pin（仍 CUT）*
