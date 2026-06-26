# EVO-103 交付验收说明

> 提交日期：2026-06-26
> 交付方：开发 Agent（glm-5.2 编排 + qwen3.6-plus 实现）
> 验收方：架构组
> 验证基线：`cargo test --workspace` 0 failures / `cargo clippy --workspace --all-targets -- -D warnings` 0 errors
> 远端状态：origin/main = `0064549`（全部已推送）

---

## 1. 交付范围

| Story | 类型 | 状态 | 迭代 | 核心交付 |
|---|---|---|---|---|
| EVO-103-A | feature/api | **Done** | ITERATION-044 | Repo CRUD（POST/GET/PATCH/DELETE + list）+ `gix::init` 裸仓 + RBAC tenant-scoped + seed-template（`.evolith/agents.yaml` + `policy.yaml` + README）+ storage_path 一致性（`{tenant_id}/{repo_id}.git`） |
| EVO-103-B-1 | feature/api | **Done** | ITERATION-045 | Git 客户端鉴权：Basic-Auth 提取器（password=API key）+ `/repos/` RBAC auth-required（关闭 public catch-all 安全漏洞）+ CSRF 豁免（git Smart HTTP POST）+ `from_fn + Next<B>` sound middleware（无 UnsafeCell/Mutex/unsafe） |
| EVO-103-B-2 | feature/api | **Done** | ITERATION-046 | Smart HTTP 3 端点（`info/refs` + `git-upload-pack` + `git-receive-pack`，全部 `git --stateless-rpc` subprocess）+ Docker 运行时加 `git` + actix 流式（`web::Payload` → stdin / stdout → response）+ subprocess 安全（无 shell / 固定参数 / path-from-DB / timeout / kill_on_drop / stderr drain）+ API key 权限边界（read-only key 拒绝 receive-pack） |
| EVO-115 | feature/test | **Done** | ITERATION-047 | `WWW-Authenticate: Basic` on `/repos/` 401（标准 git server 行为，使 git 客户端用 URL 凭证重试）+ 解禁真实 `test_git_clone_push_pull_e2e` 并通过 |
| EVO-103-C | feature/api | **Done** | ITERATION-048 | Repo Context API（4 只读 GET：`file-tree` / `blobs/{sha}` / `commits` / `diff`，全部纯 gix 读取，`?ref=` 默认 `repo.default_branch`） |

**EVO-103 整体**：A + B + C 全部 Done，repo 托管后端核心能力闭环。

---

## 2. 验收标准达成情况

### EVO-103-A Repo CRUD

| 验收项 | 状态 | 验证方式 |
|---|---|---|
| 6 REST endpoints（CRUD + list）实现并通过 RBAC | ✅ | `repo_e2e_tests.rs` 7 E2E |
| `POST /repos` 创建时 `gix::init` 裸仓 | ✅ | create handler + test_create_repo_returns_201_with_disk_repo |
| seed-template 写入 `.evolith/agents.yaml` + `policy.yaml` + README | ✅ | test_create_repo + no-seed 对照测试 |
| storage_path DB/磁盘一致（`{tenant_id}/{repo_id}.git`） | ✅ | Navigator 审查发现不一致 → 修复 + 双库 create 方法统一 |
| 跨 tenant → 403 | ✅ | test_cross_tenant_get_returns_403 |
| `cargo test --workspace` + `cargo clippy` 全绿 | ✅ | 独立验证 |

### EVO-103-B-1 Git 客户端鉴权

| 验收项 | 状态 | 验证方式 |
|---|---|---|
| Basic-Auth（password=API key）解析为 CurrentUser | ✅ | test_basic_auth_valid_api_key_authenticates |
| `/repos/` auth-required（不再 public） | ✅ | test_repos_path_without_auth_returns_401 |
| CSRF 豁免 git Smart HTTP POST | ✅ | test_csrf_exempt_git_post_passes_without_token |
| 中间件 sound（from_fn + Next<B>，无 unsafe/Mutex） | ✅ | `grep unsafe/Mutex/UnsafeCell rbac.rs` = empty + EVOLUTION 记录 |
| 无效 API key → 401 | ✅ | test_basic_auth_invalid_api_key_returns_401 |

### EVO-103-B-2 Smart HTTP 端点

| 验收项 | 状态 | 验证方式 |
|---|---|---|
| `GET /repos/{id}/info/refs` 正确 content-type + 广告 | ✅ | handler 级测试 test_info_refs_with_basic_auth |
| `POST /repos/{id}/git-upload-pack` 流式 | ✅ | 真实 git E2E（clone） |
| `POST /repos/{id}/git-receive-pack` 流式 | ✅ | 真实 git E2E（push） |
| subprocess timeout（`tokio::time::timeout` + `kill_on_drop`） | ✅ | 评审修复 + service-git timeout 单测 |
| API key 权限边界（read-only 拒绝 receive-pack） | ✅ | 评审修复 + test_receive_pack_with_read_only_api_key_is_forbidden |
| Docker 运行时含 `git` + SCRIPTS-RELEASE-NOTES | ✅ | Dockerfile + release-notes |
| 真实 `git clone` / `git push` / `git pull` E2E | ✅ | test_git_clone_push_pull_e2e（通过，~2-7min） |

### EVO-115 WWW-Authenticate + 真实 E2E

| 验收项 | 状态 | 验证方式 |
|---|---|---|
| `/repos/` 401 含 `WWW-Authenticate: Basic` | ✅ | unauthorized_response helper + E2E |
| `test_git_clone_push_pull_e2e` 解禁并通过 | ✅ | 1 passed, 0 ignored |

### EVO-103-C Repo Context API

| 验收项 | 状态 | 验证方式 |
|---|---|---|
| `GET /repos/{id}/file-tree?ref=` 返回文件树 | ✅ | test_file_tree_returns_entries_for_ref_main |
| `GET /repos/{id}/blobs/{sha}` 返回 blob 内容 | ✅ | test_blob_by_sha_returns_content |
| `GET /repos/{id}/commits?ref=` 返回 commit 列表 | ✅ | test_commits_returns_list_sorted_by_time_desc |
| `GET /repos/{id}/diff?base=&head=` 返回 diff | ✅ | test_diff_between_two_refs_returns_changes |
| `?ref=` 默认 `repo.default_branch` | ✅ | test_ref_defaults_to_repo_default_branch |
| 跨 tenant 私有 repo → 403 | ✅ | test_cross_tenant_private_repo_returns_403 |

---

## 3. 验证证据

| 命令 | 结果 |
|---|---|
| `cargo test --workspace` | 0 failures（全部 test binary 全 ok；含真实 git clone/push/pull E2E） |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 errors, 0 warnings |
| `cargo fmt --check` | clean |
| markdown 文档链接 | 0 断链 |
| `git diff --check` | clean |
| soundness grep（rbac.rs 无 unsafe/Mutex/UnsafeCell） | empty（仅保留 doc 注释引用） |

### 测试覆盖汇总

| 测试文件 | 测试数 | 覆盖 |
|---|---|---|
| `repo_e2e_tests.rs` | 7 | Repo CRUD（create+disk / list / cross-tenant / dup-name / update / delete / no-seed） |
| `git_smart_http_e2e_tests.rs` | 3 | info_refs auth + read-only forbidden + **真实 git clone/push/pull** |
| `repo_context_e2e_tests.rs` | 6 | file-tree / blob / commits / diff / ?ref= default / cross-tenant 403 |
| `auth_e2e_tests.rs`（B-1 新增） | +4 | Basic-Auth 有效/无效 / 无-auth 401 / CSRF 豁免 |
| `service-git` timeout 单测 | 1 | advertise_refs_times_out_slow_git_process |

---

## 4. 架构评审修复记录

本轮开发经过两轮架构评审，评审发现并修复了以下问题：

| 轮次 | 发现 | 修复 | Commit |
|---|---|---|---|
| Navigator 审查（开发中） | storage_path DB 存 `repos/{tenant}/{name}` 与磁盘 `{base}/{tenant}/{repo_id}.git` 不一致 | 统一为 `{tenant_id}/{repo_id}.git`（sqlite + pg 双库 create 方法） | `c8543ea` |
| Navigator 审查（开发中） | seed files 仅 README，BDD 要求 3 件套 | 补 `.evolith/agents.yaml` + `policy.yaml` + README + 强化 e2e 断言 | `c8543ea` |
| Navigator 审查（开发中） | rbac_middleware `UnsafeCell<S>` 并发 UB | 三轮修到 `from_fn + Next<B>`（sound，全并发） | `cf7cd72` |
| 架构组评审（交付后） | subprocess timeout 仅在 ADR 声明但 `service-git` 未实现 | 实现 `tokio::time::timeout` + `kill_on_drop` + stderr drain + 单测 | `4d35774` |
| 架构组评审（交付后） | API key `permissions` 被 RBAC 丢弃 → read-only key 可 push | rbac 保留 `ApiKey` in extensions + handler 检查 `api_key_allows_repo_write` + 测试 | `4d35774` |

---

## 5. 架构决策记录

| ADR | 状态 | 说明 |
|---|---|---|
| [ADR-0004](../decisions/ADR-0004-git-centric-storage.md) | Accepted（已有） | 内容存储从 DB 列迁移到 Git 仓库文件 |
| [ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md) | Accepted（已有） | 废弃 Skill 沙箱执行 |
| [ADR-0006](../decisions/ADR-0006-smart-http-via-git-subprocess.md) | Accepted（本轮新增） | Smart HTTP 经 `git` CLI subprocess 实现（gix 已发布 crate 无服务端协议能力；receive-pack 永久 subprocess） |

### EVOLUTION 经验写回（3 条）

1. **storage_path 一致性陷阱**（2026-06-25）：DB `storage_path` 列必须与磁盘 `gix::init_bare` 路径一致。
2. **actix async 中间件 soundness 陷阱**（2026-06-25）：全局中间件做 async 预处理必须用 `from_fn + Next<B>`。
3. **Smart HTTP 安全不变量需代码级门禁**（2026-06-26）：ADR/验收声明的安全不变量必须对应代码实现 + 测试。

---

## 6. 已知残余与处置

| 残余项 | 严重度 | 处置 |
|---|---|---|
| EVO-114：create 时 disk init 失败 best-effort（不回滚 DB） | 低 | Proposed backlog（P2）；storage_path 已正确故可后续修复/重试 |
| gix PR#2465 迁移：upload-pack/info-refs 未来可用纯 gix 替代 subprocess | 低 | EVO-103-B 长期残余；PR 未合并，不影响当前 |
| receive-pack 永久 subprocess | 低 | ADR-0006 接受；gix 无计划 |
| Basic-Auth password = API key（过渡方案） | 中 | EVO-106 scoped token 将提供专用 git token |
| 真实 git E2E 耗时 ~2-7min | 低 | CI 可能需 `#[ignore]` + 手动运行或长 timeout |

---

## 7. 提交记录（8 个 commit，已推送）

| Commit | 类型 | 说明 |
|---|---|---|
| `e699bd7` | docs(backlog) | EVO-103 拆 A/B/C + ITERATION-044 启动 |
| `c8543ea` | feat(repo) | EVO-103-A Repo CRUD + gix::init |
| `4c49f0b` | docs(iteration) | EVO-103-A Done + EVO-114 |
| `cf7cd72` | feat(auth) | EVO-103-B-1 Git 客户端鉴权 |
| `6d1066b` | docs(iteration) | EVO-103-B-1 Done + EVOLUTION from_fn |
| `35f6f00` | feat(git) | EVO-103-B-2 Smart HTTP 端点 |
| `a51b85d` | docs(iteration) | EVO-103-B-2 Review/Partial + EVO-115 |
| `1740441` | docs(decisions) | ADR-0006 Smart HTTP via git subprocess |
| `4d35774` | fix(git) | 评审修复：subprocess timeout + 权限边界 |
| `e78c673` | docs(iteration) | 评审修复记录 |
| `d804fc8` | feat(auth) | EVO-115 WWW-Authenticate + 真实 git E2E |
| `9776a25` | docs(iteration) | EVO-115 + B-2 Done |
| `f3884b2` | feat(repo) | EVO-103-C Repo Context API |
| `bc23b9b` | docs(iteration) | EVO-103-C + EVO-103 Done |
| `0064549` | docs(review) | 架构评审清单 |

---

## 8. EVO-116 验收返修记录（2026-06-26）

EVO-103 初次验收为 Conditional Accept 后，EVO-116 / Iteration 049 负责关闭发布前阻断项。架构复核发现首轮 EVO-116 完成记录仍存在三类缺口：

1. Context API 资源边界在完整读取或完整收集后才判断，不能证明 DoS 风险已关闭。
2. file-tree / diff 超过 5000 entry 的行为缺少测试。
3. `API-CONTRACT.md` 的 `Audit Logs` section 被错误拼入 `Push metadata sync`。

返修完成项：

- `service-git::read_blob` 先用 object header 判断 kind/size，未超限才加载 blob body。
- `service-git::read_file_tree` 使用 bounded visitor，第 5001 个 entry 取消遍历并返回 `ResourceExceeded`。
- `service-git::read_diff` 使用 `Tree::changes().for_each_to_obtain_tree`，第 5001 条 change 取消 diff 并返回 `ResourceExceeded`。
- `repo_context_bounds_e2e_tests.rs` 增至 6 个 E2E，覆盖 invalid ref、invalid sha、oversized blob、large file-tree、large diff 和 push metadata。
- `API-CONTRACT.md` 恢复独立 `Audit Logs` section，并修正 blob/diff 资源边界描述。

返修后本验收可重新按 **Accept** 处理；验证证据以 [Iteration 049 Review](../iterations/ITERATION-049.md#10-review) 为准。

---

## 9. 验收请求

请架构组基于以下维度进行验收：

1. **功能完整性**：EVO-103 验收标准（§2）是否全部达成。
2. **安全性**：subprocess 安全不变量（§评审项 2-3）、RBAC 门禁（§评审项 5）、CSRF 豁免范围是否正确。
3. **Soundness**：rbac_middleware（from_fn + Next<B>）是否确为 sound（无并发 UB）。
4. **gix 使用正确性**：Context API 的 gix 读取 API 调用是否正确。
5. **部署变更**：Docker 加 `git` 是否可接受（ADR-0006）。
6. **残余处置**：已知残余（§6）的严重度和归口是否合理。

验收结论请标注：**Accept / Conditional Accept（附条件）/ Reject（附原因）**。

---

## 相关文档

- [架构评审清单（详细技术项）](EVO-103-architecture-review.md)
- [ADR-0006 Smart HTTP via git subprocess](../decisions/ADR-0006-smart-http-via-git-subprocess.md)
- [EVO-103 item file](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- [EVO-100 Epic](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- [EVOLUTION.md](../../EVOLUTION.md)
- [SCRIPTS-RELEASE-NOTES.md](../reference/SCRIPTS-RELEASE-NOTES.md)
