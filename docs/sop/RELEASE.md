# SOP: 发布与部署

> 生产发布必须同时证明：可构建、可启动、权限边界正确、Git 数据持久、备份可恢复、关键依赖故障可观察。仅通过数据库 migration、文件存在或 `/health` 不构成发布闭环。

## 触发条件

- 准备 Internal Alpha、External Alpha、Agent Beta 或 Production 发布。
- 修改 Dockerfile、Compose、K8s、Nginx、Gateway、Volume 或部署脚本。
- 修改 Git Storage、备份、恢复、readiness、邮件、Redis 或生产安全配置。
- 需要回滚或验证生产部署链路。

## 必读文档

- [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)
- [配置参考](../reference/CONFIG.md)
- [脚本发布说明](../reference/SCRIPTS-RELEASE-NOTES.md)
- [安全敏感变更审查](SECURITY-REVIEW.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- 当前发布关联的 Backlog、Iteration、ADR 与 API Contract。

## 1. 发布级别与准入

| 环境 | 最低准入 |
|------|----------|
| Local | 可使用 SQLite、临时 Git 目录和 Mock 服务；必须明确非生产 |
| Internal Alpha | EVO-118-B/C 完成；Git 目录使用持久存储；DATA-01 未关闭时仅限受控环境 |
| External Alpha | EVO-118-B/C/D/E/F/G 完成，clean build、恢复和安全负向测试通过 |
| Agent Beta | External Alpha + EVO-118-H + EVO-105/106/107 可用 |
| Production | Agent Beta + 恢复演练、容量告警、PR/Main CI、回滚验证和安全复核 |

不满足对应环境 Gate 时，结论使用 `Blocked` 或限制在更低环境；不得通过修改环境名称规避要求。

## 2. 前置检查

### 代码与流程

- [ ] 工作区/分支变更范围明确，无无关文件。
- [ ] Backlog、Iteration、API Contract、ADR 和 Release Note 已按需同步。
- [ ] 后端 fmt/check/clippy/test 按风险执行并记录真实结果。
- [ ] 前端 type-check/build/lint 和必要 E2E 已执行。
- [ ] 安全或耐久性敏感变更已有独立 Navigator 结论和负向测试。
- [ ] PR 必需检查在 PR Head SHA 上通过；synthetic merge、Tag-only、旧 Head 或旧 `main` 结果不能替代 exact-head 证据。

### 生产配置

- [ ] `JWT__SECRET`、数据库、Git Storage 等关键配置缺失时会启动失败或保持 not-ready。
- [ ] `DATABASE__...`、`GIT_STORAGE__BASE_PATH` 等应用嵌套键使用双下划线。
- [ ] `APP__PUBLIC_URL` 指向用户可访问地址。
- [ ] API Base/CORS/Gateway 路径与 `/api/v1`、`/repos/`、`/mcp` 实际路由一致。
- [ ] Vite 生产构建未暴露 sourcemap 或 Secret。
- [ ] SMTP 声明启用但初始化失败时 fail closed；生产不使用 ConsoleMailer 假成功。
- [ ] Redis 降级策略按职责明确：普通缓存可降级，Session/限流/锁等安全状态不可静默退化。

### 数据与存储

- [ ] PostgreSQL 使用持久卷并已有可验证备份。
- [ ] `GIT_STORAGE__BASE_PATH` 使用持久卷，不位于容器临时层。
- [ ] Backend 容器运行 UID/GID 对 Git Volume 有读取、写入、创建和清理权限。
- [ ] 备份同时包含 PostgreSQL、Git Repo/Object/Ref、manifest 和 checksum。
- [ ] 当前发布周期执行过空环境恢复演练，或仍在有效演练窗口内且无存储格式变更。
- [ ] Git 容量、inode、只读、空间耗尽、备份失败和恢复失败可观察。
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

> DATA-01 的 Volume、backup/restore 与 readiness 验证不能豁免 DEPLOY-01。若 clean production build 因 Embedded Frontend 构建上下文失败，应归 EVO-118-E 并保持生产发布阻断；不得在 EVO-118-D 中顺手改成交付双轨。

## 4. DATA-01 联合备份

### 一致性边界

联合备份当前采用**明确维护窗口/停写**，不宣称在线快照：

1. 停止外部写流量、后台写任务和 Git push。
2. 确认没有活跃写事务或 receive-pack。
3. 在能同时访问 PostgreSQL 与实际 Git Volume 的受控维护容器/Pod 中运行 inventory。
4. 只有上述条件真实成立后，设置 `EVOLITH_BACKUP_QUIESCED=true`。
5. 执行联合 backup；任何 inventory、fsck、dump、archive、checksum 或原子落盘失败均视为本次备份失败。

布尔变量只记录操作员确认，不会自动冻结应用。未经停写而设置为 `true` 产生的归档不具备一致性承诺。

### 命令

```bash
DATABASE_URL='postgres://...' \
GIT_STORAGE_PATH='/var/lib/evolith/git' \
BACKUP_DIR='/secure/evolith-backups' \
EVOLITH_APP_VERSION='<release-or-commit-sha>' \
EVOLITH_BACKUP_QUIESCED=true \
bash scripts/backup.sh
```

成功归档至少包含：

```text
database.sql.gz
git-storage.tar.gz
git-refs.tsv
manifest.env
SHA256SUMS
```

`manifest.env` 和普通日志不得包含数据库密码、JWT Secret、API Key、SMTP/Stripe Secret 或完整连接串。SHA-256 用于完整性检测，不提供来源认证；归档仍必须存放在访问受控、加密并有不可变/离线副本的介质中。

## 5. DATA-01 安全恢复

### 目标要求

- 使用新的空 PostgreSQL 数据库和空 Git Storage 目录。
- 停止目标环境全部应用写入，并设置 `EVOLITH_RESTORE_QUIESCED=true`。
- 不直接覆盖现有生产数据；需要替换时先在隔离环境完整恢复和验收，再按受控切换方案执行。
- manifest、格式版本、必需文件、唯一 checksum 清单、归档路径与条目类型、Git layout、bare repo、fsck 和 refs 必须在目标写入前验证。
- symlink、hardlink、设备文件或其他特殊归档条目必须拒绝。
- 恢复中途失败必须非零退出，并将此前为空的目标恢复为空；不得输出成功。

### 命令

```bash
DATABASE_URL='postgres://.../evolith_restore' \
GIT_STORAGE_PATH='/var/lib/evolith/git-restore' \
EVOLITH_RESTORE_EXPECTED_APP_VERSION='<release-or-commit-sha>' \
EVOLITH_RESTORE_QUIESCED=true \
bash scripts/restore.sh /secure/evolith-backups/evolith-backup-<timestamp>.tar.gz
```

恢复脚本会自动执行 DB/Git inventory 与备份 refs 对账。随后还必须执行应用行为验收，不能以文件存在替代：

```text
启动隔离环境
→ readiness 200
→ 登录
→ Repo list
→ clone
→ 校验目标 Commit SHA
→ 校验额外 Branch 与 Tag
→ 重建 Backend 容器/进程
→ 再次登录、clone 和校验 refs/metadata
```

只有 DB、Git 和应用行为三层同时通过，才可将演练记录为成功。

## 6. DB/Git Inventory

日常巡检、备份前和恢复后运行：

```bash
DATABASE_URL='postgres://...' \
GIT_STORAGE_PATH='/var/lib/evolith/git' \
bash scripts/git-storage-inventory.sh
```

可选以备份 refs 做严格校验：

```bash
EXPECTED_REFS_FILE='/path/to/git-refs.tsv' \
DATABASE_URL='postgres://...' \
GIT_STORAGE_PATH='/var/lib/evolith/git' \
bash scripts/git-storage-inventory.sh
```

下列任一情况必须非零退出：DB-only Repo、disk-only Repo、路径格式异常、symlink、意外/孤儿目录、无效 bare repository、`git fsck` 失败、默认 Branch 缺失、数据库 last Commit 缺失或 refs 不匹配。完整 Repo create/delete 状态机与自动 Reconciler 仍归 EVO-118-F。

## 7. 发布验证矩阵

| 层面 | 验证 |
|------|------|
| Liveness | `/health/live` 返回 200，仅表示进程存活 |
| Readiness | DB 与 Git Storage 健康时 200；路径缺失、非目录、不可读写或 DB 故障时 503 |
| API | 登录、刷新、受保护 GET、状态变更 POST、Cookie/CSRF |
| 权限 | Member/API Key/Agent Token 角色矩阵、跨租户、过期/撤销和 capability 负向测试 |
| MCP/Egress | 无 execute 拒绝；localhost/私网/Metadata/Redirect SSRF 拒绝 |
| Git | Repo create、clone、push、pull、Context；适用时 Commit/Promote/Policy |
| 静态资源 | 首屏、深层 SPA、hashed asset Content-Type/Cache-Control |
| 路径隔离 | `/api/v1`、`/repos/`、`/mcp`、`/health`、`/assets/`、SPA fallback 不互相截获 |
| 数据持久化 | Push Commit/Branch/Tag 后重建 Backend，clone/refs/metadata 保留 |
| 恢复 | DB+Git 备份恢复到空环境，登录/list/clone/Commit/Branch/Tag/SHA 校验 |
| 失败输入 | DB-only、Git-only、损坏、checksum/version 不匹配、恶意归档、非空目标全部拒绝 |
| 假成功 | backup/restore/inventory 任一失败返回非零，不继续或输出成功 |
| 邮件 | Reset/Invite/Verify 链接使用公开 URL；失败不返回假成功、不泄露 Token 日志 |
| 事件 | 适用时 Outbox 重启不丢、幂等、重试有界、失败可重放 |

## 8. 最小 Smoke Test

```text
启动生产栈
→ readiness 200
→ 注册/登录
→ 创建 Repo
→ Git clone
→ Push 两个 Commit + 额外 Branch + Tag
→ UI/API 读取 Repo Metadata
→ 重建 Backend 容器
→ 再次 clone 并校验 Commit/Branch/Tag
```

External Alpha 及以上还必须执行：

```text
进入维护窗口并停写
→ inventory 通过
→ 联合备份 DB + Git
→ 恢复到新的空环境
→ readiness 200
→ 登录、列出 Repo、clone、校验历史
→ 再重建 Backend 并复验
```

## 9. 容量、inode 与失败告警基线

无需在 DATA-01 中建设完整监控平台，但接入生产监控前至少采用以下基线：

| 信号 | Warning | Critical / Action |
|------|---------|-------------------|
| Git Volume bytes used | ≥ 80% | ≥ 90%；停止非必要写入并扩容/清理 |
| Git Volume inode used | ≥ 80% | ≥ 90%；即使字节仍空闲也按不可写风险处理 |
| 最近成功联合备份年龄 | 超过计划周期（默认 24h） | 超过计划周期 2 倍或连续两次失败 |
| backup/restore/inventory 退出非零 | 立即告警 | Critical；不得以旧备份或 warning 代替成功 |
| Git readiness 503 | 立即告警并摘除实例 | 持续失败、只读、满盘或权限漂移按发布阻断处理 |
| checksum/version/manifest 拒绝 | 立即记录 | 视为归档损坏、错误介质或不兼容恢复事件，禁止强制绕过 |
| `git fsck`/refs 对账失败 | 立即记录 | 数据完整性事件；停止恢复切换和相关写入 |

监控日志不得包含 Secret 或完整数据库连接串。成功备份的时间、格式版本、应用版本、归档标识和非敏感校验结果应进入审计/运维记录。

## 10. 发布阻断条件

出现任一情况不得发布：

- 低权限调用者可以创建高权限凭证或执行未授权 Tool。
- 租户可控 URL 可以访问 localhost、私网、Metadata 或通过 Redirect/DNS 绕过。
- Git Repo 只保存在容器层，或备份不能恢复 Git Commit/Ref。
- clean build 未通过，或 Embedded/Standalone Frontend 交付口径冲突。
- readiness 在 DB/Git Storage 故障时仍返回 2xx。
- restore 可以覆盖非空目标、跳过 checksum/版本，或在部分恢复后报告成功。
- SMTP/安全配置失败后静默降级并返回业务成功。
- 必需测试未运行、失败或仅引用历史结果。
- 已知 DB/Git 分裂、事件丢失或数据损坏路径没有状态、补偿或明确阻塞归口。

## 11. 回滚

1. 停止新写入或切换维护模式，避免在回滚窗口继续改变 Git/DB 状态。
2. 回退镜像标签、Gateway 或配置。
3. migration 不可逆时，先使用对应 DB+Git 一致性备份恢复到新环境。
4. 验证 Git Storage Volume 未被新容器覆盖或重置。
5. 启动后执行 readiness、inventory、登录、Repo list、clone/pull 和核心安全负向测试。
6. 如事件 Worker 已投递部分消息，按 idempotency key 和 delivery 状态重放，不盲目清表。
7. 在 Iteration/事故记录中写明数据状态、残余和后续修复 Story。

## 12. 失败恢复

| 现象 | 处理 |
|------|------|
| Production image build 找不到 `frontend/dist` | 检查根 build context 和 multi-stage 顺序，不复制宿主残留产物；归 EVO-118-E |
| Backend 重建后 Repo 丢失 | 停止进一步重建，导出可用 Git 目录，修复 Volume 后恢复/对账 |
| DB 恢复但 clone 失败 | 校验 Git backup、storage path、权限和 DB/FS Repo 对账 |
| readiness 200 但 DB/Git 不可用 | 视为 Gate 实现缺陷，阻断发布；Git 直接问题归 D，综合依赖归 G |
| restore 拒绝归档 | 不强制绕过；检查 manifest/version/checksum/条目类型与来源介质 |
| 邮件流程返回成功但无邮件 | 检查 SMTP fail-closed 和 Outbox，不使用 ConsoleMailer 掩盖 |
| Webhook/Indexer 缺事件 | 检查 Outbox/Worker/幂等状态，不依赖进程内 spawn 补发 |
| 安全负向测试失败 | 禁用相关入口或回滚，登记 P0 并轮换受影响凭证 |

## 13. 闭环记录

发布 Iteration/PR 至少记录：

```markdown
- 发布级别：Internal Alpha / External Alpha / Agent Beta / Production
- Gate 状态：SEC-01 / SEC-02 / DATA-01 / DEPLOY-01 / DATA-02 / REL-01 / EVENT-01
- Base SHA / exact Head SHA：
- 构建命令与结果：
- Smoke Test：
- 备份与恢复证据：
- 安全负向测试：
- Navigator 结论：
- 回滚点：
- 残余与风险接受者：
- 闭环状态：Complete / Partial / Blocked
```

## 相关文档

- [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)
- [配置参考](../reference/CONFIG.md)
- [脚本发布说明](../reference/SCRIPTS-RELEASE-NOTES.md)
- [生产就绪执行计划](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)
- [安全敏感变更审查](SECURITY-REVIEW.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- [本地开发](LOCAL-DEV.md)
- [项目地图](../reference/PROJECT-MAP.md)
- [架构](../reference/ARCHITECTURE.md)
