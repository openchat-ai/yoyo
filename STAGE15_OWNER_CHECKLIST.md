# Stage 15 负责人看板（v0.9 · 洞清单收口 / 预跑门禁）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 15 的 A/B/C/D 每一项，毕业时必须能回答：**它如何收口洞清单（关或诚实 SCOPE-CUT），或加厚预跑可观测面？** v0.9 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.8 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.9.md` — 洞清单收口 · 预跑门禁；**非** MCU / Morph 主赛道。  
> **基线**：Stage 14 已毕业（2026-08-29）；v0.8.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.8.md` / `SCOPE-CUT-v0.8-outside-window.md`（SCOPE-CUT ACTIVE；OW-\*；仍 Rust runtime + LoadLibrary/libdl；seed 仍 Rust 发射）。

## 🎯 Stage 15 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.9 已毕业（2026-08-29）· 见 SCOPE-v0.9.md / RELEASE-v0.9.md
                              → 下一主线 v1.0：SCOPE-v1.0.md + STAGE16_OWNER_CHECKLIST.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE15_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE15_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.9.md` |

相关：`STAGE14_OWNER_CHECKLIST.md`（v0.8 已毕业）、`RELEASE-v0.8.md`、`RELEASE-v0.9.md`、`SCOPE-CUT-v0.9-hole-inventory.md`、`BACKEND_SUPPORT.md`。  
**下一主线**：`STAGE16_OWNER_CHECKLIST.md` · `SCOPE-v1.0.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v1.0/Stage16（已定）并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–14 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 洞清单收口（关或 SCOPE-CUT）（**DONE** · closed=0 cut=7）
2. **B** — 预跑门禁（**DONE** · `stage15-prerun.ps1` ALL_GREEN）
3. **C** — v0.8 回归不退化（**DONE** · `stage15-v08-regress.ps1` ALL_GREEN）
4. **D** — v0.9 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：勾选顺序仍 A→B→C→D；D 依赖 A/B/C。

**下一关** = 已毕业 → 见 `STAGE16_OWNER_CHECKLIST.md`（v1.0）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 15 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage14-outside-window-scope-cut.ps1
.\scripts\stage14-lock-harden.ps1
.\scripts\stage14-v07-regress.ps1
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 15 毕业四门

### 待做 / 已勾

- [x] **A：洞清单收口（关或 SCOPE-CUT）** — `SCOPE-CUT-v0.9-hole-inventory.md` + `scripts/stage15-hole-inventory.ps1`（alias `stage15-a.ps1`）；OW-\*/REL-\* 逐项 `HOLE id=… disposition=CLOSED|CUT`；**closed=0 cut=7** · `HOLE_INVENTORY status=ACTIVE` · full `.text` DIFF 诚实 · nested stage14-A exit 0 · **信任链**：洞从「lump CUT」变「可验 disposition」；不得假 EQUAL / 假 CLOSED；Lock pin Decision #25 未改
- [x] **B：预跑门禁** — `scripts/stage15-prerun.ps1`（alias `stage15-keep-green.ps1` / `stage15-b.ps1`）exit 0 · **信任链**（2026-08-29）：一键串行 keep-green；`wait cargo → cargo SKIP(-SkipBuild) → stage15-hole-inventory -SkipBuild (0) → stage14-v07-regress -SkipBuild (0)`；嵌套覆盖 Stage 15-A + Stage 14 A/B/C + stage13–9 + `yoyo test all/lock/gen12/fullbody` + WSL stage10-linux；`driver.lock` 防并发；named `-SkipBuild`（禁 `@("-SkipBuild")` splat）；**零并行 cargo** · ALL_GREEN · 毕业前可机器复验；不宣称 Thompson / full `.text` EQUAL
- [x] **C：v0.8 回归不退化** — `scripts/stage15-v08-regress.ps1`（alias `stage15-c.ps1`）exit 0 · **信任链**（2026-08-29）：串行 fail-closed；`wait cargo → cargo SKIP(-SkipBuild) → yoyo.exe test all/lock/gen12/fullbody (0) → stage13–9 + stage14 A/B + stage15-hole-inventory -SkipBuild (0) + WSL stage10-linux (0)`；`driver.lock` 防并发；named `-SkipBuild`（禁 `@("-SkipBuild")` splat）；**零并行 cargo** · ALL_GREEN（stamp 01:34:49）；Stage 15-B 覆盖面由本门一等公民复列（不嵌套 prerun 防递归）；不宣称 Thompson / full `.text` EQUAL
- [x] **D：v0.9 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.9.md` 毕业判定 + `RELEASE-v0.9.md` / `RELEASE-NOTES-v0.9.md`（2026-08-29）· **信任链**：Decision #25 pin 仍权威；`stage14-lock-harden.ps1 -SkipBuild` exit 0 · `LOCK_HARDEN status=PINNED relock=NO`；`stage15-v08-regress.ps1 -SkipBuild` ALL_GREEN（stamp **01:40:31**）；RELEASE 诚实写 DDC=detection、**HOLE_INVENTORY ACTIVE closed=0 cut=7**、仍 Rust runtime + LoadLibrary/libdl、full `.text` DIFF、stub OS 仍 stub、**seed 仍 Rust 发射**；下一主线已定 `SCOPE-v1.0.md` + `STAGE16_OWNER_CHECKLIST.md`

### 可选 · 低优先级（不挡 v0.9 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — 仅当 A 明确关 OW-RT；勿偷塞进 v0.9 IN 并宣称已关

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — 洞清单收口

```text
Stage 15 毕业项 A：洞清单收口（关或 SCOPE-CUT）（v0.9 主信任门）。
目标：对 OW-* / RELEASE-v0.8 剩余面逐项关或再 CUT；不得假 EQUAL。
验收：新增/加固 scripts/stage15-* 退出码 0；
cargo run -- test all 退出码 0；stage14 / stage13 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — 预跑门禁

```text
Stage 15 毕业项 B：预跑门禁。
验收：可机器脚本 exit 0；串行；零并行 cargo；v0.8 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.8 回归不退化

```text
Stage 15 毕业项 C：加固/确认 v0.8 门禁不退化。
验收：stage14-* + stage13-* + stage12-* + stage11-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.9 毕业门禁

```text
Stage 15 毕业项 D：v0.9 毕业收口。
写 RELEASE-v0.9.md；SCOPE-v0.9 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v0.9 = 洞清单收口 · 预跑门禁** — MCU / Morph 不挡毕业。
2. **v0.8 不退化** — 每项验收必须含 stage14 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary / seed 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-29 · v0.9.0 毕业 · Stage 15 A/B/C/D 全绿 · 见 `RELEASE-v0.9.md` · 下一主线 → `SCOPE-v1.0.md` / `STAGE16_OWNER_CHECKLIST.md`*

**Post-v1.0（2026-08-29 · three-peer EQUAL · Win smoke）：** OW-H00 **CLOSED** — JS dropped stale `LoadLibraryA` from `KERNEL32_IO_FUNCS` (6-func sync); `three_peer_full=EQUAL` · **`72c27c9f`** · stub_nz **905**. OW-IAT/OW-STUB still **CUT** (manual-map + cwd `yoyo_rt.dll`; Linux dlopen@PLT no libdl). Next: **YOYO-built runtime**.
