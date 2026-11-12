# AUTO-LLMWRITEDRIFT — 极端环境漂移 · 源与产物皆无歧义

> 接 LLMWRITESCALE。「电子漂移」收成可机检：**源唯一** + **编译确定性** + **产物钉死**；任一 bit 漂 → 门禁红。≠ 抗辐射硅 / ≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITESCALE.md
NORTH_STAR=同源反复 link byte-equal；artifact sha256 pin；源/产物 bitflip 必检出
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 极端环境本轨定义

| 层 | 要求 | 门禁 |
| --- | --- | --- |
| **源** | 规范形唯一（继承 SCALE） | validate GREEN |
| **编译** | 同 `.ty` 连链 N 次 → exe **byte-equal** | rebuild_determinism |
| **产物** | 每样本 `artifact_pins.sha256` 钉死 | pin EQUAL |
| **漂移检出** | 源翻 1 bit → 契约红 **或** 产物 hash 变；产物翻 1 bit → pin 红 | bitflip_detect |

**不是**：硬件 ECC / 抗 SEU 芯片；不是全 ISA。只保证软件链上「漂了必看见」。

## Checklist

- [x] **D0** contract v4：`artifact_pins` + `drift`  
- [x] **D1** rebuild N 次 byte-equal  
- [x] **D2** 样本 exe sha256 钉 + stage 验  
- [x] **D3** 源 bitflip / 产物 bitflip 必红  
- [x] **D-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-12)

- contract pin `89d4c60792364dab` · artifact pins ×5 · rebuild=3 EQUAL  
- source bitflip `05→04` hash_changed · invalid token RED · artifact bitflip pin_mismatch  
- `& F:\yoyo\scripts\stage-llmwrite.ps1` GREEN  
- 诚实：软件 hash 完整性 ≠ 抗辐射硅 ≠ Thompson  

---

*COMPLETED=1 · ACTIVE=0*
