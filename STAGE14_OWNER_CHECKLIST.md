# Stage 14 负责人看板（v0.8 · 窗外字节 / SCOPE-CUT · Lock 硬化）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 14 的 A/B/C/D 每一项，毕业时必须能回答：**它如何缩小窗外仍 DIFF 字节（或诚实 SCOPE-CUT），或加厚 Lock 硬化可观测面？** v0.8 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.7 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.8.md` — 窗外字节 / SCOPE-CUT 草案 + Lock 硬化；**非** MCU / Morph 主赛道。  
> **基线**：Stage 13 已毕业（2026-08-29）；v0.7.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.7.md`（仍 Rust runtime + LoadLibrary/libdl；full `.text` 可 DIFF；seed 仍 Rust 发射）。

## 🎯 Stage 14 进度

```text
[ ] A  [ ] B  [ ] C  [ ] D   →  从 A 起；见 SCOPE-v0.8.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE14_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE14_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.8.md` |

相关：`STAGE13_OWNER_CHECKLIST.md`（v0.7 已毕业）、`RELEASE-v0.7.md`、`BACKEND_SUPPORT.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v0.9/Stage15 并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–13 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 窗外字节收口或 SCOPE-CUT 草案（**full `.text` DIFF 主洞**）
2. **B** — Lock 硬化
3. **C** — v0.7 回归不退化（门禁加固 / 观测）
4. **D** — v0.8 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；D 依赖 A/B/C。

**下一项** = **A**（窗外字节收口或 SCOPE-CUT 草案）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 14 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage13-v06-regress.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
# Stage 14 gates: stage14-* （落地后补）
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 14 毕业四门

### 待做

- [ ] **A：窗外字节收口或 SCOPE-CUT 草案** — 缩小 H_00/extract/runtime/IAT 等窗外盲区，或起草诚实 SCOPE-CUT · **信任链**：须可脚本验收；不得假装 full `.text` EQUAL
- [ ] **B：Lock 硬化** — pin / Relock 纪律门禁加厚 · **信任链**：改源必 Relock+Decision；未改则钉 Decision #25
- [ ] **C：v0.7 回归不退化** — stage13/stage12/stage11/stage10/stage9/fullbody/lock/gen12 门禁保持绿 · **信任链**：扩面不丢 v0.7 基线
- [ ] **D：v0.8 毕业门禁** — A/B/C 全绿 + Lock 复验（改源则 Relock）+ `SCOPE-v0.8.md` 毕业判定 + `RELEASE-v0.8.md` · **信任链**：RELEASE 诚实写 DDC=detection 非 proof

### 可选 · 低优先级（不挡 v0.8 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — 仍诚实剩余；仅当负责人点名或 A 明确关洞，勿偷塞进 v0.8 IN

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — 窗外字节 / SCOPE-CUT

```text
Stage 14 毕业项 A：窗外字节收口或 SCOPE-CUT 草案（v0.8 主信任门）。
目标：缩小 full .text DIFF 窗外盲区，或起草诚实 SCOPE-CUT；不得假 EQUAL。
验收：新增/加固 scripts/stage14-* 退出码 0；
cargo run -- test all 退出码 0；stage13 / stage12 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — Lock 硬化

```text
Stage 14 毕业项 B：Lock 硬化。
验收：可机器脚本 exit 0；改源必 Relock；v0.7 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.7 回归不退化

```text
Stage 14 毕业项 C：加固/确认 v0.7 门禁不退化。
验收：stage13-* + stage12-* + stage11-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.8 毕业门禁

```text
Stage 14 毕业项 D：v0.8 毕业收口。
写 RELEASE-v0.8.md；SCOPE-v0.8 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v0.8 = 窗外字节 / SCOPE-CUT · Lock 硬化** — MCU / Morph 不挡毕业。
2. **v0.7 不退化** — 每项验收必须含 stage13 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary / seed 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-29 · v0.7.0 后定稿 Stage 14 / v0.8 · 见 `SCOPE-v0.8.md` · 长期 → `SCOPE-v1.0.md` / `ROADMAP-TO-1.0.md`*
