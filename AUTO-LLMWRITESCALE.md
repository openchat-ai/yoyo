# AUTO-LLMWRITESCALE — 行·句·段·任意尺度无歧义

> 接 LLMWRITEX。把「极端/全域」从抽象收成：**每一行、每一句、每一段、任意长度程序** 都有唯一规范形 + 可机检 AST。人写后置。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEX.md
NORTH_STAR=组合文法权威；多尺度 AST；任意 N 生成 round-trip；命名 shape 仅别名
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 极致会长什么样（本轨定义）

| 尺度 | 唯一性 | 门禁 |
| --- | --- | --- |
| **行** | 规范 token 串（已有 v2 词法） | lex fail-closed |
| **句 stmt** | 一种 opcode+args ↔ 一种序列化 | grammar 白名单 |
| **段 body** | stmt 序列由自动机合法，非仅 3 个 named shape | dataflow/grammar |
| **程序** | `entry + body* + exit`；长度任意 | 生成 N=1..K round-trip |
| **多尺度 AST** | `lines` / `stmts` / `program` 金样 | byte-equal |

**不是**：全 ISA / 控制台 RAW / Winsock 已收编。那些仍 coverage=out，直到各自有同级契约。

## Checklist

- [x] **S0** contract v3：grammar + multi_scale_ast  
- [x] **S1** validate：shape 降为别名；文法+数据流  
- [x] **S2** 生成器：随机合法程序 N∈[1,K] canonicalize idempotent  
- [x] **S3** 多样本 AST（含非三 shape 的合法组合）+ stage 绿  
- [x] **S-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-12)

- contract pin `be890eb10e3e2155` · samples=5（含 copy08/resize 非三 shape）· poisons=22 · distinct_reasons=22 · generative trials=64 GREEN  
- `& F:\yoyo\scripts\stage-llmwrite.ps1` GREEN  
- 诚实：写面尺度极致 ≠ 全 ISA / Winsock / Thompson-proof  

---

*COMPLETED=1 · ACTIVE=0*
