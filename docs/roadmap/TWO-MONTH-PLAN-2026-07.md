# Evolith 两个月执行规划（2026-07 ~ 2026-08）

> 状态：Planning baseline（2026-06-29）
> 范围：Phase E' Git 托管 + Vibe Coding 主线。
> 使用规则：本文是路线图，不是迭代启动记录。实际开工仍必须按
> [START-ITERATION](../sop/START-ITERATION.md) 盘点既有 iteration，并从
> [Product Backlog](../backlog/PRODUCT-BACKLOG.md) 选择满足 DoR 的 story。

## 1. 当前基线

- EVO-101 / EVO-102 / EVO-103 / EVO-113 / EVO-115 / EVO-116 已完成，Git repo 后端基础、Smart HTTP、Repo Context API、权限边界和资源边界已可作为后续 UI / agent / indexer 的稳定基础。
- EVO-100 Epic 仍为 `In Progress`。剩余主线集中在仓库管理 UI、commit/promote 写路径、agent session、webhook、vibe coding UI、indexer/discovery、旧表兼容和 sandbox 删除。
- EVO-104 UX gate U-01~U-05 已解除，但实现依赖 EVO-112 基础 UI、EVO-105 写路径和 EVO-106 agent session。
- Iteration 025 / 026 仍为 Phase F 独立 blocked plan，不应抢占 Phase E' 主线，除非生产发布或计费闭环成为显式目标。

## 2. 两个月目标

到 2026-08 末，建议达成以下结果：

1. Web 应用从旧 Tools / Skills / CLI 入口切换为 repo-centric 主入口，用户能在 UI 中创建、浏览和管理 Git 仓库。
2. API 支持受 policy 控制的 commit / promote 流程，为 agent 写代码和直推直合提供后端闭环。
3. 外部 agent engine 可通过 session + scoped token 读取 repo、提交变更、触发 promote，并留下审计与事件记录。
4. Webhook Out 可把 push / promote / agent event 通知外部 engine，具备 HMAC 签名、重试和失败记录。
5. Vibe Coding UI MVP 可进入可用状态：conversation-first workspace + repo file context + diff / commit 状态反馈。
6. Git push 或 commit 触发 Skill / CLI / MCP indexer，`SKILL.md` / `interface.yaml` / `tool.yaml` 可自动进入索引表。
7. 若前述链路稳定，启动 sandbox 删除收尾；若 agent/indexer 风险超出预估，EVO-111 顺延，避免把清理工作和核心链路风险混在同一窗口。

## 3. 推荐节奏

| 周期 | 主目标 | 候选 backlog | 交付结果 | 关键验证 |
|------|--------|--------------|----------|----------|
| Week 1 | Repo UI 基础入口 | EVO-112-A（从 EVO-112 拆出） | `/repos` 列表、`/repos/new` 创建、repo-centric 导航、Dashboard 改版 | `bun run type-check`; `bun run build`; Playwright 桌面/移动截图 |
| Week 2 | Repo Detail 只读浏览 | EVO-112-B（从 EVO-112 拆出） | `/repos/:id` Files / Commits / Settings；clone URL；旧页面仍可路由访问 | 创建 repo → push 文件 → UI 查看文件树、blob、commits、settings |
| Week 3 | Commit API 写路径与 policy evaluator | EVO-105-A（从 EVO-105 拆出） | `POST /repos/{id}/commits`；file ops；auto_merge / require_review / block 单元与 E2E | policy 矩阵测试；repo commit E2E；audit log 验证 |
| Week 4 | Promote API 与写路径硬化 | EVO-105-B（从 EVO-105 拆出） | `POST /repos/{id}/promote`；fast-forward / merge commit；冲突与非 fast-forward 明确错误边界 | promote E2E；权限/RBAC 测试；`cargo clippy --workspace --all-targets -- -D warnings` |
| Week 5 | Agent Session + Scoped Token | EVO-106-A/B | `agent_sessions` / scoped token / append-only event log / token middleware | token scope 403 测试；状态机单元测试；agent mock E2E |
| Week 6 | Webhook Out + agent integration seam | EVO-107 | push/promote 事件 delivery；HMAC；retry；失败记录；mock external engine | 本地 mock server；重试/签名/超时测试；audit/event 记录 |
| Week 7 | Vibe Coding UI MVP | EVO-104-A（从 EVO-104 拆出） | conversation-first workspace；repo context artifact；commit/promote 状态展示 | Playwright 截图；mock session flow；frontend build/type-check |
| Week 8 | Indexer MVP + discovery seed | EVO-108-A，视情况带 EVO-109-A refinement | push/commit 触发 indexer；`skill_index`/`cli_index`/`mcp_tool_index` 写入、更新、删除 | 5s 内索引 E2E；损坏 frontmatter 不崩溃；100 文件性能测试 |

## 4. 拆分建议

### EVO-112 拆分

- `EVO-112-A Repo UI Shell`：列表、创建、导航、Dashboard。目标是让平台第一屏转为 Git 仓库。
- `EVO-112-B Repo Detail Read-only`：详情页 Files / Commits / Settings。目标是验证 Repo Context API 的 UI 消费路径。

### EVO-105 拆分

- `EVO-105-A Commit API`：只处理 file ops + commit 创建 + policy 三态决策；promote 先不做。
- `EVO-105-B Promote API`：处理 branch promote、fast-forward、merge commit、冲突边界、audit。

### EVO-106 拆分

- `EVO-106-A Session Schema + Token`：migration、repository、JWT/scoped token、RBAC 创建规则。
- `EVO-106-B Session Events + Middleware`：append-only event、scope middleware、read/commit/promote 操作事件记录。

### EVO-104 拆分

- `EVO-104-A Vibe Workspace MVP`：conversation-first shell + repo context artifacts + commit/promote 状态反馈。
- `EVO-104-B` 后置：diff viewer 深化、冲突呈现、keyboard shortcuts、移动端深度适配。

### EVO-108 拆分

- `EVO-108-A Indexer Schema + Push Trigger`：三张 index 表、repository、trigger、路径匹配。
- `EVO-108-B Parser Reuse + Failure Handling`：复用现有 parser、错误隔离、性能基准。

## 5. 依赖与门禁

| 依赖 | 影响 | 门禁 |
|------|------|------|
| EVO-112 前端页面依赖 API contract 稳定 | UI 会暴露后端字段和错误语义 | 实施前重读 `API-CONTRACT.md`，发现字段缺口先补合约 |
| EVO-105 写路径影响裸仓库 refs | 数据损坏风险高 | 使用临时 repo E2E；每个失败路径验证 refs 不移动 |
| EVO-105 policy evaluator 影响安全默认 | auto_merge 误开会造成未审变更进入 main | 默认 require_review；protected_paths 覆盖测试必须通过 |
| EVO-106 scoped token 是新认证边界 | 越权调用 admin/force push 风险 | 所有 scope 判定单一 helper 归口；403 必须有测试和 audit |
| EVO-107 触发外部请求 | 出站请求和重试可能造成雪崩 | 本地 mock server；超时、最大重试、HMAC 必测 |
| EVO-108 indexer 读 git 内容 | 大 commit / 损坏 frontmatter 可能拖垮服务 | 限制匹配文件数、单文件大小、总耗时；错误隔离 |

## 6. 不建议纳入本窗口

- SSH / LFS / protected branch 完整规则。
- 多人协作、live preview、web terminal、Yjs。
- 全仓 federation 或跨租户公开 discover。
- Wasmer/WASI 替代 Docker sandbox（EVO-080），除非 EVO-111 删除后仍需要新的 runtime 候选。
- Phase F 计费/租户增强（EVO-012/013/014），除非产品发布目标明确要求。

## 7. 验收总门槛

每个实现迭代最低门槛：

- 后端改动：`cargo test -p <crate>` 或相关 integration tests，最终跨边界 story 跑 `cargo test --workspace` 和 `cargo clippy --workspace --all-targets -- -D warnings`。
- 前端改动：`bun run type-check`、`bun run build`、Playwright 截图覆盖桌面与移动基本布局。
- DB 改动：SQLite/PostgreSQL 双轨 migration、repository trait 双实现和双侧测试。
- API 合约变更：先更新 `docs/reference/API-CONTRACT.md`，再实现，最后同步前端 client。
- 治理收口：iteration review 记录实际命令、状态同步、残余归口；`git diff --check` 和 Markdown link check 通过。

## 8. 里程碑

| 日期目标 | 里程碑 | 判定方式 |
|----------|--------|----------|
| 2026-07-12 | Repo UI 可用 | 用户可通过 UI 创建 repo，并看到 repo-centric Dashboard |
| 2026-07-26 | Commit / Promote API 可用 | mock agent 或用户 API commit 能按 policy 落到 main 或 agent branch |
| 2026-08-09 | Agent session + webhook 可联调 | mock external engine 收到签名 webhook，session event 可审计 |
| 2026-08-23 | Vibe Coding MVP + Indexer MVP | Web workspace 可展示 repo context，push `SKILL.md` 后 5s 内可查询 index |
| 2026-08-31 | Phase E' 收尾复核 | 决定 EVO-111 是否当月启动，或将 sandbox 删除顺延到下个窗口 |

## 9. 风险与调整规则

- 如果 EVO-112 超过两周，暂停 UI 扩展，只保留列表/创建/详情基础闭环，把高级视觉和移动端深度适配后移。
- 如果 EVO-105 写路径出现 refs 数据一致性问题，停止推进 EVO-106/EVO-107，优先做数据损坏复盘和修复。
- 如果外部 agent engine 尚不可用，Week 5~7 全部使用本地 mock engine，不阻塞 Evolith integration surface。
- 如果 indexer 触发模型与 push metadata 事件不够稳定，先走显式 job / DB poll，事件总线作为后续优化。
- 如果两个月窗口内只完成到 EVO-107，也应优先保证 agent 后端链路正确，不用为了进度提前做 sandbox 删除。

## 10. 下一步动作

1. 创建 `EVO-112-A` / `EVO-112-B` 子 story，补 Required Reads、BDD 场景和验证命令。
2. 启动 `ITERATION-050`，选入 `EVO-112-A`，记录 inventory disposition。
3. 将 EVO-105 / EVO-106 先做 refinement，不在未拆分前直接标 Ready。
4. 每周收口后更新本文件实际进度；偏离计划时记录原因和新的归口。
