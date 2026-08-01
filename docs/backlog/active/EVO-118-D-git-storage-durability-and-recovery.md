# EVO-118-D Git 存储持久化、备份与恢复演练

- **类型**：Technical / Data Durability / Deploy
- **状态**：Ready / Next Activation Candidate
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A/B/C Done；PR #5 merged；SEC-01/SEC-02 Closed
- **影响范围**：deploy / backend / scripts / docs / tests
- **计划 Iteration**：053（尚未创建或激活）

## 工程目标

确保容器重建、节点故障或数据库恢复后，Git Repo、Commit、Branch、Tag 和数据库元数据能够一起恢复，避免“数据库可恢复但代码历史永久丢失”。

## 启动事实基线

截至 2026-08-02：

- `main` 为 `936ed3b26a62840ddd94cf10e5075fd19e0a1c5c`，包含 PR #5 / EVO-118-C。
- 当前没有 Active / In Progress / Review Iteration，也没有开放 PR。
- 没有 EVO-118-D 或 Iteration 053 的现有分支/PR。
- EVO-118-E 保持 Ready，但按 WIP 不与本 Story 同时激活。
- 本文件满足 Ready 基线，但不代表已开始实施；开始前必须按 `START-ITERATION.md` 创建并激活 Iteration 053。

## 必读文档

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

- **Given** Repo 已 push Commit 与 Tag
- **When** 删除并重建 Backend 容器
- **Then** clone、Commit、Branch、Tag 和 Repo Metadata 保持一致

### Scenario 2：联合备份可恢复

- **Given** PostgreSQL 与 Git Storage 已产生业务数据
- **When** 执行备份、清空环境并恢复
- **Then** 用户可重新登录、列出 Repo、clone 并校验 Commit/Tag

### Scenario 3：Git 存储不可用阻止就绪

- **Given** Git Storage 只读、路径不存在或空间耗尽
- **When** 调用 readiness 或创建/push Repo
- **Then** readiness 返回 503，写操作明确失败且不产生成功假象

### Scenario 4：DB/Git 不一致可被识别

- **Given** 数据库存在 Repo 元数据但 Git 目录缺失，或 Git 目录存在但数据库记录缺失
- **When** 执行盘点或恢复验收
- **Then** 工具返回非零/明确失败并列出不一致，不把部分恢复报告为成功

### Scenario 5：备份产物可追溯且可拒绝不完整输入

- **Given** 备份包含数据库、Git 数据、版本/配置元数据和校验信息
- **When** 缺少任一必需部分或校验不通过
- **Then** restore fail closed，不覆盖现有环境，不报告恢复完成

## 工程要求

- Compose/K8s 显式配置 `GIT_STORAGE__BASE_PATH` 和持久 Volume。
- 备份同时覆盖 PostgreSQL、Git Repo 目录和必要版本/配置元数据。
- 定义备份一致性策略：维护窗口、快照或可重放的 reconcile 方案。
- 提供 restore 脚本或明确 SOP，并执行空环境恢复演练。
- 增加 Repo/DB 对账工具或至少可重复的盘点命令。
- 增加磁盘容量、inode、备份失败和恢复失败告警基线。
- Readiness 检查 Git 路径存在、可读写和必要的最小安全条件；失败返回 503。
- 所有脚本使用明确退出码，失败不继续覆盖，敏感信息不进入归档或日志。
- 修改脚本时同步 `SCRIPTS-RELEASE-NOTES.md`。
- 涉及 SQLite/PostgreSQL 行为时明确双轨影响；本 Story 不以只验证 PostgreSQL 配置替代 Lite 路径回归。

## 启动步骤

新会话只能按以下顺序开始：

1. 重新读取 GitHub 当前 `main` SHA、开放 PR、相关分支和 Actions 状态，不复用本文件中的历史 SHA 作为事实。
2. 盘点所有非终态 Iteration，并为 018/019/020/025/026/027 记录继续阻塞/替代的 disposition。
3. 只读审查当前生产 Compose、Dockerfile、Git Storage 配置、readiness、backup/restore 脚本和 Release SOP。
4. 搜索是否已有重叠 PR、分支、Issue 或脚本实现；发现重叠先归并，不重复实现。
5. 创建 `docs/iterations/ITERATION-053.md` Planned 基线，写清威胁模型、BDD、验证矩阵、风险、回滚和闭环台账。
6. 在同一治理批次原子同步：EVO-118-D → `In Progress`、Iteration 053 → `Active`、Epic/Backlog/Board/docs/index → 当前执行状态。
7. 治理激活提交存在后，才开始实现；默认创建 `agent/evo-118-d-git-durability-recovery` 分支和 Draft PR。

## 推荐实施切片

单个 Story 内按可验证顺序推进，不把所有脚本和部署一次性堆叠：

1. **事实基线与失败测试**：证明容器重建丢 Git、backup 缺 Git、readiness 不识别不可用路径。
2. **持久卷和配置收敛**：生产 Compose/K8s 路径与 Volume 显式一致。
3. **Readiness fail closed**：路径缺失、只读/不可写等故障返回 503；不与 EVO-118-G 的其他 readiness/依赖范围混淆。
4. **联合 backup/restore**：定义一致性策略、manifest/checksum、退出码和安全恢复顺序。
5. **对账与演练**：DB/Git inventory、空环境恢复、Commit/Branch/Tag 校验、重建后 clone。
6. **运营与文档**：容量/inode/失败告警基线、Release SOP、配置和脚本 release notes。
7. **Navigator 复验**：重点验证数据损坏、部分恢复、假成功、失败覆盖和实际恢复证据。

## 不做事项

- 不在本 Story 实现多地域复制、Git LFS 或对象存储后端。
- 不宣称本地持久卷自动支持多实例；多实例仍需共享存储或 Repo affinity 设计。
- Repo 创建/删除业务状态机归 EVO-118-F。
- 不在本 Story 全面处理 Embedded Frontend 构建收敛；归 EVO-118-E。
- 不把所有 readiness、限流、SMTP/Redis fail-closed 合并进来；归 EVO-118-G。
- 不提前恢复 Repo UI 或 Agent 写入主线。

## 最小验证

- `docker compose ... up` → push Commit/Branch/Tag → 重建 backend → clone/refs 校验。
- DB+Git 备份 → 清空环境 → restore → 登录/list/clone/Commit/Tag 校验。
- Git 路径缺失、只读、权限不足、空间/inode 模拟故障测试。
- readiness 故障 503、恢复后 200。
- DB-only、Git-only、校验失败和不完整归档的 restore 拒绝测试。
- backup/restore/对账脚本非零退出码和无假成功断言。
- Frontend required gate、Rust fmt/check/clippy/workspace tests、生产 Compose clean build（按实际改动范围）。
- 文档链接、脚本 release notes 和生产配置一致性检查。

## 闭环台账模板

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 关闭 DATA-01，使 Git 目录、数据库元数据和备份恢复成为可验证的联合耐久性边界 |
| 产物 | 持久卷/配置、readiness、backup/restore、对账、故障测试、恢复证据、SOP/Release Notes |
| 状态同步归口 | EVO-118-D、Iteration 053、EVO-118 Epic、Product Backlog、Board、Baseline、Config/Release SOP/Scripts Release Notes |
| 验证证据 | 容器重建、空环境恢复、Commit/Branch/Tag、DB/Git 对账、只读/缺失/空间故障、required CI 与 Navigator 结论 |
| 残余工作归口 | Repo lifecycle → EVO-118-F；部署构建 → EVO-118-E；综合 runtime gates → EVO-118-G；多实例共享存储另行评估 |

## 解锁内容

解除 DATA-01 Gate；允许 Evolith 进入外部 Alpha 的数据耐久性评估，并为 Repo 生命周期一致性提供持久存储基础。
