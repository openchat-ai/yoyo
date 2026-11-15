# 06 — 信任与检测（附录）

> 主尺是语言；本章保证「地基不烂」与「别吹牛」。

## 检测工具

| 工具 | 作用 |
|------|------|
| DDC | 多链/多次编分歧 detection（`cargo run -- test ddc`） |
| Lock | 钉住关键窗；改则红 |
| File pins | `Assert-YoyoShaPin` / payload.pin / place_c1 / OW-SEED |
| Poison stages | stub/bins/plant 毒化必红 |

## 信任面结算（摘要）

见 `TRUST-SURFACE-v1.md`：

- 1–7 路：换字节/现场文件 → 红（CLOSED）  
- **#8 共谋改钉**：同仓改 pin+产物不可真消 0 → HARD_BLOCK；双路见证（产品 pin ∧ seed pin）已上  
- **A1**：6 导入钉死；消除 = 空盘 → HARD_BLOCK  

## 对外话术

- DDC = **detection**，不是证明  
- 禁词表：`DETECTION-BANLIST-v1.0.md`  
- 禁止 Thompson-proof / 「0 漏洞」

## Agent 规则

- 语言版毕业前：自举回归绿  
- 不得以「再堆同仓 pin」宣称消除 #8  
- 关洞轨：`POST-1.0-HOLE-CHECKLIST.md`（用户说「关洞」时）
