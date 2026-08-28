# YOYO v0.5 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.4.0** 已发布（见 `RELEASE-v0.4.md`）。Stage 10 A–D 全绿：runtime 面收缩、Linux 纯 M4、asm I/O 对齐、毕业收口。
>
> **Sources：** `RELEASE-v0.4.md` 诚实剩余面、`STAGE10_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-28。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.5 每一项 IN 范围，都必须进一步缩小对 Rust 编译 runtime / LoadLibrary 宿主的信任**，或把更多 runtime 相关字节纳入可机器观测的 DDC/parity：

- **YOYO-built / 更薄 runtime** — v0.4 仍嵌 Rust `yoyo_runtime.dll` / `.so`（窗外）；v0.5 目标是 YOYO 自建或显著更薄的可观测替代
- **收缩 LoadLibrary / libdl host** — H_00 提取后仍依赖宿主加载器；缩小或替换该旁路
- **基线不退化** — v0.4 的 runtime 面门禁 / Linux 纯 M4 / asm I/O / H_00 / JS peer / Win 纯 M4 / fullbody / lock / gen12 只增不减

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.5 抬高 practical bar，不宣称不可能藏后门。

---

## 愿景（负责人读）

YOYO v0.5 要 **收口 v0.4 RELEASE 诚实写出的最大剩余洞**：嵌入式 Rust runtime 与 LoadLibrary/libdl 宿主加载路径。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.5 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **YOYO-built / 更薄 runtime** | 替换或显著收缩每个 genN 嵌入的 Rust runtime；策略须可脚本验收（大小/parity/自建路径） | **v0.4 最大诚实剩余洞** |
| 2 | **收缩 LoadLibrary / libdl host** | 缩小 H_00→宿主加载器旁路；或把关键路径纳入可观测门禁 | **runtime 加载信任洞** |
| 3 | **v0.4 回归不退化** | stage10 + stage9 + fullbody/lock/gen12 保持绿 | 扩面时不丢已有 DDC/Lock 基线 |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.5.md`；诚实写仍存边界 | 对外 detection 话术 |

**主验收看板**：`STAGE11_OWNER_CHECKLIST.md`（A→D）。

---

## v0.5 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| 三 peer full-body section-ddc 全 EQUAL | v0.6 主题 |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.4 诚实剩余面的关系

| v0.4 / `RELEASE-v0.4.md` 状态 | v0.5 回收 |
|-------------------------------|-----------|
| **Embedded Rust runtime** — 仍嵌、窗外 | Stage 11 **A** |
| **LoadLibrary / libdl trampoline** | Stage 11 **B** |
| Linux `--selfhost` / asm I/O stubs | **已关**（v0.4）；不得回退 |
| DDC = detection 非 proof | Stage 11 **D** 继续强调 |

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
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
# + Stage 11 新门禁脚本（A/B 落地后）
```

**Stage 11 四门全 `[x]`** = v0.5 可发布候选。

**毕业判定：** （Stage 11 A/B/C/D 全绿后填写）

---

## 诚实边界（对外一句话）

**YOYO v0.5 继续收口仍须信任 Rust 嵌入 runtime / LoadLibrary 宿主的洞——目标是 YOYO-built 或更薄、可观测的 runtime 面；仍是 detection bar，不是 Thompson 证明。**

---

*维护：Stage 11 毕业或信任链变更时同步本文件与 `STAGE11_OWNER_CHECKLIST.md`。*
