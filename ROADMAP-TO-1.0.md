# ROADMAP → YOYO 1.0

> 细节：`SCOPE-v1.0.md`。开关：`AUTO_TO_1.0.md`。  
> **AUTO**：Stage 全绿后 agent **自动「定」**下一版（建 SCOPE + 看板），不等用户。勾选真相 = 对应 `STAGE*_OWNER_CHECKLIST.md`。

| 版本 | Stage | 状态 | SCOPE | 看板 | 毕业 |
|------|-------|------|-------|------|------|
| v0.3 | 9 | ✅ | `SCOPE-v0.3.md` | `STAGE9_OWNER_CHECKLIST.md` | `RELEASE-v0.3.md` |
| v0.4 | 10 | ✅ | `SCOPE-v0.4.md` | `STAGE10_OWNER_CHECKLIST.md` | `RELEASE-v0.4.md` · tag `v0.4.0` |
| **v0.5** | **11** | ✅ | `SCOPE-v0.5.md` | `STAGE11_OWNER_CHECKLIST.md` | `RELEASE-v0.5.md` · tag `v0.5.0` |
| **v0.6** | **12** | ✅ | `SCOPE-v0.6.md` | `STAGE12_OWNER_CHECKLIST.md` | `RELEASE-v0.6.md` · tag `v0.6.0` |
| **v0.7** | **13** | ✅ | `SCOPE-v0.7.md` | `STAGE13_OWNER_CHECKLIST.md` | `RELEASE-v0.7.md` · tag `v0.7.0` |
| **v0.8** | **14** | 🔄 | `SCOPE-v0.8.md` | `STAGE14_OWNER_CHECKLIST.md` | `RELEASE-v0.8.md` |
| v0.9 | 15 | AUTO 将定 | `SCOPE-v0.9.md` | `STAGE15_OWNER_CHECKLIST.md` | `RELEASE-v0.9.md` |
| **v1.0** | **16** | AUTO 将定 | `SCOPE-v1.0.md` + 可选 GRAD | `STAGE16_OWNER_CHECKLIST.md` | `RELEASE-v1.0.md` + **tag** |

## 信任主题

| 版本 | 主题 |
|------|------|
| v0.4 | runtime 面 · Linux 纯 M4 · asm I/O · 毕业（✅） |
| v0.5 | YOYO-built/更薄 runtime · 收缩 LoadLibrary host（✅） |
| v0.6 | 三 peer I/O · selfhost body section-ddc（✅） |
| v0.7 | seed/link host · 跨平台 parity · Relock（✅） |
| v0.8 | 窗外字节/SCOPE-CUT 草案 · Lock 硬化 |
| v0.9 | 洞清单收口 · 预跑 |
| v1.0 | 全关或 SCOPE-CUT · RELEASE · tag |

## AUTO 行为（coordinator）

1. 读 `AUTO_TO_1.0.md` → `ACTIVE=1` 则无人值守执行  
2. 每 tick：下一未勾项 → 验收 → 绿才勾  
3. Stage 全绿 → 毕业 D：commit + tag + GitHub Release + push（仅毕业）→ auto「定」下一版 → 继续  
4. 停：`停` / `ACTIVE=0` / hard block  

OUT：Morph · MCU 主赛道 · C/Rust 替代 · Thompson-proof — 见 `SCOPE-v1.0.md`。

---

*更新：2026-08-29 · v0.7.0 毕业 · 现主线 Stage 14 / v0.8*
