# YOYO v0.2 — Scope Boundary (Owner One-Pager)

> **前提**：v0.1 已发布（见 `RELEASE-v0.1.md`）。v0.2 回收 v0.1 **SCOPE-CUT** 的 W5.5 大项，把 YOYO 从「可验证 ISA 工具链」推进到「**通用计算机语言 + 完整自举**」——仍以 x86-64 hosted 为主，**不以芯片/MCU 为主赛道**。
>
> **Sources：** `PROMPT-v3.md`（W5.5 SCOPE-CUT · Part L · Appendix A）、`RELEASE-v0.1.md`、`STAGE8_OWNER_CHECKLIST.md` · baseline 2026-08-27。

---

## 北星：打破后门魔咒

YOYO 的核心使命不是「造一门语言」，而是 **用 DDC（多链双编译）+ Lock 协议，在实践中检测编译器级后门 / Thompson 式攻击**——打破「没有后门」这个魔咒。

**v0.2 每一项 IN 范围，都是为扩大「有多少编译器字节处在三链 DDC + Lock 监控之下」**：

- **真实平台 I/O** — 自举链不再依赖 movabs+store 占位；gen2→gen4 的读写路径进入可比对、可审计范围
- **libyoyo 迁移** — 平台 syscall 从 stub 迁入独立库，减少「只有 Rust 链能跑、JS 链看不见」的盲区
- **Full body 编译器** — 完整 `yoyo.ty` 体（非 scoped 子集）发出的机器码纳入 DDC / golden 覆盖
- **M2→M3→M4 自举链** — 更长代数链 = 更多机会暴露 genN 与 genN_direct 的分叉；gen12 / section-ddc 持续监控

**诚实边界**：DDC 在 **链间独立、输入一致** 的前提下检测 **输出分歧**——这是实用的 trust bar，**不是**数学意义上的 Thompson 免疫证明。v0.2 的价值是把这条 practical bar **抬高**：更多真实代码路径、更长自举链、更严 Lock pin——让「藏后门而不被三链抓到」更难，而非宣称不可能。

---

## 愿景（负责人读）

YOYO v0.2 要让 **`yoyo.ty` 真正成为能编译自己的完整编译器**，并在 Windows + Linux 上跑通 **真实文件 I/O 与内存分配**（不再用 movabs+store 占位）。上述能力 **服务于北星**：每多一条真实 I/O 路径、每多一段 full body 字节、每多一代 M4 自举，三链 DDC 与 Lock 的 **覆盖面就扩大一分**。

**诚实定位**：v0.2 仍是 **可审计的自举编译器 ISA**，不是 C/Rust 替代品；无类型系统、无包管理、无 GUI 应用生态。说「通用计算机语言」= 能在宿主 OS 上读写文件、分配内存、完成完整自举闭环——**目的是让更多编译器产出进入 DDC 检测范围**，不是「日常应用开发语言」。

---

## v0.2 IN（有界 · 按优先级）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **真实平台 I/O（D-1 关闭）** | Win32：VirtualAlloc + 文件读写；Linux：mmap/syscall。`0x20/0x50/0x51` 生产路径不再 movabs+store 占位 | 自举读写不再走不可比对的 stub，gen 链 I/O 字节进入 DDC 可观测面 |
| 2 | **libyoyo 迁移（W5.5 平台半部）** | `PlatformBackend` 委托 `libyoyo`（Appendix A）；Win + Linux 真实实现进 Rust 链 | 平台层独立成库，缩小「链 A 真 syscall、链 B 占位」的监控盲区 |
| 3 | **Full body 编译器路径（W5.5 body 半部）** | `yoyo.ty` 完整编译器体（非 scoped startup）能编译自身输入 → 产出等价 genN | 完整编译器发出的代码纳入 golden/DDC，不再只验 scoped 子集 |
| 4 | **扩展自举链 M2→M3→M4** | Win + Linux 各跑通 gen2→gen3→gen4；gen parity 或 section-ddc 监控 | 更长代数链 + 同 gen12/section-ddc 门禁，提高分叉被发现的概率 |
| 5 | **v0.1 回归不退化** | golden / backends / ddc / lock / 现有 M2→M3 脚本保持绿 | 扩面时不丢掉已有 DDC/Lock 基线——信任只能增不能减 |

**主验收看板**：`STAGE8_OWNER_CHECKLIST.md`（A→D 四门，绿了才勾）。

---

## v0.2 OUT（仍 ROADMAP / v0.3+）

| 项 | 为何 OUT |
|----|----------|
| **芯片/MCU 为主赛道** | `custom-mcu` 脚手架保留；v0.2 不扩 8051/AVR 等 fatal DDC；新芯片工作 **可选、低优先级** |
| **C/Rust/Go 替代** | Part L 非目标；v0.2 不引入 struct/GC/async/模块系统 |
| **Morph / SIMD / 太空级** | Part E / 12 / 15–16 ROADMAP；不广告 Thompson-proof 或 any-env morph |
| **G06+ 全套 golden** | 超出 v0.2 Conformance 声明 |
| **macOS 生产门禁** | MAY 工作，但 v0.2 毕业不要求 |
| **37 后端全 interpreter** | backends 冒烟可保留；v0.2 不承诺每 arch 语义 interpreter |
| **TheoryManifest / CDS 剧场** | N.4.1 FORBIDDEN |

---

## 与 v0.1 SCOPE-CUT 的关系

| v0.1 状态 | v0.2 回收 |
|-----------|-----------|
| **W5.5 full body + libyoyo migration** — SCOPE-CUT | Stage 8 **B + C**（full body + M4 链） |
| **D-1 平台 I/O stub** — movabs+store 与 JS 对齐但非真实 OS 调用 | Stage 8 **A**（libyoyo 真实 I/O） |
| **M2→M3 scoped self-host** — GREEN | v0.2 延伸为 **M3→M4**（Stage 8 **C**） |
| **788 handlers EXPERIMENTAL 扩写** — 不等于 full body 产品声明 | v0.2 毕业需 **Relock + 新 pin** 覆盖 full body 变更 |

v0.1 已绿项 **不得回退**：Stage 4–7 勾选保持；v0.2 在其上叠加。

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
# 基线不退化
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
cargo run -- test lock
cargo run -- test gen12

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js

# v0.1 自举（仍须绿）
.\scripts\stage5-win-selfhost.ps1
# bash scripts/stage5-linux-selfhost.sh   # WSL/Linux

# v0.2 新增（Stage 8 C/D 绿后）
.\scripts\stage8-extended-selfhost.ps1      # M2→M3→M4 Win（脚本随 A/B/C 落地添加）
# bash scripts/stage8-extended-selfhost.sh  # Linux M4
```

**Stage 8 四门全 `[x]`** = v0.2 可发布候选。

**毕业判定（2026-08-27）：** Stage 8 **A/B/C/D 全 `[x]`** — `test all` + `test lock` + `test gen12` + `test fullbody` + `verify-lock-pin` + `stage5-win-selfhost` + `stage8-extended-selfhost`（Win + Linux WSL）全 exit 0。Lock pin **不变**（Decision #25 `0275802d…` — Stage 8 未改 `yoyo.ty` 源码；扩面在 toolchain emit/runtime + M4 监控）。`RELEASE-v0.2.md` 已写。

---

## 诚实边界（对外一句话）

**YOYO v0.2 是一台在 Windows/Linux 上能读写真实文件、完成 M4 自举闭环的可审计 x86-64 编译器——面向需要打破「后门魔咒」、用三链 DDC 检测编译器级分叉的人，不是面向写应用的 C/Rust 开发者。**

---

*维护：Stage 8 毕业或 W5.5 语义变更时同步本文件与 `STAGE8_OWNER_CHECKLIST.md`。勿把 MCU 脚手架毕业误标为 v0.2 主交付。*
