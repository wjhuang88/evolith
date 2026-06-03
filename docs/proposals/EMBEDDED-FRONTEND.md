# 规划: 前端静态产物嵌入后端

> 状态：已晋升（Iteration 031 已用 `rust-embed-for-web` 实现等价目标）
> 前置：完成 `React + Vite + Bun` 静态 SPA 迁移后再实施。
>
> **改线说明**（2026-06-03 / EVO-060）：本提案中的 `--features embedded-frontend` feature flag 方案和 ZIP 打包方案均已被 Iteration 031 的 `rust-embed-for-web` `#[folder]` 方案取代。`Cargo.toml` 中未定义 `embedded-frontend` feature，`dev.sh` 中的 ZIP 构建和 feature flag 死代码已清理。下文保留为历史参考。详见 [ADR-0003](../decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md) / [Iteration 031](../iterations/ITERATION-031.md)。

## 1. 目标

参考 OpenCode 部署配置项目中 `oc-platform` 的处理方式，将前端构建产物打包进 Rust 后端发布物中，形成单二进制/单容器交付能力。

同时保留开发期前后端独立运行：

```text
开发模式：Vite dev server -> 调用 backend API
生产模式：backend 同时服务 API 和 embedded static frontend
```

## 2. 为什么需要

当前生产栈是 backend + frontend + nginx 多服务形态。对私有化部署和轻量试用场景，可以提供更简单的交付方式：

- 单 backend 容器即可提供 Web UI 和 API。
- 降低部署复杂度。
- 避免前端运行时 Node.js 依赖。
- 静态资源和后端版本天然一致。

## 3. 目标架构

```text
frontend/
  bun run build -> dist/

backend/
  build.rs 或 xtask 复制 dist/
  Rust binary embeds static files

runtime:
  GET /api/v1/*   -> API routes
  GET /mcp        -> MCP routes
  GET /assets/*   -> embedded static files
  GET /*          -> index.html SPA fallback
```

## 4. 开发模式

开发期保持独立运行：

```bash
# backend
cd backend
cargo run

# frontend
cd frontend
bun run dev
```

前端通过运行时配置或 dev env 指向后端：

```text
VITE_API_URL=http://localhost:8080/api/v1
```

后端开发模式不强制要求 embedded frontend 存在；找不到构建产物时只提供 API。

## 5. 生产模式

生产构建流程：

```bash
cd frontend
bun install --frozen-lockfile
bun run build

cd ../backend
cargo build --release --features embedded-frontend
```

Dockerfile 可采用：

1. frontend builder stage：生成 `dist/`
2. backend builder stage：编译 Rust binary 并嵌入/复制 `dist/`
3. runtime stage：只保留 Rust binary

## 6. 实现选项

### 选项 A：include_dir 编译期嵌入

优点：

- 单二进制，部署最简单。
- 静态资源和后端版本严格一致。

缺点：

- 每次前端变化都需要重新编译后端。
- binary 变大。

### 选项 B：后端容器内文件服务

优点：

- 后端无需重新编译即可替换静态资源。
- 实现更简单。

缺点：

- 不是严格单二进制。
- 需要管理静态目录路径。

### 推荐

第一阶段采用 **选项 B**，降低风险；稳定后再评估是否启用 `include_dir` 单二进制。

## 7. 路由规则

必须保证 API 优先：

1. `/api/v1/*` 交给 API routes。
2. `/health*` 交给 health routes。
3. `/mcp*` 交给 MCP routes。
4. `/assets/*` 返回静态资源。
5. 其他 GET 返回 `index.html`。
6. 其他非 GET 不做 SPA fallback。

## 8. 验收标准

- 开发期前端 dev server 可独立运行。
- 生产后端可直接访问 `/` 打开 Web UI。
- 刷新深层路由如 `/tools/<id>` 不 404。
- `/api/v1/*`、`/health/*`、`/mcp` 不被 SPA fallback 截获。
- Docker 生产镜像无需 Node.js runtime。

## 9. 分阶段计划

### Embedded Phase 0 — 设计确认

- 完成 Vite SPA 迁移。
- 确认运行时配置方式。
- 确认 Docker 发布模式。

### Embedded Phase 1 — 文件服务

- 后端支持从 `FRONTEND_DIST_DIR` 服务静态文件。
- Docker 构建把 `frontend/dist` 复制到后端镜像。
- Nginx 可选，不再是必需组件。

### Embedded Phase 2 — 单二进制实验

- 增加 `embedded-frontend` feature。
- 使用 `include_dir` 或等价方案嵌入前端产物。
- 比较 binary size、构建时间和部署收益。

## 10. 注意事项

- 不要影响本地开发 dev server。
- 不要让 SPA fallback 截获 API 404。
- 前端 API base 在 embedded 模式下应默认使用同源 `/api/v1`。
- 若使用运行时 `/config.js`，embedded 模式需要由后端动态生成。
