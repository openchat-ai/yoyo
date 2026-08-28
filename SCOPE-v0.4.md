# YOYO v0.4 — Scope Boundary（负责人一页纸）

> **前提**：v0.1 + **v0.2.0** + **v0.3.0** 已发布（见 `RELEASE-v0.1.md`、`RELEASE-v0.2.md`、`RELEASE-v0.3.md`）。Stage 9 A–D 全绿：H_00 纯 gen1、JS↔Rust I/O 对齐、Win 纯 M4（无 `bootstrap --selfhost`）。
>
> **Sources：** `RELEASE-v0.3.md` 诚实剩余面、`STAGE9_OWNER_CHECKLIST.md`、`STAGE10_OWNER_CHECKLIST.md` · baseline 2026-08-28。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC（多链双编译）+ Lock 协议，在实践中检测编译器级后门 / Thompson 式攻击**——打破「没有后门」这个魔咒。

**v0.4 每一项 IN 范围，都必须扩大「有多少自举字节处在三链 DDC + Lock 监控之下」或缩小「仍须信任 Rust 宿主 / 嵌入 runtime」的洞**：

- **收缩 / 替换嵌入式 `yoyo_runtime.dll`** — 当前每个 genN 仍嵌 Rust 编译的 runtime；这些字节 **在 gen12 窗口之外**，是最大宿主信任洞
- **Linux ELF H_00 / 纯 M4** — 关闭「Linux 只能靠 `bootstrap --selfhost` 才绿」的旁路，与 Win Stage 9-C 对齐
- **asm peer I/O 对齐** — 关闭 Python asm 仍 movabs+store 的三链盲区（JS 已在 v0.3 对齐）· **Stage 10-C DONE 2026-08-28**（`stage10-asm-peer-io.ps1`；win32 字节 EQUAL Rust/JS）
- **基线不退化** — v0.3 的 H_00 / JS peer / Win 纯 M4 / fullbody / lock / gen12 覆盖面只增不减

**诚实边界**：DDC 在 **链间独立、输入一致** 的前提下检测 **输出分歧**——这是实用的 trust bar，**不是**数学意义上的 Thompson 免疫证明。v0.4 抬高的是 practical bar（更少嵌入 host 字节、更少 `--selfhost`、更少 peer stub），**不宣称**不可能藏后门。

---

## 愿景（负责人读）

YOYO v0.4 要 **收口 v0.3 RELEASE 诚实写出的剩余信任面**：让更多 genN 字节进入可比对的 DDC 窗口，让 Linux 自举少依赖 Rust host 脚手架，并让 asm peer 在平台 I/O 上不再对三链「看不见」。

**诚实定位**：v0.4 仍是 **可审计的自举编译器 ISA**，不是 C/Rust 替代品；无类型系统、无包管理、无 GUI。说「更少宿主信任」= 更少「只有 Rust 嵌入 DLL / `--selfhost` 包装能绿」的旁路——**目的是扩 DDC/Lock 可信面**，不是加功能清单。

---

## v0.4 IN（有界 · 最多 4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **收缩 / 替换嵌入式 `yoyo_runtime.dll` 面** | 缩小每个 genN 对 Rust 编译 runtime DLL 的依赖：迁出、缩小、或把关键路径字节纳入可观测 DDC/parity 窗口（具体策略以可机器验收为准） | **最大剩余信任洞**（`RELEASE-v0.3.md`）：DLL 字节在 gen12 18432B `.text` 窗外；「绿」仍大量建立在 host 编译 runtime 上 · **Stage 10-A DONE 2026-08-28**：DLL **485888→231936** B（`stage10-runtime-surface.ps1` MAX 250000）；gen12 SHA `b609a735`→`43ffde58`；**仍**嵌 Rust DLL（诚实） |
| 2 | **Linux ELF H_00 / 纯 M4** | ELF 入口 → H_00（或文档化的等价纯路径）；Linux gen1→gen4 **不**依赖 `bootstrap --selfhost`；gen4≡gen3_direct DDC EQUAL | **平台级宿主洞** · **Stage 10-B DONE 2026-08-28**：`stage10-linux-pure-m4.sh` GREEN；gen4≡gen3_direct EQUAL（sha `085d07d4…` · 704512 B）；**仍**嵌 Rust `.so` + libdl trampoline（诚实） |
| 3 | **Python asm peer 平台 I/O 对齐** | asm 链 `0x20/0x50/0x51` 生产 emit 对齐 Rust/JS 真实 I/O 语义（或可观测等价契约），消除 movabs+store stub | **三链最后一环 peer 盲区** · **Stage 10-C DONE 2026-08-28**：`platform_io.py` win32 EQUAL Rust；linux ALLOC syscall peer-checked；stub 仍 G-SM-IO；全量 `yoyo.ty` section-ddc 可能仍 DIFF（诚实） |
| 4 | **v0.3 回归不退化 + 毕业收口** | golden / backends / ddc / lock / fullbody / stage5 / stage8 / stage9 门禁保持绿；Relock（若改 pin）+ `RELEASE-v0.4.md` | 扩面时不丢掉已有 DDC/Lock 基线；对外诚实写清仍存边界（含 DDC≠proof） |

**主验收看板**：`STAGE10_OWNER_CHECKLIST.md`（A→D 四门，绿了才勾）。

**优先级说明（信任冲击）**：A = runtime.dll（最大洞）→ B = Linux 纯 M4（整平台 host 旁路）→ C = asm I/O（peer 盲区）→ D = 毕业。不把 Morph/MCU 塞进 IN。

---

## v0.4 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| **Morph / SIMD / 太空级** | Part E / 12 / 15–16；不广告 Thompson-proof 或 any-env morph |
| **MCU / 芯片为主赛道** | `custom-mcu` 脚手架保留；v0.4 **不**扩 8051/AVR 等 fatal DDC；新芯片仅负责人点名 |
| **C/Rust/Go 替代宣称** | 不引入 struct/GC/async/模块系统；不写「日常应用语言」 |
| **「DDC = 证明无后门」/ Thompson-proof** | Forbidden；保持 detection 表述 |
| **G06+ 全套 golden** | 超出 v0.4 Conformance 声明 |
| **macOS 生产门禁** | MAY 工作，毕业不要求 |
| **TheoryManifest / CDS 剧场** | N.4.1 FORBIDDEN |

---

## 与 v0.3 诚实剩余面的关系

| v0.3 / `RELEASE-v0.3.md` 状态 | v0.4 回收 |
|-------------------------------|-----------|
| **Embedded `yoyo_runtime.dll`** — Rust 编译、嵌在每个 genN；字节在 gen12 窗外 | Stage 10 **A** · **DONE 2026-08-28**（面收缩 + fail-closed；未替换为 YOYO-built） |
| **Linux M4 仍需 `bootstrap --selfhost`** — 无 ELF H_00 纯路径 | Stage 10 **B** · **DONE 2026-08-28**（`stage10-linux-pure-m4.sh`；仍嵌 Rust `.so` + trampoline） |
| **Python asm I/O stubs** — 仍 movabs+store | Stage 10 **C** · **DONE 2026-08-28**（`stage10-asm-peer-io.ps1`；三链 win32 I/O EQUAL；stub 保留给 golden） |
| **DDC = detection 非 proof** — 已诚实写入 RELEASE | Stage 10 **D** 继续强调；不升级话术 |
| Stage 9 A–D 全绿（H_00 · JS peer · Win 纯 M4） | **不得回退**；v0.4 在其上收洞 |
| Seed 仍经 `yoyo link` + `bootstrap`（无 `--selfhost`）产 gen3_direct | 诚实边界；A/B 收洞时勿扩大 seed 信任；进一步 seed 收缩属 A/B 副作用或 v0.5+ |

v0.1 / v0.2 / v0.3 已绿项 **不得回退**：Stage 4–9 勾选保持；v0.4 只叠加信任扩面。

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
# 基线不退化（含 v0.2 / v0.3）
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
cargo run -- test lock
cargo run -- test gen12
cargo run -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage8-extended-selfhost.ps1
.\scripts\stage9-gen1-h00-selfhost.ps1
.\scripts\stage9-js-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
# bash scripts/stage8-extended-selfhost.sh   # WSL/Linux

# v0.4 新增（Stage 10 A/B/C 绿后；脚本名随落地调整）
.\scripts\stage10-runtime-surface.ps1       # A：runtime.dll 面收缩 / 可观测门禁（DONE · MAX 250000）
# bash scripts/stage10-linux-pure-m4.sh     # B：ELF H_00 / 纯 M4（DONE · 无 --selfhost）
.\scripts\stage10-asm-peer-io.ps1           # C：asm↔Rust/JS I/O 对齐（DONE · win32 EQUAL）
```

**Stage 10 四门全 `[x]`** = v0.4 可发布候选。

**毕业判定：** Stage 10 A/B/C/D **全绿**（2026-08-28）。验收：`test all` / `lock` / `gen12` / `fullbody` / `verify-lock-pin` / `verify-yoyo-ty` / `stage10-runtime-surface` / `stage10-asm-peer-io` / `stage9-pure-m4` / `stage5-win-selfhost` / `stage10-linux-pure-m4` 全 exit 0。Lock pin Decision #25 **未改**（无 Relock）。gen12 窗 **18432** B · SHA `43ffde58…`；runtime.dll **231936** B（仍 Rust 嵌入，诚实剩余）；Linux 纯 M4 EQUAL `085d07d4…` · 704512 B。见 `RELEASE-v0.4.md` · tag `v0.4.0`。

---

## 诚实边界（对外一句话）

**YOYO v0.4 继续收口自举链上仍须信任嵌入 runtime 的洞（Linux `--selfhost` M4 旁路与 asm I/O peer stub 盲区已关）——让更多字节进入 DDC/Lock 监控；仍是 detection bar，不是 Thompson 证明，也不是 C 替代品。**

---

*维护：Stage 10 毕业或信任链 SHA/脚本变更时同步本文件与 `STAGE10_OWNER_CHECKLIST.md`。勿把 MCU / Morph 误标为 v0.4 主交付。*
