# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任）

## 北星：打破后门魔咒

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。**ROADMAP 止于 Stage 16 / v1.0** — 本看板 **不是 Stage 17 功能轨**，而是 post-v1.0 **path 2 关洞**：逐项缩小 OW-* 宿主信任、诚实 CUT/CLOSED、**禁止假 CLOSED**、**禁止 invent 新 Stage 功能**。

> **用途**：用户说 `继续` / `关洞` 时的 post-v1.0 主线（`AUTO_TO_1.0.md` 为 `ACTIVE=0` 时读 **本文件**，勿启 AUTO invent Stage 17）。  
> **范围**：`SCOPE-CUT-v1.0-hole-inventory.md` 七项 disposition 的 **诚实推进**；非 MCU / Morph 主赛道。  
> **基线**：Stage 16 已毕业（2026-08-29）；tag `v1.0.0`；Lock pin `0275802d…`（Decision #25）；`HOLE_INVENTORY_V10 status=FINAL` · **closed=1 cut=6**（OW-H00 CLOSED · 其余 CUT）。

## 🎯 进度总览

```text
[ ] A  [ ] B  [ ] C   →  path 2 关洞里程碑（无 tag/release；A+B+C 全绿即里程碑）
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。  
> **脚本名 `stage17-*`** = post-v1.0 **门禁编号**（OW-IAT wire-up），**非** ROADMAP Stage 17。

---

## 阻塞

| 项 | 状态 | 说明 |
|----|------|------|
| **with-sidecar manual-map AV** | 🔴 **当前阻塞 Gate A** | master 上 no-sidecar fail-closed **GREEN**（exit≠0，非 AV）；with-sidecar **RED** — `gen1.exe` + cwd `yoyo_rt.dll` → AV（import/GPA 阶段；`Get-SmokePhase` → `access_violation` 或 phase=import） |
| **修复策略** | 本地优先 | 先本地 `& .\scripts\stage17-ow-iat-wireup.ps1` 绿再 **一次** CI push；遵守 `.cursor/rules/ci-anti-thrash.mdc`（PR #14）：禁止 push 风暴 / 默认关 `H00_BISECT` |
| **勿做** | — | 勿 fake OW-IAT CLOSED；勿启 `AUTO_TO_1.0 ACTIVE=1`；勿 invent Stage 17 功能轨 |

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\POST-1.0-HOLE-CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `POST-1.0-HOLE` |
| **洞清单定稿** | `F:\yoyo\SCOPE-CUT-v1.0-hole-inventory.md` |
| **v1.0 毕业看板（历史）** | `F:\yoyo\STAGE16_OWNER_CHECKLIST.md`（全绿 · 勿回改） |

相关：`RELEASE-v1.0.md` · `BACKEND_SUPPORT.md` · `AUTO_TO_1.0.md`（`ACTIVE=0` · `COMPLETED=1`）· `.cursor/rules/ci-anti-thrash.mdc`。

---

## 零指令执行（post-v1.0）

| 方式 | 操作 |
| ---- | ---- |
| **触发词** | `继续` / `关洞` / `post-1.0` / `path 2`（**非** `ACTIVE=1` AUTO） |
| **单轨** | A→B→C；一项 per tick；本地验绿再勾 |
| **AUTO** | `ACTIVE=0` → **停**；读本看板，**不** invent Stage 17 |
| **CI** | gate 不是 debugger；WIP 用 `[skip ci]`；同 PR 连续 2 次红全量 CI → 停推改本地 |

**下一项** = **A**（Win OW-IAT with-sidecar smoke GREEN）。

---

## 约束

0. **打破后门魔咒（北星）** — 每项须说明如何缩小宿主信任或诚实 CUT。
1. **诚实 disposition** — CUT 项不得标 CLOSED；OW-IAT 在 sidecar `yoyo_rt.dll` 仍在时 **必 CUT**。
2. **绿了才勾** — 未跑验收命令不勾 `[x]`。
3. **v1.0 不退化** — 勾任一项前 stage16-v09-regress 或等价不得红。
4. **非里程碑 WIP 不 push** — path 2 无 tag/release；候选 fix 本地绿 → 一次 push。
5. **ROADMAP 终站** — Stage 16 已毕业；本看板 **不是** Stage 17。

---

## 洞清单映射（SCOPE-CUT v1.0 FINAL）

| ID | Disposition | 看板门 | 诚实状态 |
|----|-------------|--------|----------|
| **OW-H00** | **CLOSED** | （基线 · 已关） | `three_peer_full=EQUAL` · **`72c27c9f`** / 18944 B · stub_nz=905 |
| **OW-STUB** | CUT | A/C | manual-map stub tail · stub_nz=905 ∈ [40,950] |
| **OW-RT** | CUT | A/C | sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so` · Rust runtime |
| **OW-IAT** | CUT | **A**（主阻塞） | manual-map wired · PEB LoadLibrary DROPPED · **with-sidecar RED** |
| **OW-SEED** | CUT | C | seed 仍 Rust `yoyo.exe` 发射 · emitter+seed hash pin |
| **REL-FULLTEXT** | CUT | C | full `.text` peer compare · 禁止假 EQUAL 话术 |
| **REL-STUBOS** | CUT | C | Plan9/FreeBSD/Haiku/Serenity stub I/O |

---

## 关洞三门（A / B / C）

### 待做 / 已勾

- [x] **基线：OW-H00 CLOSED** — three-peer full `.text` EQUAL · **`72c27c9f`** / 18944 B；JS IAT sync 后 CLOSED（Stage 16 定稿 + post-v1.0 JS/asm lockstep）· **非本看板 tick 项**（已关 · 勿回改）

- [ ] **A：Win OW-IAT wire-up smoke GREEN** — **当前阻塞**  
  - **验收**：`& .\scripts\stage17-ow-iat-wireup.ps1` exit 0  
  - **子项 1（GREEN）**：no-sidecar fail-closed — exit≠0，**非 AV**（CreateFile fail-closed；master **已绿**）  
  - **子项 2（RED）**：with-sidecar — cwd `yoyo_rt.dll` + manual-map H_00 → `gen1.exe` → `output.exe` exit 0（master **AV · import/GPA**）  
  - **诚实状态**：OW-IAT **仍 CUT**（sidecar + kernel32 I/O）；全脚本 GREEN **≠ CLOSED**  
  - **信任链**：manual-map 自举链可跑通；为后续去 sidecar / YOYO-built runtime 铺路

- [ ] **B：Linux OW-IAT / tramp 回归不退化** — dlopen@PLT hybrid tramp · no libdl NEEDED  
  - **验收**：`& wsl -e bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh` exit 0  
  - **或**：`& .\scripts\stage16-v09-regress.ps1 -SkipBuild` 内 stage10-linux 子门 exit 0  
  - **诚实状态**：OW-IAT **仍 CUT**（dlopen + ld.so libc + cwd sidecar `.so`）  
  - **信任链**：Win A 修时不丢 Linux gen4≡gen3_direct EQUAL

- [ ] **C：洞清单 sync + BACKEND_SUPPORT 诚实状态** — FINAL inventory 与 BACKEND 一致  
  - **验收 1**：`& .\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild` exit 0 · `HOLE_INVENTORY_V10 status=FINAL`  
  - **验收 2**：`& .\scripts\stage15-hole-inventory.ps1 -SkipBuild` exit 0（Stage 15-A 不退化）  
  - **验收 3**：`BACKEND_SUPPORT.md` 含 OW-* CUT / OW-H00 CLOSED · 无 Thompson-proof / fake CLOSED  
  - **诚实状态**：A 绿后更新 inventory 观测行（Win smoke GREEN · **仍 CUT**）；closed=1 cut=6  
  - **信任链**：文档与机器门禁对齐；Release 边界不退化

### path 2 里程碑（A+B+C 全绿 · 无 tag）

当 A、B、C 均 `[x]`：在本节写完成日期 + master tip + 观测摘要；**不发 tag / GitHub Release**（v1.0 已毕业）。

---

## 验收命令

```powershell
cd F:\yoyo

# Gate A — Win OW-IAT wire-up（post-v1.0 门禁；脚本名 stage17 ≠ ROADMAP Stage 17）
& .\scripts\stage17-ow-iat-wireup.ps1

# Gate A 快速复验（已有 release 二进制）
& .\scripts\stage17-ow-iat-wireup.ps1 -SkipBuild

# Gate B — Linux tramp / sidecar 回归
& wsl -e bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh

# Gate C — 洞清单 FINAL + Stage 15-A 不退化
& .\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
& .\scripts\stage15-hole-inventory.ps1 -SkipBuild

# v1.0 全回归（post-v1.0 修洞前/后 sanity）
& .\scripts\stage16-v09-regress.ps1 -SkipBuild

# H00 多相 bisect（仅本地 opt-in；默认关 · CI anti-thrash）
$env:H00_BISECT = '1'
& .\scripts\stage17-ow-iat-wireup.ps1 -EnableBisect
Remove-Item Env:H00_BISECT -ErrorAction SilentlyContinue

# 日常 DDC
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test ddc
```

> **注**：`stage17-ow-iat-wireup.ps1` 文件名沿用 post-v1.0 OW-IAT 门禁编号；**不**表示 ROADMAP 存在 Stage 17。CI workflow 同名 gate 亦同。

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — Win OW-IAT smoke

```text
Post-v1.0 关洞项 A：Win OW-IAT wire-up smoke GREEN。
目标：no-sidecar fail-closed + with-sidecar gen1→output.exe 全绿。
验收：& .\scripts\stage17-ow-iat-wireup.ps1 exit 0。
约束：本地先绿；anti-thrash；OW-IAT 仍 CUT；不要 push 风暴。
```

### 任务 B — Linux 回归

```text
Post-v1.0 关洞项 B：Linux OW-IAT/tramp 回归不退化。
验收：stage10-linux-pure-m4.sh exit 0。
约束：最小 diff；OW-IAT 仍 CUT。
```

### 任务 C — 清单 sync

```text
Post-v1.0 关洞项 C：SCOPE-CUT + BACKEND_SUPPORT 诚实 sync。
验收：stage16-scope-cut-finalize + stage15-hole-inventory -SkipBuild exit 0。
约束：不 fake CLOSED；closed=1 cut=6。
```

---

## 负责人原则

0. **path 2 = 关洞，不是新功能轨** — 只缩 OW-* / 诚实 CUT；禁止 invent ROADMAP 外能力。
1. **OW-H00 已 CLOSED** — 勿回改；three-peer EQUAL 是 CLOSED 证据。
2. **OW-IAT GREEN ≠ CLOSED** — sidecar / LoadLibrary 面仍在则必 CUT。
3. **CI anti-thrash** — 本地 smoke 先绿；连续 2 次红 CI → 停推。
4. **AUTO 停手** — `ACTIVE=0` · `COMPLETED=1`；用户 `继续/关洞` 才读本看板 tick。

---

*创建：2026-08-31 · v1.0 毕业后 · post-v1.0 path 2 关洞 · 模板对齐 STAGE16_OWNER_CHECKLIST.md*

**当前 master 诚实快照（2026-08-31）：** no-sidecar **GREEN** · with-sidecar **RED AV** · anti-thrash **ACTIVE**（PR #14）· OW-IAT **CUT** · OW-H00 **CLOSED**
