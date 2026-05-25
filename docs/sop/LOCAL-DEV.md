# SOP: 本地开发

## 触发条件

- 需要启动 Evolith 进行本地调试。
- 需要验证后端、前端或全栈改动。
- 需要定位登录、CSRF、数据库或 sandbox 问题。

## 前置检查

- [ ] Rust toolchain 可用。
- [ ] Node.js / Bun 可用。
- [ ] 如使用 full 模式，Docker 和 Docker Compose 可用。
- [ ] 当前工作区没有会被误覆盖的用户改动。

## 模式选择

| 模式 | 命令 | 依赖 | 适用场景 |
|------|------|------|----------|
| lite | `./scripts/dev.sh lite` | Rust + Node.js + Bun | 快速开发，SQLite 内存数据库 |
| full | `./scripts/dev.sh start` | Docker + Rust + Node.js + Bun | 接近生产，PostgreSQL/Redis/MinIO |
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
bun run type-check
bun run build
```

## 本地测试账号

SQLite lite 模式默认使用内存库，每次后端重启都会清空数据。当前 migrations 中的 `test@example.com` 和 `admin@example.com` 仅为历史种子示例，密码哈希是占位值，不应假定可登录。

推荐在每次 lite 启动后通过注册流程创建临时账号：

```bash
curl -i -s -X POST http://127.0.0.1:8080/api/v1/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"dev@example.com","username":"dev","password":"TestPassword123!"}'
```

随后可在前端使用：

- 邮箱：`dev@example.com`
- 密码：`TestPassword123!`

如改用持久化 SQLite 文件或 PostgreSQL，账号生命周期以对应数据库为准。

## 失败恢复

| 现象 | 排查 |
|------|------|
| 端口占用 | `./scripts/dev.sh status`，再停止占用进程或换端口 |
| 后端启动后数据库异常 | 检查 `DATABASE__DATABASE_TYPE` 和 `DATABASE__URL` |
| 前端请求 404 | 检查 `VITE_API_URL` 是否包含 `/api/v1`；旧 `NEXT_PUBLIC_API_URL` 仅兼容读取 |
| CSRF 403 | 先完成登录，让浏览器拿到 `csrf_token` cookie |
| 种子账号登录 500 或失败 | 不要继续尝试历史种子账号；按“本地测试账号”注册临时账号 |
| sandbox 不生效 | 检查 Docker 是否可用，以及后端启动日志是否降级到 default executor |

## 注意事项

- `.env.development` 和 `.env.example` 应使用双下划线配置键。
- SQLite 内存库每次重启都会重置数据。
- Vite 前端默认端口是 `3001`；如需覆盖，设置 `FRONTEND_PORT` 并保持 `APP__PUBLIC_URL` 与实际前端地址一致。
- full 模式下只改前端无需重启后端；只改后端通常不需要重启 Docker 基础设施。
