# SCOPE-CUT — v0.8 outside-window bytes（Stage 14-A 草案）

> **Status:** ACTIVE（草案 · 可机器验收）  
> **Gate:** `scripts/stage14-outside-window-scope-cut.ps1`  
> **Rule:** 本文件是 **诚实裁剪**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL。

---

## 为何 CUT

三 peer 在 **selfhost-body 窗**（PE startup + post-H_00 shared handlers）已 **EQUAL**（≥17013 B；观测 **17805** B）。  
full `.text` 仍 **DIFF**，因为窗 **外** 仍有宿主 / 发射面字节。若不把窗外钉成 SCOPE-CUT，验收会落进「整段 DIFF 但窗外仍算绿」的盲区。

**DDC = detection bar，不是 Thompson 证明。**

---

## CUT 清单（窗外 · 诚实剩余）

| ID | 区域 | 在哪 | 观测 / 钉 | v0.8 态度 |
|----|------|------|-----------|-----------|
| **OW-H00** | H_00 entry slot（18 B） | PE `.text` emit 头 | selfhost-body **跳过**该槽；Rust JMP+NOPs vs JS/asm SET+RET | **CUT** — 不对齐则 full `.text` DIFF |
| **OW-STUB** | H_00 extract stub tail | `.text` emit 后 | `stub_tail_nonzero` 钉 **[100, 2048]**；观测 **159** B（Rust-only） | **CUT** — 三 peer EQUAL 窗外 |
| **OW-RT** | Embedded Rust `yoyo_runtime.dll` | PE `.data` / embed | size **≤170000**；观测 **154624**；**exact embed** | **CUT** — 不在 gen12 / body 窗 |
| **OW-IAT** | LoadLibraryA / GetProcAddress / ExitProcess | PE IAT + stub 调用 | Stage 11-B 宿主加载面仍在 | **CUT** — 仍宿主加载，非 YOYO-built loader |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | host CLI | Stage 13-A 已 observe；不消除 | **CUT** — 不挡 v0.8；继续诚实 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v0.8-A 毕业话术（窗仍 DIFF 时）
- ❌ 「窗外已关」若 OW-\* 钉仍 ACTIVE
- ❌ Thompson-proof / 已消灭 Rust runtime

允许：✅ 「selfhost-body EQUAL + 窗外 SCOPE-CUT 已机器钉住」

---

## 机器验收（exit 0）

```powershell
cd F:\yoyo
.\scripts\stage14-outside-window-scope-cut.ps1
# 或已有 release 二进制：
.\scripts\stage14-outside-window-scope-cut.ps1 -SkipBuild
```

Gate 必须同时：

1. **可比窗绿** — 委托 `stage12-selfhost-body-section-ddc.ps1 -SkipBuild`（三 peer body EQUAL + stub floor）
2. **窗外可观测且封顶** — stub 区间、runtime.dll 天花板、LoadLibrary 宿主标记、seed PE 天花板
3. **诚实 full `.text`** — JS↔Rust full section-ddc：**DIFF → SCOPE-CUT ACTIVE**；若意外 EQUAL → 记 H_00/stub 可能已收口，**仍**要求 OW-RT/OW-IAT 钉住（不得整段宣称窗外已关）
4. **打印** `SCOPE_CUT status=…` 一行，便于 RELEASE / 看板引用

---

## 与 Stage 14 / v0.8 关系

| 项 | 说明 |
|----|------|
| **Stage 14-A** | 本草案 + gate = A 验收（收口观测盲区；非假 EQUAL） |
| **B/C/D** | Lock 硬化 / 回归 / 毕业另项；D 写 RELEASE 时引用本 CUT |
| **关闭路径** | 对齐 H_00/stub peer → 可把 OW-H00/OW-STUB 标 CLOSED；OW-RT 需 YOYO-built runtime 才关 |
| **Stage 15-A** | 逐项 disposition 见 `SCOPE-CUT-v0.9-hole-inventory.md` + `stage15-hole-inventory.ps1`（本文件仍为窗外基线） |

---

## 观测基线（2026-08-29 · Stage 14-A）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust | **DIFF**（compared_bytes 17920 vs stubbed Rust） |
| stub_tail_nonzero (Rust) | **159** |
| runtime.dll | **154624** |
| seed PE (Rust link) | **248832**（≤270000） |
| Lock pin | `0275802d…` Decision #25（本草案不改 `yoyo.ty`） |

---

*Stage 14-A · 打破后门魔咒：把窗外从「看不见」变成「CUT + 可脚本钉」*
