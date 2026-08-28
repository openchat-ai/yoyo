# YOYO v0.9 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.8.0** 已发布（见 `RELEASE-v0.8.md`）。Stage 14 A–D 全绿：窗外 SCOPE-CUT、Lock 硬化、v0.7 回归、毕业收口。
>
> **Sources：** `RELEASE-v0.8.md` 诚实剩余面、`SCOPE-CUT-v0.8-outside-window.md`、`SCOPE-CUT-v0.9-hole-inventory.md`、`STAGE14_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-29。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.9 每一项 IN 范围，都必须进一步收口洞清单（关或诚实 SCOPE-CUT），或加厚预跑门禁可观测面**：

- **洞清单收口** — 对 v0.8 CUT / RELEASE 诚实剩余（OW-\*、Rust runtime、LoadLibrary、seed）逐项关或再 CUT
- **预跑门禁** — 毕业前可机器预跑（一键串行 keep-green）
- **基线不退化** — v0.8 A/B/C 门禁 + v0.7 基线只增不减

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.8/v0.9 SCOPE-CUT ACTIVE（OW-H00/STUB/RT/IAT/SEED 仍 CUT）**不自动消失**。

---

## 愿景（负责人读）

YOYO v0.9 要 **收口 v0.8 RELEASE / SCOPE-CUT 诚实写出的洞清单**：能关则关，不能关则写清 SCOPE-CUT；并加厚预跑门禁，避免「毕业当天才发现回归」。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.9 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） | 状态 |
|---|------|------|------------------|------|
| 1 | **洞清单收口（关或 SCOPE-CUT）** | 对 OW-\* / RELEASE 剩余面逐项：CLOSED 或 CUT 再钉；不得假 EQUAL | **洞从「清单」变「可验」** | **A [x]** 2026-08-29 |
| 2 | **预跑门禁** | 一键/串行预跑：v0.8 gates + regress；毕业前可机器复验 | **少盲飞毕业** | **B [x]** 2026-08-29 |
| 3 | **v0.8 回归不退化** | stage14 A/B/C + stage13–9 + fullbody/lock/gen12 保持绿 | 扩面时不丢已有 DDC/Lock/CUT 基线 | **C [x]** 2026-08-29 |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.9.md`；诚实写仍存边界 · **Stage 15-D DONE 2026-08-29**：无 Relock；`RELEASE-v0.9.md` + tag `v0.9.0` | 对外 detection 话术 | **D [x]** 2026-08-29 |

**主验收看板**：`STAGE15_OWNER_CHECKLIST.md`（A→D）。进度：`[x] A [x] B [x] C [x] D`。

---

## v0.9 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| YOYO-built runtime（非 Rust） | 若未关仍 CUT；勿偷宣称已关 |
| 1.0 全关或 SCOPE-CUT 定稿 | Stage 16 / v1.0（已定 · `STAGE16_OWNER_CHECKLIST.md`） |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.8 诚实剩余面的关系

| v0.8 / `RELEASE-v0.8.md` / SCOPE-CUT | v0.9 回收 |
|-------------------------------------|-----------|
| **OW-H00 / OW-STUB** — H_00 slot + extract stub | Stage 15 **A [x]** → **CUT**（full `.text` DIFF；stub_nz=159） |
| **OW-RT** — Embedded Rust runtime | Stage 15 **A [x]** → **CUT**（exact embed；dll=154624） |
| **OW-IAT** — LoadLibrary / libdl | Stage 15 **A [x]** → **CUT**（LoadLibraryA + yoyo_rt.dll） |
| **OW-SEED** — Seed 仍 Rust 发射 | Stage 15 **A [x]** → **CUT**（Rust `yoyo link`；seed PE≤270000） |
| **REL-FULLTEXT / REL-STUBOS** | Stage 15 **A [x]** → **CUT**（诚实；非毕业 EQUAL） |
| **SCOPE-CUT ACTIVE** · full `.text` DIFF | 继续诚实；不得假 EQUAL；gate `stage15-hole-inventory.ps1` |
| DDC = detection 非 proof | Stage 15 **D [x]** · `RELEASE-v0.9.md` 强调 |

**A 验收钉（2026-08-29）：** `stage15-hole-inventory.ps1 -SkipBuild` exit 0 · `HOLE_INVENTORY status=ACTIVE closed=0 cut=7` · nested `stage14-outside-window-scope-cut -SkipBuild` exit 0 · `stage14-lock-harden -SkipBuild` PINNED Decision #25。

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-outside-window-scope-cut.ps1
.\scripts\stage14-lock-harden.ps1
.\scripts\stage14-v07-regress.ps1
.\scripts\stage15-hole-inventory.ps1
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 15 四门全 `[x]`** = v0.9 可发布候选。

**Stage 15-A（2026-08-29）：** `stage15-hole-inventory.ps1 -SkipBuild` exit 0 · `HOLE_INVENTORY status=ACTIVE full_text=DIFF body_window=EQUAL compared=17805 stub_nz=159 dll=154624 seed_pe=248832 embed_off=85543 closed=0 cut=7` · 文档 `SCOPE-CUT-v0.9-hole-inventory.md` · OW-\* + REL-\* 全 **CUT**（无假 CLOSED）· **未**宣称 full `.text` EQUAL · 嵌套 Stage 14-A 不退化。

**Stage 15-B（2026-08-29）：** `stage15-prerun.ps1 -SkipBuild` exit 0 · ALL_GREEN · 串行：`stage15-hole-inventory=0` + `stage14-v07-regress=0`（内含 stage14 A/B + stage13–9 + test all/lock/gen12/fullbody + WSL）· `cargo SKIP(-SkipBuild)` · `driver.lock` · named `-SkipBuild` · **零并行 cargo** · 少盲飞毕业。

**Stage 15-C（2026-08-29）：** `stage15-v08-regress.ps1 -SkipBuild` exit 0 · ALL_GREEN（stamp 01:34:49；D 复验 **01:40:31**）· 串行：`yoyo.exe test all/lock/gen12/fullbody=0` + stage13–9 + stage14 A/B + `stage15-hole-inventory=0` + WSL stage10-linux · `cargo SKIP(-SkipBuild)` · `driver.lock` · named `-SkipBuild` · **零并行 cargo** · v0.8 基线不退化。

**毕业判定（2026-08-29 · Stage 15-D）：** A/B/C/D 全绿 · **无 Relock**（Decision #25）· `verify-lock-pin` + `stage14-lock-harden.ps1 -SkipBuild` PINNED · `stage15-v08-regress.ps1 -SkipBuild` ALL_GREEN（**01:40:31**）· `RELEASE-v0.9.md` / `RELEASE-NOTES-v0.9.md` 诚实写 **HOLE_INVENTORY ACTIVE closed=0 cut=7** + DDC=detection · 仍 OW-\* Rust runtime / LoadLibrary / seed · 下一主线已定 `SCOPE-v1.0.md` + `STAGE16_OWNER_CHECKLIST.md` · tag `v0.9.0`

---

## 诚实边界（对外一句话）

**YOYO v0.9 已收口洞清单（关或 SCOPE-CUT）并加厚预跑门禁——仍是 detection bar，不是 Thompson 证明；OW-\*/REL-\* 全为 CUT（0 CLOSED）；不得假装 full `.text` EQUAL。A/B/C/D 全绿；v0.9.0 已发。**

---

*维护：Stage 15 毕业 2026-08-29 · 见 `RELEASE-v0.9.md` · 下一主线 → `SCOPE-v1.0.md` / `STAGE16_OWNER_CHECKLIST.md`。*
