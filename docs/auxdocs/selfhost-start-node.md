# W-START NODE — expanded checklist（NON-NORMATIVE · EXPERIMENTAL）

> Status: **HOLD** · 未开火 · ≠ freeze · ≠ 自举 GREEN  
> Label: `EXPERIMENTAL · NON-GREEN · Rust-first · OUT-OF-v0.1-body（SCOPE-CUT 边界外点火）`  
> Hub: `PROMPT-v3.md` 文首 Week 轴 · Amendment **W-START**

## Scope
- `attempt ≠ freeze ≠ full self-host`
- 开火不把仍红翻绿；失败不 Relock / 不假 pin
- full body 仍在 W5.5 **SCOPE-CUT**；Freeze = Part 5 end gate（需 full `yoyo.ty` + 3-chain + Lock）

## Checklist（7）
1. **Cold-start re-verify**：文首 numbers + pin 与当前树一致
2. **Lock/Relock**：pin 一致；无 LOCKED 不谈 freeze
3. **Scope label**：`attempt ≠ freeze ≠ full self-host`；仍红不因开火翻绿
4. **D-1 / platform divergence**：显式风险；fail-closed
5. **Failure**：不 Relock；不假 pin
6. **Stub/RAW_BYTE**：不宣称 C-ddc / Morph / freeze / gen1≡gen2
7. **Artifacts**：仅标 `EXPERIMENTAL`；不自动仍红→绿

## Success split
| 口径 | 含义 |
|------|------|
| 「尝试已开始」 | 可复现 Rust 入口（cmd+log+scope tag）+ checklist；**可全红** |
| 「自举 GREEN」 | 仍红项全过（full body · 3-chain `section-ddc` · gen1≡gen2 · Freeze+Lock）；**START NODE 不豁免** |

## Do not
- 宣称已自举 / 已开火（本文件只是 HOLD 节点）
- Relock / 改 `yoyo.ty` / locks / goldens / peers（开火拍另议）
