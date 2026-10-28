# Iteration 070: EVO-127-A GraphQL Platform Foundation

> 文档状态：Active / In Progress
> 计划发布日期：2026-10-28
> 计划目标：建立 Evolith canonical application GraphQL 的最小可扩展 platform/schema/codegen/security foundation，同时保持现有 `/api/v1` 行为不变。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 交付单一 `/graphql` application endpoint 与 `async-graphql`/Actix 集成基础。
- 建立 Evolith-owned GraphQL request context、root Query/Mutation、稳定 error extensions 与 abuse controls。
- 建立 committed SDL、deterministic schema export/drift gate，以及 frontend GraphQL Code Generator typed transport。
- 保持现有 REST business routes、Git Smart HTTP、MCP、health/callback 等协议入口行为不变。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-127-A | GraphQL platform/schema/codegen foundation | EVO-127 | P0 | ADR-0012 / GraphQL governance baseline 已由 PR #34 合入；Iteration inventory 已处置 |

库存 disposition：

- Iteration 069 已 `Closed / Complete`，不阻塞新工作。
- Iteration 018/019/020/027 继续 `Blocked for activation / Superseded direction`。
- Iteration 025/026 继续 `Blocked for activation`，不抢占当前 P0。
- Iteration 056 保持 `Closed / Partial`，remote Branch Protection residual 归 release closure，不阻塞本轮。

## 3. 发布计划基线：不做事项

- 不迁移 Auth/Tenant/Repository/Repo Context/Capabilities 等完整业务域。
- 不删除或重写 `/api/v1` business handlers。
- 不 GraphQL 化 Git Smart HTTP、MCP、health/readiness、Webhook/OAuth/payment callbacks、LFS/bundle-uri。
- 不引入 Federation、subscriptions、Apollo/urql cache 或 cursor pagination redesign。
- 不改变 RBAC、Tenant isolation、API Key 或 Agent Token 权限模型。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] EVO-127-A 为 Technical / API Contract / Platform Story。
  - [x] transport/security 行为使用 Given/When/Then 负向场景；schema/codegen drift 使用技术验收。
- [ ] Rust 1.94.1 `fmt/check/clippy/test --locked` 全部通过。
- [ ] 单一 `/graphql` endpoint、root Query/Mutation 与 request context 可运行；只暴露最小 smoke operation。
- [ ] GraphQL error extensions 至少支持稳定 `code`、`requestId`、`retryable` 契约。
- [ ] batch request 默认拒绝；depth/complexity 上限有负向集成测试。
- [ ] cookie session 请求缺失/错误 CSRF 时 fail closed；bearer-only 且无 session cookie 时不要求 browser CSRF。
- [ ] GraphiQL 仅 development 明确启用，production 默认关闭。
- [ ] committed SDL 可 deterministic export；CI drift check 可证明 stale artifact 失败。
- [ ] frontend `graphql-request` + GraphQL Code Generator client preset 生成 typed operation/client foundation；TanStack Query 继续为 cache owner。
- [ ] frontend transport 负责 `credentials: include`、可选 Authorization、CSRF 与 `X-Request-ID`。
- [ ] `/api/v1` 与 protocol-native endpoints 没有行为回归。

### BDD / security scenarios

1. **Cookie CSRF fail closed**
   - Given 请求携带 Evolith session cookie；
   - When POST `/graphql` 未携带有效 `X-CSRF-Token`；
   - Then 在 schema execution 前拒绝，且不把请求当作成功 GraphQL operation。
2. **Bearer-only independent of browser CSRF**
   - Given 请求仅携带有效 Bearer 凭证且没有 session cookie；
   - When POST `/graphql` 执行允许的 operation；
   - Then 不因缺少 browser CSRF header 被拒绝，同时仍服从既有 auth/rate-limit 语义。
3. **Batch abuse rejected**
   - Given HTTP body 是 GraphQL batch array；
   - When 请求 `/graphql`；
   - Then 请求失败且不会批量执行 operations。
4. **Query cost bounded**
   - Given query 超出配置 depth/complexity；
   - When schema validation/execution；
   - Then 返回稳定失败并记录 request id，不进入无界 resolver work。
5. **Production GraphiQL closed**
   - Given production runtime；
   - When 请求 GraphiQL UI；
   - Then 默认不可用，只有 development 显式配置才能启用。

## 5. 发布计划基线：计划验证

```bash
# backend
cd backend
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace

# schema drift
cargo run --locked -p api --bin export-graphql-schema -- ../../docs/reference/graphql/schema.graphql
# compare/export command may be finalized by implementation, but must be deterministic and CI-enforced

# frontend
cd ../frontend
bun install --frozen-lockfile
bun run graphql:codegen
bun run type-check
bun run build
bun run lint

# docs / diff
git diff --check
python3 scripts/tests/check-markdown-links.py

# final remote gates
# exact-head GitHub Actions: ci + data-durability-container
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| POST GraphQL 让旧 verb-based CSRF 语义失效 | `/graphql` 单独执行 cookie-aware CSRF gate；bearer-only 无 cookie 不依赖 CSRF；负向集成测试先行 |
| resolver 绕过现有 auth/service boundary | request context 只复用 established principal/services；本 Story 不迁业务域，不新增授权模型 |
| GraphQL query/batch 造成资源放大 | batch disabled；body/depth/complexity 有界；负向测试 |
| schema/client generated artifacts 漂移 | committed SDL + deterministic export + codegen drift gate |
| 新 client stack 与 TanStack Query 形成双 cache | `graphql-request` 仅 transport；TanStack Query 保持 server-state owner |
| dependency/API 版本变化导致 lock/build 失败 | 锁定兼容版本并以 Rust 1.94.1 locked full gates 为准；失败时回滚本 Story dependency/route/codegen commit |

## 7. 安全威胁模型

| 项目 | 内容 |
|------|------|
| 受保护资产 | session/JWT/API credentials、Tenant identity、application data、request/audit metadata |
| 攻击者/调用者 | 匿名用户、Browser session、Bearer/API caller、低权限 Tenant member、滥用 query 的客户端 |
| 入口 | `/graphql` POST/optional development GraphiQL、frontend GraphQL transport |
| 信任边界 | Browser→Actix→GraphQL schema；Bearer caller→Actix；GraphQL resolver→Evolith services |
| 失败模式 | CSRF 绕过、认证语义漂移、跨租户 resolver 越权、batch/depth/complexity DoS、敏感 variables 日志泄露、production GraphiQL 暴露 |
| 安全默认 | fail closed；batch/过深/过复杂拒绝；GraphiQL production off；resolver 不复制授权；不记录敏感 variables |
| 验证证据 | cookie/bearer CSRF integration tests、batch/cost negative tests、现有 auth/rate-limit regression、full exact-head CI |

## 8. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 将 GraphQL governance baseline 转为第一个可运行 implementation slice，并形成后续 B/C/F/D/E 可复用 foundation |
| 产物 | backend GraphQL module/endpoint/context/error/security controls/tests；SDL export；frontend codegen/transport；CI drift gate；API/reference docs |
| 状态同步归口 | EVO-127-A、EVO-127 Epic、Product Backlog、BOARD、Iteration 070、Issue #27；必要时 API-CONTRACT/CONFIG/TESTING/EVOLUTION |
| Story/BDD 归口 | 本页技术验收 + 5 个 transport/security Given/When/Then 场景 |
| 验证证据 | backend locked full gates、frontend codegen/type/build/lint、schema drift、negative integration tests、exact-head CI/durability、Navigator review |
| 残余工作归口 | domain migrations 继续归 EVO-127-B~G；Repository/Context 仍等待 EVO-126 对应边界；任何新 blocker 建独立 backlog/issue |

## 9. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-10-28 | activation | PR #34 governance baseline 已 exact-head `ci #300` + durability `#140` 全绿并合入 `main` (`9c6e904`)；按 START-ITERATION inventory 处置后激活 EVO-127-A。 |

## 10. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-10-28 | clarification | 接受 | 用户要求“继续推进”；按已接受执行计划进入首个 GraphQL foundation Story，不扩大到业务域迁移 | 保留 governance baseline，创建 Iteration 070 |

## 11. Review

- 完成：待实施与验证。
- 未完成：全部 Story acceptance 仍待执行证据。
- 验证结果：待记录。
- 闭环状态：`Partial`（Active execution）
- 残余归口：EVO-127-B~G / EVO-126 dependent slices。

## 12. Retrospective

- 做得好的：待收口。
- 需要调整的：待收口。
- 写入 EVOLUTION：如发现可复用 GraphQL/Actix/CSRF/codegen 陷阱则追加。
