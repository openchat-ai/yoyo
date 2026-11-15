# AUTO-LLMWRITE — 大模型无歧义可编程（人写后置）

> 接 ANYPROG。尺 = **模型能否按契约写出门禁可绿的程序**；不追求给人好看的语法。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-ANYPROG.md
NORTH_STAR=契约机读+样例校验绿；模型输出面无歧义
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 成功尺（本刀）

| ID | 项 | 状态 |
| --- | --- | --- |
| L0 | `MODEL-CONTRACT.md` + `model-contract.json` | **GREEN** |
| L1 | validate + 毒红（ret/op/raw） | **GREEN** |
| L2 | `stage-llmwrite.ps1` 结构+cat/head/zero 跑通 | **GREEN** |
| L3 | COMPLETED=1；人写后置 | **DONE** |

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## 诚实

- 契约 = 文件层无歧义写面（v1）；控制台/线程仍多 RAW，未纳入 LLM 契约样例  
- 不宣称「大模型已会写」；宣称 **写面可机检**  
- Winsock 仍 OPEN  
- ≠ Thompson  

## Checklist

- [x] **LW-0** 契约 md+json  
- [x] **LW-1** validate + 毒红样例  
- [x] **LW-2** stage 绿  
- [x] **LW-DONE**  

---

*COMPLETED 2026-11-12 · gate GREEN*
