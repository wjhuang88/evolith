# 工程化路线图

本文档记录 Evolith 工程流程和文档治理的后续改进方向，不代表具体功能承诺。

## 当前已落地

- `AGENTS.md` 作为 agent 启动文档，负责硬约束、任务路由和会话结束检查。
- `docs/README.md` 作为文档地图，明确 reference / sop / roadmap / planned / archive 分层。
- `EVOLUTION.md` 作为故障速查和经验写回载体。
- `docs/sop/` 提供本地开发、新增功能、发布部署的标准流程。
- `docs/reference/PROJECT-MAP.md` 提供稳定代码地图和配置边界。
- `docs/reference/SCRIPTS-RELEASE-NOTES.md` 建立脚本行为变更记录制度。

## 近期重点

| 优先级 | 事项 | 说明 |
|--------|------|------|
| P0 | 修正 Compose 配置键 | `docker-compose.yml` 当前使用 `DATABASE_TYPE`，应与 `AppConfig` 的 `DATABASE__DATABASE_TYPE` 对齐 |
| P0 | 修正前端容器 API 前缀 | `NEXT_PUBLIC_API_URL` 应包含 `/api/v1`，除非网关统一重写 |
| P1 | 补齐未实现接口清单 | 将 API 合约中 501/TODO 汇总为可跟踪 backlog |
| P1 | 建立发布检查点 | 每次发布记录版本、验证命令、已知风险和回滚方式 |
| P2 | 增加 PostgreSQL CI 验证 | SQLite 已覆盖较多，生产主路径仍需更强自动化验证 |

## 文档治理原则

1. 状态盘点和阶段目标写 roadmap。
2. 可执行步骤写 SOP。
3. 稳定结构和边界写 reference。
4. 已讨论未实施写 planned。
5. 失败经验写 EVOLUTION。

## 后续可拆任务

- 创建 `docs/roadmap/API-GAP-CHECKPOINT.md`，汇总当前 501 和 TODO。
- 创建 `docs/sop/DATABASE-MIGRATION.md`，细化双数据库 migration 流程。
- 创建 `docs/reference/SECURITY-MODEL.md`，集中描述 cookie、CSRF、RBAC、CORS、CSP。
- 将生产发布流水线的环境变量和 secret 要求整理成 `docs/reference/CONFIG.md`。
