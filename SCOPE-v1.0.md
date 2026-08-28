# YOYO 1.0 — Scope Boundary（负责人一页纸）

> **前提**：已发布 v0.1–**v0.9.0**；**现主线** Stage 16 / v1.0。  
> **路线图**：`ROADMAP-TO-1.0.md`。**无人值守开关**：`AUTO_TO_1.0.md`（`ACTIVE=1` = 推到 1.0）。  
> **诚实**：不能一夜魔法到 1.0；AUTO = 按 ROADMAP gates 连续打磨；耗时取决于宿主面（尤其 YOYO-built runtime）。DDC = detection，非 Thompson-proof。  
> **基线**：Stage 15 已毕业（2026-08-29）；Lock pin `0275802d…`（Decision #25）；`HOLE_INVENTORY ACTIVE closed=0 cut=7`（见 `RELEASE-v0.9.md`）。

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
| 1 | **全关或 SCOPE-CUT 定稿** | 对 v0.9 七项 CUT：能关则 CLOSED+证据；不能关则定稿进 1.0 RELEASE / SCOPE-CUT | **1.0 对外洞面可验** | 待 A |
| 2 | **detection 话术 + 1.0 预跑** | 钉 detection≠proof；一键串行 keep-green（含 v0.9 gates） | **少盲飞 1.0** | 待 B |
| 3 | **v0.9 回归不退化** | stage15 A/B/C + stage14–9 + fullbody/lock/gen12 保持绿 | 定稿不买回归 | 待 C |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v1.0.md` + tag；诚实写仍存 CUT | 对外 detection 话术定稿 | 待 D |

**主验收看板**：`STAGE16_OWNER_CHECKLIST.md`（A→D）。进度：`[ ] A [ ] B [ ] C [ ] D`。

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
| **v1.0** | 16 | 全关或 SCOPE-CUT 定稿 · RELEASE · tag · detection 话术 | `STAGE16` · `RELEASE-v1.0` + tag |

> **v1.0 现状**：Stage 15 / v0.9 **已毕业**；现主线 Stage 16 **A** 全关或 SCOPE-CUT 定稿；v0.9 已毕业勿回改。

---

## v1.0 OUT（永远 / post-1.0）

Morph 产品 · MCU 主赛道 · C/Rust 替代宣称 · Thompson-proof · TheoryManifest/CDS · macOS 生产门禁不阻塞 · G06+ 默认不扩 · invent Stage 17 功能轨

---

## 与 v0.9 诚实剩余面的关系

| v0.9 / `RELEASE-v0.9.md` / HOLE_INVENTORY | v1.0 回收 |
|------------------------------------------|-----------|
| **OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED** 全 CUT | Stage 16 **A** — 关或定稿 |
| **REL-FULLTEXT / REL-STUBOS** CUT | Stage 16 **A** — 定稿进 RELEASE |
| **HOLE_INVENTORY ACTIVE closed=0 cut=7** | 继续诚实直至 A 定稿 |
| DDC = detection 非 proof | Stage 16 **B/D** 钉死 |

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
# Stage 16 gates as added: stage16-*.ps1 -SkipBuild
```

**Stage 16 四门全 `[x]`** = v1.0 可发布候选。

**毕业判定：** （Stage 16 A/B/C/D 全绿后填写）

---

## AUTO-TO-1.0 契约（无人值守）

**开关**：`AUTO_TO_1.0.md` 中 `ACTIVE=1`。

| 条件 | Agent |
|------|-------|
| `ACTIVE=1` + 每次 wake / loop tick / 空消息 / `继续` | **无问询**执行下一未勾 Stage 项 |
| Stage N 全 `[x]` | **自动「定」** ROADMAP 下一版（Stage 16 为终点） |
| Stage **D** 毕业门禁绿 | **auto commit** + **tag** + **GitHub Release**；毕业允许 push |
| 非毕业 WIP | **不 push** |
| Hard block | 停；写清原因到看板 |
| 用户 `停` 或 `ACTIVE=0` | 停手 |
| 范围 | **仅** ROADMAP/SCOPE 已列信任门；禁止发明无界功能 |

---

## 诚实边界（对外一句话）

**YOYO 1.0 要把 v0.9 洞清单定稿为全关或诚实 SCOPE-CUT 写入 RELEASE——仍是 detection bar，不是 Thompson 证明；OW-\* 未关前不得假装已关。**

---

*更新：2026-08-29 · v0.9.0 毕业后定稿 Stage 16 / v1.0 · 见 `STAGE16_OWNER_CHECKLIST.md`*
