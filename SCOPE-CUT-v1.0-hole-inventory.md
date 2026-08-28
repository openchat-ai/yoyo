# SCOPE-CUT — v1.0 hole inventory（Stage 16-A 定稿）

> **Status:** FINAL（v1.0 终态收口 · 可机器验收）  
> **Gate:** `scripts/stage16-scope-cut-finalize.ps1`（alias `stage16-a.ps1`）  
> **Rule:** 本文件是 **诚实定稿**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL；不得把仍 CUT 的洞标成 CLOSED。

**Upstream:** `SCOPE-CUT-v0.9-hole-inventory.md`（Stage 15-A ACTIVE 枚举）仍有效；本文件把同一七项 **提升为 v1.0 FINAL disposition**（能关则 CLOSED+证据；不能关则 CUT 钉进 RELEASE）。

---

## 为何定稿

v0.9 把洞从 lump 变成逐项 `CLOSED|CUT`。v1.0-A 要求洞从「v0.9 枚举」变成 **终态可验**：每一项要么 **CLOSED**（fail-closed 证据），要么 **CUT**（天花板 / 标记写入 1.0 SCOPE-CUT）。  
能关则关；不能关则诚实 CUT — **禁止假 EQUAL / 假关洞**。  
对 OW-RT / OW-IAT / OW-SEED（Rust runtime · LoadLibrary · Rust seed）**预期仍 CUT** — 无 YOYO-built runtime / loader / 非 Rust 发射路径证据时不得 CLOSED。

**DDC = detection bar，不是 Thompson 证明。**

---

## 洞清单 disposition（Stage 16-A · FINAL）

| ID | 区域 | Disposition | 关闭证据（CLOSED 才需要） | CUT 钉（机器） |
|----|------|-------------|---------------------------|----------------|
| **OW-H00** | H_00 entry slot（18 B） | **CUT** | full `.text` EQUAL 且 body 仍 EQUAL | full `.text` DIFF（JS↔Rust）；body 跳过该槽 |
| **OW-STUB** | H_00 extract stub tail | **CUT** | `stub_tail_nonzero==0`（所有 peer） | `stub_tail_nonzero` ∈ **[100, 2048]**；观测 **159** |
| **OW-RT** | Embedded Rust `yoyo_runtime.dll` | **CUT** | 无 exact embed 且无 Rust runtime 宿主信任 | size **≤170000**；观测 **154624**；**exact embed** |
| **OW-IAT** | LoadLibraryA / GetProcAddress | **CUT** | PE 无 `LoadLibraryA`（YOYO-built loader） | 标记 `LoadLibraryA` + `yoyo_rt.dll` 仍在 |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | **CUT** | seed 非 Rust host 发射 | Rust `yoyo link` 产出 PE；seed PE **≤270000**（观测 **248832**） |
| **REL-FULLTEXT** | full `.text` peer compare | **CUT** | （禁止用 EQUAL 当毕业话术） | `full_text=DIFF` → inventory FINAL+CUT；意外 EQUAL → PARTIAL（OW-RT/IAT 仍 CUT） |
| **REL-STUBOS** | Plan9/FreeBSD/Haiku/Serenity I/O | **CUT** | 生产 I/O 落地（非本 Stage） | `stage13-cross-platform-parity.ps1` stub 钉仍在源门禁中 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## CLOSED 判定（fail-closed · 不得放水）

Gate **只**在下列证据同时成立时打印 `disposition=CLOSED`（与 v0.9 同严）：

1. **OW-H00** — `full_text=EQUAL` **且** body window EQUAL  
2. **OW-STUB** — parsed `stub_tail_nonzero == 0`  
3. **OW-RT** — runtime.dll **not** exactly embedded in Rust seed PE  
4. **OW-IAT** — ASCII `LoadLibraryA` **absent** from Rust seed PE  
5. **OW-SEED** — （v1.0-A **不自动 CLOSED**；需另立非 Rust 发射路径证据）  
6. **REL-FULLTEXT** — **永不**标 CLOSED 作为毕业（EQUAL 时仅 PARTIAL / 观测）  
7. **REL-STUBOS** — **永不**标 CLOSED 除非 stage13 stub 门改为 production I/O（超出 v1.0-A）

当前基线下 **全部为 CUT**（诚实定稿）。若未来某项变 CLOSED，本表与 gate 输出必须同步；`status=FINAL` 仍成立（终态表可含 CLOSED）。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v1.0-A / RELEASE 毕业话术（窗仍 DIFF 时）
- ❌ 「洞已全关」若任一项仍 `disposition=CUT`
- ❌ Thompson-proof / 已消灭 Rust runtime / LoadLibrary / Rust seed
- ❌ 把 SCOPE-CUT 写成失败或倒退；不得假 EQUAL / 假 CLOSED

允许：✅ 「v1.0 SCOPE-CUT FINAL · 七项 CLOSED|CUT 已机器钉 + selfhost-body EQUAL」

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

## 观测基线（2026-08-29 · Stage 16-A · gate GREEN）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust | **DIFF** |
| stub_tail_nonzero (Rust) | **159**（pin [100, 2048]） |
| runtime.dll | **154624**（exact embed_off **85543**） |
| seed PE (Rust link) | **248832**（≤270000） |
| Lock pin | `0275802d…` Decision #25（本定稿不改 `yoyo.ty`） |
| Disposition | **OW-\* + REL-\* all CUT**（closed=0 cut=7） |
| Gate | `stage16-scope-cut-finalize.ps1 -SkipBuild` exit **0** · `HOLE_INVENTORY_V10 status=FINAL` |
| No-regress | nested stage15-A exit 0 · stage14-A nested via stage15 |

---

*Stage 16-A · 打破后门魔咒：洞从「v0.9 枚举」变「1.0 FINAL SCOPE-CUT + 可脚本钉」*
