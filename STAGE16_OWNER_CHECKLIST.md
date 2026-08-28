# Stage 16 负责人看板（v1.0 · 全关或 SCOPE-CUT 定稿 / RELEASE）

## 北星：打破后门魔咒

YOYO 的存在理由 **不是造语言**，而是 **用 DDC + Lock 在实践中检测编译器级后门**。Stage 16 的 A/B/C/D 每一项，毕业时必须能回答：**它如何关洞或把 SCOPE-CUT 定稿进 RELEASE，或钉死 detection 对外话术？** v1.0 是 **终态收口**，不是功能堆砌。

> **用途**：v0.9 发布后下一主线（**ROADMAP 终站**）。负责人每日扫命令绿不绿、勾没勾。  
> **范围**：`SCOPE-v1.0.md` — 全关或 SCOPE-CUT 定稿 · detection 话术 · RELEASE · tag；**非** MCU / Morph 主赛道。  
> **基线**：Stage 15 已毕业（2026-08-29）；v0.9.0 已发；Lock pin `0275802d…`（Decision #25）；诚实剩余面见 `RELEASE-v0.9.md` / `SCOPE-CUT-v0.9-hole-inventory.md`（HOLE_INVENTORY ACTIVE；closed=0 cut=7；仍 Rust runtime + LoadLibrary/libdl；seed 仍 Rust 发射）。

## 🎯 Stage 16 进度

```text
[ ] A  [ ] B  [ ] C  [ ] D   →  下一项 A；见 SCOPE-v1.0.md
```

> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。

---

## 如何打开看板

| 方式 | 操作 |
| ---- | ---- |
| **完整路径** | `F:\yoyo\STAGE16_OWNER_CHECKLIST.md` |
| **Cursor 内** | `Ctrl+P` → `STAGE16_OWNER` |
| **规格一页纸** | `F:\yoyo\SCOPE-v1.0.md` |

相关：`STAGE15_OWNER_CHECKLIST.md`（v0.9 已毕业）、`RELEASE-v0.9.md`、`SCOPE-CUT-v0.9-hole-inventory.md`、`BACKEND_SUPPORT.md`。  
**→ 1.0 终站**：本看板 · `SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · **`AUTO_TO_1.0.md`**（`ACTIVE=1` 无人值守）。

---

## 零指令自动执行

| 方式 | 操作 |
| ---- | ---- |
| **AUTO-TO-1.0** | `AUTO_TO_1.0.md` `ACTIVE=1` → 每 tick 无问询执行下一未勾项直至 1.0 / hard block / `停` |
| **单轨** | A→B→C→D；全绿后 **毕业 1.0**（无 Stage 17） |
| **毕业 D** | 绿后 auto commit + tag + GitHub Release + push（WIP 不 push） |
| **看板** | Agent 优先打开本文件；Stage 4–15 勿回改 |

### 毕业顺序（严格 · 按信任冲击）

1. **A** — 关洞或 SCOPE-CUT 定稿
2. **B** — detection 话术 / RELEASE 边界
3. **C** — v0.9 回归不退化
4. **D** — v1.0 毕业门禁 + Relock（若需要）+ RELEASE + tag

**不要跳关**：勾选顺序仍 A→B→C→D；D 依赖 A/B/C。

**下一项** = **A**（关洞或 SCOPE-CUT 定稿）。

---

## 每日例行（固定）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
```

Stage 16 进行中加跑（见各项验收）：

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
# Stage 16 gates as landed:
# .\scripts\stage16-*.ps1 -SkipBuild
```

---

## Stage 16 毕业四门

### 待做 / 已勾

- [ ] **A：关洞或 SCOPE-CUT 定稿** — 对 OW-\* / REL-\*（v0.9 cut=7）逐项 CLOSED（有证据）或写入 `SCOPE-CUT-v1.0-*.md` 定稿；机器门 `scripts/stage16-*` exit 0；不得假 EQUAL / 假 CLOSED · **信任链**：洞从「v0.9 枚举」变「1.0 终态」
- [ ] **B：detection 话术 / RELEASE 边界** — 对外 detection-only 禁词表 + 剩余 CUT 清单入 RELEASE 草案；可机器验收（脚本或文档门）· **信任链**：少误宣称 Thompson / 假关洞
- [ ] **C：v0.9 回归不退化** — stage15 A/B/C + stage14–9 + fullbody/lock/gen12 + WSL 全绿（`scripts/stage16-*-regress.ps1` 或等价）· 串行；named `-SkipBuild`；**零并行 cargo** · **信任链**：扩面不丢基线
- [ ] **D：v1.0 毕业门禁** — A/B/C 全绿 + Lock 复验（改源则 Relock）+ `SCOPE-v1.0.md` 毕业判定 + `RELEASE-v1.0.md` · **信任链**：RELEASE 诚实写 DDC=detection；剩余 CUT 全列；tag `v1.0.0`

### 可选 · 低优先级（不挡 v1.0 毕业）

- MCU / Morph / SIMD — 仅当 A–D 全绿且负责人点名；**默认不做**
- YOYO-built runtime — 仅当 A 明确关 OW-RT；勿偷塞进 v1.0 IN 并宣称已关（无证据）

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
6. **终站** — Stage 16 / v1.0 毕业后无 Stage 17；可 `ACTIVE=0`。

---

*创建：2026-08-29 · v0.9.0 后定稿 Stage 16 / v1.0 · 见 `SCOPE-v1.0.md` · ROADMAP 终站*
