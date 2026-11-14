# 07 — 门禁矩阵

> 绿才勾。命令均在仓库根；PowerShell `&`。

## 语言主尺（必跑）

| 门禁 | 命令 | 证明 |
|------|------|------|
| LANG1 | `& .\scripts\stage-lang1.ps1` | SURFACE 文档 + ANYPROG |
| ANYPROG | `& .\scripts\stage-anyprog.ps1` | 七工具 |

## 自举回归（动自举面或毕业前）

| 门禁 | 命令 |
|------|------|
| LESSHOSTX | `& .\scripts\stage-lesshostx.ps1` |
| BREAK3 | `& .\scripts\stage-break3.ps1` |
| YC2 | `& .\scripts\yc2-compiler-payload-repro.ps1` |
| FULLBAR | `& .\scripts\stage-fullbar-trust-root.ps1 -SkipEmitRt` |
| CLOSED | `& .\scripts\stage-yoyo-closed.ps1` |

## 检测 / 链接回归

| 门禁 | 命令 |
|------|------|
| DDC | `cd yoyo-rust\verifier; cargo run -- test ddc` |
| LLMWRITE（冻结回归） | `& .\scripts\stage-llmwrite.ps1`（非 wake 主路径） |

## Pin 辅助

| 库 | 路径 |
|----|------|
| Assert helpers | `scripts/lib/Assert-YoyoFilePin.ps1` |
| Unattended UI | `scripts/lib/unattended-ui-harden.ps1` |

## 工作区垃圾

`scripts\_stage-*` · `scripts\_tmp-*` 为跑出来的工作目录；**不要**当 SoT commit（除非刻意 fixture）。

## 失败处理

1. 读脚本打印的 `status=RED reason=`  
2. 用最小复现（单 exe / 单 ty）隔离  
3. 验证二进制时间戳与 hash（防旧 exe）  
4. 修 → 重跑 → 绿才勾
