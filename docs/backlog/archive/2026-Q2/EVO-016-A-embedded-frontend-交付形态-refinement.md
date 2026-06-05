# EVO-016-A Embedded Frontend 交付形态 refinement

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Deferred
- Priority: P2
- Source: EVO-016 split / Iteration 024
- Decision Context: 已被 Iteration 031 的 rust-embed-for-web 实施覆盖，不再单独激活

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Deferred
- 父 Epic：EVO-016
- Story 形态：Technical
- 用户故事或技术目标：
  - 作为/为了：维护者需要明确前端嵌入后端发布物的交付边界。
  - 我希望/需要：形成后端静态服务、SPA fallback、Nginx 角色、Docker 构建和验证方案。
  - 以便：Iteration 012 的 `EVO-016-B` 能按清晰方案实施，不再依赖过渡 Nginx 静态托管。
- 范围：
  - 明确前端静态产物由后端服务的目录、路由优先级和 SPA fallback 行为。
  - 明确 `/api/v1`、`/health`、`/mcp` 与 `/assets/` 的优先级和验证矩阵。
  - 明确生产 Docker 形态：是否单 backend 容器承载 API + 静态资源，Nginx 是否仅保留为可选网关。
  - 明确 Bun/Vite 构建产物如何进入后端发布物或镜像。
  - 输出 `EVO-016-B` 的实施边界、风险和验证命令。
- 不做：
  - 不在 refinement 中改业务代码或切换生产部署。
  - 不实现单二进制 `include_dir` 实验；可作为后续候选。
  - 不重建 GitHub CI/CD；归 EVO-030。
- 验收标准：
  - [ ] 写清 embedded frontend 的目标部署形态与当前 Nginx 过渡策略差异。
  - [ ] 写清路由优先级、SPA fallback 和静态资源缓存/路径策略。
  - [ ] 写清 Docker/build 输入输出和本地/容器验证矩阵。
  - [ ] 更新 Iteration 012 的激活条件或确认仍阻塞。
  - [ ] 将实施切片 `EVO-016-B` 的范围、依赖和验证方式补齐。
- 依赖或阻塞：已被 EVO-016-B / Iteration 031 的 rust-embed-for-web 实施覆盖；不再单独激活。
- 影响范围：docs / deploy / backend / frontend
- 最小验证方式：Markdown 链接检查；`git diff --check`；必要时只读检查当前 Docker/Nginx/backend route 配置。
