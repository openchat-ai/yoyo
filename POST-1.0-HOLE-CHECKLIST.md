# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.626**（625=STB any-slot；下一刀=MCP any-slot / yoyo.tyb / M4）。

| 阻塞 | 说明 |
| --- | --- |
| **长杆** | OW-RT → S3（去 Rust sidecar） |
| **勿做** | fake CLOSED · `AUTO ACTIVE=1` · invent Stage · 定点/oracle 当 CLOSED · CI 当调试器 |

Disposition / 洞表 → `SCOPE-CUT-v1.0-hole-inventory.md`。门禁明细 → `git log`。

---

## S3 主线

| | |
| --- | --- |
| **S1** ✅ | in-DLL mini codegen |
| **S2** ✅ | slim 真 I/O · StubPlatform **仍 stub** |
| **S3** ⏳ | child parse1 扩 ISA · 真自举不排假工期 |
| **S4/S5** | 全 ISA / 形式化 · 地图 |

**诚实底线**：fixed10 / parse1 可 GREEN ≠ 全 mini-ISA ≠ OW-* CLOSED ≠ StubPlatform 变真 OS。明细 → `git log --grep=S3.`。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–521** ✅ | `5dd7651` | fixed10→host compose→DATA；禁折 FIXED10 |
| **S3.522–555** ✅ | `8ea1ee5` | CreateProcess child PE / M4-prep；yoyo.tyb ABSENT |
| **S3.556–582** ✅ | `7fb0d14` | live1→live2→emit1；≠ general emit_x86 |
| **S3.583–593** ✅ | `1ab26a8` | emit2 field（imm/disp）· all CHAINS |
| **S3.594–605** ✅ | `50f3c56` | parse1 记录环：fixed10→beyond + any-size |
| **S3.606–608** ✅ | `c371eef` | JMP cliff→+JMP/+CALL label table |
| **S3.609–617** ✅ | `7cb45c2` | parse1 mini 主路径：JCC→…→DATA |
| **S3.618** ✅ | `7a5f435` | parse1≈mini cliff；yoyo.tyb 仍 ABSENT |
| **S3.619–622** ✅ | `f2740ce` | SET/GET/INC/DEC/ADD/SUB slot≤255 |
| **S3.623** ✅ | `267c218` | CMP/IMUL/ALUV slot≤255 |
| **S3.624** ✅ | `9ccfea4` | LDB slot≤255 (disp8\|disp32 ±oo) |
| **S3.625** ✅ | — | STB slot≤255 (disp8\|disp32 ±oo) |
| **S3.626** ⏳ | — | **本步**：MCP any-slot / yoyo.tyb / M4 |

**缺口**：MCP slot>15·disp32 / yoyo.tyb / M4 flip / general emit_x86 仍 **ABSENT**。`ready=false`。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_compiler_pe_parse1_ --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.626**（MCP any-slot；禁 densify / 假 CLOSED）。S3.625=STB any-slot。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.626*
