# AUTO_TO_1.0 — unsupervised flag

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-08-29T02:16:00+08:00
SET_AT=2026-08-28T18:54:00+08:00
SET_BY=user
REASON=离电脑；无人值守推到1.0（或 hard block）
NORTH_STAR=打破后门魔咒
STOP_ONLY=停 | hard-blocker needing human | ACTIVE=0
NOTE=v1.0 / Stage 16 graduated — ROADMAP endpoint; no Stage 17; future ticks must NOT start new feature work
```

## Agent 必读（每次 wake / loop tick **最先**检查本文件）

1. 若 `ACTIVE=0` 且 `COMPLETED=1` → **停手**；v1.0 已毕业；**禁止** invent Stage 17 / 新功能轨
2. 若 `ACTIVE=1` → 进入 **AUTO-TO-1.0**（见 `.cursor/rules/stage4-auto-owner.mdc`）
3. **立刻**执行当前 Stage 下一未勾项；**不问**用户、**不**教用户打字
4. Stage 全绿 → 自动「定」下一版（建 SCOPE + STAGE 看板）→ 继续第一项
5. Stage D 毕业绿 → auto commit + tag + GitHub Release（同 v0.1–v0.3）；**允许** push 毕业产物
6. 未完成 WIP：**不** push
7. 仅当用户写 `停` 或把本文件改为 `ACTIVE=0`，或遇到需人类的 hard block → 停

详规：`SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · `RELEASE-v1.0.md`
