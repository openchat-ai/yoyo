# 00 — 自主驱动协议（Autonomous Driver）

> Agent **唯一操作手册**。无用户口头部落知识也可推进。  
> 用户说 `停` / 「只讨论」→ 立即停改代码。

---

## 0. 身份与北星

| 项 | 值 |
|----|-----|
| 产品 | YOYO：**独立语言** + 自举地基 |
| 主尺 | 人能否按表面写程序、编出来、跑起来、门禁绿 |
| 附录 | DDC/Lock/缩宿主（回归，不偷换主尺） |
| 禁区 | Thompson-proof 宣称 · 空盘创世 · 无界 tip 工厂 · 回改已毕业勾选 |

---

## 1. 每次 wake 状态机

```text
READ AUTO-LANG.md / ACTIVE 语言开关
  │
  ├─ ACTIVE=1 → 打开当前 SCOPE + LANG*_OWNER_CHECKLIST
  │              → 下一未勾项 → §3 执行环 → 绿才勾
  │
  ├─ 用户「定 1.N …」→ 按 10-VERSIONING 建 SCOPE/看板/ACTIVE=1 → 立刻做第一项
  │
  ├─ 用户「关洞」→ POST-1.0-HOLE-CHECKLIST（附录轨，不改语言主尺）
  │
  └─ 皆停且无触发 → 可读 ARCH/答问；不擅自开大刀
```

**Glass**：打开当前看板（`LANG*_OWNER_CHECKLIST` 或用户点名的 SCOPE）。

---

## 2. 单一真相源（冲突决议）

优先级高 → 低：

1. **机器门禁绿/红**（`stage-*.ps1` / `cargo` 实测）  
2. **当前 SCOPE-vX.Y.md** 的 IN/OUT / SCOPE-CUT  
3. **本 ARCH 目录**（尤其本 DRIVER）  
4. `YOYO-LANG-PLAN.md` / `YOYO-机构图.md`  
5. `LANG-SURFACE.md`  
6. 历史 STAGE*/AUTO-* 叙事（已 COMPLETED 勿回改勾选）  
7. 聊天口头（仅当用户明确改口）

代码与文档冲突 → **先修到门禁绿，再改文档对齐**（除非 SCOPE 宣布 CUT）。

---

## 3. 执行环（每一项勾选）

```text
1. 读看板未勾项的验收命令与 SCOPE 约束
2. 最小实现（见 09-DEV-LOOP）
3. 本地跑门禁；红则修到绿（禁止「文档先假装绿」）
4. 更新看板勾选 + 必要 pin/SURFACE
5. 若本版全绿 → 10-VERSIONING 毕业协议
6. 加深刀 COMPLETED=1 → 同 tick git commit（一刀一落；默认不 push）
7. 毕业 RELEASE → commit + tag + push + gh release
```

PowerShell：用 `&`，忌 `Start-Process | Out-Null` 丢 ExitCode。

---

## 4. 工作类型路由

| 目标 | 读 | 门禁 |
|------|-----|------|
| 写/改应用语言能力 | 03-LANGUAGE, LANG-SURFACE, anyprog | `stage-lang1` / `stage-anyprog` |
| 改链接/后端 | 04-TOOLCHAINS, BACKEND_SUPPORT | `cargo run -- test ddc` + 相关 stage |
| 自举/cold | 05-SELFHOST | `stage-lesshostx` / `stage-break3` |
| 检测/pin | 06-TRUST | 对应 poison/pin stage |
| 新版本 | 10-VERSIONING | 本版全部 stage |

---

## 5. Hard block（必须停并写入看板「阻塞」）

- 需人类密钥 / 跨信任域封印（#8 真消）  
- 空盘 / 消除全部 Win32 A1（政策禁止）  
- 外部网络/账号不可替代且失败  
- 用户政策 SCOPE-CUT 禁止继续的方向  

停时：**不假装 COMPLETED**；写清需要人类做什么。

---

## 6. 禁止清单（驱动红线）

- 发明无 SCOPE 的 Stage17 式无限清单  
- 用「检测更炫」替代「语言能写」主尺  
- 门禁红仍勾选  
- 回改 `STAGE16` / 已 COMPLETED AUTO 勾选历史  
- 未授权 push 风暴 / 用 CI 当 debugger（见 `ci-anti-thrash`）  

---

## 7. 最小「我醒了」检查表

```powershell
# 1) 开关
Get-Content F:\yoyo\AUTO-LANG.md -TotalCount 20
# 2) 当前语言门禁（若在语言轨）
& F:\yoyo\scripts\stage-lang1.ps1
# 3) 自举回归（改动自举面时）
& F:\yoyo\scripts\stage-lesshostx.ps1
```

---

*ARCH SoT · 驱动协议*
