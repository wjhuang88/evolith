# EVO-118-D Git 存储持久化、备份与恢复演练

- **类型**：Technical / Data Durability / Deploy
- **状态**：Done / Complete / Merged
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A/B/C Done；PR #5 / #6 merged；SEC-01/SEC-02 Closed
- **影响范围**：deploy / backend / scripts / docs / tests
- **所属 Iteration**：[Iteration 053](../../iterations/ITERATION-053.md)（Closed / Complete）
- **实施分支**：`agent/evo-118-d-git-durability-recovery`
- **审核入口**：[Navigator Review Packet](../../review/EVO-118-D-navigator-review.md)

## 工程目标

确保容器重建、节点故障或数据库恢复后，Git Repo、Commit、Branch、Tag 和数据库元数据能够一起恢复，避免“数据库可恢复但代码历史永久丢失”。

## 激活与基线同步事实

2026-08-02 首次激活时已验证：

- 当时 `main` 为 `711e63227aeb5089a1858b6f38ef0c06374a7f97`，包含 PR #5 与 post-merge PR #6。
- 当时没有开放或重叠的 EVO-118-D / Iteration 053 实现；Iteration 018/019/020/025/026/027 已记录继续阻塞、替代或延期 disposition。
- [Iteration 053](../../iterations/ITERATION-053.md) 已在运行时代码之前建立 Planned 基线并激活，本 Story 是 DATA-01 owner。
- EVO-118-E 当时保持 Ready，不与本 Story 同时实施；按 ADR-0010 现作为最终发布 Gate。
- 当时生产 Compose 未为 Backend 挂载 Git Storage，`scripts/backup.sh` 仅覆盖 PostgreSQL，readiness 未检查 Git Storage。

2026-08-02 主线漂移复核：

- 当前 `main` 已推进到 `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`；PR #8 合入 EVO-112-A Repo UI Shell，PR #9 写回 Iteration 054 merge closure。
- Iteration 054 已 Closed / Complete，不是 Active/Review WIP，不替代或关闭 Iteration 053；EVO-112-B 后续等待 F/G/H 直接边界稳定。
- PR #7 仍是唯一开放 PR，保持 Draft；漂移前 Head `608ad6acd20229498898eaf16afaff6ec8a79878` 落后 `main` 4 个提交且不可合并。
- 本 Story 采用“从最新 `main` 建立新基线并迁移 DATA-01 改动”的方式跟进，必须保留 Repo UI 与 Iteration 054 的已合并事实，不得回滚主线。
- 基线同步后所有 exact-head CI、恢复演练与 Navigator 证据必须重新建立；旧 Head 的通过项只能作为定位线索。

## 必读文档

- [Iteration 053](../../iterations/ITERATION-053.md)
- [Navigator Review Packet](../../review/EVO-118-D-navigator-review.md)
- [Release SOP](../../sop/RELEASE.md)
- [Security Review SOP](../../sop/SECURITY-REVIEW.md)
- [Start Iteration SOP](../../sop/START-ITERATION.md)
- [Task Closure SOP](../../sop/TASK-CLOSURE.md)
- [Production Readiness Baseline](../../reference/PRODUCTION-READINESS-BASELINE.md)
- [Production Readiness Plan](../../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)
- [Architecture](../../reference/ARCHITECTURE.md)
- [Config Reference](../../reference/CONFIG.md)
- [Scripts Release Notes](../../reference/SCRIPTS-RELEASE-NOTES.md)

## 已确认失败模式

- 生产 Compose 没有为 `GIT_STORAGE__BASE_PATH` 挂载持久卷。
- 当前备份脚本只执行 PostgreSQL `pg_dump`，不包含 Git 对象和 Ref。
- Readiness 不检查 Git 存储目录可读写。
- 缺少恢复演练、容量告警和 DB/Git 一致性盘点。
- 备份与恢复如果只验证文件存在，可能出现 DB 元数据与 Git Ref/对象不一致但仍被报告成功。
- 容器重建、权限错误、只读文件系统、磁盘/inode 耗尽等故障缺少 fail-closed 证据。

## 验收场景

### Scenario 1：容器重建不丢仓库

- **Given** Repo 已 push 多个 Commit、额外 Branch 与 Tag
- **When** 删除并重建 Backend 容器
- **Then** clone、Commit、Branch、Tag 和 Repo Metadata 保持一致

### Scenario 2：联合备份可恢复

- **Given** PostgreSQL 与 Git Storage 已产生业务数据并进入明确一致性窗口
- **When** 执行联合备份、清空隔离环境并恢复
- **Then** 用户可重新登录、列出 Repo、clone 并校验 Commit/Branch/Tag/目标 SHA

### Scenario 3：Git 存储不可用阻止就绪

- **Given** Git Storage 只读、路径不存在、不是目录、不可读或不可写
- **When** 调用 readiness
- **Then** readiness 返回 503；存储恢复后返回 200

### Scenario 4：DB/Git 不一致可被识别

- **Given** 数据库存在 Repo 元数据但 Git 目录缺失，或 Git 目录存在但数据库记录缺失
- **When** 执行盘点或恢复验收
- **Then** 工具返回非零/明确失败并列出不一致，不把部分恢复报告为成功

### Scenario 5：备份产物可追溯且可拒绝不完整输入

- **Given** 备份包含数据库、Git 数据、格式版本、非敏感配置元数据和校验信息
- **When** 缺少任一必需部分、checksum 不匹配、版本不兼容或目标非空
- **Then** restore 在写入目标前 fail closed，不覆盖现有环境，不报告恢复完成

### Scenario 6：中途失败不破坏原环境

- **Given** restore 使用 staging 路径
- **When** DB、Git 解包、校验或对账任一阶段失败
- **Then** 返回非零并清理 staging；对已通过空目标门禁且由本次 restore 写入的目标执行完整 rollback-to-empty，回滚不完整必须输出 `CRITICAL`

## 工程要求

- Compose/K8s 显式配置 `GIT_STORAGE__BASE_PATH` 和持久 Volume。
- 备份同时覆盖 PostgreSQL、Git Repo 目录和必要版本/配置元数据。
- 定义备份一致性策略：维护窗口、快照或经过证明的其他边界；不得默认认为顺序 `pg_dump` + `tar` 天然一致。
- 提供 restore 脚本或明确 SOP，并执行空环境恢复演练。
- 增加 Repo/DB 对账工具或至少可重复的盘点命令。
- 增加磁盘容量、inode、备份失败和恢复失败告警基线。
- Readiness 检查 Git 路径存在、为目录、可创建/写入/sync，关闭写句柄后独立 reopen/read/精确比对，并清理探针；失败返回 503。
- 所有脚本使用明确退出码，失败不继续覆盖，敏感信息不进入归档或普通日志。
- 修改脚本时同步 `SCRIPTS-RELEASE-NOTES.md`。
- 涉及 SQLite/PostgreSQL 行为时明确双轨影响；本 Story 不以只验证 PostgreSQL 配置替代 Lite 路径回归。

## 推荐实施切片

1. **事实基线与失败测试**：证明容器重建丢 Git、backup 缺 Git、readiness 不识别不可用路径。
2. **持久卷和配置收敛**：生产 Compose/K8s 路径与 Volume 显式一致。
3. **Readiness fail closed**：路径缺失、只读/不可写、独立读取失败或内容不一致时返回 503；不与 EVO-118-G 的其他 readiness/依赖范围混淆。
4. **联合 backup/restore**：定义一致性策略、manifest/checksum、退出码和安全恢复顺序。
5. **对账与演练**：DB/Git inventory、空环境恢复、Commit/Branch/Tag 校验、重建后 clone。
6. **运营与文档**：容量/inode/失败告警基线、Release SOP、配置和脚本 release notes。
7. **Navigator 复验**：重点验证数据损坏、部分恢复、假成功、失败覆盖和实际恢复证据。

## 不做事项

- 不在本 Story 实现多地域复制、Git LFS、对象存储后端或多副本共享存储架构。
- 不宣称本地持久卷自动支持多实例；多实例仍需共享存储或 Repo affinity 设计。
- Repo 创建/删除业务状态机归 EVO-118-F。
- 不在本 Story 全面处理 Embedded Frontend 构建收敛；归 EVO-118-E。
- 不把所有 readiness、限流、SMTP/Redis fail-closed 合并进来；归 EVO-118-G。
- 不提前实施 EVO-112-B 或 Agent 写入；已合并的 EVO-112-A Repo UI Shell 必须保留。

## 最小验证

- `docker compose ... up` → push Commit/Branch/Tag → 重建 backend → clone/refs 校验。
- DB+Git 备份 → 清空隔离环境 → restore → 登录/list/clone/Commit/Branch/Tag/SHA 校验。
- Git 路径缺失、只读、权限不足、独立读取失败和内容不一致的故障测试。
- readiness 故障 503、恢复后 200，且失败路径尽最大努力清理探针。
- DB-only、Git-only、损坏归档、checksum/version 失败、非空 `public`、非 `public` 用户 schema，以及仅含空 Publication 的数据库级非空目标拒绝测试。
- 含非 `public` 用户 schema/table 与数据库级 Publication 的 backup 在 post-write failure 后完整 rollback-to-empty。
- backup/restore/inventory 非零退出码、无 restore success 假阳性及回滚不完整 `CRITICAL` 语义。
- 真实 Subscription credential sentinel 必须在 source catalog 中存在，但解开的最终联合归档、解压后的 `database.sql`、manifest、refs、checksum 与普通 stdout/stderr 中均不得出现；dump 不得包含 `CREATE SUBSCRIPTION`，Publication 仍需恢复。
- hostile `PSQLRC` 动态测试必须实际执行 restore 并命中 non-empty pre-write database gate；原 marker 与完整 schema snapshot 不变，`public.psqlrc_sentinel` 不得创建，Git 保持空，不进入 PostgreSQL restore、Git installation 或 inventory phase。
- Frontend required gate、Rust fmt/check/clippy/workspace tests、生产 Compose clean build（按实际改动范围）。
- Markdown 链接、`git diff --check`、exact-head CI 与 Navigator 独立复验。
- 最新 `main` 上 Repo UI type-check/build 不回归。

## 实施与验证证据

- 持久化：生产 Compose、K8s 与 Backend runtime 路径统一为 `/var/lib/evolith/git`，并使用显式持久卷；未宣称本地卷支持多实例共享。
- Readiness：PostgreSQL 与 Git Storage 联合判定；Head `7608be9f5e7234c0797e4aeba23a133ca91552dc` 已实现 create/write/sync/close/reopen/read/精确比对/cleanup，第二次独立 Navigator 已明确判定该 blocker **Resolved**。
- Backup/Restore：维护窗口前提、版本化 manifest/checksum、PostgreSQL + Git 联合归档、staging-first restore、空目标保护、路径/类型/版本/checksum/bare repo/refs 校验和失败回滚；第一次复验发现的非 `public` schema 缺陷及第二次复验发现的 database-level Publication 对象面均已进入统一共享空库 helper、rollback cleanup 与专项负向矩阵。生产 dump 显式使用 `--no-owner --no-privileges --no-subscriptions`：Publication 仍属于 DATA-01 数据面，Subscription conninfo 属于环境级敏感配置，不进入普通联合归档，必须由独立、安全、受控的运维流程重建；共享空库 helper 仍检测 Subscription，含 Subscription 的目标不得被静默覆盖。
- Inventory：识别 DB-only、Git-only、重复/异常 UUID 布局、无效 bare repo、缺默认分支或最后 Commit，并以非零退出阻止假成功。
- 应用恢复：真实 Backend 完成 register/create/Smart HTTP push、联合备份、空环境恢复、login/list/clone/refs、readiness 故障注入和 Backend 重启后再次 clone。
- 容器重建：真实 Backend 容器在同一 PostgreSQL 与命名 Git volume 上删除/重建后，login/list/clone/Branch/Tag/SHA 均保持。
- 数据库兼容：修复 PostgreSQL migration 005 将文本 plan ID 声明为 UUID 的缺陷，与 SQLite/API 文本 ID 契约对齐。
- 演练缺陷修复：状态变更请求遵循 CSRF 双提交协议；CI-only runtime image 使用 UID/GID 10001，避免 Ubuntu 基础镜像 UID 1000 冲突。
- 被审核旧 Head `c8b83dec54c1ed7df75c29b34b3d1b3a0f40a885` 的 `ci` run `30756053633`（#177）与 `data-durability-container` run `30756053641`（#23）均成功，但未覆盖 Navigator 发现的两个失败路径。
- 任何整改提交都会使上述旧 Head CI 失效；只有整改最终 Head 的 required exact-head CI 和独立 Navigator re-review 可作为下一步验收证据。

## Navigator 阻塞与整改

2026-08-03 独立 Navigator 对 Head `c8b83dec54c1ed7df75c29b34b3d1b3a0f40a885` 给出 `Blocked`：

1. PostgreSQL 目标空判断及失败 rollback 只处理 `public`，可能接受含 `legacy.marker` 的非空目标，或在含非 `public` 备份对象的 post-write failure 后留下未报告的部分数据库。
2. Git Storage readiness 没有关闭写句柄后重新打开并读取、精确比对探针内容，可能在读取路径失败时错误返回 200。

整改必须补充：统一用户数据库空检查、全用户 schema/object rollback-to-empty、非 `public` 拒绝与 rollback 负向测试、readback/read-error/mismatch/cleanup 测试，以及最新 Head 的两条 required workflow。独立 Navigator re-review 前不得转 Ready、合并或关闭 DATA-01。

2026-08-03 独立 Navigator 对整改 Head `7608be9f5e7234c0797e4aeba23a133ca91552dc` 再次给出 `Blocked`：

1. Git Storage readiness 的独立 reopen/read/compare 与失败清理已被判定 **Resolved**。
2. 非 `public` schema 的目标拒绝和 rollback 已被判定对 schema-scoped state 有效。
3. 唯一剩余 blocker 是数据库级 PostgreSQL 对象不在空库与 rollback 契约内：生产 `pg_dump` 为数据库级 dump，可携带空 Publication；仅含该 Publication 的目标会被错误视为 `0`，恢复失败后的 Publication 也可能残留而不触发 `CRITICAL`。

本轮整改必须让共享 helper、restore cleanup 与生产 `pg_dump` 的对象范围一致，并增加 Publication-only preflight 拒绝、Publication restore/rollback、helper 前后计数和既有 incomplete-rollback `CRITICAL` 回归。Head `7608be9...` 的 `ci` #191 / run `30797578993` 与 `data-durability-container` #37 / run `30797579026` 自本轮提交起仅为历史证据。

整改实现 Head `75f7868b8d14fe2ac95132643289620d2c50be89` 已完成上述对象面收敛：共享 helper 与 rollback cleanup 新增 Publication、Subscription、Event Trigger、Extension、Large Object、FDW/Server/User Mapping、非内置 Language/Cast/Transform/Access Method；新增 Publication 专项矩阵已证明生产 backup/restore 携带 Publication、Publication-only 目标写前拒绝、post-write failure 后 Publication/schema/Git 全量回滚及 helper=`0`。`ci` #195 / run `30830738917` 与 `data-durability-container` #41 / run `30830734213` 为实现切片证据；治理同步提交改变 Head 后必须再跑最终 exact-head CI。

2026-08-03 第三次独立 Navigator 对 Base `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`、Head `551e71331a579020a0037c3db2a7ccd98c781c05` 给出 `Blocked`。此前非 `public`、Publication、rollback `CRITICAL` 与 Git Storage readback finding 均被明确判定 **Resolved**；本轮只剩两个新 blocker：生产超级用户 `pg_dump` 未排除可能携带明文 conninfo 的 Subscription，以及 restore/inventory 非交互 `psql` 未使用 `-X`、可能执行环境 `psqlrc`。该 Head 的 `ci` #197 / run `30832407367` 与 `data-durability-container` #43 / run `30832407357` 仅为旧实现矩阵证据，不覆盖 credential sentinel 与 hostile `PSQLRC`。

本轮整改范围固定为：生产 `pg_dump --no-subscriptions`；Subscription 环境级敏感配置边界；真实 `connect=false` Subscription sentinel 解包/解压泄漏测试；Subscription-only 目标拒绝；production `psql -X` 统一 wrapper；hostile `PSQLRC` preflight 动态测试；保留 Publication、非 `public`、post-write rollback、incomplete rollback `CRITICAL`、readiness 与完整 required matrix。不得扩展为加密备份或 EVO-118-E。

## 最终独立复验与合并收口

- 最终 Base：`38c19b19cff5aab7a08ac40a1cf417e1712e1b07`。
- 最终 exact Head：`158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`。
- Required workflows：`ci` #204 / run `30838250911` 与 `data-durability-container` #50 / run `30838250875` 均为 `success`，并验证 exact Head、Subscription credential sentinel、hostile `PSQLRC`、Publication/非 `public` rollback、`CRITICAL`、readiness、应用恢复与真实 Backend 容器重建矩阵。
- 独立 Navigator 于 2026-08-04 对最终 exact Head 返回 `Complete`，Blocking findings 为 None；Subscription credential archive boundary 与 ambient `psqlrc` 两项 finding 均判定 Resolved。
- PR #7 已转 Ready，并于 2026-08-04 以 squash 方式合并到 `main`；merge commit：`932def05717b678f6f44dc23f137933d56158957`。
- DATA-01 已关闭；本 Story 为 `Done / Complete / Merged`，Iteration 053 为 `Closed / Complete`。
- EVO-118-E 当时为 `Ready / Not Started`，只是下一候选，不在本次收口中自动启动；现已按 ADR-0010 后移为最终发布 Gate。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 DATA-01，使 Git 目录、数据库元数据和备份恢复成为可验证的联合耐久性边界 |
| 产物 | 持久卷/配置、readiness、backup/restore、对账、故障测试、恢复证据、SOP/Release Notes、Navigator review packet |
| 状态同步归口 | EVO-118-D、Iteration 053、EVO-118 Epic、Product Backlog、Board、Baseline、Config/Release SOP/Scripts Release Notes |
| 验证证据 | 容器重建、空环境恢复、Commit/Branch/Tag、DB/Git 对账、只读/缺失/读取失败故障、required exact-head CI 与独立 Navigator 结论 |
| 残余工作归口 | Repo lifecycle → EVO-118-F；部署构建 → EVO-118-E；综合 runtime gates → EVO-118-G；多实例共享存储另行评估 |

## 当前执行状态

- Iteration 053：Closed / Complete。
- PR #7：Merged；final Head `158ba98fb2d1e33fe5821f2e75431e86a5cf6ffd`，merge commit `932def05717b678f6f44dc23f137933d56158957`。
- Navigator：Complete；最终 exact Head 无 blocking finding。
- DATA-01：Closed；联合 PostgreSQL + Git durability/recovery Gate 已解除。
- EVO-118-E：最终发布 Gate；不在本 PR 中提前实施。

## 解锁内容

完成全部验收并经最新 exact-head CI、独立 Navigator 和恢复演练关闭后，解除 DATA-01 Gate；允许 Evolith 进入外部 Alpha 的数据耐久性评估，并为 Repo 生命周期一致性提供持久存储基础。
