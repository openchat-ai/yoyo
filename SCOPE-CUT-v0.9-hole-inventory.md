# SCOPE-CUT — v0.9 hole inventory（Stage 15-A 收口 · post-v1.0 OW-RT sync）

> **Status:** ACTIVE（洞清单逐项 CLOSED|CUT · 可机器验收）  
> **Gate:** `scripts/stage15-hole-inventory.ps1`  
> **Rule:** 本文件是 **诚实收口**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL；不得把仍 CUT 的洞标成 CLOSED。

**Upstream:** `SCOPE-CUT-v0.8-outside-window.md`（Stage 14-A 窗外草案）仍有效；本文件把 OW-\* / RELEASE-v0.8 剩余面 **逐项 disposition**。

**Post-v1.0 OW-RT：** Win H_00 **无 exact embed**；cwd sidecar `yoyo_rt.dll`；DLL **141312**。仍 **CUT**（Rust runtime）。
**Post-v1.0 OW-IAT：** GetProcAddress **ABSENT**；PE export walk；stub_nz **235**。仍 **CUT**（LoadLibraryA）。
**Post-v1.0 OW-H00 slot align（2026-08-29）：** JS/asm H_00 entry **JMP+NOP 18B** ≡ Rust link patch；三 peer slot EQUAL。**仍 CUT**（Rust stub tail）— **禁止** CLOSED / 假 full `.text` EQUAL。

---

## 为何再收口

v0.8 把窗外钉成 SCOPE-CUT ACTIVE（lump）。v0.9-A 要求洞从「清单」变成 **可验 disposition**：每一项要么 **CLOSED**（有关闭证据），要么 **CUT**（再钉天花板 / 标记）。  
能关则关；不能关则诚实 CUT — **禁止假 EQUAL / 假关洞**。

**DDC = detection bar，不是 Thompson 证明。**

---

## 洞清单 disposition（Stage 15-A）

| ID | 区域 | Disposition | 关闭证据（CLOSED 才需要） | CUT 钉（机器） |
|----|------|-------------|---------------------------|----------------|
| **OW-H00** | H_00 entry slot（18 B） | **CUT** | full `.text` EQUAL 且 body 仍 EQUAL | slot **JMP+NOP aligned**（JS=Rust=asm）；full `.text` JS↔Rust **DIFF**（stub）；body 跳过该槽 |
| **OW-STUB** | H_00 LoadLibrary stub tail | **CUT** | `stub_tail_nonzero==0`（所有 peer） | `stub_tail_nonzero` ∈ **[40, 512]**；观测 **235** |
| **OW-RT** | Sidecar Rust `yoyo_runtime.dll` | **CUT** | 无 exact embed **且** 无 Rust sidecar LoadLibrary 面 | size **≤150000**；观测 **141312**；**no exact embed** |
| **OW-IAT** | LoadLibraryA / libdl host | **CUT** | PE 无 `LoadLibraryA`（YOYO-built loader） | 标记 `LoadLibraryA` + `yoyo_rt.dll`；**无** GetProcAddress（PE export walk） |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | **CUT** | seed 非 Rust host 发射 | Rust `yoyo link`；seed PE **≤270000**（观测 **248832**）；emitter+seed sha256_prefix + path=h00 |
| **REL-FULLTEXT** | full `.text` peer compare | **CUT** | （禁止用 EQUAL 当毕业话术） | `full_text=DIFF` → inventory ACTIVE；意外 EQUAL → PARTIAL（OW-RT/IAT 仍 CUT） |
| **REL-STUBOS** | Plan9/FreeBSD/Haiku/Serenity I/O | **CUT** | 生产 I/O 落地（非本 Stage） | `stage13-cross-platform-parity.ps1` stub 钉仍在源门禁中 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## CLOSED 判定（fail-closed · 不得放水）

Gate **只**在下列证据同时成立时打印 `disposition=CLOSED`：

1. **OW-H00** — `full_text=EQUAL` **且** body window EQUAL  
2. **OW-STUB** — parsed `stub_tail_nonzero == 0`  
3. **OW-RT** — no exact embed **且** no Rust sidecar LoadLibrary surface（**仅无 embed ≠ CLOSED**）  
4. **OW-IAT** — ASCII `LoadLibraryA` **absent** from Rust seed PE  
5. **OW-SEED** — （v0.9-A **不自动 CLOSED**；需另立非 Rust 发射路径证据）  
6. **REL-FULLTEXT** — **永不**标 CLOSED 作为毕业（EQUAL 时仅 PARTIAL / 观测）  
7. **REL-STUBOS** — **永不**标 CLOSED 除非 stage13 stub 门改为 production I/O（超出 v0.9-A）

当前基线下 **全部为 CUT**（诚实）。若未来某项变 CLOSED，本表与 gate 输出必须同步。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v0.9-A 毕业话术（窗仍 DIFF 时）
- ❌ 「洞已全关」若任一项仍 `disposition=CUT`
- ❌ Thompson-proof / 已消灭 Rust runtime / LoadLibrary / Rust seed
- ❌ 把「无 exact embed」说成 OW-RT CLOSED
- ❌ 把 SCOPE-CUT 写成失败或倒退

允许：✅ 「洞清单逐项 CLOSED|CUT 已机器枚举 + selfhost-body EQUAL」

---

## 机器验收（exit 0）

```powershell
cd F:\yoyo
.\scripts\stage15-hole-inventory.ps1
# 或已有 release 二进制：
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
```

Gate 必须同时：

1. **不退化 Stage 14-A** — `stage14-outside-window-scope-cut.ps1 -SkipBuild` exit 0  
2. **逐项枚举** — 每个 OW-\* / REL-\* 打印一行 `HOLE id=… disposition=CLOSED|CUT evidence=…`  
3. **诚实 full `.text`** — DIFF → inventory ACTIVE；不得把 DIFF 说成 EQUAL  
4. **禁止假 CLOSED** — 无证据不得 CLOSED  
5. **打印** `HOLE_INVENTORY status=…` 汇总行

---

## 观测基线（2026-08-29 · post-v1.0 OW-RT + OW-IAT）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust | **DIFF** |
| stub_tail_nonzero (Rust) | **235**（pin [40, 512]） |
| runtime.dll | **141312**（**no exact embed**；sidecar） |
| seed PE (Rust link) | **248832**（≤270000） |
| Lock pin | `0275802d…` Decision #25 |
| Disposition | **OW-\* + REL-\* all CUT**（closed=0 cut=7） |
| Gate | `stage15-hole-inventory.ps1 -SkipBuild` exit **0** |

---

*Stage 15-A · 打破后门魔咒：洞从「清单」变成「CLOSED|CUT + 可脚本钉」· post-v1.0 OW-RT + OW-IAT sync*
