# YOYO v0.7 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.6.0** 已发布（见 `RELEASE-v0.6.md`）。Stage 12 A–D 全绿：三 peer I/O、selfhost body section-ddc、v0.5 回归、毕业收口。
>
> **Sources：** `RELEASE-v0.6.md` 诚实剩余面、`STAGE12_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-28。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.7 每一项 IN 范围，都必须进一步缩小对 seed/link 宿主旁路的信任**，或把更多跨平台 / Relock 纪律纳入可机器观测面：

- **seed/link host** — 纯 M4 仍经 `yoyo link` + `bootstrap` 种子；收缩或观测该宿主旁路
- **跨平台 parity** — Win/Linux（及已钉住的 stub OS 诚实分叉）parity 门禁加厚，减少「一平台绿、另一平台盲」
- **Relock 纪律** — 改 `yoyo.ty` 则必须 Relock + Decision；未改则钉住 Decision #25
- **基线不退化** — v0.6 的三 peer I/O / selfhost-body / stage11 runtime·LoadLibrary / pure M4 / fullbody / lock / gen12 只增不减

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.6 诚实剩余（Rust runtime + LoadLibrary/libdl + full `.text` 仍可 DIFF）**不自动消失**。

---

## 愿景（负责人读）

YOYO v0.7 要 **收口 v0.6 RELEASE 诚实写出的下一最大洞**：seed/link 宿主种子旁路与跨平台 parity，并把 Relock 纪律写成可验收门。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.7 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） |
|---|------|------|------------------|
| 1 | **seed/link host** | 收缩或 fail-closed 观测 `link`/`bootstrap` 种子旁路；须可脚本验收 | **自举入口宿主洞** |
| 2 | **跨平台 parity** | Win/Linux（+ stub OS 诚实钉）parity 门禁加厚；减少平台分叉盲区 | **多平台 detection 一致** |
| 3 | **v0.6 回归不退化** | stage12 + stage11 + stage10/9 + fullbody/lock/gen12 保持绿 | 扩面时不丢已有 DDC/Lock 基线 |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.7.md`；诚实写仍存边界 | 对外 detection 话术 |

**主验收看板**：`STAGE13_OWNER_CHECKLIST.md`（A→D）。

---

## v0.7 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| YOYO-built runtime（非 Rust） | 仍诚实剩余；不挡 v0.7，勿宣称已关 |
| 窗外字节 / SCOPE-CUT 草案主门 | v0.8 主题 |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.6 诚实剩余面的关系

| v0.6 / `RELEASE-v0.6.md` 状态 | v0.7 回收 |
|-------------------------------|-----------|
| **Embedded Rust runtime** — 仍嵌、窗外 | **不挡** v0.7；继续诚实 |
| **LoadLibrary / libdl** — 仍宿主加载 | **不挡** v0.7；继续诚实 |
| **Full `.text` peer may DIFF** | **不挡**；B 窗已 EQUAL；进一步收洞属 v0.8+ |
| **Seed / link host** | Stage 13 **A** 主门 |
| **Non-Win/Linux stub OS** | Stage 13 **B** 可观测 / 钉住 |
| DDC = detection 非 proof | Stage 13 **D** 继续强调 |

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
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
# Stage 13 gates: stage13-* （A/B 落地后补）
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 13 四门全 `[x]`** = v0.7 可发布候选。

**毕业判定：** （Stage 13 A/B/C/D 全绿后填写）

---

## 诚实边界（对外一句话）

**YOYO v0.7 继续收口 seed/link 宿主与跨平台 parity——仍是 detection bar，不是 Thompson 证明；Rust runtime / LoadLibrary 宿主洞若未关须继续诚实写出。**

---

*维护：Stage 13 毕业或信任链变更时同步本文件与 `STAGE13_OWNER_CHECKLIST.md`。*
