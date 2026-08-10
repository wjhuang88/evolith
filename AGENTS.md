# Evolith — AI-Native Git Platform

> AI-native git hosting + vibe coding + agent capability discovery.
>
> 本文件是 AI Agent 的启动文档。先建立硬约束和当前事实；流程步骤读取 `docs/sop/`，稳定事实读取 `docs/reference/`，历史经验读取 `EVOLUTION.md`。

## Agent Engineering Rules

### Hard Constraints

- **流程操作先查 Task Router**：需求进入、迭代、权限、安全、发布、数据库迁移等流程必须先读对应 SOP。
- **先看工作区状态**：修改前运行 `git status --short --branch`；不要回滚无关改动。
- **Backlog first**：新功能、缺陷、技术债先进入 `docs/backlog/PRODUCT-BACKLOG.md` 和 active item。P0 紧急止血允许先处理，但同一工作批次必须补记录。
- **开始迭代先盘点库存**：先处置 `Active / In Progress / Review / Planned / Blocked` iteration，再选择新 Story。
- **小批次**：默认一个 Agent 会话只推进一个 Ready Story；Epic 不直接进入迭代。
- **计划基线不可覆写**：已提交 Planned iteration 的目标、范围和风险必须保留；换目标新建编号。
- **实施必须闭环**：产物、状态、验证和残余缺任一项，只能报告 `Partial` 或 `Blocked`。
- **复杂任务分阶段审查**：跨层、权限、Git 写入、数据库、出站网络或发布改动先 Driver 实现，再 Navigator 审查。
- **中途变更先停手**：范围变化先按 CHANGE-CONTROL 分类并同步 Backlog/ADR/Iteration。
- **双数据库一致性**：Schema 或 Repository 行为变化同时考虑 SQLite/PostgreSQL migration、实现和测试。
- **重大取舍写 ADR**：技术栈、部署、认证、存储和数据边界变化必须写决策记录。
- **脚本行为变更写 Release Note**：参数、默认值、退出码、顺序或副作用变化同步 `SCRIPTS-RELEASE-NOTES.md`。
- **开发与上线 Gate 分离**：SEC-01/SEC-02/DATA-01/DATA-02 已关闭；按 ADR-0010，EVO-118-E / DEPLOY-01 在目标产品开发与清理完成后执行。关闭前可继续开发，但不得发布 External Alpha/生产或声明生产就绪。
- **安全敏感变更强制审查**：认证、API Key、MCP Tool、Git 写入、Webhook、出站 HTTP、备份和生产配置必须读 `SECURITY-REVIEW.md`。
- **Git 是代码事实源**：Git Repo 保存代码和历史；PostgreSQL 保存身份、权限、元数据、索引和事件状态。不要把数据库兼容行描述为新能力事实源。
- **Agent 写入受策略控制**：Agent Scoped Token 默认走 Commit/Promote + PolicyEvaluator；不得把 `commit:<scope>` 当作通用 Smart HTTP write。
- **持久事件**：Webhook、Indexer、Agent Event 和关键审计不得依赖可丢失的进程内 `spawn`；目标边界是 Outbox + Worker。

### Coding Behavior

#### 约束分类

| 类型 | 含义 | 处理 |
|------|------|------|
| Hard | 平台、外部合约、数据损坏或安全边界 | 推导必要门禁 |
| Soft | 风格、范式、工具偏好 | 影响选择时记录 |
| Assumption | 未验证负载、兼容性、行为 | 标记并验证；阻塞时建 Spike |

不要用“行业通常如此”替代项目事实。门禁必须追溯到代码、配置、合约、ADR、Baseline 或明确用户要求。

#### 简单优先

- 只实现 Story 要求的可验收结果。
- 单次使用代码不提前抽象。
- 不为推测需求添加灵活性。
- 能用模块化单体内边界解决时，不拆微服务。
- 不以 Kafka、Kubernetes 或 Service Mesh 代替权限、数据和交付闭环。

#### 手术刀式变更

- 不顺手格式化或重构无关文件。
- 匹配现有风格。
- 只清理本次变更产生的孤儿代码。
- 每个变更行都应追溯到用户请求、Backlog 或必要验证。

#### 目标驱动

```text
修 bug -> 先复现，再修复并证明回归
加校验 -> 先建立无效输入/越权失败，再让测试通过
安全硬化 -> 先列攻击者、资产、入口和负向测试
数据修复 -> 证明失败时不返回假成功且可恢复
发布改动 -> clean build + smoke + persistence + restore
文档校准 -> 对照 Code/Config/ADR，完成链接和状态检查
```

多步任务先写“步骤 -> 验证”。

### Git Rules

- 提交使用 `feat:`、`fix:`、`docs:`、`refactor:`、`test:`、`chore:`、`perf:` 或 `security:`。
- Agent-authored or Agent-assisted commits 必须使用完整格式 `type(scope): description (#story-id) [model: <model-name>]`；Story ID 可选，模型标签 required。
- 提交前检查 `git diff --cached`；不用 `git add .` 盲加。
- 一次提交表达一个清晰主题。
- 不使用破坏性命令覆盖用户改动。
- 默认在 `agent/<description>` 分支提交并创建 Draft PR，不直接污染 `main`。

## Task Router

| 任务类型 | 必读文档 | 按需参考 |
|----------|----------|----------|
| 了解项目结构 | [项目地图](docs/reference/PROJECT-MAP.md) | [架构](docs/reference/ARCHITECTURE.md)、[完成度基线](docs/reference/PRODUCTION-READINESS-BASELINE.md) |
| 需求进入/拆分/排期 | [需求进入](docs/sop/REQUIREMENT-INTAKE.md) | [Product Backlog](docs/backlog/PRODUCT-BACKLOG.md) |
| 开始迭代 | [开始迭代](docs/sop/START-ITERATION.md) | [迭代目录](docs/iterations/README.md) |
| 迭代执行 | [迭代工作流](docs/sop/ITERATION-WORKFLOW.md) | [测试](docs/sop/TESTING.md) |
| 迭代中变更 | [变更控制](docs/sop/CHANGE-CONTROL.md) | [ADR](docs/decisions/README.md) |
| 复杂任务审查 | [结对工作流](docs/sop/PAIRING-WORKFLOW.md) | [安全审查](docs/sop/SECURITY-REVIEW.md) |
| 认证/RBAC/API Key/Agent Token | [安全审查](docs/sop/SECURITY-REVIEW.md) | [权限](docs/reference/PERMISSIONS.md)、[Baseline](docs/reference/PRODUCTION-READINESS-BASELINE.md) |
| MCP Tool/Webhook/出站 HTTP | [安全审查](docs/sop/SECURITY-REVIEW.md) | [架构](docs/reference/ARCHITECTURE.md) |
| Git 写入/Commit/Promote | [安全审查](docs/sop/SECURITY-REVIEW.md) | [API 合约](docs/reference/API-CONTRACT.md)、[Baseline](docs/reference/PRODUCTION-READINESS-BASELINE.md) |
| Git Storage/备份/恢复 | [发布](docs/sop/RELEASE.md) | [安全审查](docs/sop/SECURITY-REVIEW.md)、[Readiness Plan](docs/roadmap/PRODUCTION-READINESS-PLAN-2026-07.md) |
| 本地启动/调试 | [本地开发](docs/sop/LOCAL-DEV.md) | [配置](docs/reference/CONFIG.md)、[经验](EVOLUTION.md) |
| 新增功能/API/页面 | [新增功能](docs/sop/NEW-FEATURE.md) | [API 合约](docs/reference/API-CONTRACT.md) |
| API 合约变更 | [Contract First](docs/sop/CONTRACT-FIRST.md) | [API 合约](docs/reference/API-CONTRACT.md) |
| 数据库迁移 | [数据库迁移](docs/sop/DATABASE-MIGRATION.md) | [配置](docs/reference/CONFIG.md) |
| 测试与验证 | [测试 SOP](docs/sop/TESTING.md) | [测试参考](docs/reference/TESTING.md) |
| 任务收口 | [任务闭环](docs/sop/TASK-CLOSURE.md) | [迭代工作流](docs/sop/ITERATION-WORKFLOW.md) |
| 发布/部署/回滚 | [发布](docs/sop/RELEASE.md) | [Baseline](docs/reference/PRODUCTION-READINESS-BASELINE.md) |
| Git 提交 | [Git 工作流](docs/sop/GIT-WORKFLOW.md) | [经验](EVOLUTION.md) |
| 问题排查/经验写回 | [经验反馈](docs/sop/EVOLUTION-FEEDBACK.md) | [经验](EVOLUTION.md) |
| 文档整理 | [文档检查](docs/sop/DOC-CHECK.md) | [文档地图](docs/README.md) |
| 技术决策 | [ADR](docs/decisions/README.md) | [实施路线图](docs/roadmap/IMPLEMENTATION-ROADMAP.md) |

## Current Known Traps

1. Rust MSRV 是 **1.88**；以 `backend/Cargo.toml` 和 Dockerfile 为准。
2. 当前是**模块化单体**，不是多个独立微服务。
3. 嵌套配置使用双下划线，例如 `DATABASE__URL`、`GIT_STORAGE__BASE_PATH`。
4. `VITE_API_URL` 通常包含 `/api/v1`，除非 Gateway 明确重写。
5. 浏览器状态变更受 CSRF；Git Smart HTTP 使用独立认证路径。
6. Sandbox 默认关闭且待 EVO-111 删除；新功能不得依赖它。
7. Git Smart HTTP 使用 git subprocess；Repo Context 使用 `gix`。
8. Git Repo 在文件系统；多实例不能只共享 PostgreSQL。
9. Git 单实例持久化与恢复已由 EVO-118-D 关闭；Repo 生命周期一致性已由 EVO-118-F 关闭。
10. Embedded Frontend 已是目标交付形态；production build/Compose 最终收敛归 EVO-118-E，并按 ADR-0010 在产品开发与清理后执行。
11. API Key 管理、Typed Capability 与 MCP `execute` 已由 EVO-118-B 硬化；历史 legacy Key 仍需按权限合约轮换，不能重新接受任意权限字符串。
12. HTTP Tool Egress/SSRF 已由 EVO-118-C 关闭：生产默认仅允许受策略约束的 HTTPS 公网目标；未来 Webhook/其他租户可控出站必须复用统一 Egress Policy。
13. `.evolith/policy.yaml` Parser 已有，不代表策略三态已经执行；行为归 EVO-105。
14. `commit:<scope>` 当前不能被视为真实 Branch/Path 限制；Agent 写入设计必须重新验证。
15. Repo 创建/删除已通过 EVO-118-F 生命周期状态与 Reconciler 收敛；Seed 是真实 Initial Commit。后续不得退回 best-effort warning 或直接删除未知 disk-only 数据。
16. Push 后内存异步任务不是可靠事件边界；Webhook/Indexer 依赖 EVO-118-H。
17. CI 当前历史配置为 Tag-driven；PR/Main Gate 归 EVO-118-G。
18. `/health` 成功不证明 DB/Git Storage ready；发布使用 readiness 503 语义。
19. SMTP/Redis 的生产降级必须按职责 fail closed，不能用开发 fallback 返回假成功。
20. 文件或代码存在不等于完成；验证、状态同步和残余归口缺失时必须报告 `Partial`。
21. EVO-118 的 SEC-01/SEC-02/DATA-01/DATA-02 已关闭；当前先推进 G/H 与目标产品链，EVO-118-E 保留为最后发布 Gate。
22. GitHub Actions required CI 通过不等于安全/发布复验通过；必须同时关闭 Navigator、稳定契约和治理状态。
23. 实现 PR 合并后仍需回写 merge commit、owner Story/Iteration、Gate 和派生入口；合并本身不会自动完成治理收口。
24. 项目尚未上线；ADR-0009 已取消 EVO-110 旧表双写、回填和旧 API 兼容。Repo-derived read/execute 承接后由 EVO-121-F/EVO-122 直接删除旧 UI/runtime/table，不得重新建设 Legacy 兼容层。

## Current Project Baseline

### Product Direction

Evolith 是 Git-centric AI development platform：

- Git repository hosting and Repo Context；
- controlled Agent commit/promote and Vibe Coding；
- Skill/MCP/CLI discovery derived from Repo content；
- tenant isolation, typed capability, audit and scoped credentials。

旧 DB-centric Registry 只保留 Compatibility；Git Repository 是基础载体。

### Current Maturity

- Engineering foundation：较完整。
- Git backend：Alpha foundation。
- Web product：EVO-112-A/B/C Repo UI、Repo Detail、Commit Evidence 与 EVO-120 Onboarding 已实现；最终 Entry/App Shell/Dashboard/Settings/Activity 与 Vibe Coding 仍未实现。
- Agent loop：Commit/Promote/Session/Webhook 未实现。
- Production：SEC-01/SEC-02/DATA-01/DATA-02 已关闭；仍被 DEPLOY-01 生产构建阻断。

不得把 EVO-103/116 或已关闭的安全 Gate 描述成平台整体生产就绪。

### Current Execution State

- `main` 已包含 PR #7 / EVO-118-D；merge commit `932def05717b678f6f44dc23f137933d56158957`，DATA-01 已关闭。
- EVO-118-D Done / Merged，Iteration 053 Closed / Complete。
- EVO-118-F Done / Complete，Iteration 055 Closed / Complete，DATA-02 已关闭。
- EVO-118-G 已拆为 G-A/B/C/D；G-B/C/D Complete，G-A 因远端 Branch Protection 证据保持 Partial。
- EVO-118-H / Iteration 060 Closed / Partial 后拆为 H-A/B/C；H-A/H-B / Iterations 066/067 Done / Complete，H-C / Iteration 068 Review / Partial，EVENT-01 未关闭；EVO-125 负责完整 Smart HTTP E2E 残余。
- EVO-120、EVO-112-A/B/C 与 EVO-121-C/D Done / Complete；Iterations 064/065 Closed / Complete。EVO-105/106 不能绕过 H，H-C 核心已实现但等待 Review / Partial 收口。
- EVO-118-E 保持 Proposed / final release gate。

### Current Order

```text
EVO-118-A ✓
→ EVO-118-B ✓
→ EVO-118-C ✓
→ EVO-118-D ✓
→ EVO-118-F ✓
→ EVO-118-G-A/B/C/D（G-B/C/D ✓；G-A Partial residual）
→ EVO-118-H-A ✓ → H-B ✓ → H-C（EVENT-01）
→ EVO-120 ✓ / EVO-112-A/B/C ✓
→ EVO-121-C/D ✓
→ EVO-105/106/107/104
→ EVO-121-E/B
→ EVO-108/109
→ EVO-121-A/F
→ EVO-111
→ EVO-122-A/B/C
→ EVO-118-E（Final production convergence）
```

## Development Baseline

### Backend

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Frontend

```bash
cd frontend
bun install --frozen-lockfile
bun run type-check
bun run build
bun run lint
```

### Production

```bash
docker compose -f docker-compose.prod.yml build --no-cache
docker compose -f docker-compose.prod.yml up -d
```

Production validation must include security negative tests, Git persistence and DB+Git restore; build success alone is insufficient.

## Session End Checklist

- [ ] 新需求/缺陷/技术债是否进入 Backlog？
- [ ] 是否先更新 owner docs，再同步 Board？
- [ ] 是否处理非终态 Iteration 库存？
- [ ] Story 类型、依赖和验收是否符合 DoR？
- [ ] 权限、出站 HTTP、Git 写入、存储或发布是否读了 Security Review？
- [ ] 是否保留已发布计划基线？
- [ ] 是否执行实际验证，而不是引用历史或预期结果？
- [ ] 安全变更是否有负向测试和 Navigator 结论？
- [ ] DB/Git 数据变更是否验证失败、补偿和恢复路径？
- [ ] 生产改动是否验证 clean build、readiness、持久化和 restore？
- [ ] 状态、API Contract、ADR、EVOLUTION、Release Note 是否同步？
- [ ] 最终结论是否严格使用 `Complete / Partial / Blocked`？
- [ ] Agent commit 是否带 `[model: <model-name>]`？

## Links

- [Documentation Map](docs/README.md)
- [Production Readiness Baseline](docs/reference/PRODUCTION-READINESS-BASELINE.md)
- [Security Review SOP](docs/sop/SECURITY-REVIEW.md)
- [Architecture](docs/reference/ARCHITECTURE.md)
- [Product Backlog](docs/backlog/PRODUCT-BACKLOG.md)
- [Operating Board](docs/BOARD.md)
- [Implementation Roadmap](docs/roadmap/IMPLEMENTATION-ROADMAP.md)
- [Production Readiness Plan](docs/roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)
- [Evolution Notes](EVOLUTION.md)
