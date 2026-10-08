# 分支审计 — cursor/post-1.0-gate-g-in-dll-compile

**审计日期**：2026-09-30
**审计目标**：脚本和文档里所有硬编码的数字（SHA、bytes、hash）是否都能核对到真实运行或 git 历史
**触发**：`b3d6637` commit 捏造了 `yoyo_runtime_h00_compile` 符号名（在 `c8d9675` 撤回），用户要求全面审计避免类似问题

## 审计结论

**活着的假数据：0 处。**
**已撤回的假数据：1 处**（`b3d6637` 的 `yoyo_runtime_h00_compile`，已在 `c8d9675` 修正）。

## 全部核对通过的 pin

| Pin 值 | 文件 | 实测 | 匹配 |
|-------|------|-----|------|
| `yoyo.exe` bytes=`22292992` | SCOPE-CUT-v1.0-ow-seed-observe.md | 22292992 | ✓ |
| `yoyo.exe` sha256_prefix=`52f0a813b354a0f5` | 同上 | 52f0a813b354a0f5 | ✓ |
| `gen1.exe` bytes=`251392` | 同上 | 251392 | ✓ |
| `gen1.exe` sha256_prefix=`b0a8dbb0d3133e2a` | 同上 | b0a8dbb0d3133e2a | ✓ |
| `gen4.exe` bytes=`251392` | scripts/ow-seed-trust-root.pin | 251392 | ✓ |
| `gen4.exe` sha256=`b0a8dbb0d3133e2a…` | 同上 | 匹配 | ✓ |
| `.text` sha256_prefix=`1ec3766f` | SCOPE-CUT-v1.0-ow-seed-observe.md | 1ec3766f (`yoyo diff` 输出 hash_a) | ✓ |
| `compared_bytes=20992` | 同上 | 20992 | ✓ |
| Rust `yoyo_rt.dll` 导出 = `{yoyo_runtime_selfhost_main, yoyo_runtime_selfhost_paths}` | SCOPE-CUT I-1（已修正）| 实测 | ✓ |
| YOYO in-DLL-recompile pe_dll 导出 = `{yoyo_runtime_selfhost_main, yoyo_in_dll_recompile(marker)}` | 同上 | 实测 | ✓ |
| 真实导出差集 = `{yoyo_runtime_selfhost_paths}` | 同上 | 实测 | ✓ |
| `gen4.exe` PE import dir = 0 条 | 同上 | 实测（`imp_rva=0x0 imp_size=0`）| ✓ |

## 硬编码 SHA 分类（设计意图 vs 观察值）

**决策锚点（硬编码是设计意图）**：

- `scripts/stage14-lock-harden.ps1`: `Decision25Pin` / `Decision25Prev` — 决策链锚点
- `scripts/_probe/parallel-batch-*.mjs`: `PIN` 常量 — 每批次的决策锚点
- `scripts/rebuild_golden.py` / `rebuild.py`: `previous_sha256=20391de…` — 上次 golden 锚点

这些是**不可变的决策点**，硬编码是正确设计。

**观察值（必须每次实测）**：

- `scripts/ow-seed-trust-root.pin`: `bytes=251392`, `sha256=…` — 观察 gen4 现状（fail-closed）
- 所有 stage17 脚本里的 SHA 输出 —— 全部通过 `Get-FileHash` 实时计算，无硬编码

**结论**：脚本里没有任何硬编码的观察值 SHA，全部实时计算。

## 撤回的捏造记录

| Commit | 内容 | 修正 commit |
|--------|------|------------|
| `b3d6637` | "gen4 需要导出 `yoyo_runtime_h00_compile`" — 捏造符号名 | `c8d9675` 修正为真实差集 `{yoyo_runtime_selfhost_paths}` |

## 审计方法

```powershell
# 1) dump 所有 SHA-like 常量
rg -i "sha256_prefix|sha256=[0-9a-f]{8,}|sha=[0-9a-f]{16,}|[0-9a-f]{32,}" scripts/
rg -i "sha256_prefix|sha256=[0-9a-f]{8,}|sha=[0-9a-f]{16,}|[0-9a-f]{32,}" SCOPE-CUT-v1.0-ow-seed-observe.md POST-1.0-HOLE-CHECKLIST.md

# 2) 逐个核对 pin
# yoyo.exe / gen1.exe / gen4.exe 用 Get-FileHash 实时比对
# .text hash 用 yoyo.exe diff 输出 hash_a 比对
# Rust yoyo_rt.dll 和 YOYO pe_dll 的导出通过 ASCII 字符串扫描比对

# 3) 分类硬编码：决策锚点 vs 观察值
```

## 诚实度

- **0** 处活着的假数据
- **1** 处已撤回的捏造（在 git 历史可见）
- 所有观察 pin 都通过实时运行核对
- 所有决策 pin 都是设计上必须硬编码的锚点
- 分支没有假 CLOSED，没有假绿
- OW-SEED 依然是 CUT，J 依然未勾
