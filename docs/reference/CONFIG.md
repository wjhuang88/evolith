# 配置参考

本文档记录 Evolith 的稳定配置边界。操作步骤见 [本地开发 SOP](../sop/LOCAL-DEV.md) 和 [数据库迁移 SOP](../sop/DATABASE-MIGRATION.md)。

## 配置命名规则

后端使用 `config` crate，嵌套配置通过双下划线环境变量映射：

```text
DATABASE__DATABASE_TYPE
DATABASE__URL
JWT__SECRET
SANDBOX__ENABLED
RATE_LIMIT__UNAUTHENTICATED_RPM
```

不要使用 `DATABASE_TYPE`、`DATABASE_URL` 这类单下划线形式表达嵌套配置。

## 开发环境

```bash
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=:memory:
ENVIRONMENT=development
LOG__LEVEL=debug
```

可选文件 SQLite：

```bash
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=sqlite:dev.db?mode=rwc
```

使用 PostgreSQL：

```bash
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL=postgres://evolith:dev_password@localhost:5432/evolith
```

## 生产环境

```bash
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL=postgres://user:password@postgres:5432/evolith
ENVIRONMENT=production
LOG__LEVEL=info
```

生产环境要求：

- `JWT__SECRET` 必须替换开发默认值。
- `JWT__SECRET` 至少 32 字符。
- 不能使用 Stripe test key。
- `STRIPE__WEBHOOK_SECRET` 不能是 placeholder。

## 常用变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `APP__PUBLIC_URL` | 对外访问前端 URL，用于邮件链接和用户可点击链接 | `http://localhost:3001` |
| `APP__API_BASE_URL` | `/config.js` 注入给嵌入式前端的 API base URL | `/api/v1` |
| `SERVER__HOST` | 服务监听地址 | `0.0.0.0` |
| `SERVER__PORT` | 服务端口 | `8080` |
| `DATABASE__DATABASE_TYPE` | `sqlite` / `postgres` / `mysql` | `sqlite` |
| `DATABASE__URL` | 数据库连接串 | `:memory:` |
| `DATABASE__MAX_CONNECTIONS` | 数据库连接池大小 | `10` |
| `DATABASE__SEED_DATABASE` | 是否写入开发种子数据 | `false` |
| `REDIS__URL` | Redis 地址 | `redis://localhost:6379` |
| `JWT__SECRET` | JWT 签名密钥 | dev only |
| `JWT__EXPIRATION` | JWT 过期时间 | `24h` |
| `STORAGE__ENDPOINT` | S3/MinIO endpoint | `localhost:9000` |
| `STORAGE__ACCESS_KEY` | 对象存储 access key | `minioadmin` |
| `STORAGE__SECRET_KEY` | 对象存储 secret key | `minioadmin` |
| `STORAGE__USE_SSL` | 对象存储是否使用 SSL | `false` |
| `STORAGE__BUCKET` | 默认 bucket | `evolith` |
| `SANDBOX__ENABLED` | 是否启用 sandbox executor | `true` |
| `SANDBOX__TIMEOUT_SECONDS` | 执行超时 | `30` |
| `SANDBOX__MEMORY_MB` | 内存限制 | `256` |
| `SANDBOX__CPU_SHARES` | Docker CPU shares | `512` |
| `SANDBOX__PIDS_LIMIT` | Docker PID 数限制 | `256` |
| `SANDBOX__NETWORK_ENABLED` | 沙箱网络访问 | `false` |
| `SANDBOX__MAX_OUTPUT_BYTES` | stdout/stderr 最大捕获字节数 | `10485760` |
| `SMTP__ENABLED` | 是否启用 SMTP | `false` |
| `SMTP__HOST` | SMTP host | `localhost` |
| `SMTP__PORT` | SMTP port | `1025` |
| `SMTP__USERNAME` | SMTP 用户名 | 空 |
| `SMTP__PASSWORD` | SMTP 密码 | 空 |
| `SMTP__FROM_ADDRESS` | 发件地址 | `noreply@evolith.io` |
| `SMTP__FROM_NAME` | 发件名称 | `Evolith` |
| `RATE_LIMIT__UNAUTHENTICATED_RPM` | 未认证请求限流 | `30` |
| `RATE_LIMIT__AUTHENTICATED_RPM` | 已认证请求限流 | `300` |
| `RATE_LIMIT__API_KEY_RPM` | API key 请求限流 | `1000` |

`mysql` 当前只在配置和 pool 层保留入口，主服务启动会提示 MySQL repositories 尚未实现；生产主路径是 PostgreSQL。

## 前端配置

当前 Vite 前端使用：

```text
VITE_API_URL
```

兼容层仍会读取旧 `NEXT_PUBLIC_API_URL`，但新配置应使用 `VITE_API_URL`。该值必须包含 `/api/v1`，例如：

```text
VITE_API_URL=http://localhost:8080/api/v1
```

嵌入式前端运行时配置：

```text
/config.js -> window.__EVOLITH_CONFIG__.apiBaseUrl
```

未设置 `APP__API_BASE_URL` 时，`/config.js` 默认注入同源 `/api/v1`。独立前端开发或
旧部署仍可通过 `VITE_API_URL` / `NEXT_PUBLIC_API_URL` 在构建时指定 API 地址。

## 配置陷阱

| 陷阱 | 影响 | 正确做法 |
|------|------|----------|
| `DATABASE_TYPE=postgres` | 后端不会按嵌套配置读取 | 使用 `DATABASE__DATABASE_TYPE=postgres` |
| 前端 API URL 缺少 `/api/v1` | 登录等请求 404 | 配置完整 base URL |
| `APP__PUBLIC_URL` 指向后端或容器内地址 | 邮件中的重置密码/邀请链接用户打不开 | 设置为用户可访问的前端地址 |
| 开发 JWT secret 用于生产 | 启动失败或安全风险 | 生产设置强随机密钥 |
| SMTP disabled | 邮件不会真实发送 | 开发看 ConsoleMailer，生产启用 SMTP |
