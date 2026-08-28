# Stage 10 负责人看板（v0.4 · 宿主信任面再收）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 10 的 A/B/C/D 每一项，毕业时必须能回答：**它如何把更多自举字节纳入三链 DDC + Lock 监控，或缩小对嵌入 runtime / Rust 宿主包装的信任？** v0.4 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.3 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.4.md` — runtime.dll 面 + Linux 纯 M4 + asm I/O；**非** MCU / Morph 主赛道。  
> **基线**：Stage 9 已毕业（2026-08-28）；v0.3.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.3.md`。

## 🎯 Stage 10 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.4 已毕业（2026-08-28）· 见 SCOPE-v0.4.md / RELEASE-v0.4.md
                              → 下一主线 v0.5：SCOPE-v0.5.md + STAGE11_OWNER_CHECKLIST.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE10_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE10_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.4.md` |

相关：`STAGE9_OWNER_CHECKLIST.md`（v0.3 已毕业）、`STAGE8_OWNER_CHECKLIST.md`、`STAGE4_OWNER_CHECKLIST.md`（历史）、`BACKEND_SUPPORT.md`、`RELEASE-v0.3.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守；Stage 全绿后 agent 自动定下一版）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v0.5/Stage11 并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–9 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 收缩 / 替换嵌入式 `yoyo_runtime.dll` 面（**最大剩余信任洞**）
2. **B** — Linux ELF H_00 / 纯 M4（关 `--selfhost`）
3. **C** — Python asm peer 平台 I/O 对齐
4. **D** — v0.4 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；C 建议 A/B 已有可观测契约后再对齐 peer；D 依赖 A/B/C。

**第一次 `继续`** = 做 **A**（runtime.dll 面）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 10 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage8-extended-selfhost.ps1
.\scripts\stage9-gen1-h00-selfhost.ps1
.\scripts\stage9-js-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage10-runtime-surface.ps1
.\scripts\verify-lock-pin.ps1
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 10 毕业四门

### 待做

- [x] **A：收缩 / 替换嵌入式 `yoyo_runtime.dll` 面** — 缩小每个 genN 对 Rust 编译 runtime DLL 的信任：迁出、缩小、或把关键路径纳入可机器观测的 DDC/parity（策略落地须可脚本验收）· **信任链**（2026-08-28）：`scripts/stage10-runtime-surface.ps1` GREEN；DLL **485888→231936** B（fail-closed MAX **250000**；`default-features=false` + `opt-level=z`）；genN PE **576512→322560**；gen12 窗仍 **18432** B，SHA `b609a735`→`43ffde58`（H_00 stub 含 `dll_embed_size`）；**仍**嵌 Rust DLL、窗外主体仍不可 DDC（诚实边界）
- [x] **B：Linux ELF H_00 / 纯 M4** — ELF 纯路径 gen1→gen4 **无** `bootstrap --selfhost`；gen4≡gen3_direct EQUAL；Win stage9-pure-m4 / stage8 不退化 · **信任链**（2026-08-28）：`scripts/stage10-linux-pure-m4.sh` GREEN（WORKDIR prefer `/tmp` 避 DrvFs race）；`yoyo link --target=linux` → H_00 extract+`execve` trampoline（嵌 `libyoyo_runtime.so` + `linux_h00_tramp.elf`；LEA REX=`0x49`；page-aligned PT_LOAD；partial-write loop）；gen1→gen4 零参；gen4≡gen3_direct full-ELF DDC EQUAL（sha `085d07d4…` · **704512** B）；stage8-extended-selfhost.sh / Win stage9-pure-m4 / stage10-runtime-surface 仍绿；**仍**嵌 Rust `.so` + 系统 libdl trampoline（诚实边界）
- [x] **C：Python asm peer 平台 I/O 对齐** — asm 对 `0x20/0x50/0x51` 对齐 Rust/JS 真实 I/O（或可观测等价）；消除 movabs+store 盲区 · **信任链**（2026-08-28）：`scripts/stage10-asm-peer-io.ps1` GREEN（fail-closed）；`yoyo-asm/platform_io.py` + `asm.py` `--target=win32|linux|stub`；win32 ALLOC/LOAD/WRITE **byte-equal** Rust（41/144/127 B；==JS）；linux ALLOC not stub + `0F 05`；stub 仍 G-SM-IO movabs+store；`cargo run -- test all` + `stage9-js-peer-io.ps1` 仍绿；**仍**可能 DIFF 全量 `yoyo.ty` section-ddc（H_00 / IAT / embedded runtime）
- [x] **D：v0.4 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.4.md` 毕业判定 + `RELEASE-v0.4.md` / `RELEASE-NOTES-v0.4.md`（2026-08-28）· **信任链**：Decision #25 pin 仍权威；gen12 `43ffde58` / 18432B；DLL 231936B fail-closed；Linux pure M4 / asm I/O 门禁绿；RELEASE 诚实写 DDC=detection、Rust runtime 仍嵌入

### 可选 · 低优先级（不挡 v0.4 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- 进一步收缩 seed/`yoyo link` 参考宿主 — 仅当 A/B 已绿且负责人点名

---

## 对 AI 说什么（复制粘贴话术）

### 每日 test all 红了

```text
F:\yoyo STAGE10 看板：cargo run -- test all 失败。
请根据完整终端输出修回归，保证 v0.3 基线（ddc/golden/backends/lock/fullbody/H_00/JS peer/Win 纯 M4）不退化。
约束：最小 diff；不要 push。
```

### 任务 A — 收缩 / 替换嵌入式 runtime.dll 面

```text
Stage 10 毕业项 A：收缩或替换每个 genN 嵌入的 yoyo_runtime.dll 宿主面（v0.3 最大诚实剩余洞）。
目标：减少对 Rust 编译 runtime DLL 的信任——迁出、缩小、或把关键路径字节纳入可脚本观测的 DDC/parity 窗口；不得用「为能跑而跳过 DDC」捷径。
验收：新增或扩展 scripts/stage10-runtime-surface.ps1（名称可调整）退出码 0；
文档化前后信任边界（相对 gen12 窗口 / SHA）；
cd F:\yoyo\yoyo-rust\verifier && cargo run -- test all 退出码 0；
stage5 / stage8 / stage9-h00 / stage9-peer-io / stage9-pure-m4 仍绿（不退化）。
更新 BACKEND_SUPPORT.md / STAGE10 勾选说明中的信任链观测点。
信任链验收：窗外 host 字节减少或进入监控；「绿」不再只能靠不可比对的嵌入 DLL 黑箱。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — Linux ELF H_00 / 纯 M4

```text
Stage 10 毕业项 B：Linux ELF H_00 / 纯 M4（关闭 RELEASE-v0.3 的 --selfhost 剩余面）。
目标：至少一条可机器验收的 Linux 路径：gen1→gen4 不调用 bootstrap --selfhost；gen4≡gen3_direct DDC EQUAL。
验收：scripts/stage10-linux-pure-m4.sh（或 WSL 等价）退出码 0；
stage8-extended-selfhost.sh 回归仍绿；Win stage9-pure-m4.ps1 不退化。
信任链验收：Linux「绿」不再只能靠 Rust host --selfhost 编排；不得引入跳过 DDC 的捷径。
依赖：建议 A 已绿或并行实现但勾选在 A 之后。
约束：最小 diff；不要 push。
```

### 任务 C — Python asm peer 平台 I/O 对齐

```text
Stage 10 毕业项 C：Python asm peer 平台 I/O 对齐以服务 DDC。
asm 链对 0x20/0x50/0x51 的生产 emit 须对齐 Rust/JS 真实 I/O 语义（或可观测的等价契约），消除 movabs+store stub 盲区。
验收：scripts/stage10-asm-peer-io.ps1（或等价）退出码 0；相关 peer/DDC 冒烟绿；
cargo run -- test all 退出码 0；v0.3 stage9 门禁仍绿。
信任链验收：三链对平台 I/O 相关字节/语义进入可比对范围；文档写明观测点与仍存分叉（若有）。
依赖：建议 A/B 已绿或并行但勾选在其后。
约束：最小 diff；不要 push。
```

### 任务 D — v0.4 毕业门禁

```text
Stage 10 毕业项 D：v0.4 毕业收口。
A/B/C 均已绿；若改了 yoyo.ty 则走 Lock Protocol 8-step Relock；verify-lock-pin + verify-yoyo-ty 退出码 0。
写 RELEASE-v0.4.md；SCOPE-v0.4.md 填写毕业判定；PROMPT Week 轴加 Stage 10 GREEN 行（若适用）。
验收：test all + test lock + fullbody + gen12 + verify-lock-pin + stage5/8/9 + stage10 门禁全 0；STAGE10 A/B/C/D 可勾。
信任链验收：RELEASE 诚实写 DDC=detection 非 proof；逐项写明如何加强 DDC/Lock / 缩小宿主信任；禁止 Thompson-proof / C 替代话术。
约束：不要 push（tag 由负责人决定）。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — YOYO 核心是 DDC + Lock 检测编译器级后门，不是造语言。每项毕业须说明：**如何加强 DDC/Lock 覆盖或缩小宿主信任洞**；不得为「能跑」牺牲监控面。
1. **v0.4 = 宿主信任面再收** — MCU / Morph / SIMD 不挡毕业；除非明确点名否则不做。
2. **v0.3 不退化** — Stage 10 每项验收必须含 `test all` 或等价，且 stage9 门禁仍绿。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 C/Rust 替代；DDC 仍是 detection 不是 proof；不广告 Thompson 免疫；不宣称 DDC 覆盖 genN 每个字节。
5. **不替 AI push** — 发布 v0.4 tag 由负责人另行决定。

---

## 维护节拍（v0.3 已毕业后）

在 Stage 10 进行中，每周至少一轮：

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
cargo run -- test lock
cargo run -- test gen12
cargo run -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage8-extended-selfhost.ps1
.\scripts\stage9-gen1-h00-selfhost.ps1
.\scripts\stage9-js-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage10-runtime-surface.ps1
# WSL: bash scripts/stage8-extended-selfhost.sh
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

红了先回归，再继续未勾的 Stage 10 项。

---

*创建：2026-08-28 · v0.3.0 后定稿 Stage 10 / v0.4 · 见 `SCOPE-v0.4.md` · 长期 → `SCOPE-v1.0.md` / `ROADMAP-TO-1.0.md`*
