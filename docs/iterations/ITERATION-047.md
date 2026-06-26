# Iteration 047: git Smart HTTP WWW-Authenticate + 真实 git-client E2E（EVO-115）

> 文档状态：Closed（2026-06-26）
> 计划发布日期：2026-06-26
> 计划目标：实现 EVO-115——让真实 git 客户端能 clone/push/pull Evolith 仓库：rbac_middleware 在 `/repos/` 路径的 401 响应加 `WWW-Authenticate: Basic realm="evolith"`（标准 git server 行为，git 客户端据此用 URL 凭证重试）；解禁 `test_git_clone_push_pull_e2e` 并使其通过。完成后把 EVO-103-B-2 转 Done。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 实现 EVO-115：让真实 git 客户端能 clone/push/pull Evolith 仓库。
- rbac_middleware 在 `/repos/` 路径的 401 响应加 `WWW-Authenticate: Basic realm="evolith"`（标准 git server 行为，git 客户端据此用 URL 凭证重试）。
- 解禁 `test_git_clone_push_pull_e2e` 并使其通过。
- 完成后把 EVO-103-B-2 转 Done，完成 "git clone/push/pull works" 核心能力闭环。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-115](../backlog/active/EVO-115-git-www-authenticate-and-real-e2e.md) | git Smart HTTP WWW-Authenticate + 真实 git-client E2E | 父 Epic EVO-103-B（祖父 EVO-103，曾祖 EVO-100） | P1 | 启动条件: EVO-103-B-2 实现已就绪（B-2 handler/subprocess 存在） |

## 3. 发布计划基线：不做事项

- 不改变 B-2 端点协议逻辑（已完成）。
- 不改变现有 auth 基础设施语义（仅加 WWW-Authenticate header）。
- 不实现 SSH 或 LFS（Phase 5）。
- 不实现 Digest auth 或其他认证方式。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-115 标协议/测试形态（git Smart HTTP WWW-Authenticate + 真实 git-client E2E）；BDD 不适用，使用等价技术验收（真实 git CLI E2E）。
- [x] 7 条 acceptance 已在 [EVO-115 item file](../backlog/active/EVO-115-git-www-authenticate-and-real-e2e.md) 中定义。
- [x] 技术验收：`cargo test --workspace` 全绿 + `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

### EVO-115 验收（来自 item file）

- [ ] `/repos/` 路径 401 响应包含 `WWW-Authenticate: Basic realm="evolith"` header
- [ ] `test_git_clone_push_pull_e2e` 解禁（移除 `#[ignore]`）并通过
- [ ] `git clone http://user:apikey@host/repos/{id}` 成功（exit 0）
- [ ] 使用具备写权限的 API key 时，`git push` 成功推送 commits
- [ ] `git pull` 成功拉取远程更新
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors

### 等价技术验收（替代 BDD）

1. **真实 git clone**：使用真实 git CLI `git clone http://user:apikey@host/repos/{id}` 成功（exit 0），`.git` 目录存在。
2. **真实 git push**：在克隆仓库中创建 commit 后 `git push` 成功，远程仓库包含新 commit。
3. **真实 git pull**：另一克隆仓库 `git pull` 可获取 push 的内容。
4. **WWW-Authenticate header**：使用 curl/wireshark 验证 401 响应包含 `WWW-Authenticate: Basic`。
5. **权限边界**：read-only API key 仍被 receive-pack 拒绝（EVO-103-B-2 已覆盖）。

## 5. 发布计划基线：计划验证

```bash
# backend — 编译 + 测试
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# E2E 验证（真实 git CLI）
# 1. 启动后端服务
# 2. 创建 repo（通过 EVO-103-A API）
# 3. git clone http://localhost:8080/repos/{id}
# 4. cd test-clone && echo "test" > test.txt && git add . && git commit -m "test" && git push
# 5. cd /tmp && rm -rf test-pull && git clone http://localhost:8080/repos/{id} test-pull && git pull
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| WWW-Authenticate 仅对 `/repos/` 加（不影响 web 前端的 401 行为——前端用 JWT cookie，不应弹 basic-auth 框） | 必须只对 `/repos/` 路径加 WWW-Authenticate header；其他路径 401 保持现状 |
| 真实 git E2E 可能暴露其它协议/auth 问题 | 若发现，定位后修或归口新 follow-up，不得虚报通过 |
| 使用具备 repo:write 权限的 API key 做 push E2E | read-only key 已被 B-2 拒绝 receive-pack；本迭代使用写权限 key |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 实现 EVO-115 |
| 产物 | rbac_middleware /repos/ 401 WWW-Authenticate: Basic + 解禁真实 git E2E |
| 状态同步归口 | EVO-115（In Progress）、EVO-103-B-2（完成后转 Done）、PRODUCT-BACKLOG、BOARD、iterations/README、ITERATION-046（B-2 转 Done） |
| Story/BDD 归口 | [EVO-115 item file](../backlog/active/EVO-115-git-www-authenticate-and-real-e2e.md)（等价技术验收：真实 git CLI） |
| 验证证据 | cargo fmt clean；cargo clippy --workspace --all-targets -- -D warnings clean；cargo test --workspace 0 failures（含解禁的 test_git_clone_push_pull_e2e：真实 git clone + commit + push（write-permission key）+ pull 全通过）。 |
| 残余工作归口 | 无（B-2 转 Done 后 git 托管核心能力闭环；gix PR#2465 migration、receive-pack 永久 subprocess 仍归 EVO-103-B 残余） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-26 | activation | Iteration inventory 完成：ITERATION-046 Review/Partial（B-2 实现完成 + 架构评审已修 timeout/权限边界）；无其它 Active；018-020/027 Superseded；025/026 Blocked。EVO-115（B-2 收口残余）Proposed → Ready → In Progress，选入本轮。ITERATION-047 Active。 |
| 2026-06-26 | completion | EVO-115 完成：rbac_middleware 对 /repos/ 401 加 WWW-Authenticate: Basic（InternalError::from_response，非 /repos/ 401 不变以避免前端 basic-auth 弹窗）；解禁 test_git_clone_push_pull_e2e 并通过（真实 git clone/push/pull）。修了一个 replace 引入的 helper 自递归 bug（clippy needless_borrow 暴露）。验证全绿。EVO-103-B-2 → Done。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **Active / In Progress**：无（BOARD.md `Now` 段为空；iterations/ 中无 Active 文档）
- **Review / Partial**：ITERATION-046（EVO-103-B-2 实现完成 + 架构评审已修 timeout/权限边界；真实 git-client E2E 待 EVO-115）
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，与 Phase E' 独立）
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整 → EVO-108/109）
- **Recently Closed**：Iteration 042（EVO-101/102，2026-06-24）、Iteration 043（EVO-113，2026-06-25）、Iteration 044（EVO-103-A，2026-06-25）、Iteration 045（EVO-103-B-1，2026-06-25）

**disposition 决策**：
- ITERATION-046 — Review/Partial（同一 B-2 收口的两个迭代，本轮并行推进其残余）。继续推进 EVO-115。
- Iterations 018-020/027 — Superseded by 2026-06-23 方向调整。不激活。
- Iterations 025/026 — Planned/Blocked（Phase F 独立）。维持阻塞。
- Iterations 042/043/044/045 — Closed。无需处置。
- 无 Active 迭代，可直接从 backlog 选取 Ready story。

**新 Story 选取**：EVO-115 依赖 EVO-103-B-2（Review）已实现，Proposed → Ready → In Progress，选入本轮。WIP 限制 = 1 story。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

- 完成：WWW-Authenticate: Basic on /repos/ 401（InternalError::from_response，非 /repos/ 401 不变）；解禁 test_git_clone_push_pull_e2e 并通过（真实 git clone + commit + push（write-permission key）+ pull 全通过）。
- 未完成：无。
- 验证结果：cargo fmt clean；cargo clippy --workspace --all-targets -- -D warnings clean；cargo test --workspace 0 failures（含解禁的 test_git_clone_push_pull_e2e）。
- 闭环状态：`Complete`
- 残余归口：无（B-2 收口；gix PR#2465 migration + receive-pack 永久 subprocess 仍归 EVO-103-B 长期残余）。

## 11. Retrospective

- 做得好的：WWW-Authenticate: Basic 针对性修改（仅 /repos/ 路径，不影响前端 JWT cookie 401 行为）避免了浏览器 basic-auth 弹窗。
- 需要调整的：git Smart HTTP 401 必须带 WWW-Authenticate: Basic（已在 ITERATION-046 retro 记录，本次验证）。
- 写入 EVOLUTION：git smart HTTP 401 需 WWW-Authenticate: Basic（如确认为重复陷阱再写入 EVOLUTION.md）。

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 子 Story：[EVO-103-B Smart HTTP git 协议（父项）](../backlog/active/EVO-103-B-smart-http-git-protocol.md)
- 子 Story：[EVO-103-B-2 Smart HTTP Endpoints（Review）](../backlog/active/EVO-103-B-2-smart-http-endpoints.md)
- 本迭代 Story：[EVO-115 git WWW-Authenticate + 真实 git-client E2E（Active）](../backlog/active/EVO-115-git-www-authenticate-and-real-e2e.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
