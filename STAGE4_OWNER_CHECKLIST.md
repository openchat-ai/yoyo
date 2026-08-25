# Stage 4 负责人看板

> **用途**：YOYO 项目负责人每日扫一眼——命令绿不绿、勾没勾。不必懂底层实现。  
> **基线**：commits 至 `62a9f2d`（Item 8c：Stage 4 毕业 A/B/C 全绿）。

## 🎓 Stage 4 已毕业

```text
[x] A  [x] B  [x] C   →  Stage 5 已毕业 → Stage 6 已毕业 → **下一主线：Stage 7**（见下方预置任务）
```

> **关于「打钩」**：本文件用 Markdown 写法 `- [x]` / `- [ ]`。在**源码视图**里看到的是方括号字母；用 **Markdown 预览**（右上角预览图标）才会显示为可勾选符号。`[x]` = 已勾，`[ ]` = 未勾。

---

## 如何打开看板


| 方式           | 操作                                         |
| ------------ | ------------------------------------------ |
| **完整路径**     | `F:\yoyo\STAGE4_OWNER_CHECKLIST.md`        |
| **Cursor 内** | `Ctrl+P` → 输入 `STAGE4_OWNER` → 回车          |
| **资源管理器**    | 打开 `F:\yoyo\`，双击本文件                        |
| **命令行打开**    | `cursor F:\yoyo\STAGE4_OWNER_CHECKLIST.md` |


相关矩阵文档：`[BACKEND_SUPPORT.md](./BACKEND_SUPPORT.md)`（DDC 表 + 已知缺口）。

---

## 零指令自动执行

> **你不必每次说「做 B」** — 发一个字即可，AI 自己读看板、挑下一项、干完、勾框。

### 怎么触发

| 方式 | 操作 |
| ---- | ---- |
| **单轨（默认）** | `继续` / `开工` / `auto` — master 上按 A→B→C 下一项顺序做 |
| **两板并行** | `继续 两板` 或 `继续两板` — PE + ELF 两路 worktree 并行（见下） |
| **右侧看板** | Agent 每次触发时自动在 **Glass 右侧面板**打开本文件（MCP `open_resource`） |
| **默认** | 打开本仓库 Agent 即加载规则 `.cursor/rules/stage4-auto-owner.mdc` |
| **定时（可选）** | Agent 里 `/loop 1d 继续 Stage4 自动负责人` — 每天自动跑一轮（见 Cursor loop 技能） |

### 单轨：`继续`

1. MCP 右侧打开本看板
2. 在 **master**（`F:\yoyo`）按毕业顺序找第一个未勾项：**A→B→C**（Stage 4）→ **Stage 5** → **Stage 6** → **Stage 7** 预置任务第一项
3. 按「对 AI 说什么」实现 + `cargo run -- test ddc` 验收
4. 绿了才勾 `[x]`；**不 push**；commit 仅当你明确说要 commit

### 两板并行：`继续 两板`

**适用**：B 项 Container DDC — PE 与 ELF 可并行，互不阻塞。

| 角色 | 位置 | 做什么 |
| ---- | ---- | ------ |
| **你** | 负责人 | 发 `继续 两板`；看 coordinator 汇报；合并冲突时扫一眼 |
| **Coordinator** | `F:\yoyo` master | 建 worktree、派两路 agent、两路都绿后 merge + 勾 B |
| **Agent PE** | `F:\yoyo-worktrees\stage4-pe` | 仅 PE32+ x64 container NOP+RET |
| **Agent ELF** | `F:\yoyo-worktrees\stage4-elf` | 仅 ELF64 x64 container NOP+RET |

**自动流程**：

1. `& F:\yoyo\scripts\stage4-two-board.ps1` — 创建/确认 worktree（可重复运行）
2. Coordinator **并行**启动 PE + ELF 两路子 agent（background）
3. 各 agent 在自家 worktree 跑 `cargo run -- test ddc`，汇报绿/红
4. **两路都绿** → `& F:\yoyo\scripts\stage4-two-board-merge.ps1` 合并进 master → master 上 ddc 全绿 → 勾 B `[x]`
5. **仅一路绿** → 汇报状态，**不勾 B**，不 merge

**半自动（你可能要看一眼）**：

- **Merge 冲突**：PE/ELF 都改了同一 verifier 文件时，merge 脚本会停住；在 master 解决冲突后 `git add` + `git commit`，再跑 ddc
- **看板**：只有 coordinator 在 master 改本文件；worktree 里 agent 不改看板

### AI 会自动做什么（单轨与两板共通）

1. MCP 右侧打开 `STAGE4_OWNER_CHECKLIST.md`（`file:///F:/yoyo/STAGE4_OWNER_CHECKLIST.md`）
2. 按触发词选单轨或两板（见上）
3. 绿了才勾框，并简短汇报：命令、绿/红、勾了哪一格
4. **不 push**；commit 仅当你明确说要 commit

### 你仍只需盯两个信号

- 终端：`test ddc` 退出码 `0`、无 `FAIL`/`FATAL`
- 看板：待做区 checkbox 有没有少勾

---

## 每日例行（固定）

在 PowerShell 中执行：

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test ddc
```

### 「绿」是什么意思


| 输出信号                          | 含义             | 你要做什么                                       |
| ----------------------------- | -------------- | ------------------------------------------- |
| 退出码 `0`                       | DDC 套件全过       | 不用动；看板「已完成」区保持 `[x]`                        |
| 含 `PASS`、无 `FAIL` / `FATAL`   | 各 fixture 语义一致 | 同上                                          |
| 退出码非 `0` 或出现 `FAIL` / `FATAL` | 某条 DDC 路径断了    | **不要自己改代码** → 复制下方「对 AI 说什么」话术，把完整终端输出贴给 AI |
| 含 `SKIP`（仅 container）         | 预期内跳过项         | Stage 4 毕业前 container 仍是 SKIP；不算回归          |


**扩展抽检（每周一次即可，非每日必跑）：**

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test golden    # 期望 739/739 PASS
cargo run -- test backends  # 期望 36/36 PASS
```

---



## Stage 4 毕业看板



### 已完成 ✅

- [x] **00_nop_ret DDC** — 23 paths PASS（sim + 22 arch interps，含 wasm trap）
- [x] **01_arith DDC** — 11/11 core fatal + MCU soft（SET+ADDV → slot0=8；Item 7i 起 core fatal）
- [x] **02_branch DDC** — 11/11 core fatal（CMP+JE → slot0=5；含 x86 ZF 修复 7j）
- [x] **03_mem MEMCPY_STATE DDC** — 11/11 core fatal（含 Plan9/x86 slot-form 覆盖 7k）
- [x] **Golden** — 739/739 PASS
- [x] **Backends** — 36/36 编译链接冒烟 PASS
- [x] **BACKEND_SUPPORT.md DDC matrix** — 含 8a–8c（container PASS、04_ldb_ptr）
- [x] **A：Win/Linux 生产路径对齐** — Item 8a；13/13 core fatal（含 win32/linux）
- [x] **B：Container DDC** — Item 8b；PE+ELF PASS，不再 SKIP
- [x] **C：LDB 指针内存 DDC** — Item 8c；`04_ldb_ptr` sim/PE/ELF PASS

### Stage 4 毕业三门（已全部完成 ✅）

> A/B/C 均已 `[x]`，本节仅作记录；**新任务请看 Stage 5**。

---



## 对 AI 说什么（复制粘贴话术）



### 每日 DDC 红了

```text
F:\yoyo STAGE4 负责人看板：cargo run -- test ddc 失败。
请根据完整终端输出定位回归，修到 00–03 全 PASS；container SKIP 可暂保留。
约束：最小 diff；修完在 BACKEND_SUPPORT.md 更新 DDC 表若语义变；不要 push。
```



### 任务 A — Win/Linux MEMCPY_STATE 生产路径

```text
Stage 4 毕业项 A：默认 win32/linux x64 的 MEMCPY_STATE 改为 slot-form emit（对齐 Plan9/x86 DDC override），
01_arith / 02_branch / 03_mem 的 Win+Linux 纳入 core fatal。
验收：cd F:\yoyo\yoyo-rust\verifier && cargo run -- test ddc 退出码 0；
BACKEND_SUPPORT.md Known gaps 删掉「default Win/Linux pointer-form」一句。
```



### 任务 B — Container DDC

```text
Stage 4 毕业项 B：实现 container DDC（取消 SKIP）。
最小范围：PE + ELF 上对 NOP+RET fixture 做 container 解释执行，纳入 test ddc。
验收：ddc 输出 container 行 PASS；BACKEND_SUPPORT.md 表 Status 从 SKIP 改 PASS。
```

**两板并行时**（coordinator 派子 agent，各 agent 只跑自家话术）：

```text
Stage 4 B-PE（worktree F:\yoyo-worktrees\stage4-pe，分支 stage4/container-pe）：
仅实现 PE32+ x64 container NOP+RET 最小解释执行；cargo run -- test ddc 侧重 container PE 路径 PASS。
禁止改 ELF、禁止改 STAGE4_OWNER_CHECKLIST.md；可在 stage4/container-pe 上 commit；不 push。
```

```text
Stage 4 B-ELF（worktree F:\yoyo-worktrees\stage4-elf，分支 stage4/container-elf）：
仅实现 ELF64 x64 container NOP+RET 最小解释执行；cargo run -- test ddc 侧重 container ELF 路径 PASS。
禁止改 PE、禁止改 STAGE4_OWNER_CHECKLIST.md；可在 stage4/container-elf 上 commit；不 push。
```



### 任务 C — LDB 指针内存 + container

```text
Stage 4 毕业项 C：新增 LDB absolute-pointer 内存 DDC fixture；
Win + Linux container 路径 PASS（依赖 B 的 container 基础设施）。
验收：fixture 进 test ddc；03_mem 或独立 04_ldb 文档化；三门 A/B/C 可在同一 PR 但看板分项勾选。
```



### Golden / Backends 回归

```text
Stage 4 看板抽检：golden 或 backends 失败（贴完整输出）。
请修到 golden 739/739、backends 36/36，且不破坏 ddc 全绿。
```

---



## Stage 5 预置任务（Stage 5 已毕业 ✅）

> Stage 4 已毕业。Stage 5 **8/8 全绿**（2026-08-24 Freeze+Lock 复验 + 文档收口）。

- [x] **test all 一键** — `cargo run -- test all`（golden + backends + ddc）CI 级绿
- [x] **Windows M2→M3 自举** — `stage5-win-selfhost.ps1` 全绿（gen2rt embedded startup → output.exe，无 AV）
- [x] **3-chain section-ddc 持续绿** — JS==Rust==Python asm peer 字节 EQUAL（SHA 监控）
- [x] **gen1≡gen2 持续绿** — `cargo run -- test gen12` + `scripts/verify-gen12-ddc.ps1`（SHA `4fb8b87f`）
- [x] **全架构 DDC 扩展** — 01/02/03 均 11/11 MCU fatal；`test all` 绿
- [x] **selfhost startup 完整实现** — gen2rt 零参 → output.exe（embedded startup + yoyo_runtime.dll sidecar）
- [x] **Freeze + Lock 复验** — 走 Lock Protocol 8-step；pin 与 `yoyo/tests/yoyo.ty.lock` 一致
- [x] **文档收口** — `PROMPT-v3.md` Week 轴与看板状态同步；删掉过时 Known gaps

---

## Stage 6 预置任务（维护 + Phase 2 · 已毕业 ✅）

> Stage 5 已毕业。Stage 6 **3/3 全绿**（2026-08-25 Lock pin 常驻监控）。

- [x] **日常维护复验** — `cargo run -- test all` + `.\scripts\stage5-win-selfhost.ps1` 每周至少一轮全绿；红了即回归任务
- [x] **D-2 MOVRR Phase 2** — `0x64` 独立语义对齐（JS/Rust `emit_movrr` 解耦 GET；规范与 golden 已更新）
- [x] **Lock pin 常驻监控** — 解除 PROMPT #18 HOLD：`cargo run -- test lock` + CI + `scripts/verify-lock-pin.ps1`；pin 漂移 fail-closed（非 Relock）

---

## 任务从哪来（四层来源）

```text
                    ┌─────────────────────┐
                    │  ① 本看板 STAGE4    │  ← 负责人每日唯一入口
                    │     OWNER_CHECKLIST │
                    └──────────┬──────────┘
                               │
         ┌─────────────────────┼─────────────────────┐
         ▼                     ▼                     ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ ② 每日命令输出   │  │ ③ BACKEND_      │  │ ④ PROMPT-v3     │
│ test ddc/golden │  │ SUPPORT.md      │  │ Week 轴 +       │
│ 绿/红即任务     │  │ DDC matrix 缺口 │  │ git log 7a–7k   │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```


| 层   | 来源                       | 谁维护          | 你怎么用                |
| --- | ------------------------ | ------------ | ------------------- |
| ①   | 本文件                      | AI 按毕业进度更新勾选 | 每天打开，看 `[ ]`        |
| ②   | `cargo run -- test *`    | 代码真相         | 红了就贴输出给 AI          |
| ③   | `BACKEND_SUPPORT.md`     | 与 ② 同步       | 查「Known gaps」= 待办灵感 |
| ④   | `PROMPT-v3.md` / commits | 规格与历史        | Stage 5 大项对齐 Week 轴 |


---



## 负责人原则

1. **不懂底层 OK** — 你不需要读 `emit.rs`、不需要懂 REX prefix。
2. **只看两个信号** — 终端退出码 / 输出里有没有 `FAIL`；看板 checkbox 有没有漏勾。
3. **红了不自己动手** — 复制「对 AI 说什么」+ 完整日志；让 AI 修完再跑同一条命令验收。
4. **毕业有顺序** — 先 A/B/C（Stage 4），再 Stage 5 → Stage 6 → Stage 7；不要跳关。
5. **SKIP 要知情** — 目前仅 **container** 允许 SKIP；其它 fixture 出现 SKIP 视为未完成或回归。
6. **不替 AI push** — 本地绿了即可；发布由你另行决定。

---

## Stage 7 预置任务（peer 对齐 + 自举深化）

> Stage 6 已毕业。Stage 7 以 **3-peer 诚实对齐** + **自举链深化** 为主；仍不启动 SCOPE-CUT 大项（full body / libyoyo migration · W5.5）。

- [ ] **Stage 7 维护节拍** — 每周一轮全栈复验全绿：`cargo run -- test all` + `.\scripts\stage5-win-selfhost.ps1` + `.\scripts\verify-lock-pin.ps1` + `node .\yoyo-js\scripts\golden.js`；红了即回归任务，不勾
- [ ] **JS golden MEMCPY 真实 emit** — JS `encodeOp(0x84/0x85)` 对齐 Rust `rep movsb`；stub fixture 改真实 expected hex；验收：`node .\yoyo-js\scripts\golden.js` 退出码 0，MEMCPY 项无 `stub=C3; semantic gap`
- [ ] **Windows 无 sidecar 自举** — `bootstrap --selfhost` 产出单文件 gen2rt（runtime 嵌入 PE，不依赖旁路 `yoyo_runtime.dll`）；验收：`.\scripts\stage5-win-selfhost.ps1` M2→M3 全绿且 workdir 无 sidecar DLL
- [ ] **Linux M2→M3 自举链** — 新增 `scripts/stage5-linux-selfhost.sh`（或 WSL 等价），gen2.elf embedded startup → gen3.elf 无挂死/AV；验收：脚本退出码 0 + `output.elf` 存在
- [ ] **定制 MCU 后端脚手架** — 为芯片/定制 ISA 工作预留：`BACKEND_SUPPORT.md` 登记 hook 步骤 + 最小 stub target 纳入 `test backends`；验收：`cargo run -- test backends` 仍 36/36 PASS，新 target 冒烟 PASS 且 DDC 表有行

---

*最后更新：2026-08-25 · Stage 7 预置任务立项 · Stage 6 已毕业*