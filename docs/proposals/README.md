# 远期提案

本目录记录已经提出但尚未进入实施的工程或产品提案。准备排期时，应把相关内容迁移到 `docs/backlog/` 并补齐验收标准；如需正式立项，可拆分到：

- `docs/roadmap/`：阶段目标和里程碑。
- `docs/sop/`：具体操作流程。
- `docs/reference/`：稳定事实和配置边界。

## 当前提案

| 事项 | 状态 | 下一步 |
|------|------|--------|
| API gap checkpoint | 待整理 | 汇总 API 合约中的 501、TODO 和前端占位动作 |
| Database migration SOP | 待整理 | 固化 SQLite/PostgreSQL 双轨 migration 流程 |
| Security model reference | 待整理 | 集中说明 JWT cookie、CSRF、RBAC、CORS 和安全头 |
| [Evolith Rust CLI](RUST-CLI.md) | 远期目标 | API 合约稳定后启动 CLI 子项目 |
| [前端嵌入后端](EMBEDDED-FRONTEND.md) | 远期目标 | 完成 React + Vite + Bun 迁移后实施 |
| [Artifact Repository](ARTIFACT-REPOSITORY.md) | 远期目标 | snippet/skill 稳定后再定义制品元数据和存储模型 |
| [AI Gateway](AI-GATEWAY.md) | 远期想法 | 核心平台稳定后展开技术方案评审 |
