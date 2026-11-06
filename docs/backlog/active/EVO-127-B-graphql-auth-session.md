# EVO-127-B — Auth/session GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Security Story
- **优先级**：P0
- **状态**：In Progress / Iteration 071（first read-only session slice; Story 尚未 Done）
- **GitHub Issue**：[#28](https://github.com/wjhuang88/evolith/issues/28)
- **依赖或阻塞**：EVO-127-A 已由 PR #37、merge `3b864039` 完成；当前无硬依赖阻塞
- **影响范围**：backend / frontend / security / docs

## 目标

把 login/register/refresh/logout/password/email/invitation 等浏览器 auth/session 交互迁到 GraphQL，保持 cookie/JWT/CSRF/Bearer 语义。

## 本轮实施切片（Iteration 071）

- 先建立只读 `currentSession` claim projection：已验证 principal -> user/tenant/role；匿名 -> `null`。本字段不返回 token、cookie、password 或 API key material，也不冒充包含 email/username 的 persisted `/auth/me`。
- 复用既有 `CurrentUser` 和 optional-auth middleware，绝不在 resolver 解析原始 credentials，绝不引入第二套 auth 规则；invalid supplied credential 继续 fail closed。
- 后续同 Story 依次处理 domain auth service extraction、login/register/refresh/logout 的 cookie/CSRF response boundary、password/email/invitation 与 frontend typed GraphQL/E2E cutover。Stage 1 合并不代表本 Story Done。

## BDD / Security acceptance

1. Given 无凭证的匿名调用者；When 查询 `currentSession`；Then 只得到 `null`，不泄露 identity/credential。
2. Given 合法已解析的 JWT/API Key principal；When 查询 `currentSession`；Then 只返回该 caller 自己的 userId/tenantId/role/tenantRole，不跨租户读取或暴露存储内部字段。
3. Given 请求提供无效/过期凭证；When 访问受保护 GraphQL auth/session 路径；Then 认证层拒绝，而不是静默降级 anonymous；错误码稳定。
4. Given 无已有 session cookie 的新用户；When 调用公开 login/register mutation；Then 不被旧 CSRF state 阻断。已有 session cookie 时须通过 double-submit CSRF。
5. Given browser 完成 login/register/refresh/logout；When 状态/Token 过期或 refresh 失败；Then cookie 与 UI 状态一致且不进入 refresh loop，已迁移的前端 auth 调用不再访问 REST。

## 安全威胁模型

| 项目 | 内容 |
|------|------|
| 受保护资产 | JWT/httpOnly session、CSRF token、API Key、user/tenant/role、password/reset/verification token |
| 调用者 | Anonymous、Browser cookie session、Bearer JWT、API Key、Tenant Member/Admin/Owner |
| 入口/信任边界 | Browser/Client → Actix optional auth + CSRF + rate limit → GraphQL resolver → auth services/repos |
| 失败模式 | 身份降级、跨租户 claims 泄露、CSRF 绕过、cookie 属性漂移、refresh loop、credential/log 泄露 |
| 安全默认 | invalid supplied credential fail closed；匿名只返回 null；只返回必要 claims；cookie mutations 不绕过 CSRF |
| 验证证据 | GraphQL schema negative tests、既有 GraphQL transport security tests、后续 browser E2E / exact-head CI |

## Acceptance

- Browser login/register/refresh/logout E2E 通过；失败 refresh 不循环。
- Public auth mutation 无既有 session cookie 时可用；cookie session 请求维持 CSRF。
- stable auth error codes 进入 GraphQL `extensions.code`。
- frontend auth flows 不再调用 `/api/v1/auth/*`。

## 本轮验证与残余归口

- 新 schema field + tests 以本 Story/Iteration 071 的 exact-head `ci` + `data-durability-container` 为准；不能以静态审阅替代编译与 schema drift gate。
- 全量 Auth mutation、cookie response、persisted viewer profile、typed frontend operations 和 browser E2E 未完成前，本 Story 保持 In Progress，REST auth 兼容入口不删除；最终 REST retirement 归 EVO-127-G。

## 不做

Tenant admin、repo、MCP/protocol endpoints。
