# AUTO_TO_1.0 — unsupervised flag

```
ACTIVE=1
SET_AT=2026-08-28T18:54:00+08:00
SET_BY=user
REASON=离电脑；无人值守推到1.0（或 hard block）
NORTH_STAR=打破后门魔咒
STOP_ONLY=停 | hard-blocker needing human | ACTIVE=0
```

## Agent 必读（每次 wake / loop tick **最先**检查本文件）

1. 若 `ACTIVE=1` → 进入 **AUTO-TO-1.0**（见 `.cursor/rules/stage4-auto-owner.mdc`）
2. **立刻**执行当前 Stage 下一未勾项；**不问**用户、**不**教用户打字
3. Stage 全绿 → 自动「定」下一版（建 SCOPE + STAGE 看板）→ 继续第一项
4. Stage D 毕业绿 → auto commit + tag + GitHub Release（同 v0.1–v0.3）；**允许** push 毕业产物
5. 未完成 WIP：**不** push
6. 仅当用户写 `停` 或把本文件改为 `ACTIVE=0`，或遇到需人类的 hard block → 停

详规：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md`
