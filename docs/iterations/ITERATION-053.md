# Iteration 053 — Git Storage Durability and Recovery

- **状态**：Active / Awaiting independent Navigator
- **计划基线**：Planned baseline established on 2026-08-02 before runtime/deploy/script implementation
- **目标 Story**：[EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md)
- **父 Epic**：[EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
- **Gate**：DATA-01
- **实施分支**：`agent/evo-118-d-git-durability-recovery`
- **Driver**：GPT-5.6 Thinking
- **Navigator**：独立复验，结论不得由 Driver 或 CI 代替
- **审核入口**：[EVO-118-D Navigator Review Packet](../review/EVO-118-D-navigator-review.md)

## 1. 迭代目标与用户价值

建立 PostgreSQL 与 Git Storage 的联合耐久性边界，使用户在 Backend 容器重建、存储故障和空环境恢复后，仍可登录、列出仓库、clone 并验证 Commit、Branch、Tag、Ref 与 Repo Metadata；不完整备份、校验失败、存储不可用或 DB/Git 不一致必须 fail closed，不能报告部分成功。

## 2. 启动事实基线

- 2026-08-02 首次启动时重新读取 GitHub：`main` 为 `711e63227aeb5089a1858b6f38ef0c06374a7f97`。
- PR #5 和 PR #6 已合并；当时无开放 PR、Draft PR 或相关 Issue。
- 当时未发现 `EVO-118-D`、`118-d`、`durability`、`backup`、`restore` 或 `Iteration 053` 的重叠实现分支/PR。
- 当时没有 Active / In Progress / Review Iteration。
- 生产 Compose 仅挂载 PostgreSQL/Redis volume，Backend 未挂载 Git Storage。
- `scripts/backup.sh` 只生成 PostgreSQL `pg_dump` 压缩文件，不包含 Git objects/refs、manifest、checksum 或 restore。
- 当前生产 readiness 未提供直接服务 DATA-01 的 Git Storage 可用性证据。
- EVO-118-E 保持 Ready，不与本迭代并行实施。

## 3. 非终态库存 disposition

| Iteration | disposition | 本轮影响 |
|-----------|-------------|----------|
| 018 | 继续 `Blocked for activation / Superseded direction`；旧 Skill 导入方向由 Git-centric 主线替代 | 不阻塞 053 |
| 019 | 继续 `Blocked for activation / Superseded direction`；无 deliberate replan 不激活 | 不阻塞 053 |
| 020 | 继续 `Blocked for activation / Superseded direction`；版本能力由 Git 历史/Indexer 主线承接 | 不阻塞 053 |
| 025 | 继续 `Blocked for activation`；需 refinement，且不得抢占 EVO-118 S1 | 不阻塞 053 |
| 026 | 继续 `Blocked for activation`；计费/Webhook 需独立安全与 Mock 验收 | 不阻塞 053 |
| 027 | 继续 `Blocked for activation / Superseded direction`；由 EVO-108/109 承接 | 不阻塞 053 |

## 4. 数据资产、故障源、入口与信任边界

| 项目 | 内容 |
|------|------|
| 受保护资产 | Git objects、Commit、Branch、Tag、Ref、Repo 目录；用户、租户、权限和 Repo Metadata；备份 manifest/checksum |
| 攻击者/故障源 | 被攻陷运维脚本、错误配置、容器重建、权限漂移、只读/满盘/inode 耗尽、损坏或不完整归档、人工误恢复 |
| 入口 | `docker-compose.prod.yml`、K8s volume、`GIT_STORAGE__BASE_PATH`、readiness、backup/restore/inventory 脚本、Git Smart HTTP |
| 信任边界 | Backend→Git Storage；Backend→PostgreSQL；Backup tool→DB/Git；Restore staging→目标环境；Operator→生产数据 |
| 安全默认 | 任一必需部分缺失、校验失败、路径不可用或对账不一致时非零退出/503；不覆盖现有非空目标，不报告部分成功 |
| 证据 | 失败优先测试、容器重建、联合备份、空环境恢复、Git 行为校验、inventory、exact-head CI、Navigator 复验 |

## 5. 已确认失败模式

1. Backend 容器使用可写层保存 Git 数据，重建可能永久丢失代码历史。
2. 数据库可恢复但 Git objects/refs 未进入备份。
3. 顺序执行 `pg_dump` 与 `tar` 若无维护窗口/停写边界，可能形成时间窗口不一致。
4. readiness 只证明进程或 DB 存活，Git 路径缺失、非目录、不可读写仍可能报告健康。
5. DB-only、Git-only、损坏归档、checksum/version 不匹配可能被误当可恢复输入。
6. restore 直接覆盖非空目标或中途失败后留下半恢复状态。
7. DB 有 Repo 而 Git 目录缺失，或 Git 目录孤立于 DB，当前缺少非零对账门禁。
8. 脚本管道/子命令失败后仍可能继续或输出“Done”。
9. manifest/log 误写数据库密码、JWT Secret、API Key 或其他凭证。

## 6. BDD 验收场景

### Scenario A：容器重建保持 Git 历史

- **Given** PostgreSQL 与 Git Storage 已创建用户、租户和 Repo，并 push 多个 Commit、额外 Branch 与 Tag
- **When** 重建 Backend 容器
- **Then** 登录、Repo list、clone、Commit/Branch/Tag/目标 SHA 与 Metadata 均保持一致

### Scenario B：联合备份在空环境恢复

- **Given** 已进入明确维护窗口并停止写入
- **When** 生成包含 DB、Git、manifest、版本和 checksum 的联合备份，并恢复到空环境
- **Then** restore 仅在全部校验通过后切换目标；登录、list、clone 和 Git refs 校验成功

### Scenario C：不完整或损坏输入拒绝

- **Given** DB-only、Git-only、缺 manifest、文件损坏、checksum 不匹配或不兼容版本备份
- **When** 执行 restore
- **Then** 在写入目标前拒绝，返回非零，不覆盖现有环境，不输出成功

### Scenario D：Git Storage 不可用时 readiness fail closed

- **Given** 路径不存在、不是目录、不可读、不可写或无法创建必要临时文件
- **When** 请求 readiness
- **Then** 返回 503；恢复存储后返回 200

### Scenario E：DB/Git 不一致可盘点

- **Given** DB-only Repo、disk-only Repo、异常路径、无效 bare repo、缺 Ref/Commit/Tag 或重复目录
- **When** 执行 inventory
- **Then** 输出可定位结果并返回非零，不把 warning 当成功

### Scenario F：restore 中途失败不破坏原环境

- **Given** 目标环境非空或故障注入发生在 DB/Git staging 恢复过程中
- **When** 执行 restore
- **Then** 默认拒绝非空目标；中途失败只清理 staging，原目标保持可用

## 7. 测试与故障注入矩阵

| 切片 | 成功证据 | 失败证据 |
|------|----------|----------|
| Compose/K8s volume | Backend 重建后 clone/refs 不丢 | 移除挂载可复现丢失风险 |
| Git readiness | 可读写目录 200 | missing/not-dir/read-only/unwritable 503 |
| Backup | DB+Git+manifest+checksum 完整，退出 0 | 任一命令失败、停写边界缺失或归档不完整时非零 |
| Restore | 空目标 staging 恢复并通过对账 | DB-only/Git-only/corrupt/checksum/version/non-empty/mid-failure 拒绝 |
| Inventory | 完整环境退出 0 | DB-only/disk-only/invalid bare/missing ref 等退出非零 |
| 双数据库回归 | SQLite workspace tests 无无关回归 | PostgreSQL 生产路径需真实演练证据 |
| 凭证保护 | manifest/log 仅含非敏感配置 | Secret grep 或归档检查发现敏感字段即失败 |

## 8. 计划实现切片

1. 建立失败优先证据和脚本级自动测试。
2. 显式收敛 `GIT_STORAGE__BASE_PATH`、目录初始化、运行用户和生产 volume。
3. 增加仅服务 DATA-01 的 Git Storage readiness 门禁。
4. 实现维护窗口/停写前提下的联合 backup，生成版本化 manifest 和 checksum。
5. 实现 staging-first restore，校验全部输入后才写入空目标。
6. 实现 DB/Git inventory 与恢复后验收。
7. 完成真实空环境演练、Backend 重建复验和凭证泄露检查。
8. 同步 CONFIG、RELEASE、SCRIPTS-RELEASE-NOTES、Baseline、Story/Epic/Backlog/Board/索引。
9. Driver 完成后由 Navigator 独立复验；门禁全部通过前 PR 保持 Draft。

## 9. 不做事项

- 不实施 EVO-118-E Embedded Frontend/生产构建收敛。
- 不实施 EVO-118-F 的完整 Repo create/delete 状态机、补偿和 Reconciler。
- 不实施 EVO-118-G 的完整 runtime reliability、限流、SMTP/Redis。
- 不实现多地域复制、Git LFS、对象存储 Git Backend 或多副本共享存储。
- 不实现 EVO-112-B Repo Detail、Commit/Promote、Agent Session、Webhook、Indexer 或 Durable Outbox；已合并的 EVO-112-A 必须保留。

## 10. 风险与回滚

- **一致性风险**：联合备份采用明确维护窗口/停写前提；未满足时 backup 拒绝或标记不可恢复，不宣称在线一致快照。
- **覆盖风险**：restore 默认仅接受空目标，使用 staging；任何校验/恢复/对账失败不切换目标。
- **权限风险**：容器运行用户必须可访问固定 volume；回滚保留旧镜像与原 volume，不删除原数据。
- **兼容风险**：manifest 版本不兼容时 fail closed；格式升级必须新增版本路径。
- **范围风险**：发现生命周期、构建或综合 readiness 问题分别归 EVO-118-F/E/G。

回滚：停止新写入，回退镜像/配置，重新挂载原 Git volume 与 PostgreSQL volume，执行 readiness、inventory、登录/list/clone/refs 验证；不得用未校验备份覆盖原环境。

## 11. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 DATA-01，形成可验证的 PostgreSQL + Git 联合耐久性、备份与恢复边界 |
| 产物 | 持久卷/配置、readiness、backup/restore/inventory、故障测试、空环境演练证据、运营与稳定文档、Navigator review packet |
| 状态同步归口 | EVO-118-D、Iteration 053、EVO-118 Epic、Product Backlog、Board、docs map、Iteration index、Production Readiness Baseline |
| 验证证据 | frontend required gates；Rust fmt/check/clippy/tests；Compose clean build；重建持久性；联合恢复；故障注入；Markdown links；diff check；exact-head CI；独立 Navigator |
| 残余工作归口 | lifecycle→EVO-118-F；build→EVO-118-E；综合 runtime gates→EVO-118-G；多实例共享存储另行评估 |

## 12. 第一条执行记录

- 2026-08-02：重新建立 GitHub 事实基线，确认前置 PR 已闭环、无开放/重叠工作；读取治理、发布、安全、架构、配置与脚本基线；记录历史非终态 Iteration disposition；建立本 Planned 基线并激活 EVO-118-D。治理激活提交存在前未提交运行时代码、部署或脚本实现。

## 13. 执行与验证记录

- 2026-08-02：治理激活提交 `c2f25bbbf7c157c0fe73f533fb0879cc58c1a18f` 先于运行时代码；创建 Draft PR #7。
- 2026-08-02：旧基线 Head 上实现 Git Storage readiness、Compose/K8s volume、联合 backup/restore/inventory 与恢复测试；CI #142 证明联合恢复矩阵及跨容器 volume clone/refs 可执行，但因 rustfmt 失败未形成完整门禁。
- 2026-08-02：后续旧 Head CI #143 已通过 fmt/check/clippy，并进入 SQLite workspace tests；这些事实仅证明切片可执行，不是新主线 exact-head 证据。
- 2026-08-02：`main` 随 PR #8 / #9 推进至 `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`，新增 EVO-112-A Repo UI Shell 与 Iteration 054 Closed / Complete 事实。PR #7 漂移前 Head `608ad6acd20229498898eaf16afaff6ec8a79878` 落后 4 个提交且不可合并。
- 2026-08-02：执行基线同步 disposition：从最新 `main` 建立临时迁移分支，保留 Repo UI/Iteration 054，迁移 DATA-01 授权改动；旧 Head CI、恢复演练与 mergeability 全部降级为定位线索，必须在新 Head 重跑。
- 2026-08-02：真实空环境恢复演练发现 PostgreSQL migration 005 将文本 `plan-free` 等 ID 声明为 UUID；提交 `994f192fc4cad92cc7e2e5c10bd7566722826cfd` 将 PostgreSQL plan ID 与 SQLite/API 文本契约对齐。
- 2026-08-02：增加真实 Backend 容器删除/重建演练及独立 workflow；容器共享同一 PostgreSQL 与命名 Git volume，验证 register/create/push、删除首容器、第二容器 login/list/clone/Branch/Tag/SHA。
- 2026-08-02：exact-head 失败定位出两个演练缺陷：状态变更请求未满足 CSRF 双提交协议；Ubuntu 24.04 基础镜像 UID 1000 冲突。最终修复使用 cookie jar + `X-CSRF-Token`，并采用 UID/GID 10001。
- 2026-08-02：实现 Head `83351a2375b6537ddb83d71d84a2d22bfaaacc15` 的 `ci` run `30736864612` / #157 成功；frontend install/type-check/build、脚本语法、Compose mapping、联合备份恢复矩阵、通用 volume recreation、fmt/check/clippy、SQLite workspace tests、应用级恢复演练与 clean-build diagnostic 均通过。
- 2026-08-02：同一实现 Head 的 `data-durability-container` run `30736864631` / #3 成功，真实 Backend 容器删除/重建演练通过。
- 2026-08-02：提交 [Navigator Review Packet](../review/EVO-118-D-navigator-review.md)，明确当前 Driver 不得代替独立 Navigator；该文档及后续治理提交改变 Head，因此最终 Head 必须重新跑 required CI。
- 2026-08-02：Owner Story 已更新为 `In Progress / Awaiting independent Navigator`；DATA-01 保持 Open，EVO-118-E 保持 Ready 且未启动。

后续只追加实际执行事实、commit/head、命令结果、故障注入、Navigator 结论和偏差；不得改写以上 Planned 基线。

## 14. Review / Retrospective

### Driver review

- 实现切片、负向矩阵、真实 Git Smart HTTP 行为、容器重建、空环境恢复和 exact-head CI 已完成。
- Driver 发现并修复 fresh PostgreSQL migration、CSRF 演练和 runtime-image UID 三类缺陷。
- 当前没有已知未修复的 DATA-01 blocking defect；该陈述只是 Driver 状态，不是独立接受结论。

### Independent Navigator

- 状态：Pending。
- 审核目标：最新 PR Head，而不是实现 Head `83351a2` 或任一旧 Head。
- 审核入口：[EVO-118-D Navigator Review Packet](../review/EVO-118-D-navigator-review.md)。
- 必须独立检查数据损坏、部分恢复、假成功、路径/归档安全、凭证泄露、readiness、真实容器重建和范围边界。
- Navigator 发现 blocking finding 时，本 Iteration 继续 Active，PR 保持 Draft，修复后重跑 exact-head CI。
- Navigator 无 blocking finding 且最新 exact-head CI 全绿后，才允许进入 Ready/merge 流程；合并后仍需独立治理 closeout。

最终只能记录 `Complete`、`Partial` 或 `Blocked`。当前尚未形成最终结论，PR 合并也不自动等于 DATA-01 或 Iteration 关闭。
