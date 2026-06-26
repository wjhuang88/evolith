# Iteration 046: Phase E'-1b Smart HTTP Endpoints（EVO-103-B-2）

> 文档状态：Closed（2026-06-26）
> 计划发布日期：2026-06-26
> 计划目标：实现 EVO-103-B-2 Smart HTTP git 协议端点（info/refs + git-upload-pack + git-receive-pack，全部 `git --stateless-rpc` subprocess），让标准 git 客户端可 clone/push/pull Evolith 托管的仓库；生产 Docker 运行时加 `git`。解锁 "可用的 git 托管" 核心能力。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 实现 EVO-103-B-2 Smart HTTP git 协议端点（info/refs + git-upload-pack + git-receive-pack，全部 `git --stateless-rpc` subprocess），让标准 git 客户端可 clone/push/pull Evolith 托管的仓库。
- 生产 Docker 运行时加 `git`（`apt-get install git`）。
- 解锁 "可用的 git 托管" 核心能力。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-103-B-2](../backlog/active/EVO-103-B-2-smart-http-endpoints.md) | Smart HTTP Endpoints（git subprocess） | 父 Epic EVO-103-B（祖父 EVO-103，曾祖 EVO-100） | P0 | 启动条件: EVO-103-B-1 Done, EVO-103-A Done |

## 3. 发布计划基线：不做事项

- 不实现 auth 基础设施（B-1 已完成）。
- 不使用 gix 服务端（已发布 crate 中不存在）。
- 不实现 SSH server（Phase 5）。
- 不实现 LFS（Phase 5）。
- 不实现纯 gix push（gix 维护者：receive-pack 无计划）。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-103-B-2 标 API/协议形态（git smart HTTP 协议端点）；BDD 不适用，使用等价技术验收。
- [x] 10 条 acceptance + 7 条等价技术验收（E2E clone/push/pull + subprocess/权限边界）已在 [EVO-103-B-2 item file](../backlog/active/EVO-103-B-2-smart-http-endpoints.md) 中定义。
- [x] 技术验收：`cargo test --workspace` 全绿 + `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。
- [ ] 真实 git CLI E2E（clone/push/pull）仍待 EVO-115 解锁 `WWW-Authenticate: Basic` 后完成。

### EVO-103-B-2 验收（来自 item file）

- [x] `GET /repos/{id}/info/refs` 返回正确 content-type（`application/x-git-*-advertisement`），git 客户端可解析（handler 级测试验证）
- [ ] `POST /repos/{id}/git-upload-pack` 流式返回 packfile 结果（`application/x-git-upload-pack-result`）
- [ ] `POST /repos/{id}/git-receive-pack` 流式返回 receive-pack 结果（`application/x-git-receive-pack-result`）
- [ ] `git clone http://host/repos/{id}` 成功克隆（exit 0）
- [ ] `git push` 成功推送 commits 到远程仓库
- [ ] `git pull` 成功拉取远程更新
- [x] 跨 tenant repo → 403/404
- [x] subprocess timeout enforced（`tokio::time::timeout` + `kill_on_drop(true)`；`service-git` timeout 单测覆盖）
- [x] Basic-Auth API key read-only 权限不得获得 receive-pack / push 能力（handler 级测试覆盖）
- [x] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿
- [x] `docs/reference/SCRIPTS-RELEASE-NOTES.md` 已更新（Docker 镜像新增 git 包）

### 等价技术验收（替代 BDD）

1. **E2E clone**：`git clone http://localhost:8080/repos/{id} /tmp/test-clone` 成功退出码 0，`.git` 目录存在。
2. **E2E push**：在克隆仓库中创建 commit 后 `git push` 成功，远程仓库包含新 commit。
3. **E2E pull**：另一克隆仓库 `git pull` 可获取 push 的内容。
4. **协议观察**：info/refs 请求返回 `application/x-git-upload-pack-advertisement` 或 `application/x-git-receive-pack-advertisement` content-type。
5. **跨租户隔离**：非所属 tenant 的 repo 访问 → 403/404。
6. **超时强制**：subprocess 超时后请求被终止，不挂起服务。
7. **权限边界**：Basic-Auth API key 请求必须使用现有 `permissions` 字段；read-only key 不得获得 push / receive-pack 能力。

## 5. 发布计划基线：计划验证

```bash
# backend — 编译 + 测试
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# E2E 验证（真实 git CLI）
# 1. 创建 repo（通过 EVO-103-A API）
# 2. git clone http://localhost:8080/repos/{id}
# 3. cd test-clone && echo "test" > test.txt && git add . && git commit -m "test" && git push
# 4. cd /tmp && rm -rf test-pull && git clone http://localhost:8080/repos/{id} test-pull && git pull
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| subprocess 安全（Command::new("git") 无 shell，固定参数向量，repo_path 从 DB 按 id 查找不从 URL，tokio::time::timeout 防挂起） | 使用 `Command::new("git")`（NO shell），固定参数向量，repo_path 通过 DB 按 repo id 查找（绝不从 URL 读取），不透明 body 传入 stdin，`tokio::time::timeout` + `kill_on_drop(true)` 防止挂起；stderr drain 防止子进程写满 pipe。service 参数验证为枚举 {upload-pack, receive-pack}。 |
| Docker 加 git 改镜像（需更新 SCRIPTS-RELEASE-NOTES） | 在 `backend/Dockerfile` 的 `debian:bookworm-slim` 运行时阶段添加 `git`（`apt-get install git`）；更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。 |
| 流式大 packfile 内存（用 web::Payload 流式不缓冲整包） | 响应通过 `HttpResponse::Ok().insert_header(...).streaming(stream)`（返回 `HttpResponse<BoxBody>`）；请求通过 `web::Payload`。Actix-web 4.13.0。 |
| receive-pack 改仓库内容（需验证 push 后远程有新 commit） | E2E push 验证远程仓库包含新 commit。 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 实现 EVO-103-B-2（3 Smart HTTP 端点：info/refs + git-upload-pack + git-receive-pack，git --stateless-rpc subprocess） |
| 产物 | 3 Smart HTTP 端点（info/refs + git-upload-pack + git-receive-pack，git --stateless-rpc subprocess）+ service-git subprocess 扩展 + actix 流式 + Docker 加 git + SCRIPTS-RELEASE-NOTES + handler 级测试 + subprocess timeout/stderr drain + API key read-only receive-pack 拒绝测试 |
| 状态同步归口 | EVO-103-B-2（Review）、EVO-103-B 父项子表、PRODUCT-BACKLOG、BOARD、iterations/README、SCRIPTS-RELEASE-NOTES |
| Story/BDD 归口 | [EVO-103-B-2 item file](../backlog/active/EVO-103-B-2-smart-http-endpoints.md)（等价技术验收：协议级） |
| 验证证据 | cargo test 0 failures / clippy 0 errors / info_refs handler 测试通过（test_info_refs_with_basic_auth_returns_advertisement：auth 解析 + git subprocess + content-type 200）；2026-06-26 review-fix 定向验证通过：`cargo test -p service-git`、`cargo test -p api --test git_smart_http_e2e_tests test_receive_pack_with_read_only_api_key_is_forbidden` |
| 残余工作归口 | EVO-115（WWW-Authenticate: Basic + 真实 git-client clone/push/pull E2E） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-26 | activation | Iteration inventory 完成：ITERATION-045 Closed（EVO-103-B-1 Done）；无 Active/In Progress/Review；018-020/027 Superseded；025/026 Blocked。EVO-103-B-2 依赖（B-1 Done + A Done）已满足，Proposed → Ready → In Progress，选入本轮。ITERATION-046 Active。 |
| 2026-06-26 | progress | Driver 实现 EVO-103-B-2：3 Smart HTTP 端点（git --stateless-rpc subprocess）+ Docker 加 git + actix 流式 + SCRIPTS-RELEASE-NOTES。Navigator + 测试修复：SQLite 测试 file mode=rwc + max_connections(1)（修 migration 009 表重建跨连接 schema cache 问题）；clippy from_str→parse_service + dead_code allow；加 handler 级测试 test_info_refs_with_basic_auth（验证 auth 解析 + git subprocess + content-type）。验证：cargo test 0 failures / clippy 0 errors。状态 → Review（真实 git-client E2E 待 WWW-Authenticate，→ EVO-115）。 |
| 2026-06-26 | review-fix | 架构评审修复：补齐 subprocess timeout 实现（`tokio::time::timeout` + `kill_on_drop(true)` + stderr drain）和 API key 权限边界（read-only key 不得 receive-pack）。验证：`cargo test -p service-git` 通过；`cargo test -p api --test git_smart_http_e2e_tests` 通过（真实 git-client E2E 仍按 EVO-115 ignored）；`cargo test -p api` 通过；`cargo test --workspace` 通过；`cargo clippy --workspace --all-targets -- -D warnings` 通过。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **Active / In Progress**：无（BOARD.md `Now` 段为空；iterations/ 中无 Active 文档）
- **Review**：无
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，与 Phase E' 独立）
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整 → EVO-108/109）
- **Recently Closed**：Iteration 042（EVO-101/102，2026-06-24）、Iteration 043（EVO-113，2026-06-25）、Iteration 044（EVO-103-A，2026-06-25）、Iteration 045（EVO-103-B-1，2026-06-25）

**disposition 决策**：
- Iterations 018-020/027 — Superseded by 2026-06-23 方向调整。不激活。
- Iterations 025/026 — Planned/Blocked（Phase F 独立）。维持阻塞。
- Iterations 042/043/044/045 — Closed。无需处置。
- 无 Active/Review 迭代，可直接从 backlog 选取 Ready story。

**新 Story 选取**：EVO-103-B-2 硬依赖 EVO-103-B-1（Done）和 EVO-103-A（Done）已满足，状态 Ready → In Progress，选入本轮。WIP 限制 = 1 story。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

- 完成：3 Smart HTTP 端点（info/refs + git-upload-pack + git-receive-pack，git --stateless-rpc subprocess）+ service-git subprocess 扩展 + actix 流式 + Docker 加 git + SCRIPTS-RELEASE-NOTES + handler 级测试（test_info_refs_with_basic_auth_returns_advertisement）+ subprocess timeout/stderr drain + API key read-only receive-pack 拒绝测试
- 未完成：真实 git-client clone/push/pull E2E（git 客户端等待服务器 401 返回 `WWW-Authenticate: Basic` challenge，当前 rbac_middleware 401 缺少该 header）
- 验证结果：cargo test 0 failures / clippy 0 errors / info_refs handler 测试通过（auth 解析 + git subprocess + content-type 200）；2026-06-26 review-fix 验证通过：`cargo test -p service-git`、`cargo test -p api --test git_smart_http_e2e_tests`、`cargo test -p api`、`cargo test --workspace`、`cargo clippy --workspace --all-targets -- -D warnings`
- 闭环状态：`Complete`（2026-06-26：EVO-115 完成（WWW-Authenticate: Basic on /repos/ 401 + 真实 git clone/push/pull E2E 通过），EVO-103-B-2 转 Done。git 托管核心能力（真实 git clone/push/pull）端到端验证通过。）
- 残余归口：EVO-115（WWW-Authenticate + 真实 git-client E2E）、gix PR#2465 migration、receive-pack 永久 subprocess、SSH/LFS Phase 5

## 11. Retrospective

- 做得好的：handler 级测试（test_info_refs_with_basic_auth_returns_advertisement）有效验证了 auth 解析 + git subprocess + 协议 framing 的正确性，即使真实 git 客户端 E2E 暂受阻也能确认端点实现无误
- 需要调整的：git Smart HTTP 服务端 401 必须带 `WWW-Authenticate: Basic`（标准 git server 行为），否则 git 客户端不会使用 URL 中的凭证重试——应在 B-1/B-2 集成时即包含该 header，而非等到 E2E 阶段才发现
- 写入 EVOLUTION：git smart HTTP 401 需 WWW-Authenticate: Basic（如确认为重复陷阱再写入 EVOLUTION.md）
- 2026-06-26 补充：WWW-Authenticate follow-up（EVO-115）已解决 Partial 状态；真实 git clone/push/pull E2E 全通过。

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 子 Story：[EVO-103-B Smart HTTP git 协议（父项）](../backlog/active/EVO-103-B-smart-http-git-protocol.md)
- 子 Story：[EVO-103-B-1 Git Client Auth Infra（Done）](../backlog/active/EVO-103-B-1-git-client-auth-infra.md)
- 子 Story：[EVO-103-B-2 Smart HTTP Endpoints（Review）](../backlog/active/EVO-103-B-2-smart-http-endpoints.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
