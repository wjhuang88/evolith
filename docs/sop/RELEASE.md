# SOP: 发布与部署

> 生产发布必须同时证明：可构建、可启动、权限边界正确、Git 数据持久、备份可恢复、关键依赖故障可观察。仅通过数据库 migration 或 `/health` 不构成发布闭环。

## 触发条件

- 准备 Internal Alpha、External Alpha、Agent Beta 或 Production 发布。
- 修改 Dockerfile、Compose、K8s、Nginx、Gateway、Volume 或部署脚本。
- 修改 Git Storage、备份、恢复、readiness、邮件、Redis 或生产安全配置。
- 需要回滚或验证生产部署链路。

## 必读文档

- [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)
- [安全敏感变更审查](SECURITY-REVIEW.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- 当前发布关联的 Backlog/Iteration/ADR/API Contract。

## 1. 发布级别与准入

| 环境 | 最低准入 |
|------|----------|
| Local | 可使用 SQLite、临时 Git 目录和 Mock 服务；必须明确非生产 |
| Internal Alpha | EVO-118-B/C 完成；Git 目录使用持久存储 |
| External Alpha | EVO-118-B/C/D/E/F/G 完成，clean build、恢复和安全负向测试通过 |
| Agent Beta | External Alpha + EVO-118-H + EVO-105/106/107 可用 |
| Production | Agent Beta + 恢复演练、容量告警、PR/Main CI、回滚验证和安全复核 |

不满足对应环境 Gate 时，结论使用 `Blocked` 或将部署限制在更低环境；不得通过修改环境名称规避要求。

## 2. 前置检查

### 代码与流程

- [ ] 工作区/分支变更范围明确，无无关文件。
- [ ] Backlog、Iteration、API Contract、ADR 和 Release Note 已按需同步。
- [ ] 后端 fmt/check/clippy/test 按风险执行并记录真实结果。
- [ ] 前端 type-check/build/lint 和必要 E2E 已执行。
- [ ] 安全敏感变更已有 Navigator 结论和负向测试。
- [ ] PR/Main 必需检查通过；Tag-only 历史结果不能替代当前分支验证。

### 生产配置

- [ ] `JWT__SECRET`、数据库、Git Storage 等关键配置缺失时会启动失败。
- [ ] `DATABASE__...`、`GIT_STORAGE__BASE_PATH` 等嵌套键使用双下划线。
- [ ] `APP__PUBLIC_URL` 指向用户可访问地址。
- [ ] API Base/CORS/Gateway 路径与 `/api/v1`、`/repos/`、`/mcp` 实际路由一致。
- [ ] Vite 生产构建未暴露 sourcemap 或 Secret。
- [ ] SMTP 声明启用但初始化失败时 fail closed；生产不使用 ConsoleMailer 假成功。
- [ ] Redis 降级策略按职责明确：普通缓存可降级，Session/限流/锁等安全状态不可静默退化。

### 数据与存储

- [ ] PostgreSQL 使用持久卷并已有备份。
- [ ] `GIT_STORAGE__BASE_PATH` 使用持久卷，不位于容器临时层。
- [ ] 备份同时包含 PostgreSQL 和 Git Repo/Ref/Object。
- [ ] 当前发布周期执行过恢复演练，或仍在有效演练窗口内且无存储格式变更。
- [ ] Git 容量、inode、只读、空间耗尽和备份失败可观察。
- [ ] 多实例部署已解决共享存储、Repo affinity 或一致路由；仅共享 PostgreSQL 不足。

## 3. Clean Production Build

发布构建必须从干净 checkout 或等价 CI 环境执行：

```bash
docker compose -f docker-compose.prod.yml build --no-cache
docker compose -f docker-compose.prod.yml up -d
```

Embedded Frontend 默认顺序：

```text
bun install --frozen-lockfile
→ bun run type-check / build
→ cargo build --release（嵌入 frontend/dist）
→ runtime image
```

构建不得依赖宿主机已有 `frontend/dist`、未跟踪文件或历史容器缓存。生产默认只能有一种静态资源事实交付路径；Nginx 可作 Gateway，但不与 Embedded Frontend 形成双主路径。

## 4. 发布验证矩阵

| 层面 | 验证 |
|------|------|
| Liveness | `/health/live` 返回 200，仅表示进程存活 |
| Readiness | DB、Git Storage 和适用关键依赖健康时 200；故障时 503 |
| API | 登录、刷新、受保护 GET、状态变更 POST、Cookie/CSRF |
| 权限 | Member/API Key/Agent Token 角色矩阵、跨租户、过期/撤销和 capability 负向测试 |
| MCP/Egress | 无 execute 拒绝；localhost/私网/Metadata/Redirect SSRF 拒绝 |
| Git | Repo create、clone、push、pull、Context；适用时 Commit/Promote/Policy |
| 静态资源 | 首屏、深层 SPA、hashed asset Content-Type/Cache-Control |
| 路径隔离 | `/api/v1`、`/repos/`、`/mcp`、`/health`、`/assets/`、SPA fallback 不互相截获 |
| 数据持久化 | Backend 容器重建后 Repo/Commit/Branch/Tag 保留 |
| 恢复 | DB+Git 备份恢复到空环境，登录/list/clone/commit/tag 校验 |
| 邮件 | Reset/Invite/Verify 链接使用公开 URL；失败不返回假成功、不泄露 Token 日志 |
| 事件 | 适用时 Outbox 重启不丢、幂等、重试有界、失败可重放 |

## 5. 最小 Smoke Test

```text
启动生产栈
→ readiness 200
→ 注册/登录
→ 创建 Repo
→ Git clone
→ Push Commit + Tag
→ UI/API 读取 Files/Commits
→ 重建 Backend 容器
→ 再次 clone 并校验 Commit/Tag
```

External Alpha 及以上还必须执行：

```text
备份 DB + Git
→ 清空新环境
→ 恢复
→ 登录、列出 Repo、clone、校验历史
```

## 6. 发布阻断条件

出现任一情况不得发布：

- 低权限调用者可以创建高权限凭证或执行未授权 Tool。
- 租户可控 URL 可以访问 localhost、私网、Metadata 或通过 Redirect/DNS 绕过。
- Git Repo 只保存在容器层，或备份不能恢复 Git Commit/Ref。
- clean build 未通过，或 Embedded/Standalone Frontend 交付口径冲突。
- readiness 在关键依赖故障时仍返回 2xx。
- SMTP/安全配置失败后静默降级并返回业务成功。
- 必需测试未运行、失败或仅引用历史结果。
- 已知 DB/Git 分裂、事件丢失或数据损坏路径没有状态、补偿或明确阻塞归口。

## 7. 回滚

1. 停止新写入或切换维护模式，避免在回滚窗口继续改变 Git/DB 状态。
2. 回退镜像标签、Gateway 或配置。
3. migration 不可逆时，先使用对应 DB+Git 一致性备份恢复。
4. 验证 Git Storage Volume 未被新容器覆盖或重置。
5. 启动后执行 readiness、登录、Repo list、clone/pull 和核心安全负向测试。
6. 如事件 Worker 已投递部分消息，按 idempotency key 和 delivery 状态重放，不盲目清表。
7. 在 Iteration/事故记录中写明数据状态、残余和后续修复 Story。

## 8. 失败恢复

| 现象 | 处理 |
|------|------|
| Production image build 找不到 frontend/dist | 检查根 build context 和 multi-stage 顺序，不复制宿主残留产物 |
| Backend 重建后 Repo 丢失 | 停止进一步重建，导出可用 Git 目录，修复 Volume 后恢复/对账 |
| DB 恢复但 clone 失败 | 校验 Git backup、storage path、权限和 DB/FS Repo 对账 |
| readiness 200 但 DB/Git 不可用 | 视为 Gate 实现缺陷，阻断发布并归 EVO-118-G |
| 邮件流程返回成功但无邮件 | 检查 SMTP fail-closed 和 Outbox，不使用 ConsoleMailer 掩盖 |
| Webhook/Indexer 缺事件 | 检查 Outbox/Worker/幂等状态，不依赖进程内 spawn 补发 |
| 安全负向测试失败 | 禁用相关入口或回滚，登记 P0 并轮换受影响凭证 |

## 9. 闭环记录

发布 Iteration/PR 至少记录：

```markdown
- 发布级别：Internal Alpha / External Alpha / Agent Beta / Production
- Gate 状态：SEC-01 / SEC-02 / DATA-01 / DEPLOY-01 / DATA-02 / REL-01 / EVENT-01
- 构建命令与结果：
- Smoke Test：
- 备份与恢复证据：
- 安全负向测试：
- 回滚点：
- 残余与风险接受者：
- 闭环状态：Complete / Partial / Blocked
```

## 相关文档

- [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)
- [生产就绪执行计划](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)
- [安全敏感变更审查](SECURITY-REVIEW.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- [本地开发](LOCAL-DEV.md)
- [项目地图](../reference/PROJECT-MAP.md)
- [架构](../reference/ARCHITECTURE.md)
