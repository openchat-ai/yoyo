# AUTO-STUBDDC — 冷启 stub 双链检测

> 用户授权（2026-11-11）：**下一刀**。  
> 判据（`TRUST-SURFACE-v1.md`）：DDC/Lock 覆盖更实的自举字节。  
> 同一 trunc bins：JS `linkPeWin32(...,0x2000)` ≡ Rust `link_place_stub` ≡ pin fixture。  
> **不是**消 stub / Stage17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=TRUST-SURFACE-v1.md
NORTH_STAR=js_stub≡rust_stub≡pin；stage-stubddc GREEN
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **D1** | Rust `link_place_stub`（floor=0x2000）peer JS | **绿** |
| **D2** | `stage-stubddc`：rebuild JS + pin EQUAL + rust≡fx（1 test） | **绿** |
| **D3** | COMPLETED；hard-block 仍 stub+A1 | **绿** |

## 诚实

- 已真：JS rebuild ≡ pin ≡ Rust `link_place_stub`（`0ba3f4d2b9090a13`）  
- 抓 **linker 投毒**；不消 `trust_root=place_stub`  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*
