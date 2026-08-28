# YOYO v0.8 — Scope Boundary（负责人一页纸）

> **前提**：v0.1–**v0.7.0** 已发布（见 `RELEASE-v0.7.md`）。Stage 13 A–D 全绿：seed/link host、跨平台 parity、v0.6 回归、毕业收口。
>
> **Sources：** `RELEASE-v0.7.md` 诚实剩余面、`STAGE13_OWNER_CHECKLIST.md`、`ROADMAP-TO-1.0.md`、`SCOPE-v1.0.md` · baseline 2026-08-29。

---

## 北星：打破后门魔咒

YOYO 的核心使命 **不是造一门语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。

**v0.8 每一项 IN 范围，都必须进一步收口窗外字节（或诚实 SCOPE-CUT），或加厚 Lock 硬化可观测面**：

- **窗外字节 / SCOPE-CUT 草案** — **Stage 14-A DONE**：窗外 OW-\* 已写成诚实 SCOPE-CUT + 机器门（非假 full `.text` EQUAL）
- **Lock 硬化** — **Stage 14-B DONE**：`stage14-lock-harden.ps1` 钉 Decision #25（或 Relock+Decision note）
- **基线不退化** — **Stage 14-C DONE**：`stage14-v07-regress.ps1` ALL_GREEN（v0.7 seed/link / parity / stage12–9 / fullbody / lock / gen12 + A/B）

**诚实边界**：DDC = detection bar，**不是** Thompson 证明。v0.7 诚实剩余（Rust runtime + LoadLibrary/libdl + full `.text` 仍可 DIFF + seed 仍 Rust 发射）**不自动消失**。

---

## 愿景（负责人读）

YOYO v0.8 要 **收口 v0.7 RELEASE 诚实写出的下一最大洞**：窗外仍 DIFF 字节（或起草 SCOPE-CUT），并把 Lock 硬化写成可验收门。

**诚实定位**：仍是可审计自举编译器 ISA，不是 C/Rust 替代品。

---

## v0.8 IN（有界 · ≤4 门 · 按信任冲击排序）

| # | 范围 | 说明 | 信任链（为何 IN） | 状态 |
|---|------|------|------------------|------|
| 1 | **窗外字节收口或 SCOPE-CUT 草案** | 诚实 SCOPE-CUT 草案 + 机器门：`SCOPE-CUT-v0.8-outside-window.md` · `scripts/stage14-outside-window-scope-cut.ps1`（alias `stage14-scope-cut.ps1`） | **full `.text` DIFF 主洞 → 窗外钉成 CUT** | **A DONE** |
| 2 | **Lock 硬化** | pin / Relock 纪律门禁加厚；改源必 Relock+Decision · `scripts/stage14-lock-harden.ps1`（alias `stage14-lock.ps1`） | **信任钉不可漂** | **B DONE** |
| 3 | **v0.7 回归不退化** | stage13 + stage12–9 + fullbody/lock/gen12 + A/B 保持绿 · `scripts/stage14-v07-regress.ps1` | 扩面时不丢已有 DDC/Lock 基线 | **C DONE** |
| 4 | **毕业收口** | Relock（若改 pin）+ `RELEASE-v0.8.md`；诚实写仍存边界 · **Stage 14-D DONE 2026-08-29**：无 Relock；`RELEASE-v0.8.md` + tag `v0.8.0` | 对外 detection 话术 | **D 绿** |

**主验收看板**：`STAGE14_OWNER_CHECKLIST.md`（A→D）。

---

## v0.8 OUT（仍 ROADMAP / 更后）

| 项 | 为何 OUT |
|----|----------|
| Morph / SIMD / MCU 主赛道 | 见 `SCOPE-v1.0.md` |
| C/Rust/Go 替代宣称 | Forbidden |
| Thompson-proof | Forbidden |
| YOYO-built runtime（非 Rust） | 仍诚实剩余（OW-RT CUT）；不挡 v0.8，勿宣称已关 |
| 洞清单全面收口 | v0.9 主题 |
| macOS 生产门禁 | 毕业不要求 |

---

## 与 v0.7 诚实剩余面的关系

| v0.7 / `RELEASE-v0.7.md` 状态 | v0.8 回收 |
|-------------------------------|-----------|
| **Embedded Rust runtime** — 仍嵌、窗外 | Stage 14 **A** → **OW-RT CUT**（≤170000 · exact embed；观测 154624） |
| **LoadLibrary / libdl** — 仍宿主加载 | Stage 14 **A** → **OW-IAT CUT**（标记钉住；未关） |
| **Full `.text` peer may DIFF** | Stage 14 **A** → SCOPE-CUT ACTIVE；可比窗仍 selfhost-body EQUAL（17805） |
| **Seed still Rust-emitted** | **OW-SEED CUT**；不挡 v0.8；继续诚实 |
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
.\scripts\stage14-outside-window-scope-cut.ps1
# alias: .\scripts\stage14-scope-cut.ps1
.\scripts\stage14-lock-harden.ps1
# alias: .\scripts\stage14-lock.ps1
.\scripts\stage14-v07-regress.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 14-A（2026-08-29）：** `stage14-outside-window-scope-cut.ps1 -SkipBuild` exit 0 · `SCOPE_CUT status=ACTIVE full_text=DIFF body_window=EQUAL compared=17805 stub_nz=159 dll=154624` · 草案 `SCOPE-CUT-v0.8-outside-window.md` · **未**宣称 full `.text` EQUAL。

**Stage 14-B（2026-08-29）：** `stage14-lock-harden.ps1 -SkipBuild` exit 0 · `LOCK_HARDEN status=PINNED decision=25 pin=0275802d… relock=NO ty_eq_lock=YES` · Decision #25 权威钉；漂移 → `RELOCK_REQUIRED`（禁静默改 lock）· **无 Relock**（未改 `yoyo.ty`）；A + stage13-link-host spot 不退化。

**Stage 14-C（2026-08-29）：** `stage14-v07-regress.ps1 -SkipBuild` exit **0** · `ALL_GREEN`（stamp 01:08:59）· serial：wait cargo/rustc → `yoyo.exe test all|lock|gen12|fullbody` → stage13/12/11/10/9 + Stage 14 A/B named `-SkipBuild` + WSL pure-m4 · driver：`driver.lock` 禁并发；Invoke-Gate 无 `| Out-Host`（PS5.1 LASTEXITCODE）· **无 Relock**；不退化 A/B。

**Stage 14 四门全 `[x]`** = v0.8 可发布候选。

**毕业判定（2026-08-29 · Stage 14-D）：** A/B/C/D 全绿 · **无 Relock**（Decision #25）· `stage14-lock-harden.ps1 -SkipBuild` PINNED · `stage14-v07-regress.ps1 -SkipBuild` ALL_GREEN（01:14:12）· `RELEASE-v0.8.md` / `RELEASE-NOTES-v0.8.md` 诚实写 **SCOPE-CUT ACTIVE** + DDC=detection · 下一主线已定 `SCOPE-v0.9.md` + `STAGE15_OWNER_CHECKLIST.md`

---

## 诚实边界（对外一句话）

**YOYO v0.8 已起草窗外 SCOPE-CUT（Stage 14-A）并加厚 Lock 钉（Stage 14-B · Decision #25 / Relock 纪律）——仍是 detection bar，不是 Thompson 证明；Rust runtime / LoadLibrary / seed 宿主洞仍 CUT，不得假装 full `.text` EQUAL。**

---

*维护：Stage 14 毕业或信任链变更时同步本文件与 `STAGE14_OWNER_CHECKLIST.md`。*
