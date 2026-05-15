# SOP: 本地开发

## 触发条件

- 需要启动 Evolith 进行本地调试。
- 需要验证后端、前端或全栈改动。
- 需要定位登录、CSRF、数据库或 sandbox 问题。

## 前置检查

- [ ] Rust toolchain 可用。
- [ ] Node.js / npm 可用。
- [ ] 如使用 full 模式，Docker 和 Docker Compose 可用。
- [ ] 当前工作区没有会被误覆盖的用户改动。

## 模式选择

| 模式 | 命令 | 依赖 | 适用场景 |
|------|------|------|----------|
| lite | `./scripts/dev.sh lite` | Rust + Node.js | 快速开发，SQLite 内存数据库 |
| full | `./scripts/dev.sh start` | Docker + Rust + Node.js | 接近生产，PostgreSQL/Redis/MinIO |
| infra only | `docker compose up -d postgres redis minio` | Docker | 手动启动后端/前端 |

## 标准流程

```bash
# 查看状态
./scripts/dev.sh status

# 快速启动
./scripts/dev.sh lite

# 停止
./scripts/dev.sh stop
```

日志和 PID 由脚本写入：

- `.dev-logs/backend.log`
- `.dev-logs/frontend.log`
- `.dev-pids/backend.pid`
- `.dev-pids/frontend.pid`

## 常用验证

```bash
# 后端
cd backend
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace

# 前端
cd frontend
npm run type-check
npm run build
```

## 失败恢复

| 现象 | 排查 |
|------|------|
| 端口占用 | `./scripts/dev.sh status`，再停止占用进程或换端口 |
| 后端启动后数据库异常 | 检查 `DATABASE__DATABASE_TYPE` 和 `DATABASE__URL` |
| 前端请求 404 | 检查 `NEXT_PUBLIC_API_URL` 是否包含 `/api/v1` |
| CSRF 403 | 先完成登录，让浏览器拿到 `csrf_token` cookie |
| sandbox 不生效 | 检查 Docker 是否可用，以及后端启动日志是否降级到 default executor |

## 注意事项

- `.env.development` 和 `.env.example` 应使用双下划线配置键。
- SQLite 内存库每次重启都会重置数据。
- full 模式下只改前端无需重启后端；只改后端通常不需要重启 Docker 基础设施。
