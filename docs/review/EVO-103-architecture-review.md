# 架构评审清单 — Evolith EVO-103 Repo 托管后端

> 生成日期：2026-06-26
> 评审范围：EVO-103-A / B-1 / B-2 / EVO-115 / EVO-103-C 全部交付
> 提交方：开发 Agent（glm-5.2 编排）
> 验证基线：`cargo test --workspace` 0 failures / `cargo clippy --workspace --all-targets -- -D warnings` 0 errors

---

## 交付总览

| Story | 状态 | 核心内容 | Commit |
|---|---|---|---|
| EVO-103-A | Done | Repo CRUD + gix::init 裸仓 + RBAC + seed-template | `c8543ea` |
| EVO-103-B-1 | Done | Git 客户端鉴权（Basic-Auth + `/repos/` RBAC + CSRF 豁免） | `cf7cd72` |
| EVO-103-B-2 | Done | Smart HTTP 3 端点（git subprocess + Docker git + 流式） | `35f6f00` + `4d35774`（评审修复） |
| EVO-115 | Done | WWW-Authenticate: Basic + 真实 git clone/push/pull E2E | `d804fc8` |
| EVO-103-C | Done | Repo Context API（file-tree/blobs/commits/diff via gix） | `f3884b2` |
| ADR-0006 | Accepted | Smart HTTP via git subprocess 决策 | `1740441` |
| 评审修复 | Done | subprocess timeout + API key 权限边界 | `4d35774` + `e78c673` |

---

## 评审项 1：全局 RBAC 中间件 Soundness（最高优先级）

**文件**：`backend/crates/api/src/middleware/rbac.rs` — `rbac_middleware` 函数
**Commit**：`cf7cd72`（B-1 实现）+ `d804fc8`（EVO-115 WWW-Authenticate）

### 背景

全局中间件需在调用内部 service 前**异步**解析 API key（DB 查找）。经历了三轮纠偏：

1. ~~`UnsafeCell<S>` + `unsafe { &mut *self.service.get() }.call(req)`~~ → 并发 UB（共享实例重叠 `&mut`），**已拒**
2. ~~`tokio::sync::Mutex<S>` 跨 `srv.call(req).await`~~ → 把整个服务器串行化，**已拒**
3. **`actix_web::middleware::from_fn` + `async fn rbac_middleware<B>(req, next: Next<B>)`** → `next: Next<B>` 在 `.await` 后仍可用，无锁无 unsafe，全并发。**采纳**

### 评审要点

- 确认 `from_fn` + `Next<B>` 模式确为 sound（无 `&mut` aliasing、无锁、无并发 UB）。
- 确认 `rbac.rs` 中**零** `unsafe` / `Mutex` / `RwLock` / `UnsafeCell`。
- 确认配置（JWT secret、api_key_repo）从 `app_data::<web::Data<AppState>>()` 读取（不闭包捕获）。
- EVO-115 新增：`unauthorized_response(path)` 对 `/repos/` 路径的 401 加 `WWW-Authenticate: Basic realm="evolith"`（通过 `InternalError::from_response`），非 `/repos/` 401 不变（避免浏览器弹 basic-auth 框）。
- 确认 helper 的 else 分支是 `Error::from(RbacError::Unauthorized)` 而非递归自调用（曾有 replace 引入的递归 bug，已修）。

---

## 评审项 2：Git Subprocess 安全不变量

**文件**：`backend/crates/service-git/src/lib.rs`（`advertise_refs`、`spawn_rpc`、`GIT_SUBPROCESS_TIMEOUT`）
**Commit**：`35f6f00` + `4d35774`（评审修复 timeout）

### 评审要点

- **无 shell**：`Command::new("git")`，固定参数向量 `["upload-pack"|"receive-pack", "--advertise-refs"?, "--stateless-rpc", <path>]`。确认无 `sh -c`。
- **repo_path 来源**：始终来自 DB 按 repo_id 查找后拼 `base_path`（`service_git::repo_path(base, tenant_id, repo_id)`），**绝不取自 URL**。
- **service 参数枚举校验**：`GitService::parse_service(s)` 仅接受 `git-upload-pack` / `git-receive-pack`，其余拒绝。
- **timeout**：`GIT_SUBPROCESS_TIMEOUT`（30s）+ `kill_on_drop(true)` + stderr drain（防 pipe 写满挂起）。确认有 `service-git` timeout 单测（`advertise_refs_times_out_slow_git_process`，50ms 超时）。
- **body 不透明**：POST body 原样传入 git stdin，不解析/不验证（git 内部处理协议帧）。

---

## 评审项 3：API Key 权限边界

**文件**：`backend/crates/api/src/handlers/git_smart_http_handlers.rs`（`api_key_allows_repo_read` / `api_key_allows_repo_write`）
**Commit**：`4d35774`（评审修复）

### 背景

架构评审发现：rbac_middleware 把 Basic-Auth API key 解析成 `CurrentUser { tenant_role: Member }` 但**丢弃了 `permissions`**——read-only key 会被提升为可 receive-pack（push）。

### 评审要点

- rbac.rs 现在把原始 `ApiKey` 也放入 `req.extensions_mut().insert(api_key)`（保留 permissions）。
- Smart HTTP handler 读取 `ApiKey`，对 `ReceivePack` 检查 `api_key_allows_repo_write`（需要 `write` / `repo:write` / `admin` / `commit` / `commit:*`），read-only key → 403 Forbidden。
- 确认有 `test_receive_pack_with_read_only_api_key_is_forbidden` 测试覆盖。
- `api_key_allows_repo_read` 对 `UploadPack`（clone/fetch）允许 read-only key。

---

## 评审项 4：Storage Path 一致性

**文件**：`backend/crates/infra/src/db/{git_repo_repo,pg_git_repo_repo}.rs`（create 方法）
**Commit**：`c8543ea`

### 背景

Navigator 审查发现 DB `storage_path` 列存 `repos/{tenant}/{name}`，磁盘 `gix::init_bare` 路径是 `{base}/{tenant}/{repo_id}.git`，两者不一致。修复为统一 `{tenant_id}/{repo_id}.git`。

### 评审要点

- 确认 SQLite + PostgreSQL 两处 create 方法都使用 `format!("{}/{}.git", tenant_id, id)`。
- 确认 `service_git::repo_path(base, tenant_id, repo_id)` 产出 `{base}/{tenant}/{repo_id}.git`，与 DB 列 join 后一致。
- 确认 delete handler 也用 `repo_path()` 而非读 DB `storage_path` 列。

---

## 评审项 5：RBAC 公开路径门禁 + CSRF 豁免范围

**文件**：`api/src/middleware/rbac.rs`（`is_public_path`）+ `csrf.rs`（`exempt_paths`）
**Commit**：`cf7cd72`（B-1）

### 评审要点

- `/repos/` 前缀**不再**被 public catch-all 豁免（原 `is_public_path` 把非 `/api/` 非 `/mcp` 路径当公开 → `/repos/` 无认证 → 安全漏洞）。
- SPA 静态路由（`/assets/`、`/login`、`/dashboard`）仍为公开。
- CSRF 豁免**仅限** `/repos/{id}/git-upload-pack` + `/repos/{id}/git-receive-pack` POST。确认范围不包含其他路径。
- info/refs 是 GET（CSRF 只查 POST/PATCH/DELETE）。

---

## 评审项 6：gix 读取正确性（Context API）

**文件**：`backend/crates/service-git/src/lib.rs`（`read_file_tree` / `read_blob` / `read_commits` / `read_diff`）
**Commit**：`f3884b2`

### 评审要点

- gix features：确认 `Cargo.toml` 有 `features = ["revision", "blob-diff"]`。
- **所有 gix 调用是 blocking**，handler 用 `web::block(|| { ... })` 包装。
- **file-tree**：确认 tree traversal 使用正确。
- **commits**：`id.ancestors().sorting(ByCommitTimeNewestFirst).all()` → 确认 commit decode 正确获取 author/message/timestamp。
- **diff**：`diff_tree_to_tree(Some(&old), Some(&new), None)` → 确认 `ChangeDetached` 匹配正确。
- **error handling**：gix 错误 → `GitStorageError::ReadError(String)` → handler 映射为 500。
- **?ref= 默认**：确认默认为 `repo.default_branch`（不是硬编码 "main"）。
- **blob 二进制检测**：content 含 null byte → base64；否则 utf-8。

---

## 评审项 7：Actix 路由注册（scope shadowing）

**文件**：`backend/crates/api/src/routes/repos.rs`
**Commit**：`f3884b2`

### 背景

EVO-103-C 初始用独立的 `web::scope("/repos")` 注册 C 路由，但 actix-web 4 中两个同前缀 scope 冲突（第一个 scope 吞掉请求 → 404）。修复：把 C 路由合并到现有 `repos::configure` scope 内。

### 评审要点

- 确认 `repos.rs` 的 `web::scope("/repos")` 包含全部 9 个 route（5 CRUD + 4 context read）。
- 确认 `routes/repo_context.rs` 的 `configure` 为空（no-op）。
- 确认 CRUD 测试在合并后仍通过。

---

## 评审项 8：Docker 部署变更

**文件**：`backend/Dockerfile` + `docs/reference/SCRIPTS-RELEASE-NOTES.md`
**Commit**：`35f6f00`

### 评审要点

- `debian:bookworm-slim` runtime stage 新增 `git` 包。
- 确认 release-notes 已记录。
- 确认 `evolith` 运行时用户有权限执行 `/usr/bin/git`。

---

## 评审项 9：真实 git E2E 覆盖

**文件**：`backend/crates/api/tests/git_smart_http_e2e_tests.rs`
**Commit**：`d804fc8`

### 评审要点

- 测试启动真实 HTTP 服务器，用真实 `git clone` → `commit` → `push` → `pull` 验证完整 git 工作流。
- 确认使用**具备写权限**的 API key 做 push（review-fix 后 read-only key 无法 push）。
- 测试耗时 ~2-7min（CI 可能需要长 timeout 或 `#[ignore]` + 手动运行）。

---

## 评审项 10：双数据库 migration 一致性

**文件**：`backend/migrations/{sqlite,postgres}/009_safe_repo_policy_defaults.sql`
**Commit**：`7989ebf`（EVO-113）

### 评审要点

- SQLite 表重建 vs PostgreSQL ALTER COLUMN SET DEFAULT — 语义等价。
- `auto_merge` DEFAULT FALSE / `require_review` DEFAULT TRUE（安全默认）。
- 不回填历史行（EVO-101 刚落地，无历史数据）。

---

## 已知残余 / 风险

| 项 | 描述 | 归口 |
|---|---|---|
| EVO-114 | create 时 disk init 失败为 best-effort（不回滚 DB） | Proposed backlog |
| gix PR#2465 迁移 | upload-pack/info-refs subprocess 未来可迁移到纯 gix | EVO-103-B 长期残余 |
| receive-pack 永久 subprocess | gix 无服务端 receive-pack 计划 | ADR-0006 接受 |
| SQLite 测试跨连接 schema cache | migration 009 表重建需 max_connections(1) | 测试 harness 限制 |
| Basic-Auth password = API key | 过渡方案；EVO-106 scoped token 将提供专用 git token | EVO-106 |

---

## 文件索引

| 评审项 | 关键文件 |
|---|---|
| 1 中间件 | `api/src/middleware/rbac.rs` |
| 2 subprocess | `service-git/src/lib.rs` |
| 3 权限 | `api/src/handlers/git_smart_http_handlers.rs` |
| 4 storage_path | `infra/src/db/{git_repo_repo,pg_git_repo_repo}.rs` |
| 5 RBAC/CSRF | `api/src/middleware/rbac.rs` + `csrf.rs` |
| 6 gix 读取 | `service-git/src/lib.rs` + `api/src/handlers/repo_context_handlers.rs` |
| 7 路由 | `api/src/routes/repos.rs` + `repo_context.rs` |
| 8 Docker | `backend/Dockerfile` + `docs/reference/SCRIPTS-RELEASE-NOTES.md` |
| 9 E2E | `api/tests/{git_smart_http,repo_context}_e2e_tests.rs` |
| 10 migration | `migrations/{sqlite,postgres}/009_*.sql` |

---

## EVOLUTION 经验写回（3 条）

1. **storage_path 一致性陷阱**（2026-06-25）：DB `storage_path` 列必须与磁盘 `gix::init_bare` 路径一致；否则后续 Smart HTTP / Context API 读 `storage_path` 定位仓库会指向错误路径。
2. **actix async 中间件 soundness 陷阱**（2026-06-25）：全局中间件做 async 预处理必须用 `from_fn` + `Next<B>`，不能用 `Transform/Service` + `let fut = self.service.call(req)` 模式（仅支持 sync），也不能用 `UnsafeCell`（UB）或 `Mutex`（串行化）。
3. **Smart HTTP 安全不变量需代码级门禁**（2026-06-26）：ADR/验收声明的安全不变量（subprocess timeout、权限边界）必须对应代码实现 + 测试，不能只在 backlog 勾选。

---

## 相关 ADR

- [ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 Deprecate Sandbox Runtime](../decisions/ADR-0005-deprecate-sandbox-runtime.md)
- [ADR-0006 Smart HTTP via git subprocess](../decisions/ADR-0006-smart-http-via-git-subprocess.md)
