# AUTO-AUDITZERO — 八路后门消 0

> 1–7 = **CLOSED**（改被测字节必红）。  
> 8 = **HARD_BLOCK / 不可真正消 0**。  
> 用户若要求「8 也真正=0 才许停」→ **与禁区冲突：停，不假装 CLOSED**。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
HARD_BLOCK=1
HARD_BLOCK_PATH=8
HARD_BLOCK_REASON=collude_rewrite_pins_is_Trusting_Trust_residue;any_gate_that_reads_pins_can_be_rewritten_with_the_pins;labeling_CLOSED_equals_Thompson-proof_claim_FORBIDDEN
PARENT=AUTO-AUDITLIVE.md
NORTH_STAR=1-7 CLOSED; 8 明示不可消（勿假绿）
STOP_ONLY=停 | ACTIVE=0 | HARD_BLOCK
```

## 八路结果

| # | 手法 | 状态 | 说明 |
| --- | --- | --- | --- |
| 1 | 改 code.bin | **CLOSED** | 全文件 sha + 毒红 |
| 2 | 改 data.bin | **CLOSED** | 全文件 sha + 毒红 |
| 3 | 改 stub 门控 | **CLOSED** | stub sha + 毒红 |
| 4 | 改 seed | **CLOSED** | stub/.text + bins |
| 5 | 窗外跳 | **CLOSED** | OOW 窗 + beyond-place 整段 |
| 6 | 改 try_place 几何 | **CLOSED** | 在 place 半截 → stub sha |
| 7 | 毒现场文件 | **CLOSED** | live ≡ pin + 毒红 |
| 8 | 共谋改钉 | **HARD_BLOCK** | 见下 |

## 为何 #8 不能「真正消到 0」

共谋 = **同时改「被测物」和「绿的定义」（pin / 生成器 / 门禁脚本 / CI）**。

任何新增检查若仍读同一批钉、同一仓库、同一套流程，攻击者可以**连检查一起改掉**。  
把 #8 标成 CLOSED = 宣称已防住 Trusting Trust = **Thompson-proof 宣称 → 本仓禁止**。

可继续做的只是**抬成本**（多机独立复现、人眼仪式、带外公证）——那是**流程/人类**，不是再写一个 `verify-*.js` 能结案，也**不等于消到 0**。

## 门禁（1–7）

```powershell
& F:\yoyo\scripts\stage-auditzero.ps1
```

## 阻塞（给用户）

- **已做到**：方案 1–7 工程消 0（静默换被测物必红）。  
- **做不到且禁止假装**：方案 8 真正消 0。  
- **因此停**：不是偷懒；是 hard-block。若只要「假 CLOSED」请直接说，但规则下我不会写。

---

*HARD_BLOCK #8 · 2026-11-12*
