# Iteration 012: Embedded Frontend 文件服务与单容器交付

> 状态：Superseded by Iteration 031
> 计划目标：在交付边界确定后，实现后端提供 Vite 静态产物与单容器生产交付的第一阶段能力。
> 原阻塞原因：原前置 `Iteration 011` 未执行 EVO-016 refinement，而被改线用于
> EVO-010/EVO-011；必须先用新的 iteration 编号完成或确认 `EVO-016-A`。
> Superseded 原因：EVO-016-B 已由 Iteration 031 使用 rust-embed-for-web 方案完成。

## 1. 计划边界

本轮基于 `EVO-016` 的候选第二个子 Story 编排。只有 Iteration 011 完成设计确认且下述 Story
已进入 backlog、通过 DoR 后，才能启动本轮。

## 2. 候选拆分与选入 Story

| 候选 ID | 标题 | 父 Epic | 当前状态 | 依赖/顺序 |
|---------|------|---------|----------|-----------|
| EVO-016-B | 后端静态文件服务与单容器交付基线 | EVO-016 | 待建项 | EVO-016-A 完成 |

> `EVO-016-B` 是规划编号，尚未进入 backlog；当前文档不构成开工授权。

## 3. 目标范围

- 让生产后端能够服务构建后的前端静态资源与 SPA fallback。
- 保证 `/api/v1`、`/health`、`/mcp` 不被前端 fallback 截获。
- 调整生产镜像，使 Node.js 不作为运行时依赖，Nginx 不再是前端资源托管必需组件。
- 验证 embedded 模式下同源 `/api/v1` 与深层路由刷新。

## 4. 不做事项

- 不实现可选的单二进制 `include_dir` 实验；可作为 `EVO-016-C` 后续候选。
- 不在构建/部署命令未稳定前实施 GitHub CI/CD（EVO-030）。
- 不顺带处理无关 Skill 生命周期或管理后台功能。

## 5. 计划验收标准

- [ ] 生产后端可直接从 `/` 提供 Web UI，深层 SPA 路由刷新不返回 404。
- [ ] API、health 与 MCP 路径具有优先路由且不被 SPA fallback 覆盖。
- [ ] 生产交付至少支持单 backend 容器承载 API 与前端静态资源。
- [ ] 当前 Nginx 的角色更新为可选网关/SSL/反代，而非静态托管硬依赖。
- [ ] 构建命令稳定后重新评估 EVO-030 的排期。

## 6. 计划验证

```bash
bun run build
cargo test --workspace
cargo clippy --workspace -- -D warnings
# 按最终 Docker 方案验证容器内 `/`、深层路由、`/assets/`、`/api/v1`、`/health` 与 `/mcp`。
```

## 7. 风险与进入条件

| 风险 | 计划控制 |
|------|----------|
| SPA fallback 截获 API 或 health 路由 | 明确 route ordering 并加入集成验证 |
| 静态资源路径在构建镜像内错位 | 验证 `/assets/` 实际响应和缓存行为 |
| 过早固定 CI 命令 | 完成本轮后再决定 EVO-030 是否 Ready |

## 8. 计划记录

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Future iteration planned only. 规划候选子 Story `EVO-016-B`，依赖 `EVO-016-A`；未变更 backlog。 |
| 2026-05-27 | Activation blocked. 原 Iteration 011 计划被其他目标占用，等待以新 iteration 编号重新完成 EVO-016 refinement。 |
| 2026-05-27 | Planning disposition. 后续 Phase E 排期与本 embedded frontend 计划无依赖关系；本迭代继续 `Blocked for activation`，不得因另行规划而视作已解除 EVO-016 前置条件。 |
| 2026-05-29 | Disposition update. EVO-016-B 已由 Iteration 031 完成（rust-embed-for-web）。阻塞已解除，但本迭代计划目标已被 Iteration 031 覆盖。标记为 `Superseded by Iteration 031`。 |
