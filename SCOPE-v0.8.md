# YOYO v0.8 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.7.0** 已发布（见 `RELEASE-v0.7.md`）。Stage 13 A–D 全绿：seed/link host、跨平台 parity、v0.6 回归、毕业收口。
>
> **Sources：** `RELEASE-v0.7.md` 诚实剩余面、`STAGE13_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-29。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.8 每一项 IN 范围，都必须进一步收口窗外字节（或诚实 SCOPE-CUT），或加厚 Lock 硬化可观测面**：

- **窗外字节 / SCOPE-CUT 草案** — H_00 slot / extract stub / embedded runtime / IAT 等仍 DIFF 面；能收则收，不能收则写 SCOPE-CUT 草案
- **Lock 硬化** — pin / Relock 纪律可机器验收加厚；未改源则钉住 Decision #25
- **基线不退化** — v0.7 的 seed/link / 跨平台 parity / stage12–9 / fullbody / lock / gen12 只增不减

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.7 诚实剩余（Rust runtime + LoadLibrary/libdl + full `.text` 仍可 DIFF + seed 仍 Rust 发射）**不自动消失**。

---

## 愿景（负责人读）

YOYO v0.8 要 **收口 v0.7 RELEASE 诚实写出的下一最大洞**：窗外仍 DIFF 字节（或起草 SCOPE-CUT），并把 Lock 硬化写成可验收门。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.8 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） | 状态 |
|---|------|------|------------------|------|
| 1 | **窗外字节收口或 SCOPE-CUT 草案** | 缩小 H_00/extract/runtime/IAT 等窗外盲区，或起草诚实 SCOPE-CUT | **full `.text` DIFF 主洞** | 待 A |
| 2 | **Lock 硬化** | pin / Relock 纪律门禁加厚；改源必 Relock+Decision | **信任钉不可漂** | 待 B |
| 3 | **v0.7 回归不退化** | stage13 + stage12–9 + fullbody/lock/gen12 保持绿 | 扩面时不丢已有 DDC/Lock 基线 | 待 C |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.8.md`；诚实写仍存边界 | 对外 detection 话术 | 待 D |

**主验收看板**：`STAGE14_OWNER_CHECKLIST.md`（A→D）。

---

## v0.8 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| YOYO-built runtime（非 Rust） | 仍诚实剩余；不挡 v0.8，勿宣称已关（除非 A 明确关） |
| 洞清单全面收口 | v0.9 主题 |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.7 诚实剩余面的关系

| v0.7 / `RELEASE-v0.7.md` 状态 | v0.8 回收 |
|-------------------------------|-----------|
| **Embedded Rust runtime** — 仍嵌、窗外 | Stage 14 **A** 可尝试收 / 否则 SCOPE-CUT 草案 |
| **LoadLibrary / libdl** — 仍宿主加载 | 可纳入 A 窗或 SCOPE-CUT；不默认宣称已关 |
| **Full `.text` peer may DIFF** | Stage 14 **A** 主门 |
| **Seed still Rust-emitted** | 不挡 v0.8；继续诚实；进一步替换属更后 |
| **Non-Win/Linux stub OS** | 已钉；继续诚实 |
| DDC = detection 非 proof | Stage 14 **D** 继续强调 |

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
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage13-v06-regress.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
# Stage 14 gates: stage14-* （落地后补）
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 14 四门全 `[x]`** = v0.8 可发布候选。

**毕业判定：** （Stage 14 A/B/C/D 全绿后填写）

---

## 诚实边界（对外一句话）

**YOYO v0.8 继续收口窗外字节或起草 SCOPE-CUT，并加厚 Lock 硬化——仍是 detection bar，不是 Thompson 证明；Rust runtime / LoadLibrary / seed 宿主洞若未关须继续诚实写出。**

---

*维护：Stage 14 毕业或信任链变更时同步本文件与 `STAGE14_OWNER_CHECKLIST.md`。*
