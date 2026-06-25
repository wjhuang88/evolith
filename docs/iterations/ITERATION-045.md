# Iteration 045: Phase E'-1b Git Client Auth Infra（EVO-103-B-1）

> 文档状态：Closed（2026-06-25）
> 计划发布日期：2026-06-25
> 计划目标：实现 EVO-103-B-1 Git 客户端认证基础设施（Basic-Auth extractor + `/repos/` RBAC 注册为 auth-required + CSRF 豁免），为 EVO-103-B-2 Smart HTTP 端点解锁 git 客户端鉴权，同时关闭 RBAC catch-all 把 `/repos/...` 当公开路径的安全漏洞。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 实现 EVO-103-B-1 Git 客户端认证基础设施：Basic-Auth extractor + `/repos/` RBAC 注册为 auth-required + CSRF 豁免。
- 这是 Phase E'-1b 的第二个切片，在 EVO-103-A（Repo CRUD）完成后，补齐 git 客户端认证能力。
- 解锁后续依赖：EVO-103-B-2（Smart HTTP 端点）、EVO-106（Agent Session scoped token 迁移路径）。
- 关键安全门禁：当前 RBAC `is_public_path` catch-all 将非 `/api/`/非 `/mcp` 路径视为 PUBLIC——这意味着 `/repos/...` 将无认证（push 安全漏洞）。B-1 必须将 `/repos/` 注册为 auth-required 路径。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-103-B-1](../backlog/active/EVO-103-B-1-git-client-auth-infra.md) | Git 客户端认证基础设施（Basic-Auth + RBAC + CSRF） | 父 Epic EVO-103-B（祖父 EVO-103，曾祖 EVO-100） | P0 | 启动条件: EVO-103-A Done, ApiKeyRepository exists, extract_token/actix-web-httpauth available |

## 3. 发布计划基线：不做事项

- 不实现 Smart HTTP 端点协议逻辑（→ EVO-103-B-2）。
- 不调用 Docker git 二进制（→ EVO-103-B-2）。
- 不实现 gix server-side（已发布 crate 中不存在）。
- 不改变现有 Bearer/cookie auth 行为。
- 不实现 EVO-106 scoped token（B-1 使用 API key 作为过渡方案）。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-103-B-1 标 API 形态（Basic-Auth + RBAC + CSRF）；适用 BDD 验收。
- [x] 4 个 Given/When/Then 场景已在 [EVO-103-B-1 item file](../backlog/active/EVO-103-B-1-git-client-auth-infra.md) 中定义。
- [x] 技术验收：`cargo test --workspace` 全绿 + `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

### EVO-103-B-1 验收（来自 item file）

- [ ] `extract_token()` 或新增 extractor 支持 `Authorization: Basic base64(user:apikey)` 解析为 `CurrentUser`
- [ ] `/repos/` 路径前缀注册为 auth-required（RBAC 不再将其视为 public）
- [ ] git Smart HTTP POST 路径（`/repos/{id}/git-receive-pack`、`/repos/{id}/git-upload-pack`）添加到 CSRF `exempt_paths`
- [ ] SPA 静态资源 fallback 对真正前端路由仍然有效（不被 `/repos/` 注册误拦截）
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 场景（4 个，详见 item file）

1. **场景 1**：有效 Basic-Auth → 认证成功
2. **场景 2**：无效 API key → 401
3. **场景 3**：无认证 → 401（非公开）
4. **场景 4**：CSRF 豁免 → git POST 通过

## 5. 发布计划基线：计划验证

```bash
# backend — 编译 + 测试
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Auth 单元/集成测试覆盖 4 BDD 场景：
# - Basic-Auth 解析（有效 key → 认证成功）
# - Basic-Auth 解析（无效 key → 401）
# - RBAC /repos/ 非公开测试
# - CSRF 豁免测试
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Basic-Auth 凭证泄露（仅 API key 作 password，TLS 保护，日志 redact） | 仅接受 API key 作为 password 字段；生产环境强制 TLS；日志 sanitize 已在 common::sanitize 实现 |
| RBAC catch-all 改动误伤 SPA fallback（测试 `/assets/`、`/api/v1`、深路由不被 `/repos/` 拦截） | 显式注册 `/repos/` 为 auth-required；测试验证 `/assets/`、`/api/v1`、SPA 路由仍可访问 |
| CSRF 豁免范围过大（仅豁免 `/repos/{id}/git-*pack` POST，不豁免其他） | 精确路径匹配：仅 `/repos/{id}/git-upload-pack` 和 `/repos/{id}/git-receive-pack` POST 豁免 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 启动 EVO-103-B-1 迭代（Phase E'-1b 第二切片） |
| 产物 | 待实现：Basic-Auth extractor + RBAC 改动 + CSRF exempt + 测试 |
| 状态同步归口 | EVO-103-B-1（In Progress）、EVO-103-B 父项子表、PRODUCT-BACKLOG、BOARD、iterations/README |
| Story/BDD 归口 | [EVO-103-B-1 item file](../backlog/active/EVO-103-B-1-git-client-auth-infra.md) 4 场景 |
| 验证证据 | cargo test --workspace 0 failures；cargo clippy --workspace --all-targets -- -D warnings 0 errors；4 BDD 场景通过（有效 Basic-Auth→认证、无效 key→401、无 auth 访问 /repos/→401、CSRF 豁免→POST 通过）。中间件通过 actix `from_fn` + `Next<B>` 实现异步 API-key 解析后调用内部 service，无 Mutex/UnsafeCell/unsafe。 |
| 残余工作归口 | EVO-103-B-2（Smart HTTP 端点）、EVO-106（PAT 迁移）、SSH/LFS（Phase 5） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-25 | activation | Iteration inventory 完成：ITERATION-044 Closed（EVO-103-A Done）；无 Active/In Progress/Review 迭代；018-020/027 Superseded；025/026 Blocked（Phase F）。EVO-103-B 拆为 B-1/B-2（gix 无服务端，3 端点需 subprocess；auth/CSRF/Docker 缺口发现后拆分）。EVO-103-B-1 Ready，选入本轮。状态 Ready → In Progress；ITERATION-045 Active。 |
| 2026-06-25 | completion | EVO-103-B-1 完成。Navigator 审查发现中间件 UnsafeCell UB → Mutex 串行化 → 最终用 from_fn + Next<B> 解决（async 预处理后调用内部 service，无锁无 unsafe，全并发）。配置从 app_data 读取。验证全绿。状态 → Done；ITERATION-045 Closed。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **Active / In Progress**：无（BOARD.md `Now` 段为空；iterations/ 中无 Active 文档）
- **Review**：无
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，与 Phase E' 独立）
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整 → EVO-108/109）
- **Recently Closed**：Iteration 042（EVO-101/102，2026-06-24）、Iteration 043（EVO-113，2026-06-25）、Iteration 044（EVO-103-A，2026-06-25）

**disposition 决策**：
- Iterations 018-020/027 — Superseded by 2026-06-23 方向调整。不激活。
- Iterations 025/026 — Planned/Blocked（Phase F 独立）。维持阻塞。
- Iterations 042/043/044 — Closed。无需处置。
- 无 Active/Review 迭代，可直接从 backlog 选取 Ready story。

**新 Story 选取**：EVO-103-B-1 满足 DoR（4 BDD 场景、依赖 EVO-103-A Done、ApiKeyRepository 存在、extract_token/actix-web-httpauth 可用），选入本轮。WIP 限制 = 1 story。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

- 完成：Basic-Auth + /repos/ RBAC 注册（关闭 public catch-all 漏洞）+ CSRF 豁免。
- 未完成：无。
- 验证结果：cargo test --workspace 0 failures；cargo clippy --workspace --all-targets -- -D warnings 0 errors；4 BDD 场景通过。
- 闭环状态：`Complete`。
- 残余归口：API-key 作为 Basic-Auth password 是过渡方案，EVO-106 scoped token 将提供专用 git token。

## 11. Retrospective

- 做得好的：Navigator 审查在代码"测试通过"的情况下仍捕获了 UnsafeCell 的并发 UB（clippy 无法发现）；坚持不 ship 已知串行化（Mutex）或 UB（UnsafeCell）的中间件。
- 需要调整的：actix 全局中间件做"async 预处理后调用内部 service"必须用 `from_fn` + `Next<B>`，不能用 `Transform/Service` + `let fut = self.service.call(req)` 模式（后者只能 sync 预处理）也不能用 `&mut self.service`。Driver 实现此类中间件前应在 prompt 中明确要求 `from_fn` + `Next<B>` 签名。
- 写入 EVOLUTION：actix async 中间件 soundness 陷阱（见 EVOLUTION.md）。

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 祖父 Story：[EVO-103-B Smart HTTP git 协议](../backlog/active/EVO-103-B-smart-http-git-protocol.md)
- 子 Story：[EVO-103-B-1 Git Client Auth Infra](../backlog/active/EVO-103-B-1-git-client-auth-infra.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
