# Iteration 071: EVO-127-B Auth/Session GraphQL Migration

> 文档状态：Active / In Progress（首个 read-only session slice 已提交分支，exact-head CI 待验证）
> 计划发布日期：2026-11-06
> 计划目标：在保留现有 REST/browser 认证行为期间，将 auth/session 业务按小批次迁入 canonical `/graphql`，最终由生成的 typed operations 承接浏览器认证流程。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”不因实施/改线而覆写；执行、证据与偏差只追加到后续章节。闭环按 [TASK-CLOSURE](../sop/TASK-CLOSURE.md)。

## 1. 发布计划基线：目标

- 交付 EVO-127-B 的 auth/session GraphQL 迁移，保留 Evolith-owned auth service、tenant、JWT/httpOnly cookie、CSRF、Bearer/API Key 与 rate-limit 边界。
- 先以 `currentSession` claim projection 建立 authenticated/anonymous 只读行为，随后逐个迁移 login/register/refresh/logout、密码/邮件/邀请动作及前端调用；完整 browser parity/E2E 之前不关闭 Story。
- 保留稳定 `extensions.code` / `requestId` / `retryable` 错误契约，不把 transport cookie 副作用塞进 domain service。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-127-B | Auth/session GraphQL migration | EVO-127 | P0 | EVO-127-A / #27 Done；PR #37 merged `3b864039`；main `ci #308` success |

库存 disposition：Iteration 070 已由 PR #37 与 post-merge CI #308 闭合为 `Closed / Complete`，其余 018/019/020/027 继续 `Blocked / Superseded`，025/026 继续 `Blocked for activation`，056 保持 `Closed / Partial` release residual；069 和最近其他迭代均 Closed，不抢占本轮 P0。当前只有 EVO-127-B 进入 In Progress，C/F/D/E/G 仍 Proposed 或等待所列依赖。

## 3. 发布计划基线：不做事项

- 不改 Git Smart HTTP、MCP、health、callback/protocol endpoints 或 WalGit data-plane。
- 不迁 Tenant admin / API keys management / billing / audit / repository CRUD / repo context / capabilities。
- 不创建第二套密码、Session、JWT 或 tenant auth service；不在 resolver 解析 credentials。
- 不因 `currentSession` 初始查询上线就删除 REST auth；完整迁移与 E2E 前不宣称 EVO-127-B 或 API-GQL-01 Done。

## 4. 发布计划基线：计划验收标准

- Story 形态：API / Permission / State Story；Given/When/Then 用于匿名、有效/失效 credential、cookie/CSRF 与浏览器状态流。
- [ ] 匿名 `currentSession` 返回 null；合法 principal 只返回 caller-owned user/tenant/role claims；GraphQL schema 不暴露 JWT/CSRF/token 字段。
- [ ] 无效/过期的已提供凭证 fail closed，不能伪装 anonymous；Bearer-only 不要求 browser cookie CSRF。
- [ ] login/register/refresh/logout GraphQL mutations 与旧业务语义一致，response set/clear httpOnly JWT 与非-httpOnly CSRF cookie；不重复 domain logic。
- [ ] password/email verification/invitation actions 与 persisted viewer/current-user 信息迁移，错误码和 tenant boundary 有负向测试。
- [ ] frontend typed GraphQL auth operations 接管浏览器登录/注册/刷新/登出；无残留 `/api/v1/auth/*` frontend 调用；错误/过期刷新不 loop。
- [ ] Browser E2E 覆盖 cookie/CSRF 正反向及 expired/invalid token，full exact-head `ci` + durability gate 通过；Documented SDL/generated-client drift 通过。

### BDD scenarios

1. Given 无 credentials，When 查询 `currentSession`，Then `null`，不泄露 user/tenant/token。
2. Given 合法 JWT 或 API Key principal，When 查询 `currentSession`，Then 返回被认证的 userId/tenantId/role/tenantRole 且不接受客户端自填 tenantId。
3. Given 提供无效或过期 token，When 调用 GraphQL，Then middleware fail closed，不能把错误降格匿名。
4. Given 未登录浏览器，When login/register mutation，Then 成功写入 secure/httpOnly JWT 与 double-submit CSRF cookie；已有 cookie 的 state-changing 请求必须符合 CSRF。
5. Given 登录态变更或过期，When refresh/logout 与 frontend auth initialization，Then 无无限刷新、无 UI/session 假阳性，失败可重试或安全清空本地状态。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run --locked -p api --bin export_graphql_schema > /tmp/evolith-schema.graphql
diff -u ../docs/reference/graphql/schema.graphql /tmp/evolith-schema.graphql
cd ../frontend
bun install --frozen-lockfile
bun run graphql:codegen
git diff --exit-code -- src/graphql/generated
bun run type-check
bun run build
bun run lint
cd ..
git diff --check
python3 scripts/tests/check-markdown-links.py
# final proof: exact-head GitHub Actions ci + data-durability-container
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| CurrentUser claims 不等于 persisted user profile | 将 `currentSession` 明确定义为 nullable minimal claims；后续 `viewer` 读模型仍经 user_repo 加载授权并单独验收 |
| 无效 cookie/Bearer 导致公开 login mutation 无法恢复 | 明确 credential resolution 次序与 public mutation contract，写 invalid/stale token 负向测试后再迁移 frontend；不静默把失效 token 当 anonymous |
| GraphQL POST 把读/写混在同一 HTTP 路径 | Cookie session 坚持 double-submit CSRF；Bearer-only 无 cookie 不改变 csrf 策略；保留现有 caller-aware rate limit |
| Cookie/refresh/logout 行为与 REST 漂移 | 由 Actix transport response 持有 Set-Cookie，domain service 返回 outcome；E2E 验证 SameSite/Secure/HttpOnly/max-age，失败不 loop |
| GraphQL resolver 意外泄露 credential/跨租户内容 | 只使用 middleware 提供的 principal；不将 raw JWT/API Key/secret 放进 schema；schema negative tests + tenant tests |
| 无法本地运行 Rust/Bun 或 codegen drift | 以 PR exact-head runner 为权威；CI 未全绿则维持 Review/Partial，不合并 |

## 7. 安全威胁模型与闭环台账

| 项目 | 内容 |
|------|------|
| 受保护资产 | JWT、cookie、API Key、CSRF token、password/reset/verify tokens、tenant/user identity |
| 调用者 | 匿名、Browser cookie、Bearer JWT、API Key、普通 Member、Admin、Owner |
| 入口与信任边界 | Browser/API caller → Actix credential/CSRF/rate-limit middleware → GraphQL → Evolith auth service/repositories |
| 失败模式 | 无效 token 被匿名化、CSRF 绕过、跨租户数据泄漏、cookie 属性失配、refresh loop、credential 日志泄漏 |
| 安全默认 | 无效 credential fail closed；匿名查询 null；tenant identity 只由 server context 给出；敏感操作 require established auth + CSRF |
| 验证证据 | Anonymous/authorized GraphQL negative tests、existing GraphQL security tests、browser auth E2E、exact-head full CI |

| 归口项目 | 记录 |
|----------|------|
| 请求结果 | EVO-127-B 首个 read-only session query vertical slice；后续按同一 Story 迭代至完整 auth/session/frontend cutover |
| 产物 | Backend `currentSession` + schema tests、canonical SDL/contract、后续 auth mutations / frontend typed operations |
| 状态同步归口 | Item EVO-127-B、Epic EVO-127、Product Backlog、BOARD、Iteration 071、Issue #28；A/Iteration 070 closure 同步 |
| Story/BDD | 本页 5 个 Given/When/Then 场景；安全 review SOP 适用 |
| 验证证据 | 首个 slice 仅提交分支，待 PR exact-head `ci`/durability 与契约 drift；后续 browser parity 仍缺 |
| 残余工作 | 完整 auth mutations、persisted viewer、browser cutover/E2E -> EVO-127-B；最终业务 REST retirement -> EVO-127-G |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-11-06 | Inventory / activation | 确认 PR #37 merged `3b864039`，main `ci #308` success；Iteration 070 的原 Review 状态完成收口；依计划选择 P0 EVO-127-B，未启动 C/F/D/E。 |
| 2026-11-06 | First implementation slice | `currentSession` 只读 claim projection + anonymous/authenticated/credential-exclusion schema tests；canonical SDL 和 reference 合约同步。CI 结果未取得前不声明功能验收完成。 |

| 2026-11-14 | merge | EVO-127-B 第一只读切片 PR #38：final head `9499c5d` 的 `ci #310` 与 durability `#148` success，Navigator staged read-only security review 无 blocking findings；PR #38 merge `ba66e867b4f8ed12dba68bfe95908178d85582d8`。合并后 main `ci #311` 尚待最终结论。EVO-127-B 保持 In Progress，后续 auth mutations/front-end E2E 未完成。 |
| 2026-11-14 | second read slice | 增加 GraphQL `viewer` persisted user profile projection，直接复用 `UserRepository`，不返回密码/重置令牌；JWT tenant claim 与存储 tenant 不一致时拒绝。添加真实注册→Bearer GraphQL viewer 成功、anonymous/invalid Bearer/mismatched tenant 负向 HTTP 集成测试；待 PR exact-head CI + durability 验证。 |

| 2026-11-14 | viewer merged | GraphQL persisted `viewer` 切片 PR #39 merged `856673d412986d68bdb284a574258fb85d9136d8`；final head `1696750a` 的 `ci #313` / durability `#150` 均 success，main after PR #38 的 `ci #311` success。本 Story 仍 In Progress；完整 auth mutation/browser cutover 未完成。 |
| 2026-11-14 | login service extraction | 第三切片抽离 `service-auth::LoginService`，保留 REST 登录校验/JWT/user/tenant 业务结果与 Cookie 运输边界；对成功 JWT httpOnly、CSRF 非 httpOnly、SameSite/Path/Max-Age 和错误密码无 Set-Cookie 添回归测试。GraphQL mutation 与 cookie side effects 在后续切片实现，提交后待 exact-head CI/durability 证明。 |

| 2026-11-14 | GraphQL login slice | 在 shared `LoginService` 上实现 GraphQL `login`；HTTP transport 共享 JWT/CSRF cookie builder，resolver 只暂存 cookie 意图，成功时附加 Set-Cookie。正反向 HTTP tests 覆盖 JWT/CSRF、正常 login→viewer、invalid creds、stale cookie fail closed；此分支依赖第三切片 PR #40，待 exact-head CI/durability 门禁。Frontend 切换 blocked 于 stale-cookie 恢复和其他 auth mutations。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-11-06 | scope refinement | 将大型 auth migration 分批处理；只先合并可审查的 read-only session query，后续完成登录/注册/refresh/logout 的原完整 Story 验收，不减少范围 | 避免未测试 cookie/CSRF 副作用进入首个 PR | REST auth 保持，后续 B 实现继续 |

## 10. Review

- 完成：首批 session schema / backend 测试 / SDL / contract / 070 closure 的 branch changes。
- 未完成：本 PR exact-head CI/durability、Navigator security review、完整 auth mutations 和 frontend/browser parity。
- 验证结果：尚未得到 CI 结果。
- 闭环状态：`In Progress / Partial slice`，EVO-127-B 整体未完成。
- 残余归口：EVO-127-B；EVO-127-G 只负责最终无 consumer 后的 REST retirement。

## 11. Retrospective

- 做得好的：先隔离纯只读 principal projection，再改 cookie 副作用，避免同时重写 session/auth transport。
- 需要调整的：观察 GraphQL credential fail-closed 对 stale-cookie login 恢复场景的影响，下一切片必须先有负向 E2E。
- 写入 EVOLUTION：若后续确有跨 Story 可复用陷阱，再写入。
