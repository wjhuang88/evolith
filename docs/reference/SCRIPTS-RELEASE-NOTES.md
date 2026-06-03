# 脚本发布说明

> 本文件记录 `scripts/`、部署辅助脚本和跨平台运维入口的行为变更。
> 修改脚本参数、执行顺序、默认值、退出码或副作用时必须更新本文件。

## Unreleased

- 建立脚本发布说明制度。后续脚本行为变更需要记录用途、影响范围、验证方式和注意事项。
- **`scripts/dev.sh`**: 前端本地启动从旧 Next.js `localhost:3000` 调整为 Vite + Bun 默认 `localhost:3001`，使用 `bun install` / `bun run dev`，支持通过 `FRONTEND_PORT` 覆盖；状态输出和 ready banner 同步使用该端口。验证：脚本语法检查和前端构建。

## v0.2.0 — dev.sh 嵌入式前端死代码清理 + 单端口 lite 模式 (2026-06-03)

### 概述

Iteration 031 已将前端迁移到 `rust-embed-for-web`（debug 模式读 `frontend/dist/` 文件系统，release 模式编译时嵌入），但 `dev.sh` 仍残留旧 ZIP 方案的死代码，且 `lite` 模式无条件启动 Vite dev server 导致多余端口。本次清理死代码并将 `lite`/`embedded` 模式改为单端口启动。

### 变更

- **`scripts/dev.sh`**: 删除 `EMBEDDED_FRONTEND` 环境变量（line 10）——`rust-embed-for-web` 内置 debug/release 切换，无需外部控制。
- **`scripts/dev.sh`**: 删除 `build_frontend_zip()` 函数（原 lines 126-140）——后端使用 `#[folder]` 读文件系统，不读 ZIP。
- **`scripts/dev.sh`**: 删除 `start_backend()` 中 `--features embedded-frontend` 逻辑（原 lines 152-168）——`Cargo.toml` 未定义此 feature，调用会报 unknown feature。
- **`scripts/dev.sh`**: 新增 `build_frontend()` 函数——运行 `bun run build` 确保 `frontend/dist/` 存在。
- **`scripts/dev.sh`**: `lite` 模式改为 `build_frontend` + `start_backend`（去掉 `start_frontend`），单端口 8080 同时服务 API + 前端。
- **`scripts/dev.sh`**: `embedded` 模式简化为 `build_frontend` + `start_backend`，删除 `EMBEDDED_FRONTEND=true` 和 ZIP 构建。
- **`scripts/dev.sh`**: `start` 模式保留双端口（Vite HMR 有独立开发价值），新增 `build_frontend` 确保 dist 存在。
- **`scripts/dev.sh`**: 后端端口从硬编码 8080 改为读 `SERVER__PORT` 环境变量（默认 8080），通过 `BACKEND_PORT` 变量贯穿全脚本。
- **`scripts/dev.sh`**: `usage` 文本更新，明确三种模式的端口差异。

### 验证

- `bash -n scripts/dev.sh` ✓ 语法检查通过
- `SERVER__PORT=8090 ./scripts/dev.sh lite` ✓ 单端口启动成功
- `curl http://localhost:8090/` ✓ index.html 200
- `curl http://localhost:8090/assets/*.css` ✓ CSS 200
- `curl http://localhost:8090/assets/*.js` ✓ JS 200
- `curl http://localhost:8090/health/live` ✓ API 健康检查 200
- `lsof -i :3001` ✓ 无 Vite dev server 进程（正确）
- `./scripts/dev.sh stop` ✓ 正常停止

### 注意事项

- `start` 模式仍启动 Vite dev server（端口 3001），用于 HMR 热更新开发体验。
- `lite` 和 `embedded` 模式现在行为相同（单端口），保留两个命令名以兼容习惯用法。
- 后端端口可通过 `SERVER__PORT=8090 ./scripts/dev.sh lite` 覆盖，解决 8080 被占用的场景。
- `frontend/dist/` 必须在后端启动前存在——`build_frontend()` 已自动处理。

## 记录模板

```markdown
## vX.Y.Z — <标题> (<YYYY-MM-DD>)

### 概述

<这次脚本变更解决什么问题。>

### 变更

- **`scripts/<name>.sh`**: <行为变化>

### 验证

- <执行过的验证命令或手工检查>

### 注意事项

- <兼容性、回滚或使用方式变化>
```
