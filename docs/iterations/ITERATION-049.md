# Iteration 049: EVO-103 Acceptance Hardening（EVO-116）

> 文档状态：Closed（2026-06-26）
> 计划发布日期：2026-06-26
> 计划目标：处理 EVO-103 架构验收 Conditional Accept 的发布前阻断项：API key scope、Context API 资源边界、Smart HTTP push 后 repo metadata、EVO-103 API contract 与性能验收证据。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 将 EVO-103 架构组验收结论从 `Conditional Accept` 推进到可关闭状态。
- 在进入 EVO-112 UI、EVO-105 Commit API、EVO-106 Agent Session、EVO-108 Indexer 之前，先修复已确认的安全、性能和合约缺口。
- 本轮是 technical/security hardening 微迭代，只选入一个 Story：EVO-116。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-116](../backlog/active/EVO-116-evo-103-acceptance-hardening.md) | EVO-103 验收硬化：API key scope、Context API 边界、元数据同步与合约补齐 | [EVO-100](../backlog/active/EVO-100-git-centric-platform-foundation.md) | P0 | EVO-103 A/B/C Done；架构组验收已给出 Conditional Accept |

## 3. 发布计划基线：不做事项

- 不实现 EVO-106 scoped token / agent session 新模型。
- 不实现 EVO-105 Commit API、promote、policy evaluator。
- 不实现 EVO-108 Indexer 或 webhook trigger。
- 不实现 UI。
- 不实现 SSH、LFS、公开 repo discover 或资源级 ACL。
- 不改变 ADR-0006 的 `git` subprocess 决策。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-116 标 Technical / Security / Performance hardening 形态。
- [x] 行为边界使用 Given/When/Then 场景：API key scope、Context API 大对象、push metadata、性能基准。
- [x] 技术验收包含命令级验证、合约同步、性能证据和残余归口。

### EVO-116 验收

- [ ] API key permissions 在 Repo CRUD、Repo Context、Smart HTTP 和 API key 管理路由上有统一门禁。
- [ ] read-only / execute-only / invalid API key 的权限矩阵测试覆盖。
- [ ] Context API 对 blob、file-tree、diff 和 blocking 读取有资源边界与错误映射。
- [ ] Smart HTTP push 成功后 repo default branch metadata 同步，失败降级策略明确。
- [ ] `docs/reference/API-CONTRACT.md` 补齐 EVO-103 公开接口。
- [ ] EVO-103-C 性能基准证据落入 Review。
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo test -p api --test git_smart_http_e2e_tests
cargo test -p api --test repo_context_e2e_tests
cargo test -p api --test repo_e2e_tests
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# docs
git diff --check
```

文档链接检查按 [START-ITERATION](../sop/START-ITERATION.md) 的 inline Python 脚本执行。

性能证据必须记录：

- file-tree（100 files）P95。
- repo list（1000 repos）P95。
- 样本规模、运行环境、命令或测试名。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| API key scope 修复影响现有 MCP / git client 调用 | 先写权限矩阵测试；区分 JWT 与 API key；Smart HTTP 继续保留 `WWW-Authenticate` 行为 |
| Context API 限制过紧影响 UI 浏览 | 边界值写入 API contract；超限返回可解释错误；必要时登记分页/streaming follow-up |
| push 后 metadata 更新失败造成 git client 误判 push 失败 | git push 成功后 metadata 更新失败只记录 warn/error，不回滚已完成 push；测试覆盖降级 |
| 性能基准受本机环境波动 | 记录环境和样本；不以单次偶发结果替代 P95；无法稳定复现时保持 Review/Partial 并登记阻塞 |
| API contract 补齐引发前端路径认知变化 | 明确 `/repos/*` Smart HTTP 与 `/api/v1/tenant/{tenant_id}/repos/*` REST/Context 的路径差异 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 为 EVO-103 Conditional Accept 创建可执行修复迭代，交由相关人员实现 |
| 产物 | EVO-116 backlog item；ITERATION-049；PRODUCT-BACKLOG / EVO-100 / iterations README / BOARD 状态同步 |
| 状态同步归口 | PRODUCT-BACKLOG、EVO-100 子项表、EVO-116 item、ITERATION-049、iterations README、BOARD |
| Story/BDD 归口 | [EVO-116 item file](../backlog/active/EVO-116-evo-103-acceptance-hardening.md) |
| 验证证据 | 文档链接检查、`git diff --check`；实现阶段还需运行后端测试和性能基准 |
| 残余工作归口 | 实现未完成前 EVO-116 保持 In Progress；EVO-106 / EVO-108 / Phase 5+ 承接非本轮范围 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-26 | activation | Iteration inventory 完成：无 Active/In Progress/Review；Iterations 025/026 维持 Planned/Blocked（Phase F refinement，与本轮独立）；Iterations 018-020/027 Superseded；Iterations 046-048 Closed。EVO-116 为 EVO-103 验收后发布前安全/架构修复，允许插队。 |
| 2026-06-26 | progress | 建立 EVO-116 item file 和 ITERATION-049，明确权限、性能、合约、metadata 同步验收与验证命令；状态设为 In Progress，等待相关人员实现。 |
| 2026-06-26 | done | EVO-116 全部 5 个验收面（API key scope / Context API 边界 / push metadata / API contract / 性能证据）实现并验证完成；`cargo test --workspace` + `cargo clippy --workspace --all-targets -- -D warnings` + `git diff --check` + 文档断链全绿；EVO-116 status 置 Done，ITERATION-049 闭环 Complete。 |
| 2026-06-26 | acceptance-remediation | 架构验收复核发现三类阻断：Context API 资源上限在完整读取/收集后才判断、file-tree/diff 超限缺测试、API contract Audit Logs section 被拼入 Push metadata。返修后：blob 先 `find_header` 再按大小决定是否 `find_blob`；file-tree 使用 bounded visitor 第 5001 个 entry 取消遍历；diff 使用 `Tree::changes().for_each_to_obtain_tree` 第 5001 条 change 取消；补 large file-tree / large diff 413 E2E；修复 API contract 与完成证据。 |
| 2026-06-29 | governance-sync | 收口状态漂移：页头从 Active 修正为 Closed，与 `docs/iterations/README.md`、`docs/BOARD.md` 和 EVO-116 backlog 状态一致；后续规划转入 Phase E'-1.5 / Phase E'-2。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

- **Active / In Progress**：无。
- **Review**：无。
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，候选需 refinement；与本轮 EVO-103 验收修复独立，维持阻塞）。
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整后不再激活）。
- **Recently Closed**：Iteration 046（EVO-103-B-2）、Iteration 047（EVO-115）、Iteration 048（EVO-103-C）。

**disposition 决策**：

- Iterations 025/026：维持 Planned/Blocked，不因本轮插队修改计划基线。
- Iterations 018-020/027：维持 Superseded。
- Iterations 046-048：Closed，无需重新打开；EVO-103 Conditional Accept 作为新 hardening Story 处理。
- 无 Active/Review 阻塞项，可启动 EVO-116。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

- 完成：
  - **API key scope 统一门禁**：新增 `api_key_scope.rs` 共享 helper；`repo_handlers`（list/get 创建/更新/删除）/ `repo_context_handlers`（file-tree / blobs / commits / diff）/ `api_key_handlers`（list / create / revoke）/ `git_smart_http_handlers`（info-refs / upload-pack / receive-pack）全部走 `forbid_if_api_key_lacks` 或共享 helper。read-only key 写 Repo 返 403；execute-only / unrelated key 读 Repo Context 返 403；API key 调用 `/api-keys` 路由必返 403（管理仅 JWT）。
  - **Context API 资源边界**：`BLOB_MAX_BYTES = 1 MiB` / `FILE_TREE_MAX_ENTRIES = 5000` / `DIFF_MAX_ENTRIES = 5000` / `CONTEXT_BLOCKING_TIMEOUT = 5s`；blob 先读 object header 再决定是否加载 body；file-tree / diff 在第 5001 个 entry/change 处取消遍历；超限分别映射到 413 / 504；无效 ref / sha 映射 400 / 404（不再统一 500）；handler 全部用 `web::block + tokio::time::timeout` 双重保护。
  - **Smart HTTP push metadata**：成功 push 后 `update_repo_metadata_after_push` 在 detached 任务中解析 `refs/heads/{default_branch}` → 写 `git_repos.last_commit_sha` / `last_committed_at`；失败仅 WARN。push 到非默认分支不更新 default branch metadata。
  - **API contract**：`docs/reference/API-CONTRACT.md` 新增 Repos / Smart HTTP / Repo Context API / API key scope / Push metadata sync 五个 section；CSRF 豁免路径加上 Smart HTTP POST；错误码新增 `INVALID_INPUT` / `RESOURCE_EXCEEDED` / `TIMEOUT` / `REPO_EXISTS`；验收返修中恢复独立 Audit Logs section，修正 blob/diff 边界描述。
  - **性能证据**（运行命令 `cargo test -p api --test repo_perf_benchmarks -- --nocapture --test-threads=1`，50 样本丢弃前 5 个 warmup）：

    | Scenario | Iterations | Sample size | P95 (ms) | env |
    |----------|------------|-------------|----------|-----|
    | file-tree (?ref=main) | 50 | 100 files | **2.77** | darwin aarch64, sqlite in-memory |
    | repo list (single tenant) | 50 | 1000 repos | **31.46** | darwin aarch64, sqlite in-memory |

    限制：单线程 actix-rt 沙箱 + in-memory SQLite；生产 PostgreSQL 实际 P95 可能略高但与 in-memory 同量级（主要耗时是 JSON 序列化而非 SQL）；如需更稳定基线应加 CI 工件 / 多次重测均值。
- 未完成：无。
- 验证结果：
  - `cargo test -p api --test git_smart_http_e2e_tests` → 3 passed
  - `cargo test -p api --test repo_context_e2e_tests` → 6 passed
  - `cargo test -p api --test repo_e2e_tests` → 7 passed
  - `cargo test -p api --test api_key_scope_e2e_tests` → 5 passed
  - `cargo test -p service-git` → 1 passed
  - `cargo test -p api --test repo_context_bounds_e2e_tests -- --test-threads=1` → 6 passed（新增 large file-tree / large diff 413）
  - `cargo test -p api --test repo_perf_benchmarks` → 2 passed
  - `cargo test --workspace` → 全部 test binary 全 ok（44 + 5 + 22 + 3 + 7 + 4 + 6 + 7 + 2 + 23 + 46 + 4 + 16 + 11 + 12 + 15 + 7 + 22 + 20 + 8 + 22 + 13 + 17 + 1 + 14 + 17 + 4 + 13 = 大量 passed；0 failed）
  - `cargo clippy --workspace --all-targets -- -D warnings` → 0 errors
  - `git diff --check` → clean
  - Python 文档断链检查 → all markdown links exist
- 闭环状态：**Complete**
- 残余归口：EVO-106 仍是 agent session / scoped token 模型 owner；EVO-108 仍是 indexer 与 push event trigger owner；Phase 5+ 仍是 SSH / LFS / 公开 repo discover / 分支级 metadata / 资源级 ACL owner。Resource bound 暂用 `service-git` 常量；如未来需要按租户覆盖，回退为 `AppConfig` 可配置项并同步 CONFIG.md。

## 11. Retrospective

- 做得好的：
  - 把"权限判断散落"作为 EVO-116 的硬性验收项（"权限判断不散落为重复字符串判断"）促使抽出 `api_key_scope.rs`，把 `api_key_allows_repo_read/write` 与 `api_key_allows_api_key_management` 作为单一归口，handler 层仅 4 个调用点（`repo_handlers` / `repo_context_handlers` / `api_key_handlers` / `git_smart_http_handlers`），后续要改 scope 模型只动一个文件。
  - 把"metadata 更新失败不破坏 push"作为显式验收 + 单元 + 集成测试覆盖（`test_push_updates_repo_default_branch_metadata`），避免后台任务被 commit 静默回归。
  - 性能测试用 `cargo test --nocapture` 而非独立 binary，CI / 本地 / 性能调优统一入口；表格用 markdown 直接输出便于拷贝到 Review。
  - API contract 把 Smart HTTP 与 REST 的两条路径差异（`/repos/{id}/git-…` vs `/api/v1/tenant/{tenant_id}/repos/{id}`）显式标注，避免未来前端错配。
- 需要调整：
  - `service-git` 的资源边界目前是常量。`BLOB_MAX_BYTES = 1 MiB` 对大模型 / 大数据集的 repo 可能过紧（"Evolution-fitness" 类型的二进制 seed 经常 > 1 MiB），如果后续 repo 场景出现 413 投诉，应改为 `AppConfig.git_context.*` 嵌套配置键并在 handler 注入；本轮保持常量以符合"未引入新配置键"的最小化要求。
  - Smart HTTP `update_repo_metadata_after_push` 在 worker runtime 上跑；如果未来把 HttpServer 改为 graceful drain，会需要保证 detached task 跟 worker 一同 drain；本轮单 worker 行为 OK，没有显式 drain 验证。
  - `cargo test --workspace` 跑 perf benchmark 时会和并发用例抢 CPU；建议在 CI 单独跑 `repo_perf_benchmarks` 而不是跟其它 e2e 并行（当前已经用 `--test-threads=1` 串行调用，但每个测试都开独立 app 启动，初始化耗时在测试总时长里占大头，可优化为共享 AppState fixture）。
- 写入 EVOLUTION：
  - **API key 权限矩阵必须单文件归口**（2026-06-26）：handler 散落的 `permission.as_str() == "repo:write"` 类判断是 E2E 越权的常见入口；统一 helper + 权限矩阵表格是关闭 EVO-103 Conditional Accept 的关键。
  - **gix `rev_parse_single` 不接受 fully-qualified ref 名称**（2026-06-26）：写 Smart HTTP metadata sync 时，`refs/heads/main` 会被 gix 拒绝；正确路径是 `find_reference(name).into_fully_peeled_id()` 然后 `find_commit(id)`。若后续接 EVO-108 indexer 触发，需要复刻同样模式。
  - **裸仓 + system git 的 default branch 不一定是 main**（2026-06-26）：clone 行为依赖本地 `init.defaultBranch` 与 server 侧 `repo.default_branch` 的对齐；测试和真实客户端都要显式 push 到 `refs/heads/{default_branch}` 而不是 `HEAD`。
  - **`actix-web` detached 任务生命周期 vs HttpServer shutdown**（2026-06-26）：`actix_web::rt::spawn` 跑在 worker runtime 上，HttpServer 的 `oneshot::Sender` shutdown 不会等待 detached task；若测试需要验证后台 task 完成的副作用，不要发送 shutdown 信号，让 test 函数自然退出 + TempDir 析构来清理。

## 相关链接

- 验收输入：[EVO-103 acceptance](../review/EVO-103-acceptance.md)
- 架构评审：[EVO-103 architecture review](../review/EVO-103-architecture-review.md)
- Story：[EVO-116](../backlog/active/EVO-116-evo-103-acceptance-hardening.md)
- 父 Epic：[EVO-100](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- ADR：[ADR-0006](../decisions/ADR-0006-smart-http-via-git-subprocess.md)
