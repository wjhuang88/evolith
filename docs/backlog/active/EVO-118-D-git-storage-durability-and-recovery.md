# EVO-118-D Git 存储持久化、备份与恢复演练

- **类型**：Technical / Data Durability / Deploy
- **状态**：In Progress / Awaiting independent Navigator
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A/B/C Done；PR #5 / #6 merged；SEC-01/SEC-02 Closed
- **影响范围**：deploy / backend / scripts / docs / tests
- **所属 Iteration**：[Iteration 053](../../iterations/ITERATION-053.md)（Active）
- **实施分支**：`agent/evo-118-d-git-durability-recovery`
- **审核入口**：[Navigator Review Packet](../../review/EVO-118-D-navigator-review.md)

## 工程目标

确保容器重建、节点故障或数据库恢复后，Git Repo、Commit、Branch、Tag 和数据库元数据能够一起恢复，避免“数据库可恢复但代码历史永久丢失”。

## 激活与基线同步事实

2026-08-02 首次激活时已验证：

- 当时 `main` 为 `711e63227aeb5089a1858b6f38ef0c06374a7f97`，包含 PR #5 与 post-merge PR #6。
- 当时没有开放或重叠的 EVO-118-D / Iteration 053 实现；Iteration 018/019/020/025/026/027 已记录继续阻塞、替代或延期 disposition。
- [Iteration 053](../../iterations/ITERATION-053.md) 已在运行时代码之前建立 Planned 基线并激活，本 Story 是 DATA-01 owner。
- EVO-118-E 保持 Ready，不与本 Story 同时实施。
- 当时生产 Compose 未为 Backend 挂载 Git Storage，`scripts/backup.sh` 仅覆盖 PostgreSQL，readiness 未检查 Git Storage。

2026-08-02 主线漂移复核：

- 当前 `main` 已推进到 `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`；PR #8 合入 EVO-112-A Repo UI Shell，PR #9 写回 Iteration 054 merge closure。
- Iteration 054 已 Closed / Complete，不是 Active/Review WIP，不替代或关闭 Iteration 053；EVO-112-B 仍暂停等待 EVO-118 S1。
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

- **Given** Git Storage 只读、路径不存在、不是目录或不可写
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
- **Then** 返回非零并清理 staging，原目标保持不变

## 工程要求

- Compose/K8s 显式配置 `GIT_STORAGE__BASE_PATH` 和持久 Volume。
- 备份同时覆盖 PostgreSQL、Git Repo 目录和必要版本/配置元数据。
- 定义备份一致性策略：维护窗口、快照或经过证明的其他边界；不得默认认为顺序 `pg_dump` + `tar` 天然一致。
- 提供 restore 脚本或明确 SOP，并执行空环境恢复演练。
- 增加 Repo/DB 对账工具或至少可重复的盘点命令。
- 增加磁盘容量、inode、备份失败和恢复失败告警基线。
- Readiness 检查 Git 路径存在、为目录、可读写和必要目录可创建；失败返回 503。
- 所有脚本使用明确退出码，失败不继续覆盖，敏感信息不进入归档或普通日志。
- 修改脚本时同步 `SCRIPTS-RELEASE-NOTES.md`。
- 涉及 SQLite/PostgreSQL 行为时明确双轨影响；本 Story 不以只验证 PostgreSQL 配置替代 Lite 路径回归。

## 推荐实施切片

1. **事实基线与失败测试**：证明容器重建丢 Git、backup 缺 Git、readiness 不识别不可用路径。
2. **持久卷和配置收敛**：生产 Compose/K8s 路径与 Volume 显式一致。
3. **Readiness fail closed**：路径缺失、只读/不可写等故障返回 503；不与 EVO-118-G 的其他 readiness/依赖范围混淆。
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
- Git 路径缺失、只读、权限不足和必要目录创建失败的故障测试。
- readiness 故障 503、恢复后 200。
- DB-only、Git-only、损坏归档、checksum/version 失败和非空目标的 restore 拒绝测试。
- backup/restore/inventory 非零退出码和无假成功断言。
- Frontend required gate、Rust fmt/check/clippy/workspace tests、生产 Compose clean build（按实际改动范围）。
- Markdown 链接、`git diff --check`、exact-head CI 与 Navigator 独立复验。
- 最新 `main` 上 Repo UI type-check/build 不回归。

## 实施与验证证据

- 持久化：生产 Compose、K8s 与 Backend runtime 路径统一为 `/var/lib/evolith/git`，并使用显式持久卷；未宣称本地卷支持多实例共享。
- Readiness：PostgreSQL 与 Git Storage 联合判定；Git 路径通过目录、创建、写入、同步、读取和清理探测，失败返回 503。
- Backup/Restore：维护窗口前提、版本化 manifest/checksum、PostgreSQL + Git 联合归档、staging-first restore、空目标保护、路径/类型/版本/checksum/bare repo/refs 校验和失败回滚。
- Inventory：识别 DB-only、Git-only、重复/异常 UUID 布局、无效 bare repo、缺默认分支或最后 Commit，并以非零退出阻止假成功。
- 应用恢复：真实 Backend 完成 register/create/Smart HTTP push、联合备份、空环境恢复、login/list/clone/refs、readiness 故障注入和 Backend 重启后再次 clone。
- 容器重建：真实 Backend 容器在同一 PostgreSQL 与命名 Git volume 上删除/重建后，login/list/clone/Branch/Tag/SHA 均保持。
- 数据库兼容：修复 PostgreSQL migration 005 将文本 plan ID 声明为 UUID 的缺陷，与 SQLite/API 文本 ID 契约对齐。
- 演练缺陷修复：状态变更请求遵循 CSRF 双提交协议；CI-only runtime image 使用 UID/GID 10001，避免 Ubuntu 基础镜像 UID 1000 冲突。
- 实现 Head `83351a2375b6537ddb83d71d84a2d22bfaaacc15` 的 `ci` run `30736864612`（#157）与 `data-durability-container` run `30736864631`（#3）均成功。
- 上述成功是 review packet 提交前的 exact-head 证据；任何后续文档或修复提交都必须在最终 Head 重新跑 CI，旧 Head 只能作为支持证据。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 DATA-01，使 Git 目录、数据库元数据和备份恢复成为可验证的联合耐久性边界 |
| 产物 | 持久卷/配置、readiness、backup/restore、对账、故障测试、恢复证据、SOP/Release Notes、Navigator review packet |
| 状态同步归口 | EVO-118-D、Iteration 053、EVO-118 Epic、Product Backlog、Board、Baseline、Config/Release SOP/Scripts Release Notes |
| 验证证据 | 容器重建、空环境恢复、Commit/Branch/Tag、DB/Git 对账、只读/缺失故障、required exact-head CI 与独立 Navigator 结论 |
| 残余工作归口 | Repo lifecycle → EVO-118-F；部署构建 → EVO-118-E；综合 runtime gates → EVO-118-G；多实例共享存储另行评估 |

## 当前执行状态

- Iteration 053：Active；实施与 Driver 验证完成，等待独立 Navigator。
- Driver：已完成实现、故障定位、修复和实现 Head exact-head CI；当前只同步审核证据与治理状态。
- PR #7：Draft；Review Packet 提交后的最新 Head 必须重新通过 required CI，随后仍需独立 Navigator 才能转 Ready。
- Navigator：尚无独立结论；不得由本 Driver 或 CI 代替。审核入口见 [Navigator Review Packet](../../review/EVO-118-D-navigator-review.md)。
- DATA-01：Open；不得因分支、提交、旧 Head CI、review packet 或主线 UI 合并单独关闭。
- EVO-118-E：保持 Ready，未启动。

## 解锁内容

完成全部验收并经最新 exact-head CI、独立 Navigator 和恢复演练关闭后，解除 DATA-01 Gate；允许 Evolith 进入外部 Alpha 的数据耐久性评估，并为 Repo 生命周期一致性提供持久存储基础。
