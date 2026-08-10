# EVO-118-E 最终生产构建与部署收敛

- **类型**：Technical / Deploy / Release
- **状态**：Proposed / final release gate
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-F/G/H、目标 MVP 产品 Story、EVO-111、EVO-122-A/B/C 全部完成
- **交付决策**：[ADR-0007](../../decisions/ADR-0007-agent-write-and-production-delivery-boundaries.md) 已确定生产默认使用单一 Embedded Frontend 后端镜像；Nginx 仅为可选 Gateway/SSL/反向代理。
- **顺序决策**：[ADR-0010](../../decisions/ADR-0010-final-production-convergence-after-product-completion.md) 已确定本 Story 在改造、产品开发和 legacy cleanup 完成后执行；DEPLOY-01 关闭前禁止上线，但不阻塞开发。
- **影响范围**：Dockerfile / Compose / frontend / backend / gateway / docs / tests

## 工程目标

在目标 MVP 开发和清理完成后统一 Evolith 的最终生产交付形态，确保从干净 checkout 能构建包含正确前端静态资源的后端镜像，并让 Nginx 只承担可选 Gateway/SSL/反代职责。

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
- 不作为 Repo UI、Agent、Discovery 或 legacy cleanup 的开发前置 Gate。

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

解除 DEPLOY-01 Gate；为 External Alpha/生产发布提供可信的最终交付路径。关闭前可以继续开发，但不得上线或声明生产就绪。

## 进入条件

- EVO-118-F/G/H 已关闭 DATA-02、REL-01、EVENT-01。
- Product Interaction Architecture 的目标 MVP owner Story 已完成，主业务闭环通过真实数据验证。
- EVO-111 与 EVO-122-A/B/C 已完成 legacy runtime/schema cleanup。
- 启动前重新执行 START-ITERATION，并以当时 final head 盘点 route、API、worker、migration、配置和脚本范围。
