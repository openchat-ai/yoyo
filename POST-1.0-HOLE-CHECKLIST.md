# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 一眼进度

| | |
| --- | --- |
| **tip** | **S3.725** ⏳ **人审落地 E1–E4** → G CLOSED；OW-IAT/SEED 在后 |
| **主线** | S1✅ S2✅ **S3⏳** · S4/S5 未开 |
| **已站住** | E1–E4 · OW-IAT cliff · **OW-SEED cliff** |
| **硬闸** | M4 / G CLOSED **ABSENT**（inventory CUT）· `ready=false` |
| **禁** | silent flip / densify / 假 CLOSED / 抢 IAT/SEED |
| **近刀** | 705–724 ✅ |
| **git** | S724 → tip **S3.725** · **未 push** · bake=362 |

继续 = 人审落地 E1–E4。

## 下一刀

| tip | 做什么 | 禁 |
| --- | --- | --- |
| **S3.725 人审** | 落地 E1→E2→E3→E4；G CLOSED 后才 OW-IAT/SEED | 静默 flip · 假 CLOSED |

**缺口**：G CLOSED ABSENT。

## 验收

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_ow_seed_followon_cliff_pin yoyo_ow_iat_followon_cliff_pin --test-threads=1
```

---

*post-v1.0 · tip S3.725*
