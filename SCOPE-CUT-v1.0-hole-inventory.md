# SCOPE-CUT — v1.0 hole inventory（Stage 16-A 定稿 · post-v1.0 OW-RT shrink）

> **Status:** FINAL（v1.0 终态收口 · 可机器验收）  
> **Gate:** `scripts/stage16-scope-cut-finalize.ps1`（alias `stage16-a.ps1`）  
> **Rule:** 本文件是 **诚实定稿**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL；不得把仍 CUT 的洞标成 CLOSED。

**Upstream:** `SCOPE-CUT-v0.9-hole-inventory.md`（Stage 15-A ACTIVE 枚举）仍有效；本文件把同一七项 **提升为 v1.0 FINAL disposition**（能关则 CLOSED+证据；不能关则 CUT 钉进 RELEASE）。

**Post-v1.0 path 2（关洞）· OW-RT shrink（2026-08-29）：** Win H_00 seed/link **不再 exact-embed** `yoyo_runtime.dll`；改为 cwd sidecar `yoyo_rt.dll` + `LoadLibraryA`。DLL 观测 **141312**（LTO 后）。**仍 CUT**（Rust runtime 宿主信任未灭）— **禁止**把「无 embed」单独标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-RT Linux sidecar parity（2026-08-29）：** Linux H_00 **不再 exact-embed** `libyoyo_runtime.so`；仅嵌 trampoline，cwd sidecar `./libyoyo_runtime.so` + `dlopen`。seed ELF **253952**（MAX **300000**；was ~512000）。**仍 CUT**（Rust `.so` + glibc/libdl tramp）— **禁止**标 CLOSED。

**Post-v1.0 path 2（关洞）· OW-IAT shrink（2026-08-29）：** H_00 宿主 IAT **去掉 GetProcAddress**（host-loader **3→2**：LoadLibraryA + ExitProcess）。LoadLibraryA 之后 **in-process PE export walk** 解析 `yoyo_runtime_selfhost_main`。stub_nz **235**；gen12 `84a8c1c9` / **18432** B；seed PE **248832**。**仍 CUT**（LoadLibraryA / libdl 仍在）— **禁止**标 CLOSED。

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
| **OW-H00** | H_00 entry slot（18 B） | **CUT** | full `.text` EQUAL 且 body 仍 EQUAL | full `.text` DIFF（JS↔Rust）；body 跳过该槽 |
| **OW-STUB** | H_00 LoadLibrary stub tail | **CUT** | `stub_tail_nonzero==0`（所有 peer） | `stub_tail_nonzero` ∈ **[40, 512]**；观测 **235** |
| **OW-RT** | Sidecar Rust runtime (Win DLL / Linux `.so`) | **CUT** | 无 exact embed **且** 无 Rust LoadLibrary/libdl sidecar 宿主信任 | Win DLL **≤150000** 观测 **141312**；Linux seed ELF **253952**（MAX **300000**）；sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so`；**no exact embed**（双平台） |
| **OW-IAT** | LoadLibraryA / libdl host | **CUT** | PE 无 `LoadLibraryA`（YOYO-built loader） | 标记 `LoadLibraryA` + `yoyo_rt.dll`；**无** `GetProcAddress`（PE export walk）；仍宿主 LoadLibrary |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | **CUT** | seed 非 Rust host 发射 | Rust `yoyo link`；seed PE **≤270000**（观测 **248832**）；**emitter** size+sha256_prefix + **seed** sha256_prefix≡`SEED_HOST` + **path=h00** |
| **REL-FULLTEXT** | full `.text` peer compare | **CUT** | （禁止用 EQUAL 当毕业话术） | `full_text=DIFF` → inventory FINAL+CUT；意外 EQUAL → PARTIAL（OW-RT/IAT 仍 CUT） |
| **REL-STUBOS** | Plan9/FreeBSD/Haiku/Serenity I/O | **CUT** | 生产 I/O 落地（非本 Stage） | `stage13-cross-platform-parity.ps1` stub 钉仍在源门禁中 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## CLOSED 判定（fail-closed · 不得放水）

Gate **只**在下列证据同时成立时打印 `disposition=CLOSED`：

1. **OW-H00** — `full_text=EQUAL` **且** body window EQUAL  
2. **OW-STUB** — parsed `stub_tail_nonzero == 0`  
3. **OW-RT** — runtime.dll **not** exactly embedded **且** 无 `LoadLibraryA`/`yoyo_rt.dll` sidecar 面（YOYO-built / 无 Rust runtime 宿主）— **仅无 embed 不得 CLOSED**  
4. **OW-IAT** — ASCII `LoadLibraryA` **absent** from Rust seed PE（仅去掉 GetProcAddress **不得** CLOSED）  
5. **OW-SEED** — （**不自动 CLOSED**；需另立非 Rust 发射路径证据；CUT 须带 emitter/seed hash + path=h00）  
6. **REL-FULLTEXT** — **永不**标 CLOSED 作为毕业（EQUAL 时仅 PARTIAL / 观测）  
7. **REL-STUBOS** — **永不**标 CLOSED 除非 stage13 stub 门改为 production I/O

当前基线下 **全部为 CUT**（诚实定稿）。若未来某项变 CLOSED，本表与 gate 输出必须同步；`status=FINAL` 仍成立（终态表可含 CLOSED）。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v1.0-A / RELEASE 毕业话术（窗仍 DIFF 时）
- ❌ 「洞已全关」若任一项仍 `disposition=CUT`
- ❌ Thompson-proof / 已消灭 Rust runtime / LoadLibrary / Rust seed
- ❌ 「无 exact embed」说成 OW-RT **CLOSED**（sidecar Rust 仍在）
- ❌ 「无 GetProcAddress」说成 OW-IAT **CLOSED**（LoadLibraryA 仍在）
- ❌ 「已钉 emitter/seed hash」说成 OW-SEED **CLOSED**（仍 Rust `yoyo.exe` 发射）
- ❌ 把 SCOPE-CUT 写成失败或倒退；不得假 EQUAL / 假 CLOSED

允许：✅ 「v1.0 SCOPE-CUT FINAL · 七项 CLOSED|CUT 已机器钉 + selfhost-body EQUAL」；✅ 「OW-RT 已缩：无 exact embed / sidecar（仍 CUT）」；✅ 「OW-IAT 已缩：无 GetProcAddress / PE export walk（仍 CUT · LoadLibraryA）」；✅ 「OW-SEED 已缩盲区：emitter+seed hash + path=h00 机器钉（仍 CUT）」

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

## 观测基线（2026-08-29 · post-v1.0 OW-RT sidecar + OW-IAT no-GPA + OW-SEED pin · gate GREEN）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust | **DIFF** |
| stub_tail_nonzero (Rust) | **235**（pin [40, 512]；PE export walk） |
| runtime.dll | **141312**（**no exact embed**；sidecar） |
| seed PE (Rust link) | **248832**（≤270000；data floor 0x38000 仍主导体积） |
| seed ELF (Linux link) | **253952**（≪300000；**no exact .so embed**；tramp still embedded） |
| gen12 / fullbody `.text` | SHA prefix **`84a8c1c9`** · compared **18432** B |
| Lock pin | `0275802d…` Decision #25（本缩面不改 `yoyo.ty`） |
| Disposition | **OW-\* + REL-\* all CUT**（closed=0 cut=7） |
| Gate | `stage16-scope-cut-finalize.ps1 -SkipBuild` exit **0** · `HOLE_INVENTORY_V10 status=FINAL` |
| No-regress | nested stage15-A exit 0 · stage14-A nested via stage15 |
| OW-IAT shrink | GetProcAddress **ABSENT**；LoadLibraryA **PRESENT**（仍 CUT） |
| OW-SEED observe | emitter=`yoyo.exe` size+sha256_prefix；seed sha256_prefix≡`SEED_HOST`（16 hex）；path=h00（仍 CUT） |

---

*Stage 16-A · 打破后门魔咒：洞从「v0.9 枚举」变「1.0 FINAL SCOPE-CUT + 可脚本钉」· post-v1.0 OW-RT：Win/Linux exact embed → sidecar（仍 CUT）· post-v1.0 OW-IAT：GetProcAddress → PE export walk（仍 CUT）· post-v1.0 OW-SEED：emitter+seed hash pin（仍 CUT）*
