# YOYO v0.3 — Scope Boundary（负责人一页纸）

> **前提**：v0.1 + **v0.2.0** 已发布（见 `RELEASE-v0.1.md`、`RELEASE-v0.2.md`）。Stage 8 A–D 全绿：真实 Win/Linux I/O、fullbody 门禁、M4 自举（经 genNrt 嵌入路径）。
>
> **Sources：** `RELEASE-v0.2.md` / `RELEASE-NOTES-v0.2.md` 诚实缺口、`STAGE8_OWNER_CHECKLIST.md`、`STAGE9_OWNER_CHECKLIST.md` · baseline 2026-08-28。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC（多链双编译）+ Lock 协议，在实践中检测编译器级后门 / Thompson 式攻击**——打破「没有后门」这个魔咒。

**v0.3 每一项 IN 范围，都必须扩大「有多少自举字节处在三链 DDC + Lock 监控之下」或缩小「仍须信任 Rust 宿主包装」的洞**：

- **gen1 H_00 纯路径** — 自举不再只能靠 genNrt 嵌入 startup 才能绿；H_00 运行时路径进入可观测、可回归门禁
- **JS I/O 对齐** — 关闭「Rust 真 syscall、JS 仍 movabs+store」的 peer 盲区，让三链对平台 I/O 语义可比
- **收紧 host wrapper** — M4 少依赖 `bootstrap --selfhost` 宿主脚手架，更多代数在进程内 / 产物自身完成
- **基线不退化** — v0.2 的 fullbody / M4 / lock / gen12 覆盖面只增不减

**诚实边界**：DDC 在 **链间独立、输入一致** 的前提下检测 **输出分歧**——这是实用的 trust bar，**不是**数学意义上的 Thompson 免疫证明。v0.3 抬高的是 practical bar（更少宿主信任、更少 peer stub 分叉），**不宣称**不可能藏后门。

---

## 愿景（负责人读）

YOYO v0.3 要 **收口 v0.2 留下的诚实信任洞**，让自举链 **少依赖 Rust 宿主包装与嵌入式 startup 捷径**，并让 JS peer 在平台 I/O 上不再对 DDC「看不见」。

**诚实定位**：v0.3 仍是 **可审计的自举编译器 ISA**，不是 C/Rust 替代品；无类型系统、无包管理、无 GUI。说「更纯的自举」= 更多路径由编译器产物自己完成、更少「只有 Rust 包装能绿」的旁路——**目的是扩 DDC/Lock 可信面**，不是加功能清单。

---

## v0.3 IN（有界 · 最多 4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **gen1 H_00 / 无 sidecar 纯自举路径** | 关闭 v0.2 Known RED：gen1 零参 / H_00 runtime 自举须产出 `output.exe`（或文档化的等价产物），不得仅靠 genNrt 嵌入 startup 才绿 | **最大信任洞**：当前「绿」大量建立在 Rust 嵌入路径上；H_00 绿 = 自举入口本身进入可回归监控 |
| 2 | **JS peer 平台 I/O 对齐（DDC）** | JS 链 `0x20/0x50/0x51` 生产 emit 对齐 Rust 真实 I/O 语义（或明确可观测的等价契约），消除 movabs+store stub 盲区 | **Stage 9-B GREEN (2026-08-28)**：JS↔Rust win32 handler 字节 EQUAL；`stage9-js-peer-io.ps1`；asm 仍 stub |
| 3 | **收紧 M4 host wrapper** | **Stage 9-C GREEN (2026-08-28)**：Win `stage9-pure-m4.ps1` gen1→gen4 经 H_00、无 `--selfhost`；gen4≡gen3_direct DDC；Linux 仍 genNrt | 代数链少一层「只有 Rust host 能编排」的信任；M4 更多字节落在产物 DDC 窗口内 |
| 4 | **v0.2 回归不退化 + 毕业收口** | golden / backends / ddc / lock / fullbody / stage5 / stage8 M4 保持绿；Relock（若改 pin）+ `RELEASE-v0.3.md` | 扩面时不丢掉已有 DDC/Lock 基线；对外诚实写清仍存边界 |

**主验收看板**：`STAGE9_OWNER_CHECKLIST.md`（A→D 四门，绿了才勾）。

---

## v0.3 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| **Morph / SIMD / 太空级** | Part E / 12 / 15–16；不广告 Thompson-proof 或 any-env morph |
| **MCU / 芯片为主赛道** | `custom-mcu` 脚手架保留；v0.3 **不**扩 8051/AVR 等 fatal DDC；新芯片仅负责人点名 |
| **C/Rust/Go 替代宣称** | 不引入 struct/GC/async/模块系统；不写「日常应用语言」 |
| **G06+ 全套 golden** | 超出 v0.3 Conformance 声明 |
| **macOS 生产门禁** | MAY 工作，毕业不要求 |
| **「DDC = 证明无后门」** | Forbidden；保持 detection 表述 |
| **TheoryManifest / CDS 剧场** | N.4.1 FORBIDDEN |

---

## 与 v0.2 诚实缺口的关系

| v0.2 / Stage 8 状态 | v0.3 回收 |
|---------------------|-----------|
| **gen1 H_00 runtime selfhost RED** — 退出码 0 但无 `output.exe`（无嵌入 startup） | Stage 9 **A** |
| **JS 仍可能 movabs+store I/O** — Rust 已真 syscall，peer 盲区 | Stage 9 **B** |
| **M4 仍走 host `bootstrap --selfhost` 包装** — 非纯进程内 gen3→gen4 | Stage 9 **C GREEN**（Win H_00 纯链；Linux 仍 `--selfhost`） |
| **DDC = detection 非 proof** — 已诚实写入 RELEASE | Stage 9 **D** 继续强调；不升级话术 |
| Stage 8 A–D 全绿（真实 I/O · fullbody · M4 genNrt） | **不得回退**；v0.3 在其上收洞 |

v0.1 / v0.2 已绿项 **不得回退**：Stage 4–8 勾选保持；v0.3 只叠加信任扩面。

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
# 基线不退化（含 v0.2）
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
# bash scripts/stage8-extended-selfhost.sh   # WSL/Linux

# v0.3 新增（Stage 9 A/B/C 绿后；脚本名随落地调整）
.\scripts\stage9-gen1-h00-selfhost.ps1      # A：gen1 H_00 纯路径
# JS I/O / peer 门禁见看板 B 验收
.\scripts\stage9-pure-m4.ps1                # C：Win H_00 gen1→gen4（无 --selfhost）
```

**Stage 9 四门全 `[x]`** = v0.3 可发布候选。

**毕业判定：** **GREEN (DONE) 2026-08-28** — Stage 9 A/B/C/D 全 `[x]`；pin `0275802d…` Decision #25 **unchanged**（无 Relock）；gen12/fullbody Win `.text` **`b609a735…` / 18432B**；`RELEASE-v0.3.md` 已写；test all / lock / gen12 / fullbody / verify-lock-pin / stage5 / stage8 / stage9-h00 / stage9-peer-io / stage9-pure-m4 / WSL stage8 全 exit 0。

---

## 诚实边界（对外一句话）

**YOYO v0.3 收口自举链上仍须信任 Rust 宿主与 peer stub 的洞——让 H_00 纯路径、JS I/O、更少 host wrapper 进入 DDC/Lock 监控；仍是 detection bar，不是 Thompson 证明，也不是 C 替代品。**

---

*维护：Stage 9 毕业或信任链 SHA/脚本变更时同步本文件与 `STAGE9_OWNER_CHECKLIST.md`。勿把 MCU / Morph 误标为 v0.3 主交付。*

> **v0.3 已毕业（2026-08-28）** → 下一主线见 [`SCOPE-v0.4.md`](./SCOPE-v0.4.md) + [`STAGE10_OWNER_CHECKLIST.md`](./STAGE10_OWNER_CHECKLIST.md)。
