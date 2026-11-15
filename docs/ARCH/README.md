# YOYO Architecture — 文档驱动开发入口

> **目的**：使 agent / 人类仅凭本目录即可驱动 **YOYO 独立语言** 全流程开发（定版→实现→门禁→毕业），无需口头部落知识。  
> **北星**：独立语言（`YOYO-LANG-PLAN.md`）。v1.0 自举/信任 = 地基与回归。  
> **权威序**：冲突时以本目录 `00-DRIVER` 决议规则为准；实现代码与门禁绿为最终真相。

## 必读顺序（每次 wake）

1. [`00-DRIVER.md`](00-DRIVER.md) — **自主驱动协议**（怎么选活、怎么停、怎么毕业）  
2. [`01-VISION.md`](01-VISION.md) — 愿景 / 非目标  
3. [`02-SYSTEM.md`](02-SYSTEM.md) — 系统分层  
4. 按任务再读 03–10  

总览图：仓库根 `YOYO-机构图.md` · 版本阶梯：`YOYO-LANG-PLAN.md`

## 文档地图

| 文件 | 内容 |
|------|------|
| `00-DRIVER.md` | 状态机、工作流、验收、禁止项 |
| `01-VISION.md` | 为何存在、成功标准 |
| `02-SYSTEM.md` | 分层架构、数据流 |
| `03-LANGUAGE.md` | 语言表面、`.ty`、应用链路 |
| `04-TOOLCHAINS.md` | yoyo-rust / C1 / JS·asm 残差 |
| `05-SELFHOST.md` | 自举冷启、place、seed |
| `06-TRUST.md` | DDC/Lock/pin、硬墙 |
| `07-GATES.md` | 门禁矩阵与命令 |
| `08-REPO-MAP.md` | 目录与 SoT 文件 |
| `09-DEV-LOOP.md` | 端到端开发循环（可执行剧本） |
| `10-VERSIONING.md` | 如何「定」下一版并毕业 |

## 一句话

**实现跟着 DRIVER 走；DRIVER 跟着 LANG-PLAN / SCOPE 走；绿门禁才算数。**
