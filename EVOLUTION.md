# Evolith 故障排查与经验积累

> 本文件用于把项目中的非直觉经验沉淀为可检索规则。
> 任务开始排查问题时，先看 Part 1；任务中发现新坑时，按 Part 3 写回。

---

## Part 1: 问题速查表

| # | 现象 | 可能原因 | 快速解决 |
|---|------|----------|----------|
| 1 | Docker Compose 后端没有连 PostgreSQL | 使用了 `DATABASE_TYPE` 而不是 `DATABASE__DATABASE_TYPE` | 检查 compose/env 是否使用双下划线配置键 |
| 2 | 容器化前端请求 `/auth/login` 404 | `NEXT_PUBLIC_API_URL` 缺少 `/api/v1` | 设置为 `http://localhost:8080/api/v1` 或生产 API 前缀 |
| 3 | 修改 ConfigMap/Nginx/Compose 后线上不生效 | Git 提交不等于部署刷新 | 按发布 SOP 执行重建、重启或重新 apply |
| 4 | SQLite 与 PostgreSQL 行为不一致 | 只改了一侧 migration/repository | 同步修改 `migrations/sqlite`、`migrations/postgres` 和两套 repository |
| 5 | CSRF 403 | 状态变更请求缺少 `csrf_token` cookie 或 `X-CSRF-Token` header | 先完成登录/刷新，再由 API client 自动带 header |
| 6 | `SANDBOX__ENABLED=true` 时服务启动失败 | Docker executor 初始化失败；现在不再降级成伪成功 default executor | 默认/lite 不启用 sandbox；需要执行时先启动 Docker、构建 sandbox 镜像并显式设为 `true` |
| 7 | lite 模式种子账号不能登录 | migrations 中的测试/admin 密码哈希是占位值，且 SQLite 内存库重启即清空 | 启动后通过注册接口创建临时账号 |
| 8 | 前端改了但 release 二进制没更新 | `rust-embed-for-web` proc macro 不跟踪 dist 目录变更 | 确认 `build.rs` 中有 `cargo:rerun-if-changed` 指向前端 dist |
| 9 | `cargo fmt --check` 退出码 1 但 0 个文件 diff | `rustfmt.toml` 含 nightly-only 选项被 stable 静默忽略 | 先跑 `cargo fmt --check 2>&1 \| grep nightly`；有 warning 就删除 nightly-only 选项或切 nightly toolchain |
| 10 | `DATABASE__DATABASE_TYPE=mysql` 启动失败 | MySQL repositories 未实现；配置/连接池会快速拒绝 | 使用 SQLite 开发或 PostgreSQL 生产；不要把 MySQL 当成可用后端 |
| 11 | SQLite FK 约束失败（code 787） | 新表 FK 引用 `tenants(id)` 但 `tenant_id` 以 Uuid BLOB 绑定，而 `tenants.id` 以 TEXT 存储 | SQLite repository 中所有 UUID 绑定使用 `.to_string()`，与 tenant_repo 既有模式一致 |
| 12 | 新建 repo 缺失 policy 时默认自动合并 | DB / repository / UI 默认值没有同步方向文档的 `require_review=true` 安全策略 | Git-Centric policy 默认必须在 migration、repository、UI 和 evaluator 中同时验证；缺失 policy 时默认 require review |
| 13 | `cargo test --workspace` 在有 Docker 的机器失败 sandbox 测试 | 测试假设运行环境一定没有 Docker daemon | legacy sandbox 测试不能依赖宿主 Docker 是否存在；按实际初始化结果校验错误语义或功能语义 |
| 14 | 资源上限声明通过但仍有 DoS 风险 | 上限在完整读取/完整收集后才判断 | blob 先读 object header；tree/diff 在迭代 callback 中达到上限即取消；测试覆盖超限路径 |
| 15 | iteration README / Board 显示 Closed，但单个 iteration 页头仍是 Active | 只同步派生视图和总目录，漏改 owner 文档页头 | 收口时同时检查 iteration 文件页头、执行记录、README、Board、backlog item；owner 页头优先 |
| 16 | EVO-112-A 创建 repo 后跳转 `/repos/:id` 出现 404 | shell 切片先交付，但创建成功路径指向尚未实现的 EVO-112-B 详情页 | 详情页未交付前返回 `/repos`；已知后续能力不能作为当前可用路径的 fallback |
| 17 | `frontend/src/lib/api/members` 返回 `ApiResponse` 而非 `Member[]` | 后端 handler 走统一 `ApiResponse<T>` 包装；前端按字段读取 `.data?.members` | 在调用方 `membersResponse?.data?.members ?? []` 而不是直接把响应赋给 `setState` |
| 18 | `tenant?.id` 在闭包中被 TypeScript 推断为 `string \| undefined` | TS 对可选链的窄化在闭包边界不可靠；尤其 `await` 之后 | 在闭包外显式 `const tid: string = tenantId` 二次断言后使用 |
| 19 | 新 migration 在 fresh schema 通过，但已有环境升级仍可能失败或丢状态 | 测试只从空库执行全部 migration，没有从前一版本携带已有行升级 | 双数据库从前一 migration 建表并插入代表性行，再执行新 migration 验证数据/状态保留 |

---

## Part 2: 经验条目

> 新经验按时间倒序追加。避免重复记录同一问题。

## 2026-08-09 - Fresh schema 不能替代 existing-row migration upgrade 证据

- Trigger: EVO-118-H-A Navigator 复核 `012_outbox_claim_leases` 时发现 SQLite/PostgreSQL 测试只从空库执行到最新 schema。
- Symptom: paired migration 文件和 fresh install 都成功，但没有证据证明 `011` 中已有 Outbox Event 在升级后仍保留状态、payload 与可空 lease 字段。
- Root cause: 把“最新 schema 可创建”误当成“生产式增量 upgrade 安全”；fresh 路径不会暴露 ALTER 对已有行的默认值、约束或类型兼容问题。
- Fix: SQLite/PostgreSQL 均先执行到 `011`、插入代表性 delivered event，再执行 `012`，验证原状态保留且新增 token/lease 为 NULL；PostgreSQL 继续运行真实并发 claim。
- Prevention: 新 migration 除 fresh schema/repository 测试外，必须从直接前一版本携带代表性已有行升级；schema 或语义变化时检查数据和状态保留，不能只检查列存在。
- Promoted to rule/check: `docs/sop/DATABASE-MIGRATION.md`；`outbox_repo_tests::sqlite_011_to_012_upgrade_preserves_existing_events`；`pg_outbox_claim_tests::postgres_claims_are_disjoint_and_expired_claims_are_fenced`。

### 2026-08-02 - 仓库管理 UI 可拆分，但不能把 404 当作 fallback（EVO-112-A）
**现象**: 用户 2026-06-24 反馈 "git 相关的页面都没有出现"。EVO-103（Repo CRUD + Context API）已 Done，但前端没有仓库管理页面。完整 Vibe Coding 编辑器 EVO-104 仍依赖 UX U-01~U-05 之外的 API/UI 依赖。
**根因**: 单个 Story 跨越 0.5-2 天交付窗口过大；如果直接做"仓库列表 + 创建 + 详情 Files/Commits/Settings + 导航 + Dashboard"全部内容，agent 实施 + 验证会进入"全栈大爆炸"模式，且 E2E 测试覆盖不足。
**方案**: 将 EVO-112 拆为 EVO-112-A（仓库列表 / 创建 / 侧边栏 Repos-Dashboard-Settings + Legacy 二级菜单 / Dashboard repo-centric 改版）和 EVO-112-B（Files / Commits / Settings 三 Tab 只读浏览）。A 是 B 的硬依赖但 A 不依赖 B；A 创建成功后返回已实现的 `/repos`，B 交付后再把入口改为详情页。
**教训**:
- 当 EVO-N 的依赖（D1, D2, ..., Dn）里有「已被未来 EVO-M 解决但 M 还没做」的情况，应优先把 EVO-N 拆为「EVO-N-A 不依赖 M」+「EVO-N-B 依赖 M」，让 EVO-N-A 优先交付、避免 N 等 M。
- 已知后续 Story 不能替代当前可用路径；若目标路由尚未实现，必须返回已实现页面或提供明确可用的占位页，不能把 404 登记为“预期 fallback”后声明 Complete。
- 「旧 Tools / Skills / Interfaces 降级为 Legacy 二级菜单」是过渡期最低成本选择：路由保留 → 历史链接可访问；主导航不再暴露 → 新用户不被旧概念干扰；不删除代码 → Phase E' 完成后可单独评估是否彻底删除。
**Promoted to rule/check**: EVO-112-A BDD 与 Iteration 054 收口证据要求创建成功路径落到已实现的 `/repos`；详情页继续归口 EVO-112-B。

### 2026-06-29 - 关闭迭代时必须同步 owner 页头状态
**现象**: EVO-116 / Iteration 049 已在 backlog、iterations README 和 Board 中记录为 Complete/Closed，但 `docs/iterations/ITERATION-049.md` 页头仍写 `Active`，会在下一次 START-ITERATION 库存盘点时误导 Agent 判断仍有在途迭代。
**根因**: 收口时同步了派生视图和目录，却漏改 iteration owner 文档最顶部的状态字段；目录状态不能替代 owner 文档状态。
**方案**: 将 Iteration 049 页头修正为 `Closed（2026-06-26）`，并在执行记录追加 2026-06-29 `governance-sync`；同步刷新 Board、iterations README、Product Backlog 和 roadmap 的后续主线口径。
**教训**: iteration 收口检查必须从 owner 文档页头开始，再同步 README / Board / backlog。派生视图显示 Closed 不代表 owner 文档已关闭。
**Promoted to rule/check**: `docs/sop/TASK-CLOSURE.md` 和 `docs/sop/DOC-CHECK.md` 已要求 owner docs 状态同步；本次不新增规则，只把具体陷阱写入速查表。

### 2026-06-26 - 资源边界必须在读取前或迭代中生效（EVO-116 验收返修）
**现象**: EVO-116 首轮实现声明 Context API 已有 `BLOB_MAX_BYTES` / `FILE_TREE_MAX_ENTRIES` / `DIFF_MAX_ENTRIES`，测试也能看到超大 blob 返回 413；但验收复核发现 `read_blob` 先 `find_blob` + `to_vec()` 再判断大小，file-tree / diff 也是先完整遍历或完整 `Vec` 收集后再判断上限。
**根因**: 把“响应不返回完整内容”误当成“服务端没有完整读取/收集”。验收测试只断言 HTTP 413，没有覆盖 large tree / large diff，也没有检查上限判断位于读取前或迭代过程中。
**方案**: `read_blob` 改为先 `find_header` 检查 object kind 和 size，未超限才 `find_blob`；`read_file_tree` 改为自定义 bounded visitor，第 5001 个 entry 通过 `ControlFlow::Break` 取消遍历；`read_diff` 改用 `Tree::changes().for_each_to_obtain_tree`，第 5001 条 change 取消 diff iteration。`repo_context_bounds_e2e_tests` 增加 large file-tree / large diff 413 覆盖。
**教训**: 资源上限的验收不能只看最终 HTTP code；必须检查上限是在读取前或迭代 callback 中触发，而不是完整分配后再拒绝响应。新增资源边界故事必须同时覆盖 oversized object、large collection、large diff 和错误映射。
**Promoted to rule/check**: `api/tests/repo_context_bounds_e2e_tests::{test_large_file_tree_returns_413,test_large_diff_returns_413}`；`service-git::read_blob/read_file_tree/read_diff` 中的 header/visitor/callback 级边界。

### 2026-06-26 - API key 权限矩阵必须单文件归口（EVO-116）
**现象**: EVO-103 架构组 Conditional Accept 指出 read-only API key 仍可触发 Repo CRUD 写接口（之前只在 Smart HTTP handler 内部硬编码 `permissions.iter().any(|p| matches!(p.as_str(), "write" | ...))`，未覆盖 REST handler）。
**根因**: 权限字符串散落在 4 个 handler 文件（`repo_handlers` / `repo_context_handlers` / `api_key_handlers` / `git_smart_http_handlers`），任何一个 handler 漏检或新 handler 加入时容易绕开 scope 检查。`permissions` 数组还允许运营自定义 token（`commit:feature-x`），分散判断无法保证一致。
**方案**: 抽出 `backend/crates/api/src/middleware/api_key_scope.rs` 作为单一权限矩阵 + 共享 helper（`api_key_allows_repo_read/write` / `api_key_allows_api_key_management`），handler 层用 4 个 `forbid_if_api_key_lacks` 闭包调用；权限矩阵表格以 doc 形式写入 `middleware/api_key_scope.rs` 和 `docs/reference/API-CONTRACT.md` 的 API key scope section；JWT 用户绕过 helper 不受影响。13 个 helper 单元测试 + 5 个 E2E 测试覆盖 read-only / write / execute-only / unrelated / admin / JWT 不回归 6 个场景。
**教训**: API key / RBAC / scope 类权限判断必须有「单一权限矩阵 + 共享 helper」作为门禁；handler 层只能用 helper 函数不能直接读 permissions 数组。运营扩展权限（如 `commit:feature-x`）时只改 helper 即可，不会漏 handler。新增 API key scope story 的验收项必须包含"权限判断不散落为重复字符串判断；至少有共享 helper 或清晰的单一归口"。
**Promoted to rule/check**: `middleware::api_key_scope::tests`（13 单元测试） + `api/tests/api_key_scope_e2e_tests.rs`（5 E2E）作为 EVO-116 关闭条件之一；ITERATION-049 Retrospective 同步。

### 2026-06-26 - gix `rev_parse_single` 不接受 fully-qualified ref 名称（EVO-116）
**现象**: 实现 Smart HTTP push metadata 同步时，`gix::rev_parse_single(BStr::new("refs/heads/main"))` 返回 `NotFound: couldn't parse revision: refs/heads/main`。
**根因**: gix 的 `rev_parse_single` 是「revision 规格」解析器（接受 `HEAD`、`main`、`HEAD~1`、SHA 等），不接受 `refs/heads/main` 这类 fully-qualified 路径。fully-qualified ref 必须走 `repo.find_reference(name).into_fully_peeled_id()` 路径。
**方案**: `service-git::resolve_ref` 分两路处理：`refs/` 前缀走 `find_reference + into_fully_peeled_id`；其他走 `rev_parse_single`。两种路径统一返回 `ObjectId` → `find_commit` → `(oid, timestamp)`。EVO-108 indexer 触发器复用同一函数。
**教训**: gix 的 rev 解析 vs ref 解析是两套 API，写 `gix` 集成时必须先看 `find_reference` vs `rev_parse_single` 的语义边界。EVO-108 实现时应复用 `resolve_ref` 而不是再写一次。
**Promoted to rule/check**: `service-git::resolve_ref` 单元/E2E 覆盖 `main` 与 `refs/heads/main` 两种写法。

### 2026-06-26 - actix-web detached 任务 vs HttpServer graceful shutdown（EVO-116）
**现象**: 写 Smart HTTP push metadata E2E 时，第一次实现用 `oneshot::Sender` 在 push 后立即发送 shutdown 信号，wait loop 等 10s 仍未看到 `last_commit_sha` 更新。
**根因**: `actix_web::rt::spawn` 把 metadata update 任务 spawn 在当前 worker 的 runtime 上；HttpServer 的 `oneshot::Sender` 触发 shutdown 时，actix-web 只等待「当前 in-flight request handler future」完成，不会等待 detached spawn。worker runtime 在 request handler 返回后即可被回收，metadata update future 在 `child.wait()` 阶段或 `update_last_commit` SQL 阶段被 drop 掉。
**方案**: 集成测试不再显式发送 shutdown；让 `_shutdown_tx` 变量在 test 函数末尾自然 drop（drop 不触发 server shutdown，receiver 仍在 spawn 里），test 函数返回时 actix-rt runtime 析构 → AppState 析构 → pool 析构 → server 析构，metadata update 有完整生命周期。生产环境的 HttpServer graceful drain 路径未来需要在 worker shutdown 前手动 drain detached tasks。
**教训**: 写「HttpServer 内 detached spawn 后台任务」的测试时，不要主动 shutdown server；如果必须验证 detached 副作用，让 test 函数自然退出 + TempDir 析构清理资源。生产环境对 detached task 的 graceful drain 仍未实现，需要在后续 worker lifecycle 改造时补齐。
**Promoted to rule/check**: `api/tests/repo_context_bounds_e2e_tests::test_push_updates_repo_default_branch_metadata` 测试 pattern 作为未来类似场景的模板。

### 2026-06-26 - Resource bound + 错误映射必须 handler 层做（service-git 不应返回 HTTP code）（EVO-116）
**现象**: EVO-103-C 实现把 `GitStorageError::ReadError(String)` 一律映射为 500。gix 内部对 invalid ref / invalid blob SHA 也会返回 `ReadError`，导致 4xx 类输入错误被吞成 500，client 无法区分「我输错了」与「服务挂了」。
**根因**: service 层把 gix 错误混在一起包成 `ReadError(String)`，没有区分「输入错误 / 资源错误 / 内部错误」三种语义；handler 层只看到 `ReadError`，没有信息做精准 HTTP 映射。
**方案**: `service-git` 新增 4 个语义化错误变体：`InvalidInput(String)` / `NotFound(String)` / `ResourceExceeded(String)` / `Timeout(String)`，gix 错误根据错误消息和错误类型（`find_blob` 返回 not found 时映射为 `NotFound`；`rev_parse_single` 解析失败映射为 `NotFound`；blob/file-tree/diff 超过常量上限映射为 `ResourceExceeded`；`web::block` 外层 `tokio::time::timeout` 触发时映射为 `Timeout`）。`repo_context_handlers::map_context_error` 把 4 个新变体映射为 400/404/413/504，其余 `ReadError` 仍为 500。`docs/reference/API-CONTRACT.md` 新增 4 个错误码。
**教训**: service 层的错误枚举必须按「HTTP 语义」切分（`InvalidInput` / `NotFound` / `ResourceExceeded` / `InternalError` / `Timeout`），不要让 handler 层靠错误消息字符串做分类。新增 service 错误变体时同步更新 API contract 的错误码表格。
**Promoted to rule/check**: `service-git` 错误枚举 7 个变体（`InitError` / `SeedWriteError` / `RemoveError` / `NoBasePath` / `SubprocessError` / `InvalidService` / `ReadError` + `ResourceExceeded` / `InvalidInput` / `NotFound` / `Timeout`），每个变体在 `repo_context_handlers::map_context_error` 有显式 HTTP 映射。

### 2026-06-26 - Smart HTTP 安全不变量必须有代码级门禁和测试

- Trigger: EVO-103-B-2 架构评审发现 ADR/item file 声明了 subprocess timeout，且 backlog 已勾选完成，但 `service-git` 实现没有 timeout；同时 Basic-Auth API key 解析后丢弃 `permissions`，read-only key 会被提升为可 receive-pack 的 tenant member。
- Symptom: 文档声称 `tokio::time::timeout` 防挂起，实际 `git` subprocess 可无限等待；现有 API key 默认权限为 `read`，但 Smart HTTP 只检查 tenant_id，相当于让 read-only key 获得 push 能力。
- Root cause: 安全不变量写在 ADR/验收里，但没有落到 service 边界和 handler 测试；RBAC middleware 只把 API key 解析成 `CurrentUser`，没有把原始 key 权限继续传递给协议 handler。
- Fix: `service-git` 对 advertise refs 使用 `tokio::time::timeout` + `kill_on_drop(true)`；RPC streaming 增加 timeout、stderr drain 和 timeout error；RBAC 把解析出的 `ApiKey` 放入 request extensions，Smart HTTP handler 按 `permissions` 拦截 read-only key 的 receive-pack。新增 `service-git` timeout 单测和 read-only receive-pack forbidden handler 测试。
- Prevention: Smart HTTP / subprocess / auth 相关 ADR 里的安全不变量必须对应代码级门禁和测试，不能只在 backlog 勾选；协议 handler 需要权限细分时，不要只依赖降维后的 `CurrentUser`。
- Promoted to rule/check: `service-git` timeout test；`git_smart_http_e2e_tests::test_receive_pack_with_read_only_api_key_is_forbidden`；EVO-103-B-2 / ITERATION-046 验收记录同步。

### 2026-06-25 - actix-web 全局中间件 async 预处理 soundness 陷阱

- Trigger: EVO-103-B-1 实现全局 RBAC 中间件，需要在调用内部 service 前做 async DB 查找（API-key 解析）。
- Symptom: 第一次实现用 `UnsafeCell<S>` + `unsafe { &mut *self.service.get() }.call(req)`——测试通过但存在并发 UB（共享中间件实例上的重叠 `&mut`，clippy 不报）。第二次用 `tokio::sync::Mutex<S>` 锁住 `srv.call(req).await`——sound 但把整个服务器串行化（全局中间件实例被所有请求共享，持有锁跨越整个下游调用 = 一次只能处理一个请求）。
- Root cause: `Transform/Service` 的 `let fut = self.service.call(req)` 模式只支持 sync 预处理；`&mut self.service` 在 `.await` 后不可用（self 被 borrow 跨越 await point）。UnsafeCell 绕过 borrow checker 但引入 UB；Mutex 绕过 UB 但引入串行化。
- Fix: 使用 `actix_web::middleware::from_fn` + 签名 `async fn rbac_middleware<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<B>, Error>`。async 查找在 `next.call(req)` 之前执行，`next: Next<B>` 在 `.await` 后仍可用（不在 self 上），无锁无 unsafe，全并发。配置（JWT secret、api_key_repo）从 `app_data::<web::Data<AppState>>()` 读取而非闭包捕获。
- Prevention: Driver 实现 actix 中间件且需在调用内部 service 前做 async 预处理时，应在 prompt 中明确要求 `from_fn` + `Next<B>` 签名。Navigator 审查时应检查：(1) 无 `UnsafeCell` + `unsafe` 访问内部 service；(2) 无 `Mutex` 锁住整个 `call(req).await`；(3) 配置从 `app_data` 读取而非闭包捕获。
- Promoted to rule/check: 写入本条目；ITERATION-045 Retrospective 同步。

### 2026-06-25 - git repo storage_path 必须单一来源（DB 列值与磁盘裸仓路径一致）

- Trigger: EVO-103-A Navigator 审查发现 DB `storage_path` 列存 `repos/{tenant}/{name}` 而磁盘 bare repo 实际路径为 `{base_path}/{tenant_id}/{repo_id}.git`，两者不一致。
- Symptom: DB 记录的 `storage_path` 与磁盘实际裸仓路径不同。后续 Smart HTTP（EVO-103-B）或 Context API（EVO-103-C）读取 `storage_path` 定位仓库时会指向错误路径，导致 clone/push/file-tree 等操作失败。
- Root cause: Driver 实现 git 存储时未显式约束"DB 存储列必须与磁盘 init 路径一致"，create handler 中 DB 写入和 `gix::init_bare` 使用了不同的路径拼接逻辑。
- Fix: 统一 SQLite 和 PostgreSQL 两处 create repository 的 `storage_path` 为 `{base_path}/{tenant_id}/{repo_id}.git`，与磁盘 `gix::init_bare` 路径完全一致。EVO-103-A 收口前修复并验证。
- Prevention: Driver 实现 git 存储相关功能时，应在 prompt 约束中显式要求"DB storage_path 列值必须与磁盘 bare repo 实际路径一致，格式为 `{base_path}/{tenant_id}/{repo_id}.git`"。Navigator 审查时应将 storage_path 一致性作为必查项。
- Promoted to rule/check: 写入本条目；后续 EVO-103-B/C 的 Driver prompt 应引用此经验。

### 2026-06-25 - Git-Centric 方向修复必须同时检查默认策略、状态源和环境假设

- Trigger: 2026-06-25 方向变更全面评审发现：EVO-101/102 已完成但主表仍标 Proposed，repo 默认策略与“默认 require review”冲突，sandbox execute 仍在稳定 reference 中作为当前能力出现。
- Symptom:
  - 新建 repo 在缺失 `.evolith/policy.yaml` 时会回落到 `auto_merge=true` / `require_review=false`。
  - `PRODUCT-BACKLOG.md` / EVO-100 子项表 / BOARD / Roadmap 对 EVO-101/102/EVO-103 的状态与依赖顺序不一致。
  - `cargo test --workspace` 在当前环境中因 `docker_provider_new_fails_without_docker` 失败；测试把“没有 Docker”当成固定事实。
- Root cause:
  - Git-Centric 方向文档写了安全策略，但 schema、repository、UI 默认和测试没有一起改。
  - Iteration 042 的完成事实没有同步到所有 owner docs。
  - Sandbox 已进入 ADR-0005 废弃路径，但 legacy 测试仍把宿主环境属性当断言前提。
- Fix:
  - 新增 SQLite/PostgreSQL migration 009，把 `git_repos` 默认改为 `auto_merge=false` / `require_review=true`。
  - 更新 SQLite/Postgres repository create 默认值、git repo tests 和 EVO-112 UI 默认。
  - 同步 PRODUCT-BACKLOG、EVO-100、BOARD、Roadmap、Iterations README、AGENTS、reference docs 和 manifest risk gates。
  - 修正 sandbox legacy 测试，只在初始化失败时校验 Docker 错误语义，不再要求当前机器必须没有 Docker。
- Prevention:
  - 方向文档里的安全默认值必须追踪到 migration、repository、frontend spec、API evaluator 和测试。
  - 子 Story 完成后必须同步父 Epic 子项表、主 backlog、Board 和 roadmap；断链扫描应作为文档治理收口证据。
  - 环境依赖测试不得断言宿主机一定缺少或拥有某外部服务；需要按探测结果分支或使用本地可控 mock。
- Promoted to rule/check:
  - `.agent-governance/manifest.yaml` 增加 Git-Centric risk gates。
  - `docs/backlog/active/EVO-113-direction-pivot-review-remediation.md` 记录闭环证据。

### 2026-06-23 - 开发目标重大变更：从 Skill/CLI/MCP Registry 转向 Git 托管 + Vibe Coding 平台

- Trigger: 用户在 2026-06-23 提出方向调整：删除 skill 执行 + 智能体执行能力；保留 mcp/cli 作为 FaaS；重心转向 git 仓库（嵌入 git 引擎，类 GitHub 代码管理）；skill/mcp/cli 存储基于 git 仓库联动；git 仓库外挂外部 agent engine。
- Symptom: 原定位"企业级 AI Agent Harness 平台"在 Phase 0-7 实施过程中累积了 Phase 7 sandbox + ExecutionProvider 三种 payload + 多条 skill lifecycle 故事（EVO-019/020/027/028/029/045/046/047/049/050），技术债务与用户实际使用场景（vibe coding、agent 远端调用）出现错位；用户明确表达"git 仓库本身可以是普通项目"（GitHub Pages ↔ GitHub Repo 类比），原 per-resource 仓模型与"通用 git 托管"心智不符。
- Root cause: (1) 原实施路径（Phase E Skill 生命周期 / CLI 友好接口完整性）默认"DB-centric registry"假设，未考虑"git-centric substrate" 替代。(2) 当外部生态变化（agent engine 由用户其他项目提供）时，原"自建执行环境"的假设需要重新审视，但未在 Phase 0-7 过程中触发重新评估。(3) 把 ExecutionProvider 设计为多 payload 抽象（Code / Command / HttpProxy），但实际只有 HttpProxy 用得上；过度抽象导致后续清理成本。
- Fix:
  - 写 `docs/proposals/GIT-CENTRIC-PLATFORM.md`（设计稿，11 节），定位主线为 git 托管 + vibe coding + Pages 式 skill/CLI/MCP 索引。
  - 写 `docs/decisions/ADR-0004-git-centric-storage.md`（DB content 列 → git 文件指针）+ `docs/decisions/ADR-0005-deprecate-sandbox-runtime.md`（删除 sandbox，仅保留 FaaS）。
  - 创建 EVO-100（Epic：Git-Centric Platform Foundation）+ 10 个子 Story（EVO-101~103、105~111）+ EVO-104（Vibe Coding Web UI，含 UX 调研前置门禁 U-01~U-13）。
  - 旧 backlog 项（EVO-019/020/027/028/029/045/046/047/049/049-B/050）按 Superseded/Dropped 移至 Archived Index，决策上下文记录。
  - `IMPLEMENTATION-ROADMAP.md` 中 Phase E 整体替换为 Phase E'（Git 托管 + Vibe Coding），分 4 个子阶段（E'-1 Git Service 基础 / E'-2 Agent 集成 + Vibe Coding 形态 / E'-3 Indexer + Discovery + 旧表双写 / E'-4 Sandbox 废弃收尾）。
  - `PROPOSALS` 中 AI-GATEWAY.md / AGENT-RUNTIME.md 状态标注"整合进 GIT-CENTRIC-PLATFORM"；SERVERLESS-RUNTIME.md 保留作历史参考，ExecutionProvider 大幅简化。
- Prevention:
  - 当用户明确表达"心智模型应该是 X"（如本次的 GitHub Pages 类比），先回到心智模型本身重新设计架构，不要套用既有 backlog / phase。
  - 重大方向变更必须直接走 `proposal → ADR → backlog` 流程，禁止绕过。
  - Iteration / Phase 之间的"假设复审"应有显式检查项（如"外部执行环境是否仍由我们提供？"）。
- Promoted to rule/check:
  - `AGENTS.md` Task Router 增加"Git-Centric Platform"入口。
  - `PRODUCT-BACKLOG.md` / `IMPLEMENTATION-ROADMAP.md` / `BOARD.md` / `EVOLUTION.md` / `docs/README.md` 已同步。
  - UX 调研前置门禁写入 EVO-104（U-01~U-05 P0 阻塞）。
  - sandbox 整层删除路径写入 EVO-111 + ADR-0005。

### 2026-06-05 - sandbox fail-fast 不能和默认启用混用

- Trigger: 用户指出普通启动现在依赖 Docker。
- Symptom: `EVO-056` / `EVO-045-A` 将 sandbox 初始化失败改为 fail-fast 后，默认/lite 启动也进入 Docker provider 初始化和 prewarm。
- Root cause: 修复“Docker 不可用时静默伪成功”时只收紧了启用态失败语义，没有同步把 `sandbox.enabled`、`.env.development` 和 `.env.example` 默认值改为 `false`。
- Fix: `AppConfig`、本地 env 示例和稳定参考文档默认关闭 sandbox；显式 `SANDBOX__ENABLED=true` 时继续 fail-fast。
- Prevention: 配置默认值必须匹配 `docs/sop/LOCAL-DEV.md` 的 lite/local 依赖声明；改变 fail-fast 行为时同时检查默认 env、示例 env、AGENTS trap 和 CONFIG/TECH-STACK/ARCHITECTURE。
- Promoted to rule/check: `AGENTS.md` known trap、`docs/sop/LOCAL-DEV.md`、`docs/reference/CONFIG.md`、`docs/sop/EVOLUTION-FEEDBACK.md`。

### 2026-06-05 - skill 更新后要复核标准结构新增入口

- Trigger: 用户提示 skill 已更新，要求重新载入并纠偏。
- Symptom: 项目 governance validator 通过，但更新后的 `agent-project-governance` 1.0.7 标准结构要求 `CLAUDE.md` 和 `GEMINI.md` 单行 redirect，本项目缺失。
- Root cause: validator 尚未覆盖新版 redirect 要求，且项目上次收口只按旧 validator 和旧结构检查。
- Fix: 新增 `CLAUDE.md` / `GEMINI.md`，同步 `.agent-governance/manifest.yaml`、`docs/README.md` 和 backlog `EVO-083`。
- Prevention: 用户提示 skill 更新时，必须重新读取 `SKILL.md` 和直接相关 references，不能只跑 validator。
- Promoted to rule/check: `EVO-083`；后续可推动治理 validator 增加 redirect 检查。

### 2026-06-05 - 点名的 skill reference 必须补读后再纠偏

- Trigger: 用户追问是否读到 `backlog-compaction` 内容。
- Symptom: 已重新载入 `agent-project-governance` 1.0.7，但只读了 `SKILL.md`、closure、standard-structure 和 initialization，遗漏 `references/backlog-compaction.md`，导致 backlog 仍停留在 monolithic 结构。
- Root cause: 把新版 skill 的显性要求当成完整变更范围，没有继续读取用户点名且与当前任务直接相关的 reference。
- Fix: 补读 `references/backlog-compaction.md`，将 `PRODUCT-BACKLOG.md` 压缩为决策入口，新增 `docs/backlog/active/` item files，并将 archive 拆为短 `INDEX.md` + per-item archived files；同步 intake/doc-check 规则。
- Prevention: 用户点名 skill 的某个 reference、术语或流程时，必须读取该 reference 后再判断纠偏范围；不能用 validator 通过替代 reference 读取。

### 2026-06-05 - Archive index 不能变成新的 backlog dump

- Trigger: 用户指出 `docs/backlog/archive/2026-Q2` 的内容看起来仍不符合要求。
- Symptom: `PRODUCT-BACKLOG.md` 已压缩，但 `archive/2026-Q2/INDEX.md` 仍有 1666 行，集中承载所有 archived detail snapshots。
- Root cause: 执行 compaction 时只满足了主 backlog 变短，没有落实 `archive/<period>/<item>.md` 的归档 item file 形态。
- Fix: 将 54 个 archived detail sections 拆为独立 item files，`INDEX.md` 缩短为索引，主 backlog archived links 指向具体 item file。
- Prevention: backlog compaction 的验收必须同时检查主入口行数、active item files、archive per-item files 和 INDEX 是否只做索引。
- Promoted to rule/check: `docs/sop/REQUIREMENT-INTAKE.md`、`docs/sop/DOC-CHECK.md`、EVO-084。

### 2026-06-03 Governance board 必须按 skill 标准放在 docs/BOARD.md

**现象**: 用户要求“本项目没有做看板”，最初误判为前端看板页；随后又把治理看板放到
`docs/backlog/KANBAN.md`。用户指出“不太对，要读一下 skill 的内容”。

**根因**: 只读了 agent-project-governance skill 的入口和本项目现有 SOP，没有继续读取
`references/standard-structure.md` 中对 Board 的 artifact responsibility：Board 是可选的
`docs/BOARD.md` 派生运营视图，不能成为 backlog 子文档或新的状态源。

**方案**: 删除误放的 `docs/backlog/KANBAN.md`，新增 `docs/BOARD.md`；Board 只使用
`Item / State / Owner Doc / Gate` 四列，并同步 `docs/README.md` 与 `AGENTS.md` 检查项。
通过 markdown 链接检查、governance validator 和 `git diff --check` 后关闭 EVO-077 /
Iteration 036。

**教训**: 使用治理 skill 时，不能只读 SKILL.md 开头和本地 SOP 就落文件；涉及新治理
artifact 时必须读 `standard-structure.md`，确认 owner、职责边界和“不应成为”的反例后再改。

### 2026-06-01 clippy `#[lints]` in Cargo.toml 优先级高于 `clippy.toml` 配置

**现象**: Iteration 029 / EVO-059 修复 clippy 21 个 `-D warnings` 错误时，7 个 infra test
文件因 `.unwrap()` 调用触发 `unwrap_used` 错误。`backend/clippy.toml` 已包含
`allow-unwrap-in-tests = true`，但 clippy 仍然报错（错误信息明确写 "requested on the
command line with `-D clippy::unwrap-used`"）。

**根因**: clippy 配置优先级（高→低）：

1. 命令行显式 `-D` / `-A`（最高）
2. `[lints.clippy]` in `Cargo.toml`（`unwrap_used = "deny"` 是这一层）
3. `clippy.toml` 设置（`allow-unwrap-in-tests` 在这一层）
4. clippy 默认行为

当 `cargo clippy --workspace --all-targets -- -D warnings` 运行时，第 1 层的 `-D warnings`
触发了第 2 层的 `unwrap_used = "deny"`，二者都覆盖了第 3 层的 `allow-unwrap-in-tests`。
`clippy.toml` 看似"配置了"但实际不生效。

**方案**: 在需要 unwrap 的 test 文件顶部加文件级 `#![allow(clippy::unwrap_used)]` + 注释
解释 workspace deny 覆盖 clippy.toml 行为。`Cargo.toml` 级别 deny 保留（生产代码仍受限），
test 文件显式豁免。

**教训**: 不要假设 `clippy.toml` 的 `allow-*-in-tests` 系列会自动生效——只要
`[lints.clippy]` in Cargo.toml 设了 deny/specific 规则，clippy.toml 就会被 override。
CI 启用 `-D warnings` 后这是高频陷阱。Test 文件级 allow 是当前项目唯一可靠解。

**相关**: Iteration 029 / EVO-059

### 2026-06-01 CI trigger 选型：tag-only 优于 push-on-main

**现象**: Iteration 029 设计 CI workflow 时，用户明确说"CI 按照 tag 触发，不要每次
提交都触发了吧"。这是关于"何时跑 CI"的策略选型。

**权衡矩阵**:

| Trigger | CI 配额 | 反馈延迟 | PR review 集成 | 适合场景 |
|---------|---------|----------|----------------|----------|
| `push: branches: [main]` | 高（每次 commit） | < 1 min | 需配合 PR 流程 | PR review 模型 + trunk 频繁合并 |
| `push: tags: ['v*.*.*']` | 低（每次 release） | N/A | 不支持 PR | release-driven / trunk-based / 无 PR 流程 |
| `pull_request` | 中（每个 PR） | 提交后 | 是 | 主仓 PR review 模型 |
| `workflow_dispatch` | 手动 | 手动 | N/A | 一次性验证 / 维护性检查 |

**本项目决策**: tag-only。理由：

- trunk-based：项目无 PR 流程（main 是 trunk，feature 走本地 commit 或独立分支）
- release-driven：发布由 tag 触发验证
- 节省 CI 配额：每发一版跑一次 CI 而非每 commit
- tag pattern 选 `v*.*.*`（严格 semver）比 `v*`（任意 v- 前缀）更精准

**教训**: 在 trunk-based / 无 PR review 流程的项目里，tag-only 是 CI trigger 的
默认最佳选择。`pull_request` 触发器只在有 PR 流程时才有意义。如果项目计划加入 PR 流程，
再补 `pull_request` 触发器即可，不影响 tag-only 主线。

**相关**: Iteration 029 / EVO-030

### 2026-06-01 rustfmt 历史 nightly-only 配置在 stable toolchain 会被静默忽略为 warning

**现象**: `backend/rustfmt.toml` 含有 7 个 `Warning: can't set \`xxx\`, unstable features are only available in nightly channel.`
的配置项（`wrap_comments` / `comment_width` / `normalize_comments` / `fn_single_line` / `where_single_line` /
`imports_granularity` / `group_imports`），但 `cargo fmt --check` 没有直接报错，34 个文件
仍然显示格式 diff。stable rustfmt 1.9.0 在每次 rustfmt 进程都打印这些 warning，但最终
退出码 1 是因为文件内容未通过 stable 默认行为校验。

**根因**: 这些选项曾经或仍是 nightly-only，但项目里没有切到 nightly toolchain；rustfmt
在 stable 模式静默忽略它们，配置文件实际上"形同虚设"。如果代码是用 nightly 工具链或旧
stable 版本（含 stable 版）格式化的，移除选项后会有大量 stable-vs-historical diff。

**规则**: 执行任何 rustfmt baseline 任务时，第一步必须先跑 `cargo fmt --check`，看输出
里是否含 `unstable features are only available in nightly channel` 警告。**有警告**意味着
`rustfmt.toml` 部分选项未生效，需要先决策：
1. **优先 stable**：直接删除 nightly-only 选项，然后 `cargo fmt --all` 应用 stable 默认
   行为；diff 数量取决于历史格式与 stable 差异。适合 CI 重建、团队开发。
2. **保留 nightly 行为**：在 `rust-toolchain.toml` 切到 `channel = "nightly"`，并在 CI 中
   准备好 nightly 工具链。适合对代码风格有强约束的项目。

本项目选 (1) stable-only，理由：CI 重建（Iteration 029）依赖 stable 命令、nightly 工具链
安装成本高、stable 默认行为对功能正确性无影响。

**相关**: Iteration 028 / EVO-033

### 2026-06-01 嵌入式前端完成后要同步运行时配置和计划状态

**现象**: Iteration 031 完成 `rust-embed-for-web` 迁移后，`/config.js` 只在后端提供，
但前端未加载也未读取 `window.__EVOLITH_CONFIG__`；默认 API base URL 还错误地从
`CORS__ALLOWED_ORIGIN` 拼接。同时迭代目录仍把被覆盖的 Iteration 030 写成 Active，
EVO-016-A / Iteration 024 仍显示可激活，和已完成的 EVO-016-B 状态冲突。
**根因**: 实现收口关注了静态服务和缓存验证，但没有把运行时配置注入路径、构建时
fallback、future iteration 库存和 roadmap/reference 的终局状态一起同步。
**方案**: 前端入口加载 `/config.js`，`config.ts` 优先读取运行时配置；后端默认注入
同源 `/api/v1`，并补充 `APP__API_BASE_URL` 参考。删除被替代的 `frontend.zip`，
将 EVO-016-A / Iteration 024 标为被 Iteration 031 覆盖，并同步 roadmap、architecture、
release 和 iteration inventory。
**教训**: 交付形态从过渡策略切到终局后，要同时检查代码路径、运行时配置、旧制品、
计划库存和稳定事实；只验证 HTTP 响应头不能证明部署形态已经完全收口。

### 2026-05-28 方案选型应先广后深，避免对单一方案过度优化

**现象**: EVO-016 前端嵌入方案从一开始就聚焦在 ZIP 上，经历了基础嵌入（EVO-016-A）、
静态索引优化、zip crate 2.4→8.6 升级、OnceLock + Arc<ZipArchiveMetadata>、
web::block + channel 流式架构设计等大量深度工作。最终调研发现 rust-embed-for-web
直接消除了整个问题域。

**根因**: 方案选型时没有先做广度调研（grep 所有可选方案），而是直接沿用参考项目的
ZIP 方案并持续优化。对 ZIP 的深度优化本身就是浪费——如果先花 10 分钟广度搜索
"Rust embedded static files" 就会发现 rust-embed-for-web。

**规则**: 涉及技术方案选型时，必须先做广度调研（至少 3 个替代方案），
再对最优方案深度验证。具体步骤：
1. 明确问题域和约束
2. 广度搜索所有可行方案（librarian 交叉验证）
3. 快速对比淘汰到 1-2 个候选
4. 深度验证最终候选
5. 写 ADR 记录决策

**相关**: [ADR-0003](docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md)

### 2026-05-28 rust-embed-for-web 嵌入方案需要 build.rs 监听 dist 目录变更

**现象**: 用 `rust-embed-for-web` 替换 ZIP 方案后，只改前端代码（`bun run build` 产出新 dist），
`cargo build --release` 显示 0.66s 完成，实际二进制中嵌入的仍是旧版前端资源。
浏览器打开页面看不到前端变更。

**根因**: `rust-embed-for-web` 的 proc macro 在编译时读取 `#[folder]` 指向的文件并嵌入二进制。
Cargo 的增量编译只跟踪 Rust 源码（`.rs`）和 `build.rs` 输出的 `cargo:rerun-if-changed` 指令。
前端 dist 目录变更不触发 proc macro 重新执行。

**方案**: 在 `backend/build.rs` 中添加 `cargo:rerun-if-changed=../frontend/dist/index.html` 和
`cargo:rerun-if-changed=../frontend/dist/assets/` 指令。前端 dist 文件变更时 cargo 检测到
rerun 触发条件，重新编译包含 `#[derive(RustEmbed)]` 的 crate。

**规则**: 任何使用编译时文件嵌入（`include_bytes!`、`rust-embed`、`rust-embed-for-web`）
的 Rust 项目，如果嵌入内容来源不是 `.rs` 文件，必须在 `build.rs` 中声明 `cargo:rerun-if-changed`
指向实际嵌入文件或目录。

**相关**: [ADR-0003](docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md)、Iteration 031

### 2026-05-28 Story 格式要按任务性质分型，而不是机械套用户故事

**现象**: 讨论传统敏捷、Sprint、用户故事和 BDD 时，如果只引入通用 Scrum/BDD
概念，Agent 容易把技术债、治理修复、Spike 和用户可见功能都写成同一种
`作为...我希望...以便...`，或者把 BDD 场景当成空泛模板。
**根因**: Evolith 的 iteration 是可审计工作批次，既包含产品行为，也包含技术、
治理和计划基线修复；传统用户故事格式只适合行为类产出，不能覆盖所有 Agent
执行任务的验收和闭环要求。
**方案**: 以 EVO-041 / Iteration 022 明确 Sprint 与 Evolith iteration 的关系，
并将 Story 分为 Product / API / Technical / Governance / Spike；行为类工作使用
Given/When/Then，非行为类工作使用命令级或一致性验证、状态同步和残余归口。
**教训**: 用户故事规范的核心不是统一句式，而是让身份、目标、价值、范围、不做、
验收和验证都可判断；BDD 只强制用于行为验收，技术和治理工作必须有等价证据。

### 2026-05-28 Governance manifest 是 skill adoption 的可验证入口

**现象**: Evolith 已有完整 AGENTS、SOP、backlog、iteration 和经验记录，但运行
`agent-project-governance` bundled validator 时仍失败，原因是缺少
`.agent-governance/manifest.yaml`。
**根因**: 项目事实上已经采用治理结构，但没有写入 skill 可机械识别的 adoption
manifest；只有文档存在不足以让外部 skill 判断 profile、capability 状态和入口映射。
**方案**: 以 EVO-035 / Iteration 023 补齐 manifest，记录 `high-risk / conformant`
profile、标准 entrypoints、capabilities、风险门禁和迁移映射，并把 validator 作为
后续治理变更后的固定验证。
**教训**: 面向可复用治理 skill，manifest 是“已初始化”的机器可读证据；新增或改变
治理能力后，必须同步 manifest 并运行 validator。

### 2026-05-27 开始迭代必须先盘点既有 iteration

**现象**: 流程已经禁止覆写已发布计划，但用户要求“开始迭代”时，启动 SOP 仍先围绕
backlog 候选 story 展开，不能保证 Agent 先发现仍显示 `In Progress` 的 Iteration 010
或仍为 `Planned / Blocked` 的 Iteration 012。
**根因**: 规则保护了单份计划文档的基线，却没有定义 iteration 集合的启动优先级；
story WIP 检查不能替代对在途、待收口和已排期 iteration 的库存盘点。
**方案**: 登记 EVO-039 / Iteration 016；新增 iteration inventory gate，要求先为
`Active / In Progress / Review / Planned / Blocked` 作 disposition，再允许从 backlog
选新 story；将 Iteration 010 修复为待收口 Review，并同步规则到治理 skill。
**教训**: backlog 回答“还有什么可做”，iteration inventory 回答“已经承诺或正在做
什么”；开始新工作前必须先回答后者。

### 2026-05-27 治理 skill 必须为弱闭环模型提供机械收口协议

**现象**: skill 已能说明初始化、迁移和缺陷写回方法，但依赖使用模型自行判断是否完成；
能力有限的模型可能只生成部分文件就宣称任务完成，遗漏状态同步、验证证据与残余登记。
**根因**: 方法论指引覆盖了正确方向，但缺少加载即执行的完成门禁、部分完成表达方式和
面向失败模式的评估用例。
**方案**: 登记 EVO-037，在 `agent-project-governance` skill 中新增强制闭环契约、
低自由度闭环协议与过早完成评估场景；随后以 EVO-038 将同一原则落实到 Evolith
本地 `TASK-CLOSURE.md` 及入口/迭代/Git/审查流程，要求实施任务经过建账、执行、
核验、同步和交付。
**教训**: 面向能力不稳定的 Agent，关键流程不能只说明原则，还必须提供不可跳过的
停止条件、证据要求和未完成时的明确报告格式。

### 2026-05-27 已发布迭代计划必须保留为执行对照基线
**现象**: 已发布的 `Iteration 011` 原计划用于 EVO-016 Embedded Frontend refinement，外部实施时却直接将同一文件改写为 EVO-010/EVO-011 完成记录，导致原目标、依赖关系和后续 `Iteration 012` 的前置依据在当前文档中消失。
**根因**: 现有 SOP 约束了“开始前建计划”和“中途变更记录”，但没有定义 future `Planned` 文档提交后即成为不可覆写的计划基线，也没有要求启动时目标变化必须使用新的 iteration 编号。
**方案**: 新增 EVO-036；在 AGENTS、Start/Iteration/Change-Control/Doc-Check/Git SOP 与 iteration 模板中规定计划基线保护、改线新编号和后续依赖阻塞；为历史偏差补回基线说明。
**教训**: 迭代文档不仅记录结果，也保留计划与实际之间的差异证据；已发布计划被其他工作占用时，应追加偏差并另开编号，而不是覆盖原计划。

### 2026-05-26 Epic 拆分不能只定义大小阈值
**现象**: 需求进入 SOP 仅说明“超过 0.5-2 天或多个切片时作为 Epic”，但没有定义 Epic 与 Story 的职责差异、父子编号、依赖校验、分层 DoR 或跨 Epic 迭代选择方式；维护者无法稳定判断如何记录大需求。
**根因**: 初始规则是为一次既有大项拆分提供最低限度约束，沿用了 `EVO-002` 到独立 `EVO-021` 至 `EVO-025` 的历史记录方式，没有抽象成可复用需求层级模型。
**方案**: 以 EVO-034 / Iteration 008 补充 Epic / Story 方法论：新父子关系使用同前缀后缀编号、子项依赖闭包与差异化 DoR、跨 Epic 选取约束和文档一致性检查；同步抽象到治理 skill。
**教训**: 当 backlog 开始承载多阶段工作时，SOP 不仅要说“需要拆”，还必须规定关系标识、依赖和进入迭代的门槛；历史 ID 可以保留，但新规则要能直接执行。

### 2026-05-25 迭代状态不能替代命令级验收证据
**现象**: Iteration 006 将 EVO-005 标为 Done，并勾选全量测试和 Clippy 通过；复查发现 `HttpToolExecutor::new()` 因隐式系统代理探测导致应用状态初始化与测试 panic，匿名请求可触发公开 HTTP 工具执行，HTTP 失败被包装为成功 result，且新增超时测试依赖公网。`cargo fmt`、`cargo clippy -- -D warnings`、`cargo test --workspace` 均未满足记录中的完成声明。
**根因**: 迭代计划和 Done 状态在实现提交之后一次性补写，流程只要求写验证结果但未要求保留命令级证据或阻止失败门禁被勾选；出站执行与 API Key 边界未被列入强制 Navigator 检查；测试 SOP 未禁止用公网服务作为验收依赖。
**方案**: 新增 EVO-032 / Iteration 007；禁用 executor 的隐式系统代理探测、强制 `tools/call` API Key、修复 HTTP error 映射并用本地 mock 服务覆盖真实调用；全量验证还暴露并修复了日志脱敏替换同一字段时的无限循环；更新 Start/Iteration/Pairing/Testing 流程，要求计划先于实现、验收逐命令记录、外部执行边界必须审查。
**教训**: `Done` 必须由可重复的命令和高风险边界测试支撑；涉及出站请求、认证或权限的故事，没有先行计划、Navigator 结论和本地稳定验证，不得标记完成。

### 2026-05-17 迭代验收需要覆盖部署路径、公开入口和终局/过渡边界
**现象**: Iteration 004/005 文档显示完成，但复查发现 Nginx `/assets/` 反代会剥离路径前缀、邀请接受接口未同步 RBAC/CSRF 公开例外、邀请邮件/URL 缺少可用前端入口、密码重置链接误用后端监听地址，且 backlog 详情块没有随 Done 状态同步。
**根因**: 验证只覆盖了本地 build/type-check 和局部 happy path，缺少“生产反代资源路径”“公开状态变更接口的 RBAC/CSRF 双检查”“邮件链接必须指向 `APP__PUBLIC_URL`”“总表与详情块一致性”的流程防呆；同时把 Nginx 静态托管误当成终局，而不是 EVO-016 前的过渡部署策略。
**方案**: 修复 Nginx assets 反代、公开邀请接受 API/前端 join 页面、RBAC/CSRF 例外、邮件公开 URL、生产 sourcemap 默认关闭和 Bun 构建链；同步更新 Contract/Testing/Release/Iteration/Git/Local Dev SOP 与 backlog 状态。
**教训**: 迭代完成不能只看本地构建；凡是公开链接、认证例外或部署路径变更，必须同时验证 API 合约、RBAC、CSRF、前端公开路由、邮件 URL、Nginx path rewrite 和 backlog 详情块。

### 2026-05-17 SOP 要区分“当前故事变更”和“迭代期间新需求进入”
**现象**: 在 EVO-021 路由适配迭代期间，用户提出 Skill 多来源导入、版本验证和 Snippets 残留处理等后续规划；这些需求需要进入 backlog/roadmap，但并不改变当前路由适配故事。
**根因**: `CHANGE-CONTROL.md` 只说明“迭代中收到需求变更”要停手记录，未明确独立新需求应回到 `REQUIREMENT-INTAKE.md`，容易把未来需求写进当前 iteration 的 change request。
**方案**: 更新 `REQUIREMENT-INTAKE.md`、`CHANGE-CONTROL.md`、`START-ITERATION.md`、`DOC-CHECK.md`、`GIT-WORKFLOW.md` 和 `LOCAL-DEV.md`，补充独立新需求分流、Epic/Story 拆分、旧概念检查、混合文档提交和本地测试账号规则。
**教训**: 迭代期间的新输入先判断是否改变当前 story；不改变当前 story 的，进入 backlog/proposal，不污染当前迭代变更记录。

### 2026-05-16 结对开发应采用分阶段角色切换
**现象**: 在讨论极限编程结对编程时，直接让同一 Agent 在同一上下文中同时扮演 Driver 和 Navigator，可能导致目标混杂、责任不清和上下文污染。
**根因**: 双角色并行适合两个人或两个独立上下文；单上下文中更需要阶段边界和检查表，而不是角色互相争论。
**方案**: 新增 `docs/sop/PAIRING-WORKFLOW.md`，采用 Driver 实现小切片、Navigator 检查、Driver 修正、Navigator 提交前检查的顺序模式。
**教训**: AI Agent 的结对开发应优先做“分阶段审查”，而不是“同上下文双人格”；Navigator 必须基于 SOP、ADR、backlog、diff 或具体风险给结论。

### 2026-05-16 全量 cargo fmt 检查存在既有格式基线问题
**现象**: 在 EVO-017 parser 改动后运行 `cargo fmt --all -- --check`，命令失败并输出多个无关 crate 的格式差异，同时 stable rustfmt 对部分 nightly-only 配置项发出 warning。
**根因**: workspace 里已有未格式化文件或 rustfmt 配置与 stable 工具链不完全匹配；全量 fmt 检查会把无关历史差异和本次改动混在一起。
**方案**: 对本次触碰的 Rust 文件单独运行 `rustfmt <path>`，再运行局部测试；最终验证记录中明确说明全量 fmt 失败原因和局部格式化范围。
**教训**: 在格式基线不干净的仓库中，不要用全量 fmt 结果判断本次改动失败；先保证触碰文件格式化，再把全量基线问题作为独立技术债处理。

### 2026-05-16 开始迭代也需要独立 SOP
**现象**: 用户要求“提交一下然后开始一个新的迭代”时，执行过程包含选择 Ready story、创建 iteration 文件、更新 backlog、处理临时需求补充和验证链接多个固定动作，但 AGENTS 只把入口指向通用迭代工作流。
**根因**: `ITERATION-WORKFLOW.md` 描述的是迭代内循环，不足以约束“开始迭代”这个跨 backlog、iterations、验证和变更控制的流程动作。
**方案**: 新增 `docs/sop/START-ITERATION.md`，并将 AGENTS Task Router 的“开始一次迭代”入口指向该 SOP。
**教训**: 只要一个动作会同时修改 backlog 和 iteration，就应有独立 SOP；否则后续 Agent 容易只建文件、不改状态或漏掉验证。

### 2026-05-16 过长 SOP 要按任务入口拆分
**现象**: `ITERATION-WORKFLOW.md` 同时承载需求准入、Backlog refinement、迭代执行、变更控制和状态定义，AGENTS Task Router 只能把多个不同任务都指向同一个长文档。
**根因**: 初次流程改造优先保证闭环，把相邻流程放在一起；随着防呆要求提升，过长 SOP 会让后续 Agent 难以定位必读步骤。
**方案**: 拆出 `docs/sop/REQUIREMENT-INTAKE.md` 和 `docs/sop/CHANGE-CONTROL.md`，`docs/sop/ITERATION-WORKFLOW.md` 只保留迭代执行主循环，并在 AGENTS Task Router 中分别索引。
**教训**: SOP 应按任务入口拆分；当一个 SOP 同时回答“需求怎么进来”和“开发中怎么变更”时，就需要拆成独立文件并让 AGENTS 明确路由。

### 2026-05-15 迭代中需求变更需要明确变更控制入口
**现象**: EVO-001 开发到一半时，产品方向从旧 snippet 概念切换为 CLI 友好接口；原 SOP 只描述了进入迭代和完成迭代，没有说明中途变更如何暂停、拆分和重定范围。
**根因**: 迭代工作流缺少 change request 处理步骤，容易把产品 pivot 混进当前故事，造成验收标准漂移和半成品代码扩大。
**方案**: 在 `docs/sop/ITERATION-WORKFLOW.md` 增加“迭代中需求变更”流程和防呆表；新增 ADR-0002 和 EVO-017；当前迭代停止扩大 snippet 对齐工作，把旧 snippet 相关事项转入迁移故事。
**教训**: 中途变更先记录、分类、重定范围，再继续写代码；产品概念变化必须用 ADR 和 backlog story 固化。

### 2026-05-16 执行流程动作前必须先查 Task Router 和 SOP
**现象**: 用户要求"添加远期需求"，Agent 直接读了 backlog 文件并在表格中插入一行，没有先查 AGENTS.md Task Router 确认正确流程。SOP 明确规定远期想法放 `docs/proposals/`，排期才进 backlog。
**根因**: Agent 把"需求进入"当成 trivial 操作跳过了流程检查，先动手后查规则。AGENTS.md Task Router 已有明确映射"需求进入/拆分/排期 → 必读 ITERATION-WORKFLOW.md"，但未被遵循。
**方案**: 回滚 backlog 错误条目，按 SOP 将远期需求写入 `docs/proposals/AI-GATEWAY.md` 并在索引中注册。
**教训**: 任何涉及流程的操作（需求进入、迭代变更、发布部署等），先查 AGENTS.md Task Router 找到必读 SOP，再动手。即使操作本身看起来简单，流程约束可能不简单。

### 2026-05-15 工程化文档需要分层入口
**现象**: 项目文档较完整，但 AGENTS.md 同时承担项目介绍、状态记录、流程说明和任务路由，后续 agent 需要读大量内容才能找到操作步骤。
**根因**: 文档按功能主题沉淀，但缺少 reference / sop / roadmap / proposals / archive 的职责边界。
**方案**: 新增 `docs/README.md` 文档地图、`docs/sop/` 标准流程、`docs/reference/` 稳定事实、`EVOLUTION.md` 经验写回；将原有专题文档迁移到新分层目录，并在 AGENTS.md 中建立任务路由。
**教训**: 启动文档应该短而硬，复杂步骤放 SOP，稳定事实放 reference，失败经验写 EVOLUTION。

### 2026-05-15 Agent 提交需要可追溯模型和变更边界
**现象**: Agent 生成的提交如果只写普通 commit message，后续难以追踪生成模型、验证范围和脚本行为变更是否同步记录。
**根因**: Git 规则没有进入项目级 SOP，提交前检查、模型标识和脚本 release notes 依赖个人习惯。
**方案**: 新增 `docs/sop/GIT-WORKFLOW.md`，要求语义前缀、提交末尾 `[model: <name>]`、提交前查看 staged diff，脚本行为变更同步 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
**教训**: AI 参与提交必须保留模型和变更边界，Git 规范要写成 SOP，而不是口头约定。

### 2026-05-29 前端概念迁移应先确认后端 API 兼容边界
**现象**: EVO-026 需要将前端 "Snippet" 概念迁移为 "CLI Interface"，但后端 `/api/v1/snippets` 路径仍需保留。
**根因**: ADR-0002 已确立 CLI Interface 方向，但后端 API 路由重命名被明确推迟到后续 story。前端迁移范围取决于后端兼容边界。
**方案**: 前端代码符号全部重命名（Snippet→CliInterface），但 API client URL 路径保持 `/snippets`；旧前端路由 `/snippets*` 添加重定向到 `/interfaces*`。
**教训**: 概念迁移实施前必须确认后端 API 兼容边界——前端符号可以改名，但 API 路径变更影响契约和客户端，需单独故事处理。

### 2026-06-01 API 客户端死代码处置：删除 > 修复 > mock
**现象**: EVO-055 item 1 — `frontend/src/lib/api/members.ts` 包含 `acceptInvitation(data)` 方法，调用 `POST /auth/accept-invite`（后端真实路由为 `POST /api/v1/invitations/accept`），实际唯一调用方 `app/join/page.tsx:43` 走的是 `authApi.acceptInvitation`（已正确路由）。
**根因**: 早期 API 客户端构建时按"功能完整"思路创建了成员和认证两个平行入口，但实际前端只需要认证侧入口。
**方案**: 三个候选 —— (A) 修改 URL 改对路径；(B) 保留并加 mock；(C) 删除整个 `acceptInvitation` 方法。最终选 C：死代码 = 不应存在的代码，删除比"修复死 URL"更彻底消除静默失败风险（TypeScript 编译时即可见）。
**教训**: 当 API 客户端方法无 UI 调用方时，优先删除该方法（连同 JSDoc），而非"修对 URL"或"加 mock 假装工作"。下次再有人需要 PUT /api/v1/snippets/{id} 时，他们会发现这个方法不存在，从而主动与后端团队确认是否实现，而非假设它"工作"。

### 2026-06-01 后端未实现端点的"前端禁用 + 标注"模式
**现象**: EVO-055 item 2 — 8 个 billing/payment-method 端点后端未实现（service-payment 尚未接入），但前端 billing 页面、PaymentMethods 组件仍调用这些端点，console 持续 404 噪音。
**根因**: 计费模块按 Phase 7 计划本应完成，但 Phase 8 之后该模块未真正实现，而前端 UI 已按计划暴露入口。
**方案**: 三个候选 —— (A) mock 数据返回假装成功；(B) 后端加 stub 路由返回 501；(C) 前端模块级 `const BILLING_ENABLED = false` 闸门 + 静态「待计费」占位页（保留 i18n key 和 `BILLING_ENABLED` 常量作为后续 flip 入口）。最终选 C：UI 闸门 + 静态占位页是诚实的实现，避免后端返 200 mock 数据误导用户，也避免 404 噪音。
**教训**: 对于"暂未实现但已暴露在 UI"的端点，"模块级 const ENABLED = false + 静态占位页"是最轻的"前端禁用 + 标注"模式。后续接入真实实现时仅需 flip 常量 + 删除占位页，无需重构调用方。

### 2026-06-01 静默 501 API 客户端的"显式 throw"模式
**现象**: EVO-055 item 3 — `cliInterfacesApi.update()` 调用 `PUT /snippets/{id}`，后端 handler 返回 `HttpResponse::NotImplemented()` (501)。前端 axios 抛出后仅显示通用 error 文本，调用方难以发现"是端点未实现"还是"参数错误"。
**根因**: 后端 handler 是 stub 状态但仍返回 200-shape JSON，前端无法区分"业务错误"和"端点未实现"。
**方案**: 把 `cliInterfacesApi.update` body 改为 `throw new Error("CLI interface update is not yet implemented. Backend returns 501 for PUT /snippets/{id}. Use cliInterfacesApi.delete() + cliInterfacesApi.create() as a workaround.")`。错误信息含 (1) 端点未实现事实 + (2) 后端 HTTP 状态码 + (3) 临时方案。这样 TypeScript 编译期能 catch 误用（类型仍然签名），运行期抛出的错误自带解决路径。
**教训**: 对于"后端 stub 但前端已暴露"的方法，body 用 `throw new Error(...)` 替代 axios 调用，比"让 axios 自然抛"或"前端 try/catch 翻译"更直接。错误信息必须含三件事：事实（未实现）+ 证据（HTTP 码）+ 方案（workaround）。

### 2026-06-01 cargo caret 约束的「floor 不动」模式（依赖审计）
**现象**: Iteration 032 审计 39 个 workspace 依赖时，初看 Cargo.toml 写的 `tokio = "1.35"` `regex = "1.10"` `lazy_static = "1.4"` 等 floor 似乎"落后"，但实际 `cargo check` / `cargo test` 都过。`cargo report future-incompatibilities` 也只报 `sqlx-postgres 0.7.4` 一条警告。
**根因**: Cargo.toml 写 `"1.35"` 等价于 `^1.35` = `>=1.35, <2.0`（caret 范围）。cargo 在解析时自动选 latest within range，所以 lock 已经是 1.49（tokio）、1.12（regex）、1.5（lazy_static）。floor 写低只是声明"最低支持版本"，**不是实际安装版本**。
**方案**: 把审计拆成三段 —— ① cargo.toml floor + ② Cargo.lock 实际 + ③ crates.io latest stable；floor bump 是纯文档性变更（声明"已测试 X.Y+ 才能编译"），没有运行时差异。结论：不修改 Cargo.toml，让 floor 反映"已测试过的最低版本"而非"实际解析到的版本"。
**教训**: 依赖审计看 Cargo.lock + crates.io，不要看 Cargo.toml 字符串。`^X.Y` 范围下 floor bump 99% 的情况是无意义 churn；如果真要 bump，附"X.Y+ 才修复的安全问题"或"X.Y+ 才支持的新 feature"等具体理由。审计前先 `awk` 出 lock 当前 + `curl crates.io/api/v1/crates/<name>` 拿 latest，二者比对才有意义。

### 2026-06-01 依赖大版本迁移的成本估算方法
**现象**: Iteration 032 审计发现 16 个大版本升级（sqlx 0.7→0.8、bollard 0.17→0.21、thiserror 1→2 等），每个迁移成本差异极大。如何在「不开工」的前提下判断每个迁移的 size？
**根因**: 大版本迁移的代码改动面 ≠ 依赖数量，而是「使用该依赖的 call site 数 × 每个 call site 的 API 变化程度」。直接读 CHANGELOG 能拿到 API 变化，但 call site 数需要从代码库统计。
**方案**: 用 `rg -c '<dependency>::<macro_or_trait>' backend/crates` 给出 call site 数（sqlx 0.7→0.8：19 文件 × 平均 6 call site = ~120 call sites，需要 1 个独立迁移迭代）；用 `cargo report future-incompatibilities` 给出具体升级理由（"never-type-fallback" 比"latest 0.9 性能更好"更有说服力）；用 crates.io API `https://crates.io/api/v1/crates/<name>` 拿 `max_stable_version` + `newest_version` 区分 stable 和 RC。把"call site 数 + API 变化程度 + 是否有 future-incompat"三要素作为每个暂缓项的 backlog 详情块内容。
**教训**: 依赖大版本迁移的「成本预估」必须用 call site 数 × API 变化程度，而不是依赖列表长度。`rg -c` + `cargo report future-incompatibilities` + `crates.io API` 是零成本组合，足够在 backlog 入池阶段写出可激活的 DoR 描述。

### 2026-06-01 clippy `--all-targets` 是默认行为（假绿陷阱）
**现象**: Iteration 032 第一次跑 `cargo clippy --workspace -- -D warnings` 报 0 错误（exit 0），但 EVO-059 detail block 明确写「18 个 `-D warnings` 错误」。两者矛盾让人困惑 2 分钟。
**根因**: `cargo clippy` 默认只检查 lib + bin；`--all-targets` 才覆盖 tests + examples + benches。本项目 18 个 clippy 错误全部在 `infra/tests/*_repo_tests.rs` + `api/tests/auth_e2e_tests.rs` + `service-payment/src/config.rs`（lib test 触发）+ `api/src/middleware/rate_limit.rs`（lib test 触发），默认 scope 完全看不到。
**方案**: 把 `cargo clippy --workspace --all-targets -- -D warnings` 作为 iteration 收口的 hard required 门禁（替换原 `cargo clippy --workspace` 写法）。EVO-059 验收标准同步更新为 `--all-targets`。
**教训**: `cargo clippy` 和 `cargo check` 都有「默认不查 tests」的隐藏 scope。写进 SOP/TASK-CLOSURE 的命令模板必须是 `--all-targets` 版本，否则会假绿。同样陷阱在 `cargo build` 上也存在（默认不构建 test 目标，但 dev 模式 cargo test 反而会构建，所以不易察觉）。

---

## Part 3: 维护规则

触发以下情况时，会话结束前应追加经验条目：

- 操作失败后找到了根因。
- 发现文档未记录的非直觉行为。
- 多次尝试后才解决问题。
- 用户指出了遗漏、误解或业务口径错误。
- 修改流程、脚本、部署方式后发现新的操作顺序要求。

写回格式：

```markdown
### <YYYY-MM-DD> <一句话总结>
**现象**: <具体表现>
**根因**: <深层原因>
**方案**: <解决步骤>
**教训**: <一句话，供未来 Agent 记忆>
```

写回流程：

1. 读取本文全文，确认没有重复条目。
2. 如果 Part 2 超过 30 条，先把较老条目归档到 `docs/archive/evolution-<YYYY-MM-DD>.md`。
3. 追加新条目到 Part 2。
4. 在最终回复中说明已写回的经验。
