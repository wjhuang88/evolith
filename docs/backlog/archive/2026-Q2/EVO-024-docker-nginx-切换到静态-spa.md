# EVO-024 Docker / Nginx 切换到静态 SPA

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P0
- Source: EVO-002 split
- Decision Context: Iteration 004；过渡部署形态，Dockerfile 改为 Bun + Vite build + Nginx 静态服务，SPA fallback

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：让当时生产部署形态匹配 Vite 静态 SPA，避免继续依赖 Next standalone runtime。该 Nginx 托管静态资源方案是 EVO-016 前的过渡策略；当前已由 EVO-016-B / Iteration 031 的 `rust-embed-for-web` 嵌入方案替代。
- 范围：
  - 更新前端 Dockerfile 或生产镜像构建流程，使用 Vite `dist/` 静态产物。
  - 更新 Nginx 配置，支持 SPA history fallback、静态资源缓存和 `/api/v1` 反向代理。
  - 更新 `docker-compose.prod.yml` 中与前端构建产物、服务启动命令、挂载路径相关的配置。
  - 保留或补充运行时配置 `/config.js` 的部署方式。
- 不做：
  - 不创建或恢复 `.github/workflows/*.yml`；归属 EVO-030。
  - 不删除 Next.js 依赖、App Router 或 middleware；归属 EVO-025。
  - 不改变后端 API 合约。
- 验收标准：
  - [x] 生产前端镜像不再依赖 Next standalone server。
  - [x] Nginx 能托管 `dist/` 并对 SPA 路由返回入口 HTML。
  - [x] `/api/v1` 请求仍代理到后端，且前端 API base URL 保持 `/api/v1` 约束。
  - [x] Docker production stack 可启动到前端静态页面和后端 health endpoint。
  - [x] 文档说明 GitHub CI/CD 已拆到 EVO-030，避免误以为 EVO-024 包含 workflow。
- 依赖或阻塞：EVO-022、EVO-023。
- 影响范围：frontend / deploy / docs
- 最小验证方式：`bun run build`；本地或容器内验证 Nginx 静态托管和 `/api/v1` 代理；`git diff --check`。
