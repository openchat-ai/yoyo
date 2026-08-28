# YOYO 1.0 — Scope Boundary（负责人一页纸）

> **前提**：已发布 v0.1–**v1.0.0**；**ROADMAP 终点** Stage 16 / v1.0 **已毕业**。  
> **路线图**：`ROADMAP-TO-1.0.md`。**无人值守开关**：`AUTO_TO_1.0.md`（`ACTIVE=0` · `COMPLETED=1`）。  
> **诚实**：DDC = detection，非 Thompson-proof；`HOLE_INVENTORY_V10 status=FINAL closed=0 cut=7`（七项仍 **CUT**，无假 CLOSED）。  
> **基线**：Stage 16 已毕业（2026-08-29）；Lock pin `0275802d…`（Decision #25 · **无 Relock**）；见 `RELEASE-v1.0.md`。

---

## 1.0 是什么（实用定义）

**YOYO 1.0** = 面向 **自举编译器 ISA** 的 **实用信任工具链**：在诚实可宣称范围内，把宿主 / 嵌入 runtime / peer stub / seed 旁路压到最小，并用 **DDC + Lock** 覆盖自举主路径。

| 维度 | 1.0 要求 |
|------|----------|
| **信任** | 文档化宿主洞 **全关**，或 **SCOPE-CUT** 写入 RELEASE |
| **检测** | DDC = 链间分歧 **detection**，非数学免疫 |
| **产品** | 可审计自举 ISA + 门禁 + Lock；非 C/Rust 替代 / Morph / MCU 主赛道 |
| **发布** | `RELEASE-v1.0.md` + git tag + GitHub Release |

**一句话**：能诚实缩小多少宿主信任就缩小到那一步；剩余洞公开 SCOPE-CUT，不堆功能。

---

## 北星（NON-NEGOTIABLE）

**打破后门魔咒** — DDC+Lock 扩面 / 缩宿主信任。非 feature dump；非默认 C/Rust 替换；禁止 Thompson-proof 话术。

---

## v1.0 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） | 状态 |
|---|------|------|------------------|------|
| 1 | **全关或 SCOPE-CUT 定稿** | 对 v0.9 七项 CUT：能关则 CLOSED+证据；不能关则定稿进 1.0 RELEASE / SCOPE-CUT | **1.0 对外洞面可验** | ✅ A（FINAL · cut=7） |
| 2 | **detection 话术 + RELEASE 边界** | 钉 detection≠proof；禁词表 + CUT 入 RELEASE | **少误宣称 Thompson / 假关洞** | ✅ B（banlist ACTIVE） |
| 3 | **v0.9 回归不退化** | stage15 A/B/C + stage14–9 + fullbody/lock/gen12 保持绿 | 定稿不买回归 | ✅ C（ALL_GREEN） |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v1.0.md` graduated + tag；诚实写仍存 CUT | 对外 detection 话术定稿 | ✅ D（graduated） |

**主验收看板**：`STAGE16_OWNER_CHECKLIST.md`（A→D）。进度：`[x] A [x] B [x] C [x] D`。

---

## 版本化里程碑（v0.4 → v1.0）

| 版本 | Stage | 信任主题（≤5 门） | 产物 |
|------|-------|-------------------|------|
| **v0.4** | 10 | runtime 面 · Linux 纯 M4 · asm I/O · 毕业 | ✅ `RELEASE-v0.4` · tag `v0.4.0` |
| **v0.5** | 11 | YOYO-built/更薄 runtime · 收缩 LoadLibrary host · 回归 · 毕业 | ✅ `RELEASE-v0.5` · tag `v0.5.0` |
| **v0.6** | 12 | 三 peer I/O · section-ddc on selfhost body · 回归 · 毕业 | ✅ `RELEASE-v0.6` · tag `v0.6.0` |
| **v0.7** | 13 | seed/link host · 跨平台 parity · Relock 纪律 · 毕业 | ✅ `RELEASE-v0.7` · tag `v0.7.0` |
| **v0.8** | 14 | 窗外字节再收或 SCOPE-CUT 草案 · Lock 硬化 · 毕业 | ✅ `RELEASE-v0.8` · tag `v0.8.0` |
| **v0.9** | 15 | 洞清单 · 关或 SCOPE-CUT · 预跑门禁 · 毕业 | ✅ `RELEASE-v0.9` · tag `v0.9.0` |
| **v1.0** | 16 | 全关或 SCOPE-CUT 定稿 · RELEASE · tag · detection 话术 | ✅ `RELEASE-v1.0` · tag `v1.0.0` |

> **v1.0 现状**：**已毕业**（2026-08-29）；Stage 16 A/B/C/D 全绿；`AUTO_TO_1.0.md` COMPLETED · ACTIVE=0；**无 Stage 17**。

---

## v1.0 OUT（永远 / post-1.0）

Morph 产品 · MCU 主赛道 · C/Rust 替代宣称 · Thompson-proof · TheoryManifest/CDS · macOS 生产门禁不阻塞 · G06+ 默认不扩 · invent Stage 17 功能轨

---

## 与 v0.9 诚实剩余面的关系

| v0.9 / `RELEASE-v0.9.md` / HOLE_INVENTORY | v1.0 回收 |
|------------------------------------------|-----------|
| **OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED** 全 CUT | ✅ Stage 16 **A** — `SCOPE-CUT-v1.0` FINAL（全 CUT） |
| **REL-FULLTEXT / REL-STUBOS** CUT | ✅ Stage 16 **A/D** — 定稿进 v1.0 SCOPE-CUT + RELEASE |
| **HOLE_INVENTORY ACTIVE → V10 FINAL closed=0 cut=7** | ✅ A 定稿；✅ B/D 入 RELEASE（仍 CUT，无假 CLOSED） |
| DDC = detection 非 proof | ✅ Stage 16 **B/D** 钉死（`DETECTION-BANLIST-v1.0.md`） |

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
.\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
.\scripts\stage16-detection-wording.ps1 -SkipBuild
.\scripts\stage16-v09-regress.ps1 -SkipBuild
```

**Stage 16 四门全 `[x]`** = v1.0 已发布。

**Stage 16-A（2026-08-29）：** `stage16-scope-cut-finalize.ps1 -SkipBuild` exit 0 · `HOLE_INVENTORY_V10 status=FINAL full_text=DIFF body_window=EQUAL closed=0 cut=7` · 文档 `SCOPE-CUT-v1.0-hole-inventory.md` · OW-\* + REL-\* 全 **CUT**（无假 CLOSED）· 嵌套 Stage 15-A 不退化 · Lock pin Decision #25 未改。

**Stage 16-B（2026-08-29）：** `stage16-detection-wording.ps1 -SkipBuild` exit 0 · banlist ACTIVE · `RELEASE-v1.0.md` 列全 CUT · 禁 Thompson-proof / fully closed / fake EQUAL 肯定句 · 嵌套 Stage 16-A 不退化 · Lock pin Decision #25 未改。

**Stage 16-C（2026-08-29）：** `stage16-v09-regress.ps1` / `-SkipBuild` exit 0 · `ALL_GREEN` · stage15 A/B/C 实质 + stage14–9 + fullbody/lock/gen12 + WSL + Stage 16 A/B 串行全 0 · Lock pin Decision #25 未改。

**毕业判定（2026-08-29 · Stage 16-D）：** A/B/C/D 全绿 · **无 Relock**（Decision #25）· `verify-lock-pin` + `stage14-lock-harden.ps1 -SkipBuild` PINNED · `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN（**02:14:21**）· `RELEASE-v1.0.md` Status:** graduated** · `RELEASE-NOTES-v1.0.md` 诚实写 **HOLE_INVENTORY_V10 FINAL closed=0 cut=7** + DDC=detection · 七项 CUT 全列 · banlist ACTIVE · tag `v1.0.0` · ROADMAP 终点 · `AUTO_TO_1.0.md` COMPLETED · ACTIVE=0 · **无 Stage 17**。

---

## AUTO-TO-1.0 契约（无人值守）

**开关**：`AUTO_TO_1.0.md` 中现为 `ACTIVE=0` · `COMPLETED=1`（v1.0 已毕业；未来 tick **停手**）。

| 条件 | Agent |
|------|-------|
| `ACTIVE=1` + 每次 wake / loop tick / 空消息 / `继续` | **无问询**执行下一未勾 Stage 项 |
| Stage N 全 `[x]` | **自动「定」** ROADMAP 下一版（Stage 16 为终点） |
| Stage **D** 毕业门禁绿 | **auto commit** + **tag** + **GitHub Release**；毕业允许 push |
| 非毕业 WIP | **不 push** |
| Hard block | 停；写清原因到看板 |
| 用户 `停` 或 `ACTIVE=0` 或 `COMPLETED=1` | 停手 |
| 范围 | **仅** ROADMAP/SCOPE 已列信任门；禁止发明无界功能 |

---

## 诚实边界（对外一句话）

**YOYO 1.0 已把 v0.9 洞清单定稿为诚实 SCOPE-CUT（FINAL · closed=0 cut=7）写入 RELEASE——仍是 detection bar，不是 Thompson 证明；七项仍为 CUT，不得假装已关。**

---

*更新：2026-08-29 · Stage 16 A/B/C/D 全绿 · v1.0.0 毕业 · 见 `STAGE16_OWNER_CHECKLIST.md` · `RELEASE-v1.0.md`*
