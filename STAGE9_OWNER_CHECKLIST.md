# Stage 9 负责人看板（v0.3 · 信任洞收口）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 9 的 A/B/C/D 每一项，毕业时必须能回答：**它如何把更多自举字节纳入三链 DDC + Lock 监控，或缩小对 Rust 宿主包装的信任？** v0.3 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.2 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.3.md` — gen1 H_00 纯路径 + JS I/O 对齐 + 收紧 M4 host wrapper；**非** MCU / Morph 主赛道。  
> **基线**：Stage 8 已毕业（2026-08-27）；v0.2.0 已发；Lock pin `0275802d…`（Decision #25，**Stage 9 未改源码 · 无 Relock**）；v0.3 已毕业（2026-08-28 · `RELEASE-v0.3.md`）。

## 🎯 Stage 9 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.3 已毕业（2026-08-28）· 见 SCOPE-v0.3.md / RELEASE-v0.3.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE9_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE9_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.3.md` |

相关：`STAGE8_OWNER_CHECKLIST.md`（v0.2 已毕业）、`STAGE4_OWNER_CHECKLIST.md`（Stage 4–7 历史）、`BACKEND_SUPPORT.md`、`RELEASE-v0.2.md`。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **单轨（默认）** | `继续` / `开工` / `auto` — master 上按 **A→B→C→D** 下一未勾项 |
| **看板** | Agent 优先打开本文件；Stage 4–8 项已全部 `[x]` 时不回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — gen1 H_00 / 无 sidecar 纯自举路径（最大信任洞）
2. **B** — JS peer 平台 I/O 对齐（DDC 盲区）
3. **C** — 收紧 M4 host `bootstrap --selfhost` wrapper
4. **D** — v0.3 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；C 依赖 A 已绿（纯路径叙事成立后再收 wrapper）；D 依赖 A/B/C。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 9 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage8-extended-selfhost.ps1
.\scripts\verify-lock-pin.ps1
```

---

## Stage 9 毕业四门

### 待做

- [x] **A：gen1 H_00 / 纯自举路径** — 关闭 v0.2 Known RED（2026-08-28）：gen1 PE entry → H_00（无 genNrt 入口包装）；H_00 jmp 提取 runtime → `output.exe` exit 0；`stage5` + `stage9-gen1-h00-selfhost.ps1` GREEN · **信任链**：自举入口进入可回归监控；产物进 gen12/fullbody `.text`（18432B）；仍嵌 `yoyo_runtime.dll`（诚实边界）
- [x] **B：JS peer 平台 I/O 对齐** — 关闭 DDC 盲区（2026-08-28）：JS 生产 PE `setEmitPlatform('win32')` + `platform-io.js` 与 Rust `platform_io.rs` 对 `0x20/0x50/0x51` **字节相等**；golden 默认仍 stub（G-SM-IO）；`stage9-js-peer-io.ps1` + PEER-IO-* GREEN · **信任链**：JS↔Rust win32 I/O 进入可比对范围；asm 仍 stub（诚实分叉）
- [x] **C：收紧 M4 host wrapper** — Win 纯 H_00 链 gen1→gen4（2026-08-28）：`stage9-pure-m4.ps1` 不调用 `bootstrap --selfhost`；gen4≡gen3_direct `.text` DDC EQUAL；stage8 genNrt 回归仍绿 · **信任链**：M3→M4 在产物内 H_00 完成；仍嵌 `yoyo_runtime.dll`；Linux 仍走 `--selfhost`（诚实边界）
- [x] **D：v0.3 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.3.md` 毕业判定 + `RELEASE-v0.3.md`（2026-08-28）· **信任链**：Decision #25 pin 仍权威；gen12 `b609a735` / 18432B；RELEASE 诚实写 DDC=detection、runtime.dll / Linux `--selfhost` / asm stubs

### 可选 · 低优先级（不挡 v0.3 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**

---

## 对 AI 说什么（复制粘贴话术）

### 每日 test all 红了

```text
F:\yoyo STAGE9 看板：cargo run -- test all 失败。
请根据完整终端输出修回归，保证 v0.2 基线（ddc/golden/backends/lock/fullbody/M4）不退化。
约束：最小 diff；不要 push。
```

### 任务 A — gen1 H_00 / 纯自举路径

```text
Stage 9 毕业项 A：关闭 gen1 H_00 runtime selfhost RED（v0.2 Known RED）。
目标：gen1 在不依赖 genNrt 嵌入 startup 捷径的路径上，能完成自举并产出 output.exe（零参 H_00 或文档化的纯 runtime 定义须写清）。
验收：新增或扩展 scripts/stage9-gen1-h00-selfhost.ps1（名称可调整）退出码 0 + 产物存在；
cd F:\yoyo\yoyo-rust\verifier && cargo run -- test all 退出码 0；
stage5-win-selfhost.ps1 与 stage8-extended-selfhost.ps1 仍绿（不退化）。
更新 BACKEND_SUPPORT.md / STAGE9 勾选说明中的信任链观测点。
信任链验收：自举「绿」不再只能靠 Rust 嵌入 startup；H_00 路径产物可被脚本化 DDC/parity 或等价门禁观测。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — JS peer 平台 I/O 对齐

```text
Stage 9 毕业项 B：JS peer 平台 I/O 对齐以服务 DDC。
JS 链对 0x20/0x50/0x51 的生产 emit 须对齐 Rust 真实 I/O 语义（或可观测的等价契约），消除「Rust 真 syscall、JS movabs+store」盲区。
验收：node .\yoyo-js\scripts\golden.js 退出码 0；相关 peer/DDC 冒烟绿；
cargo run -- test all 退出码 0；v0.2 M4 脚本仍绿。
信任链验收：三链对平台 I/O 相关字节/语义进入可比对范围；文档写明观测点与仍存分叉（若有）。
依赖：建议 A 已绿或并行实现但勾选在 A 之后。
约束：最小 diff；不要 push。
```

### 任务 C — 收紧 M4 host wrapper

```text
Stage 9 毕业项 C：收紧 M4 对 host bootstrap --selfhost 包装的依赖。
目标：至少一条可机器验收的路径证明 gen3→gen4 更多在产物/进程内完成，而非仅靠 Rust host 脚手架编排。
验收：scripts/stage9-pure-m4.ps1（或扩展 stage8-extended-selfhost）退出码 0；
gen4 与 direct/parity 仍 EQUAL（文档化 SHA）；stage8 原门禁不退化。
信任链验收：减少 host-only 信任层；不得引入「为纯而跳过 DDC」捷径。
依赖 A 已绿。
约束：最小 diff；不要 push。
```

### 任务 D — v0.3 毕业门禁

```text
Stage 9 毕业项 D：v0.3 毕业收口。
A/B/C 均已绿；若改了 yoyo.ty 则走 Lock Protocol 8-step Relock；verify-lock-pin + verify-yoyo-ty 退出码 0。
写 RELEASE-v0.3.md；SCOPE-v0.3.md 填写毕业判定；PROMPT Week 轴加 Stage 9 GREEN 行（若适用）。
验收：test all + test lock + fullbody + gen12 + verify-lock-pin + stage5 + stage8 + stage9 门禁全 0；STAGE9 A/B/C/D 可勾。
信任链验收：RELEASE 诚实写 DDC=detection 非 proof；逐项写明如何加强 DDC/Lock / 缩小宿主信任；禁止 Thompson-proof / C 替代话术。
约束：不要 push（tag 由负责人决定）。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — YOYO 核心是 DDC + Lock 检测编译器级后门，不是造语言。每项毕业须说明：**如何加强 DDC/Lock 覆盖或缩小宿主信任洞**；不得为「能跑」牺牲监控面。
1. **v0.3 = 信任洞收口** — MCU / Morph / SIMD 不挡毕业；除非明确点名否则不做。
2. **v0.2 不退化** — Stage 9 每项验收必须含 `test all` 或等价，且 stage8 M4 仍绿。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 C/Rust 替代；DDC 仍是 detection 不是 proof；不广告 Thompson 免疫。
5. **不替 AI push** — 发布 v0.3 tag 由负责人另行决定。

---

## 维护节拍（v0.2 已毕业后）

在 Stage 9 进行中，每周至少一轮：

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
# WSL: bash scripts/stage8-extended-selfhost.sh
```

红了先回归，再继续未勾的 Stage 9 项。

---

*创建：2026-08-28 · v0.2.0 后定稿 Stage 9 / v0.3 · 见 `SCOPE-v0.3.md`*
