# 08 — 仓库地图（SoT）

## 顶层

| 路径 | 职责 |
|------|------|
| `docs/ARCH/` | **架构 + 自主驱动**（本目录） |
| `YOYO-LANG-PLAN.md` | 多版语言阶梯 |
| `YOYO-机构图.md` | 一页机构图 |
| `LANG-SURFACE.md` | 现行语言表面 |
| `ROADMAP-POST-1.0.md` | post-1.0 版本表 |
| `ROADMAP-TO-1.0.md` | 历史 1.0（只读） |
| `SCOPE-v*.md` / `RELEASE-v*.md` | 单版范围与发布 |
| `AUTO-*.md` | 无人值守开关 |
| `LANG*_OWNER_CHECKLIST.md` | 语言版勾选真相 |
| `POST-1.0-HOLE-CHECKLIST.md` | 关洞附录看板 |
| `AGENTS.md` | 经验 + 看板入口 |
| `.cursor/rules/` | 全局 prompt（alwaysApply） |
| `BACKEND_SUPPORT.md` | 链接后端矩阵 |
| `TRUST-SURFACE-v1.md` | 信任面结算 |

## 代码与产物

| 路径 | 职责 |
|------|------|
| `yoyo-rust/` | **主链接器 / verifier** |
| `yoyo-js/` | 历史 JS 链；payload bins 仍住在此树 |
| `yoyo-asm/` | asm peer |
| `yoyo/projects/` | 自举 `.ty` / `.tyb` 大源 |
| `scripts/` | 全部门禁与 place/emit |
| `scripts/fixtures/` | pins · anyprog · stub/compiler PE |
| `scripts/fixtures/anyprog/` | **语言样例 SoT** |

## Payload SoT

```text
yoyo-js/src/platform/yoyo-compiler-payload/
  code.bin
  data.bin
  payload.pin
  seeds/ …
```

## 不要当真相

- `scripts/_stage-*` 跑出来的缓存  
- 过期 agent 聊天记忆（以门禁与 SCOPE 为准）  
- `docs/auxdocs/` 海量历史 SPAWN 日志（考古用，非驱动）
