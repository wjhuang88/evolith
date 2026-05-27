# SOP: 发布与部署

## 触发条件

- 准备发布生产版本。
- 修改 Dockerfile、Compose、K8s、Nginx 或部署脚本。
- 需要回滚或验证生产部署链路。

## 前置检查

- [ ] 工作区变更已确认，未混入无关文件。
- [ ] 后端 `cargo fmt`、`cargo clippy`、`cargo test` 已按风险范围执行。
- [ ] 前端 `bun run type-check`、`bun run build` 已按风险范围执行。
- [ ] `.env`、Compose、K8s 使用 `DATABASE__...` 这类双下划线配置键。
- [ ] `VITE_API_URL` 包含正确 API 前缀，通常是 `/api/v1`；旧 `NEXT_PUBLIC_API_URL` 只用于兼容读取。
- [ ] `APP__PUBLIC_URL` 指向用户可访问的前端地址，用于重置密码和邀请邮件链接。
- [ ] Vite 生产构建未默认暴露 sourcemap，除非本次发布明确需要调试。
- [ ] 涉及脚本行为变更时，已更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- [ ] 发布相关实施任务已按 [任务收口与完成声明](TASK-CLOSURE.md) 记录验证、状态
      与未解除风险；存在阻塞时不得声明发布闭环完成。

## 本地生产栈验证

```bash
docker compose -f docker-compose.prod.yml build
docker compose -f docker-compose.prod.yml up -d
curl -f http://localhost/health/live
```

如果只验证开发栈：

```bash
./scripts/dev.sh start
./scripts/dev.sh status
```

## 发布验证

| 层面 | 验证 |
|------|------|
| 后端健康 | `/health/live` 和 `/health/ready` |
| API 前缀 | 登录、刷新、一个受保护 GET、一个状态变更 POST |
| Cookie/CSRF | 登录后确认 `evolith_token` 和 `csrf_token` 行为 |
| 数据库 | migration 成功，核心表可读写 |
| 前端 | 首屏、登录、dashboard、核心 CRUD 页面 |
| 静态资源 | 浏览器或 `curl -I` 验证 `/assets/<构建产物>` 返回 JS/CSS，不被 rewrite 成 `index.html` |
| 公开链接 | 邮件中的重置密码、邀请接受、邮箱验证链接指向前端公开 URL |
| sandbox | `SANDBOX__ENABLED=true` 时 Docker executor 正常初始化 |

> 当前 Nginx 托管静态 SPA 是 EVO-016 前的过渡部署形态。终局若采用“前端嵌入后端发布物”，仍需保留 API 网关/SSL/反代检查，但不再把 Nginx 作为前端静态资源的唯一托管前提。

## 回滚

1. 回退镜像标签或部署配置。
2. 如果 migration 不可逆，先确认数据备份和回滚 SQL。
3. 重启 backend/frontend/nginx。
4. 验证 health 和核心登录流程。

## 失败恢复

| 现象 | 排查 |
|------|------|
| 后端启动失败 | 查看配置校验、JWT secret、数据库连接和 migration 日志 |
| 前端调用 API 404 | 检查 `VITE_API_URL` 和 Nginx path rewrite |
| 生产首屏白屏 | 检查 `/assets/` 反代是否保留路径前缀，以及 JS/CSS Content-Type |
| 邮件链接打不开 | 检查 `APP__PUBLIC_URL` 是否误指向后端监听地址或容器内地址 |
| 登录成功但后续 401 | 检查 cookie domain、secure、sameSite、CORS credentials |
| 状态变更 403 | 检查 CSRF cookie/header 是否同源可见 |
| ready 不通过 | 检查数据库、Redis 和依赖服务 |

## 相关文档

- [任务收口与完成声明](TASK-CLOSURE.md)
- [本地开发](LOCAL-DEV.md)
- [项目地图](../reference/PROJECT-MAP.md)
- [API 合约](../reference/API-CONTRACT.md)
