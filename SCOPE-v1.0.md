# YOYO 1.0 — Scope Boundary（负责人一页纸）

> **前提**：已发布 v0.1–**v0.7.0**；**现主线** Stage 14 / v0.8。  
> **路线图**：`ROADMAP-TO-1.0.md`。**无人值守开关**：`AUTO_TO_1.0.md`（`ACTIVE=1` = 推到 1.0）。  
> **诚实**：不能一夜魔法到 1.0；AUTO = 按 ROADMAP gates 连续打磨；耗时取决于宿主面（尤其 YOYO-built runtime）。DDC = detection，非 Thompson-proof。

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

## 版本化里程碑（v0.4 → v1.0）

每版 **≤5 门**。AUTO 模式下 Stage 全绿后 **agent 自动「定」下一版**（创建 SCOPE + STAGE 看板），不等用户口头确认。

| 版本 | Stage | 信任主题（≤5 门） | 产物 |
|------|-------|-------------------|------|
| **v0.4** | 10 | runtime 面 · Linux 纯 M4 · asm I/O · 毕业 | ✅ `RELEASE-v0.4` · tag `v0.4.0` |
| **v0.5** | 11 | YOYO-built/更薄 runtime · 收缩 LoadLibrary host · 回归 · 毕业 | ✅ `RELEASE-v0.5` · tag `v0.5.0` |
| **v0.6** | 12 | 三 peer I/O · section-ddc on selfhost body · 回归 · 毕业 | ✅ `RELEASE-v0.6` · tag `v0.6.0` |
| **v0.7** | 13 | seed/link host · 跨平台 parity · Relock 纪律 · 毕业 | ✅ `RELEASE-v0.7` · tag `v0.7.0` |
| **v0.8** | 14 | 窗外字节再收或 SCOPE-CUT 草案 · Lock 硬化 · 毕业 | `SCOPE-v0.8` · `STAGE14` |
| **v0.9** | 15 | 洞清单 · 关或 SCOPE-CUT · 预跑门禁 · 毕业 | `SCOPE-v0.9` · `STAGE15` |
| **v1.0** | 16 | 全关或 SCOPE-CUT 定稿 · RELEASE · tag · detection 话术 | `STAGE16` · `RELEASE-v1.0` + tag |

> **v0.8 现状**：Stage 14 从 A（窗外字节 / SCOPE-CUT 草案）起；v0.7 已毕业勿回改。

---

## 永远 OUT / post-1.0

Morph 产品 · MCU 主赛道 · C/Rust 替代宣称 · Thompson-proof · TheoryManifest/CDS · macOS 生产门禁不阻塞 · G06+ 默认不扩

---

## AUTO-TO-1.0 契约（无人值守）

**开关**：`AUTO_TO_1.0.md` 中 `ACTIVE=1`（用户已开启：离电脑 / 自动持续 / 推到1.0）。

| 条件 | Agent |
|------|-------|
| `ACTIVE=1` + 每次 wake / loop tick / 空消息 / `继续` | **无问询**执行下一未勾 Stage 项 |
| Stage N 全 `[x]` | **自动「定」** ROADMAP 下一版：写 SCOPE + `STAGEx_OWNER_CHECKLIST.md`，立刻做第一项 |
| Stage **D** 毕业门禁绿 | **auto commit** + **tag** + **GitHub Release**（对齐 v0.1–v0.3）；毕业允许 push |
| 非毕业 WIP | **不 push** |
| Hard block（需人类密钥/硬件/不可 SCOPE-CUT 的政策） | 停；写清原因到看板；保持 `ACTIVE=1` 除非用户改 |
| 用户 `停` 或 `ACTIVE=0` | 停手 |
| 范围 | **仅** ROADMAP/SCOPE 已列信任门；禁止发明无界功能 |

---

## 诚实预期

AUTO 是连续打磨 gates，不是魔法完成。最难通常是 v0.5+ runtime。不可关的洞 → SCOPE-CUT + RELEASE 诚实写出后仍可毕业。

---

*更新：2026-08-29 · AUTO-TO-1.0 无人值守契约生效 · 现主线 Stage 14 / v0.8*
