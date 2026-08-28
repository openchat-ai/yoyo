# Stage 13 负责人看板（v0.7 · seed/link host / 跨平台 parity）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 13 的 A/B/C/D 每一项，毕业时必须能回答：**它如何缩小 seed/link 宿主信任，或加厚跨平台 / Relock 可观测面？** v0.7 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.6 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.7.md` — seed/link host + 跨平台 parity + Relock 纪律；**非** MCU / Morph 主赛道。  
> **基线**：Stage 12 已毕业（2026-08-28）；v0.6.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.6.md`（仍 Rust runtime + LoadLibrary/libdl；full `.text` 可 DIFF）。

## 🎯 Stage 13 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.7 已毕业（2026-08-29）· 见 SCOPE-v0.7.md / RELEASE-v0.7.md
                              → 下一主线 v0.8：SCOPE-v0.8.md + STAGE14_OWNER_CHECKLIST.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE13_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE13_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.7.md` |

相关：`STAGE12_OWNER_CHECKLIST.md`（v0.6 已毕业）、`RELEASE-v0.6.md`、`RELEASE-v0.7.md`、`BACKEND_SUPPORT.md`。  
**下一主线**：`STAGE14_OWNER_CHECKLIST.md` · `SCOPE-v0.8.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v0.8/Stage14（已定）并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–12 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — seed/link host（**自举入口宿主洞**）
2. **B** — 跨平台 parity
3. **C** — v0.6 回归不退化（门禁加固 / 观测）
4. **D** — v0.7 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；D 依赖 A/B/C。

**下一关** = 已毕业 → 见 `STAGE14_OWNER_CHECKLIST.md`（v0.8）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 13 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
# Stage 13 gates: stage13-* （落地后补）
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage13-v06-regress.ps1
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 13 毕业四门

### 待做

- [x] **A：seed/link host** — fail-closed 观测 + 合同 · **信任链**（2026-08-28 复验）：`scripts/stage13-link-host.ps1` exit **0**；`yoyo.exe test all` exit **0**；spot stage12-A/B + stage11-A/B + lock pin **0**。批准种子 = H_00 `seed_host_compile*`；`link`≡`bootstrap`(无 `--selfhost`) DDC EQUAL；PE/ELF ≤ MAX；Win 禁 GetTempPathA；`--selfhost` 必须 DIFF + `SEED_HOST path=gennrt`。**诚实剩余**：Rust `yoyo.exe` 仍发射种子；embedded runtime + LoadLibrary/libdl；Linux `SEED_HOST` 在当前 release binary 上仍可能报 `path=plain`（门禁以 `libyoyo_runtime.so`+`dlopen` markers fail-closed；源码 classifier 已认 `h00`，待无竞态 rebuild 钉死 `path=h00`）
- [x] **B：跨平台 parity** — Win/Linux（+ stub OS 诚实钉）parity 门禁加厚 · **信任链**（2026-08-29）：`scripts/stage13-cross-platform-parity.ps1`（alias `stage13-parity.ps1` / `stage13-cross-parity.ps1`）exit **0** ALL_GREEN；内嵌 stage12-three-peer-io + stage13-link-host（禁 SkipLinux）+ stage9-pure-m4 + WSL stage10-linux-pure-m4（禁 SkipWsl）；stub OS `freebsd`/`haiku` ALLOC/LOAD/WRITE **Rust=JS=asm** 17B G-SM-IO（禁 0F05 invent）+ `plan9`/`serenity` NOP/SERE 诚实分叉钉 + unknown→stub（apple/android）；linux ALLOC 对比须有 syscall。**诚实剩余**：stub OS 仍 stub（非生产 I/O）；Rust runtime + LoadLibrary/libdl；full `.text` 可 DIFF；macOS 非门禁
- [x] **C：v0.6 回归不退化** — stage12/stage11/stage10/stage9/fullbody/lock/gen12 门禁保持绿 · **信任链**（2026-08-29）：`scripts/stage13-v06-regress.ps1` exit **0** ALL_GREEN；串行 fail-closed：stage13-link-host + stage13-cross-platform-parity + stage12-v05-regress（内嵌 all/lock/gen12/fullbody + stage11/10/9 + stage12 A/B + lock pin + WSL stage10）；一次 release build 后子门 `-SkipBuild`；`-SkipWsl` 禁毕业。**诚实剩余**：Rust runtime + LoadLibrary/libdl；full `.text` 可 DIFF；stub OS 仍 stub；seed 仍由 Rust `yoyo.exe` 发射
- [x] **D：v0.7 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.7.md` 毕业判定 + `RELEASE-v0.7.md` / `RELEASE-NOTES-v0.7.md`（2026-08-29）· **信任链**：Decision #25 pin 仍权威；`verify-lock-pin` exit 0；`stage13-v06-regress.ps1 -SkipBuild` ALL_GREEN；RELEASE 诚实写 DDC=detection、仍 Rust runtime + LoadLibrary/libdl、full `.text` 可 DIFF、stub OS 仍 stub、**seed 仍 Rust 发射**

### 可选 · 低优先级（不挡 v0.7 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — 仍诚实剩余；仅当负责人点名，勿偷塞进 v0.7 IN

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — seed/link host

```text
Stage 13 毕业项 A：seed/link host 收缩或 fail-closed 观测（v0.7 主信任门）。
目标：缩小 yoyo link / bootstrap 种子旁路信任；不得跳过 DDC。
验收：新增/加固 scripts/stage13-* 退出码 0；
cargo run -- test all 退出码 0；stage12 / stage11 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — 跨平台 parity

```text
Stage 13 毕业项 B：跨平台 parity。
验收：可机器脚本 exit 0；诚实写 stub OS 边界；v0.6 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.6 回归不退化

```text
Stage 13 毕业项 C：加固/确认 v0.6 门禁不退化。
验收：stage12-* + stage11-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.7 毕业门禁

```text
Stage 13 毕业项 D：v0.7 毕业收口。
写 RELEASE-v0.7.md；SCOPE-v0.7 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v0.7 = seed/link / 跨平台 parity** — MCU / Morph 不挡毕业。
2. **v0.6 不退化** — 每项验收必须含 stage12 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-28 · v0.6.0 后定稿 Stage 13 / v0.7 · 见 `SCOPE-v0.7.md` · **v0.7 已毕业 2026-08-29** · 下一主线 → `SCOPE-v0.8.md` / `STAGE14_OWNER_CHECKLIST.md`*
