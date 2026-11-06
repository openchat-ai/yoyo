# AUTO_TO_1.0 — unsupervised flag

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-08-29T02:16:00+08:00
SET_AT=2026-08-28T18:54:00+08:00
SET_BY=user
REASON=离电脑；无人值守推到1.0（或 hard block）
NORTH_STAR=打破后门魔咒 / 自举
STOP_ONLY=停 | hard-blocker needing human | ACTIVE=0
NOTE=v1.0 / Stage 16 graduated — ROADMAP endpoint; no Stage 17; path2 frozen to north star (self-host + backdoor detection); banlist=claim discipline not progress ruler
NEXT_AUTO=AUTO-FIXEDPOINT.md (三链终极 + 非 Rust 唯一发射；默认 ACTIVE=0)
```

## Agent 必读（每次 wake / loop tick **最先**检查本文件）

1. **尺子** = **打破后门魔咒 / 自举**。OW-\* / banlist = 附录与对外话术纪律，**不得**偷换成进度定义或「永远搞不完」的答案。
2. 若 `ACTIVE=0` 且 `COMPLETED=1` → **本轨停手**；v1.0 已毕业；**禁止** invent Stage 17 / 无界 tip 工厂
3. **接着读 `AUTO-FIXEDPOINT.md`**：若其 `ACTIVE=1` → 按该文件 FP 状态机推进（有限门禁；非新 Stage）
4. 若本文件 `ACTIVE=1` → 进入历史 **AUTO-TO-1.0**（见 `.cursor/rules/stage4-auto-owner.mdc`）；v1.0 已完成后应保持 `ACTIVE=0`
5. 用户 `继续`/`关洞`（post-v1.0）：读 `POST-1.0-HOLE-CHECKLIST.md`；加深发射/信任根 → `AUTO-FIXEDPOINT.md`
6. 用户 `固定点自动` / `继续固定点` → 只走 `AUTO-FIXEDPOINT.md`
7. Stage D 毕业绿（历史）→ auto commit + tag + GitHub Release；**FIXEDPOINT 轨**仅 FP-DONE 且用户/规则允许才 commit
8. 未完成 WIP：**不** push
9. 仅当用户写 `停` 或两轨皆 `ACTIVE=0`，或 hard block → 停

详规：`AUTO-FIXEDPOINT.md` · `SCOPE-v1.0.md` · `ROADMAP-TO-1.0.md` · `RELEASE-v1.0.md` · `POST-1.0-HOLE-CHECKLIST.md`
