# 10 — 定版与毕业

## 「定」下一版（有限 · ≤5 门）

触发：用户「定 1.N …」或 DRIVER 在 ACTIVE 轨上版本全绿后 auto-定（须有 PLAN 行）。

### 必须创建/更新

1. `SCOPE-vX.Y.md` — IN/OUT/SCOPE-CUT/验收命令  
2. `LANG{k}_OWNER_CHECKLIST.md` 或续用约定看板 — A→… 勾选  
3. `ROADMAP-POST-1.0.md` 状态行  
4. `YOYO-LANG-PLAN.md` 阶梯状态  
5. `AUTO-LANG.md`（或 `AUTO-LANG{n}.md`）`ACTIVE=1` `COMPLETED=0`  
6. Glass 打开新看板  
7. **立刻**执行第一未勾项（AUTO 下不问「继续」）

### 禁止

- 无主题空版  
- 一门里塞 >5 个无关大项  
- 未门禁声称毕业  

## 毕业协议

当看板全 `[x]`（含 RELEASE 门）：

1. 写/更新 `RELEASE-vX.Y.md`  
2. `git commit`  
3. `git tag vX.Y.0`（或约定标签）  
4. `git push` commit + tag  
5. `gh release create`（网络失败 → 记入阻塞并重试；tag 仍有效）  
6. AUTO `COMPLETED=1` `ACTIVE=0`  
7. 按 PLAN stub 下一版草案（`STATUS=DRAFT`）——**不**自动 ACTIVE，除非用户/协议要求  

## v1.1 已毕业参考

- SCOPE/RELEASE/tag：`v1.1.0`  
- Winsock：SCOPE-CUT  
- 门禁：`stage-lang1` + `stage-lesshostx`  

## v1.2 激活检查表（模板）

- [ ] 主题一句话（源语可读性 **或** 明确 CUT）  
- [ ] ≤5 门写进 SCOPE  
- [ ] 新/改 stage 命令  
- [ ] 回归：lang1 + lesshostx  
- [ ] AUTO ACTIVE=1  
