# Iteration 043: Direction Pivot Review Remediation

> 文档状态：Closed
> 计划发布日期：2026-06-25
> 实际激活日期：2026-06-25
> 实际关闭日期：2026-06-25
> 计划目标：把 2026-06-25 方向变更全面评审中提到的所有需优化点收敛成一个可验证治理修复切片，修复 Git-Centric 主线的安全默认策略、状态源漂移、断链、legacy sandbox 误导和 manifest 风险门禁缺口。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

本轮是治理修复微迭代，归口 [EVO-113 Direction Pivot Review Remediation](../backlog/active/EVO-113-direction-pivot-review-remediation.md)，目标是把方向变更评审发现的问题从“对话结论”转为可审计产物：

- 修复 repo 默认合并策略：缺失 `.evolith/policy.yaml` 时默认 `require_review`，而不是默认自动合并。
- 同步 `EVO-101` / `EVO-102` 已完成事实，避免后续重复排期。
- 修复 `EVO-103` 被 `EVO-104` UX 门禁误阻塞的问题。
- 修复方向变更相关断链，恢复 Required Reads 可达性。
- 将 sandbox execute 标为 legacy / pending removal，避免新主线继续依赖。
- 同步 `.evolith/policy.yaml` spec 与 parser 默认语义。
- 刷新 `.agent-governance/manifest.yaml` 的 Git-Centric 风险门禁。
- 修正 legacy sandbox 测试的宿主 Docker 环境假设。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-113](../backlog/active/EVO-113-direction-pivot-review-remediation.md) | Direction Pivot Review Remediation | EVO-100 | P0 | 依赖 EVO-101 / EVO-102 已由 Iteration 042 完成；由 2026-06-25 方向变更全面评审触发 |

## 3. 发布计划基线：不做事项

- 不实现 [EVO-103](../backlog/active/EVO-103-repo-context-and-smart-http.md) Repo CRUD / Smart HTTP。
- 不完成 [EVO-104](../backlog/active/EVO-104-vibe-coding-web-ui.md) UX 决策或 UI 实现。
- 不删除 sandbox 代码；删除路径仍归口 [EVO-111](../backlog/active/EVO-111-deprecate-sandbox-runtime.md)。
- 不把旧 Phase E 的 superseded backlog 全部移动归档；本轮只修复失效链接与误导性状态。
- 不重写已关闭 Iteration 042 的发布计划基线，只同步其完成事实到 owner docs。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-113 标 Governance / Bug 修复形态；不适用 Product BDD。
- [x] 使用等价治理验收：状态源一致、断链为 0、验证命令通过、残余归口明确。
- [x] 涉及数据库默认值，因此追加 SQLite/PostgreSQL 双轨 migration 并更新 repository tests。

### 评审优化点清单

- [x] **安全默认策略**：`git_repos` DB default、SQLite/Postgres repository create 默认值、EVO-112 UI 默认全部改为 `auto_merge=false` / `require_review=true`。
- [x] **状态源同步**：`PRODUCT-BACKLOG.md`、EVO-100 子项表、`BOARD.md`、`IMPLEMENTATION-ROADMAP.md`、`docs/iterations/README.md` 同步 EVO-101/102 Done、EVO-113 Done、EVO-103 下一主线。
- [x] **EVO-103 阻塞关系纠偏**：EVO-104 UX U-01~U-05 只阻塞 Vibe Coding UI / Phase E'-2，不阻塞 EVO-103 后端基础能力。
- [x] **断链修复**：修复不存在的 `archive/2026-Q3`、ADR 旧文件名和 proposal 相对路径。
- [x] **Sandbox legacy 标注**：`AGENTS.md`、API Contract、Architecture、Config、Project Map、Tech Stack 明确 sandbox execute 是 legacy / pending removal。
- [x] **Policy spec/parser 语义同步**：`default_action` 缺省为 `require_review`；`agents[].scopes` 缺省为空列表，不隐式授予权限。
- [x] **Manifest 风险门禁刷新**：新增 git filesystem isolation、policy default、Smart HTTP subprocess、external agent scoped token/webhook、sandbox removal 等 Git-Centric gates。
- [x] **环境依赖测试修复**：legacy sandbox 测试不再假设宿主机一定没有 Docker daemon。
- [x] **经验写回**：EVOLUTION 记录 Git-Centric 默认策略和 Docker 环境依赖测试两类新坑。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo fmt --all -- --check
cargo test -p infra --test git_repo_repo_tests --quiet
cargo test -p domain policy --quiet
cargo test -p infra --quiet
cargo test -p service-skill --lib --quiet
cargo check --workspace --quiet
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --quiet

# repo root
python3 <markdown-link-scan>
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 已有环境已应用 007 migration，直接修改历史 migration 不生效 | 追加 009 双轨 migration 修正默认值，不改写已应用的历史 migration |
| policy 默认策略只改文档不改代码 | 同步 migration、repository create 默认值、UI spec 和 tests |
| sandbox 已接受删除但代码尚未删除 | reference 与 AGENTS 只标 legacy / pending removal；删除仍留给 EVO-111 |
| 文档状态源再次漂移 | 本轮用 markdown link scan、Board/Backlog/Roadmap/Iteration README 同步和 EVO-113 Review 记录收口 |
| Docker 环境依赖测试在不同机器表现不一致 | 测试按初始化结果校验语义，不断言宿主机 Docker 一定存在或不存在 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 汇总并修复方向变更全面评审提出的所有需优化点，形成可审计迭代记录 |
| 产物 | `EVO-113` backlog item；`ITERATION-043`；009 双轨 migration；repository 默认值与测试；sandbox legacy test 修复；Backlog/Board/Roadmap/Reference/Manifest/EVOLUTION 同步 |
| 状态同步归口 | `PRODUCT-BACKLOG.md`、EVO-100、EVO-113、`docs/BOARD.md`、`docs/roadmap/IMPLEMENTATION-ROADMAP.md`、`docs/iterations/README.md`、本文件 |
| Story/BDD 归口 | EVO-113 为 Governance / Bug 修复，不适用行为类 BDD；以命令级和一致性验证作为验收 |
| 验证证据 | Rust fmt/check/clippy/test、infra/domain/service-skill 局部测试、markdown link scan、`git diff --check` |
| 残余工作归口 | EVO-103 / EVO-104 / EVO-111 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-25 | activation | 用户要求“开干”，按评审结果建立 EVO-113，并作为治理修复微迭代插队；工作区已有本轮未提交变更，无其他用户改动。 |
| 2026-06-25 | progress | 新增 SQLite/Postgres 009 migration；修正 repository 默认策略；更新 test helpers 应用 009 migration。 |
| 2026-06-25 | progress | 同步 Backlog、EVO-100、Board、Roadmap、Iterations README，修复 EVO-103 被 UX 门禁误阻塞。 |
| 2026-06-25 | progress | 修复断链；标注 sandbox legacy；刷新 manifest Git-Centric risk gates；同步 policy spec/parser 语义。 |
| 2026-06-25 | progress | 修复 legacy sandbox 测试对 Docker daemon 缺失的环境假设。 |
| 2026-06-25 | validation | 运行所有计划验证，见 Review。 |
| 2026-06-25 | completion | EVO-113 标 Done；本迭代关闭。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-06-25 | clarification | 用户要求将“上面提到的所有需要优化的点”形成标准迭代文档 | 补建本 `ITERATION-043`，并将下一轮 EVO-103 候选顺延为 ITERATION-044 | 保留已完成修复产物，补齐迭代记录和目录同步 |

## 10. Review

- 完成：
  - 安全默认策略修复：新增 `backend/migrations/sqlite/009_safe_repo_policy_defaults.sql` 与 `backend/migrations/postgres/009_safe_repo_policy_defaults.sql`，repository 默认值和 UI spec 同步。
  - 状态同步：EVO-101/102 Done、EVO-113 Done、EVO-103 下一主线在 Backlog / Epic / Board / Roadmap / Iteration README 中一致。
  - 文档治理：断链清零，sandbox reference 标 legacy，policy spec 与 parser 默认语义对齐，manifest risk gates 刷新。
  - 测试治理：`service-skill` Docker 环境依赖测试改为环境中立。
  - 经验写回：EVOLUTION 增加 Git-Centric 默认策略与 Docker 环境依赖测试经验。
- 未完成：
  - 无。本轮范围内评审优化点均已处理。
- 验证结果：
  - `cargo fmt --all -- --check` → pass
  - `cargo test -p infra --test git_repo_repo_tests --quiet` → 15 passed
  - `cargo test -p domain policy --quiet` → 11 passed
  - `cargo test -p infra --quiet` → pass
  - `cargo test -p service-skill --lib --quiet` → 17 passed
  - `cargo check --workspace --quiet` → pass
  - `cargo clippy --workspace --all-targets -- -D warnings` → pass
  - `cargo test --workspace --quiet` → pass
  - Markdown link scan → all markdown links exist
  - `git diff --check` → pass
- 闭环状态：`Complete`
- 残余归口：
  - [EVO-103](../backlog/active/EVO-103-repo-context-and-smart-http.md)：Repo CRUD + Smart HTTP + Repo Context API。
  - [EVO-104](../backlog/active/EVO-104-vibe-coding-web-ui.md)：Vibe Coding UI UX 决策与实现。
  - [EVO-111](../backlog/active/EVO-111-deprecate-sandbox-runtime.md)：sandbox 整层删除。

## 11. Retrospective

- 做得好的：
  - 先做全面评审再修复，能把安全默认值、状态同步、断链和测试环境假设放到同一个闭环切片里处理。
  - 追加 migration 而不是改写 007，保留了已应用环境的演进路径。
- 需要调整的：
  - 方向变更完成后应立即跑 owner docs 同步和断链扫描，不能等后续评审才发现主表与 item file 状态冲突。
  - 环境依赖测试要避免把本机状态当固定事实。
- 写入 EVOLUTION：
  - 已写入“Git-Centric 方向修复必须同时检查默认策略、状态源和环境假设”。
