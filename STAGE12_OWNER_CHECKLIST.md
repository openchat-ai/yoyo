# Stage 12 负责人看板（v0.6 · 三 peer I/O / section-ddc）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 12 的 A/B/C/D 每一项，毕业时必须能回答：**它如何把更多自举 / peer 字节纳入三链 DDC + Lock 监控，或缩小 full-body / I/O stub 盲区？** v0.6 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.5 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.6.md` — 三 peer I/O + selfhost body section-ddc；**非** MCU / Morph 主赛道。  
> **基线**：Stage 11 已毕业（2026-08-28）；v0.5.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.5.md`（仍 Rust runtime + LoadLibrary/libdl）。

## 🎯 Stage 12 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.6 已毕业（2026-08-28）· 见 SCOPE-v0.6.md / RELEASE-v0.6.md
                              → 下一主线 v0.7：SCOPE-v0.7.md + STAGE13_OWNER_CHECKLIST.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE12_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE12_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.6.md` |

相关：`STAGE11_OWNER_CHECKLIST.md`（v0.5 已毕业）、`RELEASE-v0.5.md`、`RELEASE-v0.6.md`、`BACKEND_SUPPORT.md`。  
**下一主线**：`STAGE13_OWNER_CHECKLIST.md` · `SCOPE-v0.7.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v0.7/Stage13（已定）并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–11 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 三 peer I/O（**三链可扩观测面**）
2. **B** — selfhost body section-ddc
3. **C** — v0.5 回归不退化（门禁加固 / 观测）
4. **D** — v0.6 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；D 依赖 A/B/C。

**下一关** = 已毕业 → 见 `STAGE13_OWNER_CHECKLIST.md`（v0.7）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 12 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
# alias: .\scripts\stage12-regression.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 12 毕业四门

### 待做

- [x] **A：三 peer I/O** — Rust/JS/asm 生产 I/O 路径契约对齐；消除残余 stub / 平台分叉盲区 · **信任链**（2026-08-28）：`scripts/stage12-three-peer-io.ps1` GREEN（fail-closed；内嵌 stage10-asm + stage9-js）；win32+linux `0x20/0x50/0x51` **Rust=JS=asm** byte-equal（linux LOAD/WRITE 关闭 stage10 仅查 ALLOC 盲区）；stub G-SM-IO 17B；unknown OS→stub 钉住；`cargo run -- test all` + `test lock` 绿；**诚实仍分叉**：Plan9/FreeBSD/Haiku/Serenity 生产 I/O、full `.text` peer DIFF（B 已收 selfhost-body 窗口）、Rust runtime + LoadLibrary/libdl
- [x] **B：selfhost body section-ddc** — 自举 body（或扩大可比窗口）纳入 section-ddc；缩小窗外「仍绿」盲区 · **信任链**（2026-08-28）：`scripts/stage12-selfhost-body-section-ddc.ps1` GREEN（alias `stage12-section-ddc.ps1`）；`yoyo test body-ddc` gen1≡gen2 window EQUAL（17805B）；三 peer `diff --selfhost-body` JS=Rust=asm EQUAL（startup+post-H_00；≥17013）；Rust stub_tail_nonzero=159 pinned；`test lock`+`test gen12` 绿；**诚实仍 DIFF**：H_00 entry slot；Rust-only H_00 extract stub；full `.text` peer compare；embedded runtime DLL / `.data`；LoadLibrary/libdl
- [x] **C：v0.5 回归不退化** — stage11/stage10/stage9/fullbody/lock/gen12 门禁保持绿；必要时加固观测脚本 · **信任链**（2026-08-28）：`scripts/stage12-v05-regress.ps1` GREEN（alias `stage12-regression.ps1`；fail-closed 串行；`&` not Start-Process；无并行 cargo）；`scripts/_stage12-v05-regress/summary.txt` ALL_GREEN — `cargo run --release -- test all/lock/gen12/fullbody` + `verify-lock-pin`（pin `0275802d…`）+ stage11-rt/ll + stage10-rt/asm + stage9-js/m4 + stage12 A/B + WSL `stage10-linux-pure-m4` **EXIT=0**；stage11/10 report WriteAllText（避 PS5.1 Set-Content flake）；gen12 SHA prefix `d782166d`（18432B）；**诚实不变**：Rust runtime + LoadLibrary/libdl；full `.text` peer 仍可 DIFF（B selfhost-body 窗 EQUAL）
- [x] **D：v0.6 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.6.md` 毕业判定 + `RELEASE-v0.6.md` / `RELEASE-NOTES-v0.6.md`（2026-08-28）· **信任链**：Decision #25 pin 仍权威；`stage12-v05-regress.ps1 -SkipBuild` ALL_GREEN；三 peer I/O + selfhost-body **17805** B；gen12 `d782166d` / 18432B；RELEASE 诚实写 DDC=detection、仍 Rust runtime + LoadLibrary/libdl、full `.text` 可 DIFF、非 Win/Linux stub OS

### 可选 · 低优先级（不挡 v0.6 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — v0.5 未关洞；仅当负责人点名，勿偷塞进 v0.6 IN

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — 三 peer I/O

```text
Stage 12 毕业项 A：三 peer I/O 对齐（v0.6 主信任门）。
目标：Rust/JS/asm 生产 I/O 路径契约对齐，消除残余 stub / 平台分叉盲区；不得跳过 DDC。
验收：新增/加固 scripts/stage12-*（名称可调整）退出码 0；
cargo run -- test all 退出码 0；stage11 / stage10 / stage9 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — selfhost body section-ddc

```text
Stage 12 毕业项 B：selfhost body section-ddc。
验收：可机器脚本 exit 0；诚实写仍 DIFF 边界；v0.5 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.5 回归不退化

```text
Stage 12 毕业项 C：加固/确认 v0.5 门禁不退化。
验收：stage11-* + stage10-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.6 毕业门禁

```text
Stage 12 毕业项 D：v0.6 毕业收口。
写 RELEASE-v0.6.md；SCOPE-v0.6 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v0.6 = 三 peer I/O / section-ddc** — MCU / Morph 不挡毕业。
2. **v0.5 不退化** — 每项验收必须含 stage11 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-28 · v0.5.0 后定稿 Stage 12 / v0.6 · 见 `SCOPE-v0.6.md` · 长期 → `SCOPE-v1.0.md` / `ROADMAP-TO-1.0.md`*
