# Stage 15 负责人看板（v0.9 · 洞清单收口 / 预跑门禁）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 15 的 A/B/C/D 每一项，毕业时必须能回答：**它如何收口洞清单（关或诚实 SCOPE-CUT），或加厚预跑可观测面？** v0.9 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.8 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.9.md` — 洞清单收口 · 预跑门禁；**非** MCU / Morph 主赛道。  
> **基线**：Stage 14 已毕业（2026-08-29）；v0.8.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.8.md` / `SCOPE-CUT-v0.8-outside-window.md`（SCOPE-CUT ACTIVE；OW-\*；仍 Rust runtime + LoadLibrary/libdl；seed 仍 Rust 发射）。

## 🎯 Stage 15 进度

```text
[ ] A  [ ] B  [ ] C  [ ] D   →  下一项 A；见 SCOPE-v0.9.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE15_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE15_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.9.md` |

相关：`STAGE14_OWNER_CHECKLIST.md`（v0.8 已毕业）、`RELEASE-v0.8.md`、`SCOPE-CUT-v0.8-outside-window.md`、`BACKEND_SUPPORT.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v1.0/Stage16 并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–14 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 洞清单收口（关或 SCOPE-CUT）
2. **B** — 预跑门禁
3. **C** — v0.8 回归不退化
4. **D** — v0.9 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：勾选顺序仍 A→B→C→D；D 依赖 A/B/C。

**下一项** = **A**（洞清单收口）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 15 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage14-outside-window-scope-cut.ps1
.\scripts\stage14-lock-harden.ps1
.\scripts\stage14-v07-regress.ps1
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
# Stage 15 gates: stage15-* （落地后补）
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 15 毕业四门

### 待做 / 已勾

- [ ] **A：洞清单收口（关或 SCOPE-CUT）** — 对 OW-H00/STUB/RT/IAT/SEED 及 `RELEASE-v0.8` 剩余面逐项 CLOSED 或再 CUT；机器门 exit 0；**不得**假装 full `.text` EQUAL · **信任链**：待填
- [ ] **B：预跑门禁** — 串行预跑 / 一键 keep-green；毕业前可机器复验 · **信任链**：待填
- [ ] **C：v0.8 回归不退化** — stage14 A/B/C + stage13–9 + fullbody/lock/gen12 全绿 · **信任链**：待填
- [ ] **D：v0.9 毕业门禁** — A/B/C 全绿 + Lock 复验（改源则 Relock）+ `SCOPE-v0.9.md` 毕业判定 + `RELEASE-v0.9.md` · **信任链**：RELEASE 诚实写 DDC=detection 非 proof

### 可选 · 低优先级（不挡 v0.9 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — 仅当 A 明确关 OW-RT；勿偷塞进 v0.9 IN 并宣称已关

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — 洞清单收口

```text
Stage 15 毕业项 A：洞清单收口（关或 SCOPE-CUT）（v0.9 主信任门）。
目标：对 OW-* / RELEASE-v0.8 剩余面逐项关或再 CUT；不得假 EQUAL。
验收：新增/加固 scripts/stage15-* 退出码 0；
cargo run -- test all 退出码 0；stage14 / stage13 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — 预跑门禁

```text
Stage 15 毕业项 B：预跑门禁。
验收：可机器脚本 exit 0；串行；零并行 cargo；v0.8 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.8 回归不退化

```text
Stage 15 毕业项 C：加固/确认 v0.8 门禁不退化。
验收：stage14-* + stage13-* + stage12-* + stage11-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.9 毕业门禁

```text
Stage 15 毕业项 D：v0.9 毕业收口。
写 RELEASE-v0.9.md；SCOPE-v0.9 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v0.9 = 洞清单收口 · 预跑门禁** — MCU / Morph 不挡毕业。
2. **v0.8 不退化** — 每项验收必须含 stage14 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary / seed 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-29 · v0.8.0 后定稿 Stage 15 / v0.9 · 见 `SCOPE-v0.9.md` · 长期 → `SCOPE-v1.0.md` / `ROADMAP-TO-1.0.md`*
