# Stage 11 负责人看板（v0.5 · YOYO-built / 更薄 runtime）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 11 的 A/B/C/D 每一项，毕业时必须能回答：**它如何把更多 runtime 字节纳入三链 DDC + Lock 监控，或缩小对 Rust 编译 runtime / LoadLibrary 宿主的信任？** v0.5 是 **收洞手段**，不是功能堆砌。

> **用途**：v0.4 发布后下一主线。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v0.5.md` — YOYO-built/更薄 runtime + 收缩 LoadLibrary host；**非** MCU / Morph 主赛道。  
> **基线**：Stage 10 已毕业（2026-08-28）；v0.4.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.4.md`。

## 🎯 Stage 11 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v0.5 已毕业（2026-08-28）· 见 SCOPE-v0.5.md / RELEASE-v0.5.md
                              → 下一主线 v0.6：SCOPE-v0.6.md + STAGE12_OWNER_CHECKLIST.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE11_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE11_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v0.5.md` |

相关：`STAGE10_OWNER_CHECKLIST.md`（v0.4 已毕业）、`RELEASE-v0.4.md`、`BACKEND_SUPPORT.md`。  
**→ 1.0**：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **自动定** v0.6/Stage12 并继续 |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–10 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — YOYO-built / 更薄 runtime（**最大剩余信任洞**）
2. **B** — 收缩 LoadLibrary / libdl host
3. **C** — v0.4 回归不退化（门禁加固 / 观测）
4. **D** — v0.5 毕业门禁 + Relock（若需要）+ 文档收口

**不要跳关**：B 可与 A 技术并行，但勾选顺序仍 A→B；D 依赖 A/B/C。

**下一关** = 已毕业 → 见 `STAGE12_OWNER_CHECKLIST.md`（v0.6）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 11 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\verify-lock-pin.ps1
# WSL: bash scripts/stage10-linux-pure-m4.sh
```

---

## Stage 11 毕业四门

### 待做

- [x] **A：YOYO-built / 更薄 runtime** — 替换或显著收缩每个 genN 嵌入的 Rust `yoyo_runtime.dll` / `.so`；策略落地须可脚本验收（大小/parity/自建路径）· **信任链**（2026-08-28）：`scripts/stage11-runtime-surface.ps1` GREEN；DLL **231936→154624** B（fail-closed MAX **170000**；`profile.release-runtime` fat LTO + strip + `panic=abort` + `opt-level=z`）；`.so` **592064→407232**；genN PE **322560→248832** / ELF **704512→512000**；gen12 窗仍 **18432** B，SHA `43ffde58`→`d782166d`（H_00 stub 含 `dll_embed_size`）；embed exact + gen1 H_00 compile ≡ bootstrap `.text` DDC；**仍** Rust cdylib、窗外、非 YOYO-built（诚实）
- [x] **B：收缩 LoadLibrary / libdl host** — 缩小 H_00 提取后对宿主加载器的信任；或把关键路径纳入可观测门禁 · **信任链**（2026-08-28）：`scripts/stage11-loadlibrary-host.ps1` GREEN；Win H_00 **cwd-relative** `yoyo_rt.dll`（dropped **GetTempPathA/lstrcatA**；host-loader IAT **5→3**）；Linux tramp **14464→9768** B（nostdlib `.S`；fail-closed MAX **12000**；exact embed）；stub deterministic re-link `.text` DDC EQUAL（gen12 SHA **`d782166d`** · 18432B）；smoke cwd extract + LoadLibrary；**仍** host LoadLibraryA / libdl（诚实）
- [x] **C：v0.4 回归不退化** — stage10/stage9/fullbody/lock/gen12 门禁保持绿；必要时加固观测脚本 · **已关**（2026-08-28）：`scripts/_stage11-c-accept.log` ALL_GREEN — build/s11-rt/s11-ll/s10-rt/s10-asm/s10-linux/s9-m4/s9-js/s5/pin/all/lock/gen12/fullbody EXIT=0
- [x] **D：v0.5 毕业门禁** — A/B/C 全绿 + Lock 复验（pin 未改 · 无 Relock）+ `SCOPE-v0.5.md` 毕业判定 + `RELEASE-v0.5.md` / `RELEASE-NOTES-v0.5.md`（2026-08-28）· **信任链**：Decision #25 pin 仍权威；gen12 `d782166d` / 18432B；DLL **154624** B fail-closed；LoadLibrary cwd + tramp **9768** B；RELEASE 诚实写 DDC=detection、仍 Rust runtime + LoadLibrary/libdl

### 可选 · 低优先级（不挡 v0.5 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- 三 peer full-body section-ddc — v0.6 主题，勿提前塞进 v0.5 IN

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — YOYO-built / 更薄 runtime

```text
Stage 11 毕业项 A：YOYO-built 或显著更薄的嵌入 runtime（v0.4 最大诚实剩余洞）。
目标：减少对 Rust 编译 yoyo_runtime.dll/.so 的信任——自建、迁出、或把关键路径字节纳入可脚本观测窗口；不得跳过 DDC。
验收：新增 scripts/stage11-runtime-*.ps1（名称可调整）退出码 0；
文档化前后信任边界（相对 gen12 窗口 / SHA）；
cargo run -- test all 退出码 0；stage10 / stage9 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — 收缩 LoadLibrary / libdl host

```text
Stage 11 毕业项 B：收缩 H_00 后 LoadLibrary / libdl 宿主加载旁路。
验收：可机器脚本 exit 0；v0.4 Linux/Win 纯 M4 不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.4 回归不退化

```text
Stage 11 毕业项 C：加固/确认 v0.4 门禁不退化。
验收：stage10-* + stage9-pure-m4 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；不要 push。
```

### 任务 D — v0.5 毕业门禁

```text
Stage 11 毕业项 D：v0.5 毕业收口。
写 RELEASE-v0.5.md；SCOPE-v0.5 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小宿主信任洞。
1. **v0.5 = YOYO-built / 更薄 runtime** — MCU / Morph 不挡毕业。
2. **v0.4 不退化** — 每项验收必须含 stage10 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；不宣称 DDC 覆盖 genN 每个字节。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。

---

*创建：2026-08-28 · v0.4.0 后定稿 Stage 11 / v0.5 · 见 `SCOPE-v0.5.md` · 长期 → `SCOPE-v1.0.md` / `ROADMAP-TO-1.0.md`*
