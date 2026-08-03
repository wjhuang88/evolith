# 脚本发布说明

> 本文件记录 `scripts/`、部署辅助脚本和跨平台运维入口的行为变更。
> 修改脚本参数、执行顺序、默认值、退出码或副作用时必须更新本文件。

## Unreleased

### EVO-118-D Navigator 整改 — 全数据库空目标与 Git readback

- **统一 PostgreSQL 空目标语义**：新增 `scripts/postgres-user-object-count.sql`，由 restore 前置门禁、rollback 后置验证和 durability 测试共用。PostgreSQL 内部 schema（`information_schema` 与 `pg_*`）不计入用户数据；默认空 `public` 可存在；任意额外用户 schema 即使为空也视为非空，`public` 内关系、分区、view、materialized view、sequence、foreign table、function/procedure、用户定义 type 等 schema-scoped 对象均会阻止覆盖。
- **全用户 schema rollback-to-empty**：`scripts/restore.sh` 只在数据库与 Git 双空目标门禁通过后武装 rollback。后段失败时删除本次恢复创建的全部非内部用户 schema/object，重建空 `public`，再用同一 SQL 定义验证数据库为空；Git 目标同时清理并验证。任一清理或验证失败保持 restore 非零并输出 `CRITICAL`，不得打印 restore success。
- **非 `public` 负向矩阵**：`scripts/tests/durability.sh` 的 source backup 包含 `audit` schema、table、enum type 与 function。新增 `legacy.marker` 目标拒绝测试，证明在写入前拒绝并保留原标记；新增 post-write inventory failure，证明已恢复的 `public` 与 `audit` 对象及 Git 文件全部回滚。故障注入对象使用 schema-qualified 名称，避免被 `pg_dump` 的空 `search_path` 提前截获。
- **真实 Git Storage readback**：readiness 探针写入固定字节并 `sync_all` 后关闭写句柄，重新通过 `File::open` 读取到 EOF 并精确比对。reopen/read 失败、内容不一致或 cleanup 失败均返回 not-ready；成功、读取失败和 mismatch 都尽最大努力清理隔离 probe。
- **readback 测试**：新增正常 create/write/sync/reopen/read/compare/cleanup、精确字节与截断 mismatch、确定性 read error、内容篡改 mismatch 以及失败后 cleanup 验证；原 missing path、普通文件、不可写目录和 503 状态测试继续保留。
- 被审核旧 Head `c8b83dec54c1ed7df75c29b34b3d1b3a0f40a885` 的 `ci` run `30756053633` 与 `data-durability-container` run `30756053641` 不覆盖上述失败路径，只保留为历史定位证据。整改最终接受必须使用最新 exact-head 两条 required workflow 与原独立 Navigator re-review。

### EVO-118-D — PostgreSQL + Git 联合备份、恢复与盘点

- **`scripts/backup.sh`**：由 PostgreSQL-only `pg_dump` 替换为 DATA-01 联合备份入口。现在强制要求 `EVOLITH_BACKUP_QUIESCED=true`、`DATABASE_URL`、`GIT_STORAGE_PATH` 和可追溯的 `EVOLITH_APP_VERSION`；版本移除 CR/LF 后必须仍非空。备份前执行 DB/Git inventory 和 `git fsck`，同时归档 PostgreSQL dump、全部 Git objects/refs、refs 快照、版本化 manifest 与 SHA-256 清单。特殊文件在归档前被拒绝，生成后的内层 Git 归档与最终归档还会复用 restore 的路径/条目类型约束进行自校验，避免先报告备份成功、恢复时才发现归档不可接受。任一子命令或自校验失败时非零退出；完整归档先在 staging 中校验，再原子移动到输出目录，不留下可误用的半成品。
- **`scripts/restore.sh`**：新增安全恢复入口。恢复前验证归档路径、条目类型、精确必需文件、严格七项 manifest schema、格式/应用版本、UTC 创建时间、Git Storage layout、四个唯一 checksum、Git 目录布局、symlink、bare repository、`git fsck` 与 Commit/Branch/Tag refs 精确集合；默认拒绝含任意用户 schema/object 的 PostgreSQL 目标或非空 Git 目标。PostgreSQL dump 使用单事务恢复，Git 内容在隔离 staging 中预验证；任何 SQL、安装或 inventory 后段失败都会尝试把此前为空的全部用户数据库与 Git 目标恢复为空并非零退出，自动回滚不完整时输出 CRITICAL 运维提示，不报告部分成功。
- **`scripts/git-storage-inventory.sh`**：新增 DB/Git 对账入口。识别 DB-only、disk-only、重复/异常 storage path、意外或孤儿目录、symlink、无效 bare repository、`git fsck` 失败、默认 Branch/last Commit/指定 refs 缺失；发现任一不一致即非零退出。
- **`scripts/tests/durability.sh`**：新增真实 PostgreSQL + Git 故障矩阵。创建多 Commit、额外 Branch 与 Tag，验证联合 backup、空目标 restore、目标 SHA/refs、DB/Git inventory、非空目标拒绝、DB-only/Git-only/损坏/checksum/version/重复 checksum 拒绝、缺失或错误 manifest layout/时间拒绝、Git refs 快照缺项拒绝、恶意 symlink/非法布局拒绝、缺少应用版本、仅含 CR/LF 的清洗后空版本或 Git Storage FIFO 导致 backup fail closed、SQL 中途失败单事务回滚为空，以及 restore 后段 inventory 失败回滚为空。版本不兼容测试会重写 checksum，确保实际命中 version gate，而不是被 checksum gate 提前截获。
- **`scripts/tests/container-volume-persistence.sh`**：新增 Docker named Volume 重建证据。第一个容器创建并 push Commit/Branch/Tag，容器退出后由第二个容器挂载同一 Volume，执行 `git fsck`、refs/SHA 校验和真实 clone。
- **`scripts/tests/application-recovery-drill.sh`**：新增应用级空环境恢复演练。启动真实 PostgreSQL Backend，注册用户/租户、创建 Repo、通过 Smart HTTP push 多 Commit/Branch/Tag；联合备份后恢复到新 DB 与新 Git 目录，重新登录、列 Repo、clone/refs/SHA 校验，注入只读 readiness 503，恢复后重启 Backend 再次 clone。
- **`scripts/tests/check-markdown-links.py`**：新增仓库内 Markdown 本地链接门禁。
- **`.github/workflows/ci.yml`**：PR CI 改为显式 checkout 并验证 PR exact-head SHA；新增 `git diff --check`、Markdown links、Compose durability mapping、脚本语法、PostgreSQL+Git 恢复矩阵、Docker Volume 重建与应用级恢复演练。Frontend lint、完整 PostgreSQL workspace tests 和生产 Compose clean-build/startup 诊断保留在 release/tag/manual gate。PR #7 的一次 exact-head run 在全部 DATA-01 门禁通过后，因无缓存生产构建诊断耗尽 55 分钟 job timeout 而被误报失败；该 DEPLOY-01 诊断现已从 PR required job 解耦，避免 `continue-on-error` 无法处理 runner/job 级超时。
- **`backend/Dockerfile`**：运行时新增 `git`，创建固定非 root 用户与 `/var/lib/evolith/git`，Docker healthcheck 改为 `/health/ready`。CI-only recovery runtime image 使用 UID/GID 10001，避免 Ubuntu runner 已占用 UID 1000。
- **`docker-compose.prod.yml`**：Backend 显式设置 `GIT_STORAGE__BASE_PATH=/var/lib/evolith/git` 并挂载 `git_data` named Volume；该 Volume 只声明单实例持久性，不声明多实例共享或复制。
- **`deploy/k8s/backend.yaml`**：新增 ReadWriteOnce PVC，Backend 单副本 + `Recreate`，以受控非 root UID/GID/fsGroup 挂载 Git Storage；移除与本地/RWO Git Volume 不兼容的多副本/HPA 暗示。共享存储或 Repo affinity 另行设计。

### 运维兼容性与注意事项

- 旧用法 `DATABASE_URL=... ./scripts/backup.sh` 不再成功；必须同时提供 Git Storage、可追溯应用版本并真实进入停写维护窗口。
- `EVOLITH_BACKUP_QUIESCED=true` / `EVOLITH_RESTORE_QUIESCED=true` 是操作确认，不会自动暂停应用、后台任务或 Git push。
- 联合 restore 只支持 PostgreSQL 生产路径；SQLite 开发路径不使用这些脚本，继续由 Rust workspace tests 回归。
- SHA-256 用于损坏检测，不提供归档签名或来源认证；备份介质仍需加密、访问控制和离线/不可变副本。
- restore 默认只面向真正空目标：除 PostgreSQL 内部 schema 外只能存在默认空 `public`；任何额外用户 schema 或 `public` 用户对象都会在写入前拒绝。覆盖式灾难恢复必须先在新环境恢复、完成功能验收，再通过独立受控切换方案执行。
- `GIT_STORAGE_PATH` 不得是 symlink；源 Git Storage 中的 FIFO/socket/device 等特殊文件会使 backup 失败，归档中出现 symlink、hardlink、特殊文件、异常层级或非 UUID 布局会被拒绝。
- manifest v1 必须精确包含 `backup_format_version`、`application_version`、`created_at`、`consistency`、`database_format`、`git_format`、`git_storage_layout` 七项；缺失、重复、额外或不兼容值均 fail closed。
- GitHub Contents API 创建的新脚本可能不携带 executable bit；CI 与文档统一使用 `bash scripts/...`、`psql -f scripts/postgres-user-object-count.sql` 或 `python3 scripts/...` 调用，不依赖直接执行位。

### 验证状态

- 旧基线 CI #142 曾通过 exact-head、Frontend install/type-check/build、Compose durability mapping、真实 PostgreSQL + Git 联合 backup/restore 和 Docker Volume 重建，仅在 Rust 格式检查失败；CI #143 随后通过 fmt/check/clippy。这些结果证明切片可执行，但不再是当前 `main` 的 exact-head 证据。
- 2026-08-02 `main` 推进到 `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`，包含 PR #8 / #9 的 Repo UI 与 Iteration 054 收口。EVO-118-D 已在该基线上重建实现分支；联合恢复、应用级演练、Frontend/Rust 回归和 Navigator 必须全部在新的 PR #7 Head 重跑。
- 实现 Head `83351a2375b6537ddb83d71d84a2d22bfaaacc15` 的 `ci` run `30736864612` 与 `data-durability-container` run `30736864631` 已全绿；后续 Driver 审计强化会改变 Head，因此最终接受仍以最新 exact-head required workflows 为准。
- Head `6bbb2dabc39232f7c057c08215a9faed0e0b3b9b` 的 run `30755307482` 已通过所有 DATA-01 required steps，包括强化负向矩阵、Rust 全量回归和应用级恢复；随后仅在 DEPLOY-01 无缓存生产构建诊断期间触发 55 分钟 job timeout。该结果不是最终绿色证据，但证明诊断耦合而非 DATA-01 失败。
- 整改中间 Head `fb4b189e7c75b1fff224f9770e6b9aa269695dcb` 的 `ci` run `30794295296` 已实际通过新增非 `public` 目标拒绝、非 `public` post-write rollback、通用 Volume recreation、Frontend gates 和脚本门禁，仅因 readiness 测试文件未应用 rustfmt 而停止；格式修正后的最终 Head 仍需重新跑全部 required steps。
- `docker compose build --no-cache` 与完整生产栈 smoke 仍受 EVO-118-E / DEPLOY-01 的 Embedded Frontend 构建收敛约束；EVO-118-D 不通过改变前端交付形态规避该 Gate，PR required CI 也不再让该诊断覆盖 DATA-01 结论。

### 既有 Unreleased 记录

- 建立脚本发布说明制度。后续脚本行为变更需要记录用途、影响范围、验证方式和注意事项。
- **`scripts/dev.sh`**：前端本地启动从旧 Next.js `localhost:3000` 调整为 Vite + Bun 默认 `localhost:3001`，使用 `bun install` / `bun run dev`，支持通过 `FRONTEND_PORT` 覆盖；状态输出和 ready banner 同步使用该端口。验证：脚本语法检查和前端构建。
- **`backend/Dockerfile`**：生产运行时镜像（`debian:bookworm-slim`）安装 `git`。Smart HTTP 端点（`git-upload-pack` / `git-receive-pack`）通过 `git` CLI 子进程执行，运行时必须安装 `git`。

## v0.2.0 — dev.sh 嵌入式前端死代码清理 + 单端口 lite 模式 (2026-06-03)

### 概述

Iteration 031 已将前端迁移到 `rust-embed-for-web`（debug 模式读 `frontend/dist/` 文件系统，release 模式编译时嵌入），但 `dev.sh` 仍残留旧 ZIP 方案的死代码，且 `lite` 模式无条件启动 Vite dev server 导致多余端口。本次清理死代码并将 `lite`/`embedded` 模式改为单端口启动。

### 变更

- **`scripts/dev.sh`**：删除 `EMBEDDED_FRONTEND` 环境变量——`rust-embed-for-web` 内置 debug/release 切换，无需外部控制。
- **`scripts/dev.sh`**：删除旧 `build_frontend_zip()` 和不存在的 `embedded-frontend` Cargo feature 路径。
- **`scripts/dev.sh`**：新增 `build_frontend()`；`lite`/`embedded` 使用单端口 Backend 提供 API 与静态资源。
- **`scripts/dev.sh`**：`start` 模式保留 Vite HMR 双端口，并读取 `SERVER__PORT`。

### 验证

- `bash -n scripts/dev.sh` ✓
- `SERVER__PORT=8090 ./scripts/dev.sh lite` ✓
- 首屏、CSS、JS、`/health/live` 与 stop 流程 ✓

### 注意事项

- `start` 模式仍启动 Vite dev server，用于 HMR。
- `frontend/dist/` 必须在后端启动前存在；`build_frontend()` 负责生成。

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
