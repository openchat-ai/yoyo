# Stage 8 负责人看板（v0.2 · 通用自举主线）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 8 的 A/B/C/D 每一项，毕业时必须能回答：**它如何把更多编译器字节纳入三链 DDC + Lock 监控？** v0.2 的 full body、libyoyo、M4 链是 **扩面手段**，不是产品终点。

> **用途**：v0.1 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.2.md` — full body + libyoyo + 真实 I/O + M4 自举链；**非** MCU 主赛道。  
> **基线**：Stage 7 已毕业（2026-08-26）；v0.1 pin `0275802d2b4459e6…`。

## 🎯 Stage 8 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.2 毕业见 SCOPE-v0.2.md + RELEASE-v0.2.md
                              → v0.3 已毕业：SCOPE-v0.3.md + STAGE9 · 下一主线 v0.4：SCOPE-v0.4.md + STAGE10
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE8_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE8_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.2.md` |

相关：`STAGE4_OWNER_CHECKLIST.md`（Stage 4–7 历史）、`STAGE9_OWNER_CHECKLIST.md`（v0.3 已毕业）、`STAGE10_OWNER_CHECKLIST.md`（v0.4 下一主线）、`BACKEND_SUPPORT.md`、`RELEASE-v0.1.md`、`RELEASE-v0.2.md`、`RELEASE-v0.3.md`。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **单轨（默认）** | `继续` / `开工` / `auto` — master 上按 **A→B→C→D** 下一未勾项 |
| **看板** | Agent 优先打开本文件；Stage 4–7 项已全部 `[x]` 时不回改 |

### 毕业顺序（严格）

1. **A** — libyoyo 真实平台 I/O（D-1）
2. **B** — Full body 编译器路径（W5.5 body）
3. **C** — 扩展自举链 M2→M3→M4（Win + Linux）
4. **D** — v0.2 毕业门禁 + Relock + 文档收口

**不要跳关**：B 依赖 A（自举需真实读写）；C 依赖 B（M4 需 full body gen3）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 8 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage5-win-selfhost.ps1
.\scripts\verify-lock-pin.ps1
```

---

## Stage 8 毕业四门

### 待做

- [x] **A：libyoyo 真实平台 I/O** — D-1 关闭（2026-08-27）；Win/Linux `ALLOC`/`LOAD_FILE`/`WRITE_FILE` → `platform_io.rs`（Win kernel32 IAT + Linux syscalls）；Stub 仍 movabs+store 供 golden · **信任链**：gen12 SHA `e92520ea`（I/O 段在 `.text` DDC 窗口内）；gen1≡gen2 EQUAL；lock pin 不变
- [x] **B：Full body 编译器路径** — W5.5 body 回收（2026-08-27）；`test fullbody` 绿（788 handlers · gen12 DDC · gen2rt→output.exe）；`.ty`/`.tyb` bootstrap EQUAL · **信任链**：17920-byte `.text` 全 body 在 gen12/fullbody 窗口；非仅 W-SM scoped=34
- [x] **C：扩展自举链 M2→M3→M4** — Win + Linux gen2→gen3→gen4 全绿（2026-08-27）；`stage8-extended-selfhost.ps1/.sh` 退出码 0；gen4≡gen3_direct `.text` DDC EQUAL · **信任链**：Win `.text` hash `e92520ea…`（17920-byte gen12 窗口）；Linux `dab59f96…`；gen1 H_00 runtime selfhost 仍 RED（pre-existing，embedded path 不受阻）
- [x] **D：v0.2 毕业门禁** — A/B/C 全绿 + Lock pin 复验（不变 · Decision #25）+ `SCOPE-v0.2.md` 毕业判定 + `RELEASE-v0.2.md`（2026-08-27）· **信任链**：pin `0275802d…` 仍锁 788-handler 源码；gen12 `e92520ea` + M4 parity 扩 DDC 观测面；gen1 H_00 runtime RED（pre-existing）已记入 RELEASE

### 可选 · 低优先级（不挡 v0.2 毕业）

- MCU / `custom-mcu` 扩芯片 — 仅当 A–D 全绿且负责人点名；**默认不做**

---

## 对 AI 说什么（复制粘贴话术）

### 每日 test all 红了

```text
F:\yoyo STAGE8 看板：cargo run -- test all 失败。
请根据完整终端输出修回归，保证 v0.1 基线（ddc/golden/backends/lock）不退化。
约束：最小 diff；不要 push。
```

### 任务 A — libyoyo 真实平台 I/O

```text
Stage 8 毕业项 A：关闭 D-1 平台 I/O stub。
Win32Platform / LinuxPlatform 的 emit_alloc、emit_load_file、emit_write_file 改走 libyoyo 真实实现（VirtualAlloc/ReadFile/WriteFile · Linux mmap/syscall）。
生产路径不再 movabs+store 占位；JS peer 可保持分叉但 Rust 链 runtime 必须能真实读写文件。
验收：cd F:\yoyo\yoyo-rust\verifier && cargo run -- test all 退出码 0；
platform I/O 冒烟（读写的 golden 或新增 fixture PASS）；stage5-win-selfhost.ps1 仍绿。
更新 BACKEND_SUPPORT.md Known gaps 若删 D-1 stub 说明。
信任链验收：自举读写路径不再走 movabs+store stub；gen2 产物的 I/O 相关段可被 DDC/parity 观测（或文档说明观测点）。
```

### 任务 B — Full body 编译器路径

```text
Stage 8 毕业项 B：回收 W5.5 full body（SCOPE-CUT）。
yoyo.ty 完整编译器体（非 scoped startup 子集）须能编译自举输入并产出可链接 gen 二进制。
验收：full-body compile 测试或扩展 golden 绿；cargo run -- test all 退出码 0；gen12 或等价 parity 仍绿。
依赖 A 已绿（编译器运行时需要真实 LOAD/WRITE）。
信任链验收：full body 编译器发出的 .text/.data 字节纳入 golden 或 section-ddc 范围（非仅 scoped startup）；毕业说明中写明 DDC 覆盖从 scoped 扩到 full body 的边界。
```

### 任务 C — 扩展自举链 M2→M3→M4

```text
Stage 8 毕业项 C：Win + Linux 扩展自举 M2→M3→M4。
新增 scripts/stage8-extended-selfhost.ps1（及 .sh 或 WSL 等价）：gen2→gen3→gen4 无 AV/挂死，output 存在。
gen4 与 gen3_direct 或 section-ddc 监控 EQUAL（文档化 SHA）。
验收：.\scripts\stage8-extended-selfhost.ps1 退出码 0；stage5-win-selfhost.ps1 仍绿；Linux 脚本退出码 0。
依赖 B 已绿。
信任链验收：gen4 与 gen3_direct 或 section-ddc EQUAL；M4 链与现有 gen12 监控同一套门禁（文档化 SHA / section 列表）；不得引入「M4 跳过 DDC」捷径。
```

### 任务 D — v0.2 毕业门禁

```text
Stage 8 毕业项 D：v0.2 毕业收口。
A/B/C 均已绿；走 Lock Protocol 8-step Relock（full body pin 更新）；verify-lock-pin + verify-yoyo-ty 退出码 0。
写 RELEASE-v0.2.md 草稿；PROMPT Week 轴加 Stage 8 GREEN 行。
验收：test all + test lock + verify-lock-pin + stage8-extended-selfhost 全 0；STAGE8 A/B/C/D 可勾。
信任链验收：Relock 新 pin 覆盖 full body + M4 变更；RELEASE-v0.2.md 诚实写 DDC=detection 非 proof；每项 A/B/C 的「如何加强 DDC/Lock 覆盖」已写入毕业判定。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — YOYO 核心是 DDC + Lock 检测编译器级后门，不是造语言。每项毕业须说明：**如何加强 DDC/Lock 覆盖**；不得为「能跑」牺牲监控面。
1. **v0.2 = 通用自举主线** — MCU 脚手架不挡毕业；除非明确点名否则不做芯片后端。
2. **v0.1 不退化** — Stage 8 每项验收必须含 `test all` 或等价 CI 块。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 C/Rust 替代；DDC 仍是 detection 不是 proof；不广告 Thompson 免疫。
5. **不替 AI push** — 发布 v0.2 tag 由负责人另行决定。

---

*创建：2026-08-27 · Stage 8 A/B/C/D 全绿（2026-08-27）· v0.2 毕业门禁通过 · 见 `RELEASE-v0.2.md`*

**维护记录**：2026-08-28 — v0.2.0 后维护节拍全绿（test all/lock/gen12/fullbody · lock-pin · win+linux M4 selfhost）；无代码变更。  
**维护记录**：2026-08-28（二次）— 维护节拍再验全绿（test all/lock/gen12/fullbody · lock-pin · verify-ty · stage5/stage8 Win + WSL M4）；无代码变更。

---

## v0.2 已毕业 → v0.3 已毕业 → v0.4 下一主线

> Stage 8 勾选 **保持不变**，勿回改。发 `继续` 走 Stage 10：

| 文档 | 用途 |
| ---- | ---- |
| [`SCOPE-v0.4.md`](./SCOPE-v0.4.md) | v0.4 一页纸：北星、IN/OUT、与 v0.3 剩余面关系、毕业门禁 |
| [`STAGE10_OWNER_CHECKLIST.md`](./STAGE10_OWNER_CHECKLIST.md) | Stage 10 actionable 看板（A→D）；从 **A（runtime.dll 面）** 起 |
| [`SCOPE-v0.3.md`](./SCOPE-v0.3.md) / [`STAGE9_OWNER_CHECKLIST.md`](./STAGE9_OWNER_CHECKLIST.md) | v0.3 历史（勿回改勾选） |
