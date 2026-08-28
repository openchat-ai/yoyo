# YOYO v0.6 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.5.0** 已发布（见 `RELEASE-v0.5.md`）。Stage 11 A–D 全绿：更薄 runtime、LoadLibrary/libdl host 收缩、回归、毕业收口。
>
> **Sources：** `RELEASE-v0.5.md` 诚实剩余面、`STAGE11_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-28。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.6 每一项 IN 范围，都必须把更多自举 / peer 路径字节纳入可机器观测的三链 DDC**，或缩小仍 DIFF 的 full-body 盲区：

- **三 peer I/O** — Rust / JS / asm 生产路径 I/O 契约对齐，消除残余 stub / 平台分叉盲区
- **selfhost body section-ddc** — 把自举 body（或更大可比窗口）纳入 section-ddc；目标缩小「窗外仍绿」面
- **基线不退化** — v0.5 的 runtime 面 / LoadLibrary host / Linux 纯 M4 / asm I/O / H_00 / JS peer / Win 纯 M4 / fullbody / lock / gen12 只增不减

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.6 抬高 practical bar，不宣称不可能藏后门。v0.5 诚实剩余（Rust runtime + LoadLibrary/libdl）**不自动消失**。

---

## 愿景（负责人读）

YOYO v0.6 要 **收口 v0.5 RELEASE 诚实写出的下一最大洞**：三 peer 全路径 I/O 对齐，以及自举 body 上更强的 section-ddc 可观测性。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.6 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **三 peer I/O** | Rust/JS/asm 生产 I/O 路径契约对齐（含残余 stub/平台分叉）；须可脚本验收 | **三链最后可扩观测面** |
| 2 | **selfhost body section-ddc** | 自举 body（或扩大可比窗口）纳入 section-ddc；缩小窗外盲区 · **Stage 12-B DONE 2026-08-28**：`stage12-selfhost-body-section-ddc.ps1`；三 peer EQUAL 17805B（startup+post-H_00）；`yoyo test body-ddc` | **full `.text` 仍 DIFF**（H_00 slot / extract stub / IAT / embedded runtime） |
| 3 | **v0.5 回归不退化** | stage11 + stage10 + stage9 + fullbody/lock/gen12 保持绿 · **Stage 12-C DONE 2026-08-28**：`stage12-v05-regress.ps1`（alias `stage12-regression.ps1`）；A/B keep-green；WSL linux-pure-m4 | 扩面时不丢已有 DDC/Lock 基线 |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.6.md`；诚实写仍存边界 · **Stage 12-D DONE 2026-08-28**：无 Relock；`RELEASE-v0.6.md` + tag `v0.6.0` | 对外 detection 话术 |

**主验收看板**：`STAGE12_OWNER_CHECKLIST.md`（A→D）。

---

## v0.6 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| YOYO-built runtime（非 Rust） | v0.5 未完成；不挡 v0.6，但勿宣称已关 |
| seed/link host 主收缩 | v0.7 主题 |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.5 诚实剩余面的关系

| v0.5 / `RELEASE-v0.5.md` 状态 | v0.6 回收 |
|-------------------------------|-----------|
| **Embedded Rust runtime** — 仍嵌、窗外 | **不挡** v0.6；继续诚实；非本版主门 |
| **LoadLibrary / libdl** — 仍宿主加载 | **不挡** v0.6；继续诚实 |
| **Full-body section-ddc may DIFF** | Stage 12 **B** · **DONE 2026-08-28**（`stage12-selfhost-body-section-ddc.ps1`；三 peer selfhost-body window 17805B EQUAL；full `.text` 仍 DIFF） |
| 三 peer 残余 I/O / stub 盲区 | Stage 12 **A** · **DONE 2026-08-28**（`stage12-three-peer-io.ps1`；win32+linux `0x20/0x50/0x51` Rust=JS=asm；linux LOAD/WRITE 盲区关闭） |
| DDC = detection 非 proof | Stage 12 **D** 继续强调 |

---

## 毕业门禁（机器可验 · 全 exit 0）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run --release -- test all
cargo run --release -- test lock
cargo run --release -- test gen12
cargo run --release -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
node .\scripts\verify-yoyo-ty.mjs
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage12-three-peer-io.ps1   # A：三 peer I/O（DONE · win32+linux Rust=JS=asm）
.\scripts\stage12-selfhost-body-section-ddc.ps1  # B：selfhost body section-ddc（DONE · 三 peer window EQUAL）
.\scripts\stage12-v05-regress.ps1                 # C：v0.5 回归不退化（DONE · alias stage12-regression.ps1）
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 12 四门全 `[x]`** = v0.6 可发布候选。

**毕业判定：** Stage 12 A/B/C/D **全绿**（2026-08-28）。验收：`stage12-v05-regress.ps1` ALL_GREEN（test all/lock/gen12/fullbody + verify-lock-pin + stage11/10/9 + A/B + WSL linux-pure-m4）。Lock pin Decision #25 **未改**（无 Relock）。三 peer I/O win32+linux EQUAL；selfhost-body 窗 **17805** B EQUAL；gen12 窗 **18432** B · SHA `d782166d…`。诚实剩余：Rust runtime + LoadLibrary/libdl；full `.text` 仍可 DIFF；非 Win/Linux stub OS。见 `RELEASE-v0.6.md` · tag `v0.6.0`。下一主线：`SCOPE-v0.7.md` + `STAGE13_OWNER_CHECKLIST.md`。

---

## 诚实边界（对外一句话）

**YOYO v0.6 继续把三 peer I/O 与自举 body 纳入更强的 section-ddc 观测——仍是 detection bar，不是 Thompson 证明；Rust runtime / LoadLibrary 宿主洞若未关须继续诚实写出。**

---

*维护：Stage 12 毕业或信任链变更时同步本文件与 `STAGE12_OWNER_CHECKLIST.md`。v0.6 已毕业 2026-08-28。*
