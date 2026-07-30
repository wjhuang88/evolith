# EVO-118-E Embedded Frontend 生产构建与部署收敛

- **类型**：Technical / Deploy / Release
- **状态**：Ready
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A merge
- **影响范围**：Dockerfile / Compose / frontend / backend / gateway / docs / tests

## 工程目标

统一 Evolith 的生产交付形态，确保从干净 checkout 能构建包含正确前端静态资源的后端镜像，并让 Nginx 只承担可选 Gateway/SSL/反代职责。

## 已确认失败模式

- Rust 编译期要求 `../frontend/dist/`，生产 Backend Docker build context 却只包含 `./backend`。
- Backend Dockerfile 不构建也不复制 Frontend dist。
- Compose 同时部署 Embedded Frontend 后端和独立 Frontend Nginx，形成双事实交付口径。
- 生产配置包含尚未被代码消费或与当前结构冲突的环境变量。

## 验收场景

### Scenario 1：干净环境构建成功

- **Given** 新 clone 且无本地 `frontend/dist`
- **When** 运行 production build
- **Then** Frontend 先构建、Backend 成功嵌入产物，镜像无需依赖宿主残留文件

### Scenario 2：单一静态资源入口

- **Given** 生产栈启动
- **When** 访问首屏、深层 SPA 路由和 hashed asset
- **Then** 由同一事实交付路径返回正确内容、Content-Type 和 Cache-Control

### Scenario 3：协议路径互不截获

- **Given** Gateway + Backend 已启动
- **When** 访问 `/api/v1`、`/repos/`、`/mcp`、`/health`、`/assets/` 和 SPA fallback
- **Then** 每条路径命中正确处理器，Git Smart HTTP 不被 SPA/Nginx rewrite

## 工程要求

- 推荐仓库根目录作为 multi-stage build context：Bun build → Rust build/embed → 单一 runtime image。
- 删除或明确废弃独立 Frontend 容器；如暂时保留兼容模式，必须只有一个生产默认值。
- 配置 `APP__PUBLIC_URL`、API Base、CORS 和 Gateway 路径与实际代码一致。
- clean build 禁止依赖本地 dist、未跟踪文件或历史缓存。
- 生产镜像使用非 root 用户并确保 Git Storage Volume 权限正确。
- Smoke Test 覆盖页面、API、Git clone/push 和静态资源。
- 更新 Release SOP、Project Map、Architecture 和部署说明。

## 不做事项

- 不引入 Kubernetes 或 Service Mesh 作为完成条件。
- 不重写 React/Vite 前端。
- 不将 Nginx 作为必须的静态前端服务器重新确立。
- Git 数据恢复归 EVO-118-D。

## 最小验证

```bash
docker compose -f docker-compose.prod.yml build --no-cache
docker compose -f docker-compose.prod.yml up -d
```

随后验证：

- `/health/live`、`/health/ready`；
- SPA 首屏与深层路由；
- hashed asset header；
- 登录和受保护 API；
- 创建 Repo、Git clone/push/pull；
- 容器重建后 Git 数据仍在（与 EVO-118-D 联合验收）。

## 解锁内容

解除 DEPLOY-01 Gate；为外部 Alpha、CI 主线和后续 Repo UI 提供可信的生产交付路径。
