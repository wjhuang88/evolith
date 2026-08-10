# 配置参考

本文档记录 Evolith 的稳定配置边界。操作步骤见 [本地开发 SOP](../sop/LOCAL-DEV.md)、[数据库迁移 SOP](../sop/DATABASE-MIGRATION.md) 和 [发布 SOP](../sop/RELEASE.md)。

## 配置命名规则

后端使用 `config` crate，嵌套配置通过双下划线环境变量映射：

```text
DATABASE__DATABASE_TYPE
DATABASE__URL
GIT_STORAGE__BASE_PATH
JWT__SECRET
SANDBOX__ENABLED
RATE_LIMIT__UNAUTHENTICATED_RPM
OUTBOX__LEASE_SECONDS
```

不要使用 `DATABASE_TYPE`、`DATABASE_URL` 这类单下划线形式表达应用嵌套配置。`scripts/backup.sh`、`scripts/restore.sh` 和 `scripts/git-storage-inventory.sh` 是独立运维入口，按脚本契约使用 `DATABASE_URL` 与 `GIT_STORAGE_PATH`，不要与应用配置变量混淆。

## 开发环境

```bash
DATABASE__DATABASE_TYPE=sqlite
DATABASE__URL=:memory:
GIT_STORAGE__BASE_PATH=./.data/git
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

SQLite 仍是开发/测试路径；联合生产备份与恢复脚本只支持 PostgreSQL，不改变 SQLite 仓库与 workspace tests 的行为。

## 生产环境

```bash
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL=postgres://user:password@postgres:5432/evolith
GIT_STORAGE__BASE_PATH=/var/lib/evolith/git
ENVIRONMENT=production
LOG__LEVEL=info
```

生产环境要求：

- `JWT__SECRET` 必须替换开发默认值。
- `JWT__SECRET` 至少 32 字符。
- 不能使用 Stripe test key。
- `STRIPE__WEBHOOK_SECRET` 不能是 placeholder。
- `GIT_STORAGE__BASE_PATH` 必须挂载持久 Volume，并由 Backend 运行用户读取、写入和创建目录。
- PostgreSQL Volume 与 Git Storage Volume 是同一 DATA-01 耐久性集合；只备份其中一个不构成可恢复备份。
- 当前 Compose 与 K8s 默认均按单 Backend 实例配置 Git Storage。本地/RWO Volume 不等于多实例共享存储；多副本需要共享存储或 Repo affinity 的独立设计。

## Git Storage readiness

`GET /health/ready` 同时检查数据库与 Git Storage。Git Storage 门禁包括：

- 路径存在且为真实目录；
- 目录可枚举；
- 可创建临时子目录和探针文件；
- 可写入、同步并清理探针；
- 任一检查失败时 readiness 返回 HTTP 503，而不是带 `degraded` 文本的 200。

`GET /health/live` 只表示进程存活。Docker/K8s 的 readiness/healthcheck 应使用 `/health/ready`，liveness 使用 `/health/live`。

## Durable Outbox Worker

`outbox-worker` 是与 HTTP Server 同仓库构建、独立运行的进程：

```bash
cargo run --bin outbox-worker -- run
cargo run --bin outbox-worker -- run --once
cargo run --bin outbox-worker -- replay <event-uuid> --confirm
```

- `run` 持续领取，到 SIGINT 后等待当前有界 batch 完成并正常退出。
- `run --once` 领取至多一个 batch，适合 smoke、维护任务和进程测试。
- `replay` 只接受 `dead_letter`，且必须显式 `--confirm`；其他状态不变并非零退出。
- 已注册无副作用的 `system.outbox.probe` 与真实 `repo.push.completed.v1` subscriber；未知事件必须失败。Push producer/reconcile 由 EVO-118-H-C 提供，生产 supervisor/replica 仍归 EVO-118-E。
- Worker 与 HTTP Server 必须指向同一数据库。SQLite 是单进程 Lite 路径；生产并发路径是 PostgreSQL。

Worker 不打印 payload、idempotency key 或 handler 原始错误；`last_error` 只保存稳定错误码。
配置必须满足 `OUTBOX__BATCH_SIZE × OUTBOX__DELIVERY_TIMEOUT_SECONDS < OUTBOX__LEASE_SECONDS`，
否则进程拒绝启动，避免 batch 尚未完成时租约已被其他 Worker 回收。

## 联合备份、恢复与 inventory 脚本变量

这些变量属于脚本接口，不由 `AppConfig` 读取：

| 变量 | 用途 | 要求 |
|------|------|------|
| `DATABASE_URL` | PostgreSQL 源/目标连接串 | 必填；不得打印到普通日志或写入 manifest |
| `GIT_STORAGE_PATH` | 脚本可直接访问的 Git Storage 根目录 | 必填；不得是 symlink |
| `BACKUP_DIR` | 联合备份输出目录 | 默认 `./backups`；建议独立加密存储 |
| `RETENTION_DAYS` | 本地归档保留天数 | 默认 `30`；只清理 `evolith-backup-*.tar.gz` |
| `EVOLITH_BACKUP_QUIESCED` | 确认应用写入已停止 | backup 必须显式为 `true`，否则非零退出 |
| `EVOLITH_RESTORE_QUIESCED` | 确认恢复目标无应用写入 | restore 必须显式为 `true`，否则非零退出 |
| `EVOLITH_APP_VERSION` | 写入 manifest 的非敏感应用版本 | 默认 `unknown` |
| `EVOLITH_RESTORE_EXPECTED_APP_VERSION` | 可选的严格应用版本匹配 | 设置后不匹配即拒绝恢复 |
| `EXPECTED_REFS_FILE` | inventory 的可选 Commit/Branch/Tag ref 快照 | restore 自动使用备份内 `git-refs.tsv` |

脚本必须运行在能够同时访问 PostgreSQL 与同一 Git Volume 的受控维护环境中。对于 Compose/K8s，推荐使用挂载生产 Git Volume 的一次性维护容器/Pod，并将应用流量和写入停止后再设置 `*_QUIESCED=true`；该布尔值是操作确认，不会自行冻结应用。

## HTTP Tool 出站策略

HTTP Tool 的生产默认策略不是环境变量开关，而是代码级 fail-closed 边界：

- 仅允许 `https` 目标；明文 `http` 默认拒绝。
- 拒绝 localhost、私网、link-local、Metadata、multicast、documentation、benchmark、未分配或其他特殊用途地址。
- DNS、连接和 Redirect 共用同一个总 deadline；Redirect 每一跳重新验证并固定已批准地址。
- 系统代理和 reqwest 自动 Redirect 均关闭。
- 当前没有生产私网或明文 HTTP bypass 配置。新增此类能力必须通过独立安全 Story、显式风险接受和负向测试，不得通过开发环境自动放宽。
- 本地集成测试使用显式构造的 test-only policy 访问 loopback；该策略不会通过 Cargo feature 或生产默认构造器生效。

## 常用变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `APP__PUBLIC_URL` | 对外访问前端 URL，用于邮件链接和用户可点击链接 | `http://localhost:3001` |
| `APP__API_BASE_URL` | `/config.js` 注入给嵌入式前端的 API base URL | `/api/v1` |
| `SERVER__HOST` | 服务监听地址 | `0.0.0.0` |
| `SERVER__PORT` | 服务端口 | `8080` |
| `DATABASE__DATABASE_TYPE` | `sqlite` / `postgres` | `sqlite` |
| `DATABASE__URL` | 数据库连接串 | `:memory:` |
| `DATABASE__MAX_CONNECTIONS` | 数据库连接池大小 | `10` |
| `DATABASE__SEED_DATABASE` | 是否写入开发种子数据 | `false` |
| `OUTBOX__BATCH_SIZE` | 每次最多领取的事件数，范围 1~1000 | `10` |
| `OUTBOX__POLL_INTERVAL_MS` | 空闲轮询间隔，范围 1~60000 ms | `500` |
| `OUTBOX__LEASE_SECONDS` | claim 租约秒数，范围 1~86400；必须大于整个 batch 的最坏投递预算 | `60` |
| `OUTBOX__DELIVERY_TIMEOUT_SECONDS` | 单事件 handler 硬超时秒数，范围 1~3600 | `5` |
| `OUTBOX__BASE_BACKOFF_SECONDS` | 指数退避基数秒数，范围 1~3600 | `1` |
| `OUTBOX__DEFAULT_MAX_ATTEMPTS` | producer 默认最大尝试次数，范围 1~100；H-C 接入时使用 | `10` |
| `GIT_STORAGE__BASE_PATH` | Bare Repository 根目录；布局为 `<tenant UUID>/<repo UUID>.git` | `/srv/evolith/repos` |
| `REDIS__URL` | Redis 地址 | `redis://localhost:6379` |
| `JWT__SECRET` | JWT 签名密钥 | dev only |
| `JWT__EXPIRATION` | JWT 过期时间 | `24h` |
| `STORAGE__ENDPOINT` | S3/MinIO endpoint | `localhost:9000` |
| `STORAGE__ACCESS_KEY` | 对象存储 access key | `minioadmin` |
| `STORAGE__SECRET_KEY` | 对象存储 secret key | `minioadmin` |
| `STORAGE__USE_SSL` | 对象存储是否使用 SSL | `false` |
| `STORAGE__BUCKET` | 默认 bucket | `evolith` |
| `SANDBOX__ENABLED` | legacy Docker sandbox executor 开关；ADR-0005 已接受删除，EVO-111 收口前仅作兼容保留 | `false` |
| `SANDBOX__TIMEOUT_SECONDS` | legacy 执行超时 | `30` |
| `SANDBOX__MEMORY_MB` | legacy 内存限制 | `256` |
| `SANDBOX__CPU_SHARES` | legacy Docker CPU shares | `512` |
| `SANDBOX__PIDS_LIMIT` | legacy Docker PID 数限制 | `256` |
| `SANDBOX__NETWORK_ENABLED` | legacy 沙箱网络访问 | `false` |
| `SANDBOX__MAX_OUTPUT_BYTES` | legacy stdout/stderr 最大捕获字节数 | `10485760` |
| `SMTP__ENABLED` | 是否启用 SMTP | `false` |
| `SMTP__HOST` | SMTP host | `localhost` |
| `SMTP__PORT` | SMTP port | `1025` |
| `SMTP__USERNAME` | SMTP 用户名 | 空 |
| `SMTP__PASSWORD` | SMTP 密码 | 空 |
| `SMTP__FROM_ADDRESS` | 发件地址 | `noreply@evolith.io` |
| `SMTP__FROM_NAME` | 发件名称 | `Evolith` |
| `RATE_LIMIT__UNAUTHENTICATED_RPM` | 按 direct peer IP 的未认证请求每分钟限流 | `30` |
| `RATE_LIMIT__AUTHENTICATED_RPM` | 按 JWT user ID 的请求每分钟限流 | `300` |
| `RATE_LIMIT__API_KEY_RPM` | 按 API key ID 的全局每分钟上限；同时执行 Key 自身每小时上限 | `1000` |

当前 caller-aware limiter 在单进程内由所有 Actix worker 共享；多实例 Redis-backed
安全状态与生产 fail-closed 由 EVO-118-G-D 收敛，不能把这里的计数描述为集群全局配额。

`mysql` 当前不受主服务支持。配置校验和连接池创建都会快速返回
`MySQL repositories are not implemented`，不会先建立 MySQL 连接池再在 `main.rs` 中拒绝。
生产主路径是 PostgreSQL。

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
| 应用使用 `DATABASE_URL` | 应用不会按嵌套配置读取；该名称只属于运维脚本 | 应用使用 `DATABASE__URL`；脚本使用 `DATABASE_URL` |
| 未设置 `GIT_STORAGE__BASE_PATH` 或未挂载 Volume | Repo 写入容器可写层，重建后丢失；readiness 503 | 生产显式设置 `/var/lib/evolith/git` 并挂载持久 Volume |
| 只备份 PostgreSQL 或只归档 Git | 只能恢复半套状态，DB/Git 可能不一致 | 在停写窗口运行联合 backup，并保留 manifest/checksum/refs |
| 未停写却设置 `EVOLITH_BACKUP_QUIESCED=true` | 可能生成时间窗口不一致备份 | 先停止流量、后台任务和 readiness 探针写入，再确认标志 |
| 对非空目标执行 restore | 可能覆盖现有环境 | restore 默认拒绝；准备新的空数据库和空 Git 目录 |
| 前端 API URL 缺少 `/api/v1` | 登录等请求 404 | 配置完整 base URL |
| `APP__PUBLIC_URL` 指向后端或容器内地址 | 邮件中的重置密码/邀请链接用户打不开 | 设置为用户可访问的前端地址 |
| 开发 JWT secret 用于生产 | 启动失败或安全风险 | 生产设置强随机密钥 |
| SMTP disabled | 邮件不会真实发送 | 开发看 ConsoleMailer，生产启用 SMTP |
| HTTP Tool 使用明文 `http` 或私网地址 | 创建、更新或执行被安全边界拒绝 | 使用公网 `https` 目标；不要依赖生产 bypass |
| Outbox `batch × timeout >= lease` | 当前批次可能在完成前失去 claim ownership | 增大 lease、减小 batch 或缩短 timeout；无效配置会拒绝启动 |
| 未注册的 Outbox event type | 事件进入 retry/dead-letter，不会假成功 | H-C 为业务事件注册 handler；仅使用 probe 做运行态诊断 |
