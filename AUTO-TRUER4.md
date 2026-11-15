# AUTO-TRUER4 — 更真⁴（stub TCB + sidecar 离关键路径）

> 用户授权（2026-11-09）：**都做**。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER3.md
NORTH_STAR=冷启可执行根=place-stub；H00 sidecar 不进关键路径
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **W0** | place-stub PE（13 824B）能 place 出全量 C1 | **绿** |
| **W1** | 冷启 SoT = stub；bins = 内容 SoT；全量 PE 派生 | **绿** |
| **W2** | 关键路径无 sidecar；H00+dll 仅 contrast | **绿** |
| **W3** | `stage-truer` GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer.ps1
# trust_root=place_stub · stub_bytes=13824 · sidecar_required_on_critical=ABSENT
```

## 诚实

- 可执行冷启根：447 KB → **13 KB** stub（无 parse1）  
- 内容信任仍在 **bins pin**（~438 KB 数据）  
- sidecar **未从磁盘删除**；只是退出关键路径（contrast_only）  
- H00 无 dll 仍 exit=2；真「物理去掉 LoadLibrary 面」未做  
- ≠ Thompson  

## 若要再「更真」

物理去掉 H_00 LoadLibrary / 把 bins 也收进更小 SoT / YOYO 源改语义。

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · stub_tcb + no_sidecar_critical*
