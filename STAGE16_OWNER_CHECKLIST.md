# Stage 16 负责人看板（v1.0 · 全关或 SCOPE-CUT 定稿 / RELEASE）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 16 的 A/B/C/D 每一项，毕业时必须能回答：**它如何关洞或把 SCOPE-CUT 定稿进 RELEASE，或钉死 detection 对外话术？** v1.0 是 **终态收口**，不是功能堆砌。

> **用途**：v0.9 发布后下一主线（**ROADMAP 终站**）— **已毕业**。  
> **范围**：`SCOPE-v1.0.md` — 全关或 SCOPE-CUT 定稿 · detection 话术 · RELEASE · tag；**非** MCU / Morph 主赛道。  
> **基线**：Stage 15 已毕业（2026-08-29）；v0.9.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v1.0.md` / `SCOPE-CUT-v1.0-hole-inventory.md`（HOLE_INVENTORY_V10 FINAL；closed=0 cut=7；仍 Rust runtime + LoadLibrary/libdl；seed 仍 Rust 发射）。

## 🎯 Stage 16 进度

```text
[x] A  [x] B  [x] C  [x] D   →  v1.0 已毕业；无 Stage 17；见 SCOPE-v1.0.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE16_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE16_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v1.0.md` |

相关：`STAGE15_OWNER_CHECKLIST.md`（v0.9 已毕业）、`RELEASE-v1.0.md`、`SCOPE-CUT-v1.0-hole-inventory.md`、`DETECTION-BANLIST-v1.0.md`、`BACKEND_SUPPORT.md`。  
**→ 1.0 终站（已完成）**：本看板 · `SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=0` · `COMPLETED=1`）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` 现 `ACTIVE=0` · `COMPLETED=1` → **停手**；勿 invent Stage 17 |
| **单轨** | A→B→C→D；全绿后 **毕业 1.0**（无 Stage 17） — **DONE** |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） — **DONE** |
| **看板** | Stage 4–16 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 关洞或 SCOPE-CUT 定稿
2. **B** — detection 话术 / RELEASE 边界
3. **C** — v0.9 回归不退化
4. **D** — v1.0 毕业门禁 + Relock（若需要）+ RELEASE + tag

**不要跳关**：勾选顺序仍 A→B→C→D；D 依赖 A/B/C。

**下一项** = **无**（ROADMAP 终点）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 16 回归（毕业后仍可复验）：

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
.\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
.\scripts\stage16-detection-wording.ps1 -SkipBuild
.\scripts\stage16-v09-regress.ps1 -SkipBuild
```

---

## Stage 16 毕业四门

### 待做 / 已勾

- [x] **A：关洞或 SCOPE-CUT 定稿** — `SCOPE-CUT-v1.0-hole-inventory.md` + `scripts/stage16-scope-cut-finalize.ps1`（alias `stage16-a.ps1`）；OW-\*/REL-\* 逐项 `FINAL_HOLE id=… disposition=CLOSED|CUT`；**closed=0 cut=7** · `HOLE_INVENTORY_V10 status=FINAL` · full `.text` DIFF 诚实 · nested stage15-A exit 0 · **信任链**：洞从「v0.9 ACTIVE 枚举」变「1.0 FINAL SCOPE-CUT」；七项均 CUT（无假 CLOSED）；OW-RT/IAT/SEED 预期仍 CUT；Lock pin Decision #25 未改
- [x] **B：detection 话术 / RELEASE 边界** — `DETECTION-BANLIST-v1.0.md` + `RELEASE-v1.0.md` + `scripts/stage16-detection-wording.ps1`（alias `stage16-b.ps1`）；禁词 BAN id 可验；CUT 七项入 RELEASE；禁 Thompson-proof / fully closed / fake EQUAL 肯定句 · nested stage16-A exit 0 · **信任链**：少误宣称 Thompson / 假关洞；DDC=detection；closed=0 cut=7 钉进 RELEASE
- [x] **C：v0.9 回归不退化** — `scripts/stage16-v09-regress.ps1`（alias `stage16-c.ps1`）exit 0 · `ALL_GREEN` · stage15 A/B/C 实质 + stage14–9 + fullbody/lock/gen12 + WSL + Stage 16 A/B 全 0；one cargo → named `-SkipBuild`；**零并行 cargo** · **信任链**：扩面不丢 v0.9 基线；summary `scripts/_stage16-v09-regress/summary.txt`
- [x] **D：v1.0 毕业门禁** — A/B/C 全绿 + Lock 复验 PINNED Decision #25（**无 Relock**）+ `SCOPE-v1.0.md` 毕业判定 + `RELEASE-v1.0.md` Status:** graduated** · `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN（**02:14:21**）· tag `v1.0.0` · **信任链**：RELEASE 诚实写 DDC=detection；剩余 CUT 全列；无假 CLOSED；AUTO COMPLETED · ACTIVE=0；无 Stage 17

### 可选 · 低优先级（不挡 v1.0 毕业）

- MCU / Morph / SIMD — 仅当负责人点名；**默认不做**（v1.0 已毕业）
- YOYO-built runtime — 仍属 post-1.0；OW-RT 仍 **CUT**

---

## 对 AI 说什么（复制粘贴话术）

### 任务 A — 关洞或 SCOPE-CUT 定稿

```text
Stage 16 毕业项 A：关洞或 SCOPE-CUT 定稿（v1.0 主信任门）。
目标：对 OW-* / REL-* 能关则关，不能关则 1.0 SCOPE-CUT 定稿；不得假 EQUAL。
验收：新增/加固 scripts/stage16-* 退出码 0；
stage15 / stage14 门禁不退化。
约束：最小 diff；服务打破后门魔咒；不要 push。
```

### 任务 B — detection 话术

```text
Stage 16 毕业项 B：detection 话术 / RELEASE 边界。
验收：禁词表 + 剩余 CUT 清单可验；v0.9 门禁不退化。
约束：最小 diff；不要 push。
```

### 任务 C — v0.9 回归不退化

```text
Stage 16 毕业项 C：加固/确认 v0.9 门禁不退化。
验收：stage15-* + stage14-* + stage13–9 + test all/lock/gen12/fullbody 全 0。
约束：最小 diff；零并行 cargo；不要 push。
```

### 任务 D — v1.0 毕业门禁

```text
Stage 16 毕业项 D：v1.0 毕业收口。
写 RELEASE-v1.0.md；SCOPE-v1.0 毕业判定；验收全绿后勾 D；AUTO 下 commit+tag+push+gh。
```

---

## 负责人原则

0. **打破后门魔咒（北星）** — 每项毕业须说明如何加强 DDC/Lock 或缩小盲区。
1. **v1.0 = 全关或 SCOPE-CUT 定稿 · RELEASE** — MCU / Morph 不挡毕业。
2. **v0.9 不退化** — 每项验收必须含 stage15 门禁或等价。
3. **绿了才勾** — 未跑验收不勾 `[x]`。
4. **诚实** — 不宣称 Thompson 免疫；Rust runtime / LoadLibrary / seed 若仍在须继续写明。
5. **非毕业 WIP 不 push** — 毕业 D 按 AUTO release。
6. **终站** — Stage 16 / v1.0 毕业后无 Stage 17；`ACTIVE=0` · `COMPLETED=1`。

---

*创建：2026-08-29 · v0.9.0 后定稿 Stage 16 / v1.0 · 毕业：2026-08-29 · 见 `SCOPE-v1.0.md` · ROADMAP 终站*

**Post-v1.0（2026-08-29 · 4f3064d）：** OW-H00 **CLOSED**（three-peer full `.text` EQUAL）；`stage15/16` gate 钉 `three_peer_full`；PR #1/#3 closed/merged on master。
