# EVO-103-B-1 Git Client Auth Infra（Basic-Auth + 端点挂载 + CSRF 豁免）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103-B](EVO-103-B-smart-http-git-protocol.md)（祖父 EVO-103，曾祖 EVO-100）
- 祖父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 曾祖 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-103-A](EVO-103-A-repo-crud.md)（Done，提供 repo 上下文）
- 现有 auth 中间件: `backend/crates/api/src/middleware/rbac.rs`（`extract_token()` 仅支持 Bearer/X-API-Key/cookie）
- 现有 CSRF 中间件: `backend/crates/api/src/middleware/csrf.rs`（`exempt_paths` 机制）
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：feature / api（跨平台 auth 基础设施）
- 优先级：P0
- 状态：In Progress
- 父 Epic：EVO-103-B（祖父 EVO-103，曾祖 EVO-100）

## Problem Or Outcome

作为 git 客户端用户，我需要通过标准 `git clone/push/pull` 命令与 Evolith 托管的仓库交互，但 git 客户端使用 `Authorization: Basic base64(user:pass)` 而非 Bearer token 或 cookie。当前 RBAC `extract_token()` 不支持 Basic-Auth，且 RBAC 的 `is_public_path` catch-all 会将 `/repos/...` 路径视为公开（安全漏洞），CSRF 中间件也会拦截 git POST 请求。

## Goal And Non-Goals

- **Goal**：
  1. **Basic-Auth extractor**：扩展 `extract_token()`（`rbac.rs`）或复用已声明但未使用的 `actix-web-httpauth = "0.8.2"` 依赖，支持 `Authorization: Basic base64(user:pass)`。解码 base64 后，将 password 视为 API key（通过现有 `ApiKeyRepository` 解析）→ 解析为 `CurrentUser`。Username 为信息性字段（或用户 email），password 才是凭证。
  2. **Smart HTTP 端点挂载路径 + RBAC 注册**：git Smart HTTP URL 使用短路径 `/repos/{id}/info/refs`、`/repos/{id}/git-upload-pack`、`/repos/{id}/git-receive-pack`（基础 `http://host/repos/{id}` 提供干净的 git UX）。**关键安全门禁**：当前 RBAC `is_public_path` catch-all 将非 `/api/`/非 `/mcp` 路径视为 PUBLIC——这意味着 `/repos/...` 将无认证（push 安全漏洞）。B-1 必须将 `/repos/` 注册为 auth-required 路径（git 端点需要 Basic-Auth 身份），同时保持 SPA 静态资源 fallback 对真正前端路由有效。
  3. **CSRF 豁免**：git 客户端无法发送 `X-CSRF-Token`。将 git Smart HTTP 路径（`/repos/{id}/git-receive-pack` POST 等）添加到 CSRF 中间件 `exempt_paths`（`csrf.rs`）。注意：info/refs 是 GET（无需 CSRF），但 receive-pack/upload-pack POST 需要豁免。
- **Non-goals**：
  - 不实现 Smart HTTP 端点协议逻辑（→ EVO-103-B-2）。
  - 不调用 Docker git 二进制（→ EVO-103-B-2）。
  - 不改变现有 Bearer/cookie auth 行为。
  - 不实现 EVO-106 scoped token（B-1 使用 API key 作为过渡方案）。

## Dependencies And Blockers

- 硬依赖 EVO-103-A（Done）：repo 上下文已就绪。
- 依赖现有 `ApiKeyRepository`（Phase 0 已完成）。
- 依赖现有 `extract_token()` 函数（`rbac.rs`）或 `actix-web-httpauth 0.8.2`（已声明但未使用）。
- 无阻塞项。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

- [ ] `extract_token()` 或新增 extractor 支持 `Authorization: Basic base64(user:apikey)` 解析为 `CurrentUser`
- [ ] `/repos/` 路径前缀注册为 auth-required（RBAC 不再将其视为 public）
- [ ] git Smart HTTP POST 路径（`/repos/{id}/git-receive-pack`、`/repos/{id}/git-upload-pack`）添加到 CSRF `exempt_paths`
- [ ] SPA 静态资源 fallback 对真正前端路由仍然有效（不被 `/repos/` 注册误拦截）
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 验收场景

**场景 1：有效 Basic-Auth → 认证成功**
```gherkin
Given 用户 U 拥有有效 API key K
When git 客户端发送 GET /repos/{id}/info/refs 携带 Authorization: Basic base64(U:K)
Then 请求被认证为用户 U 及其所属 tenant
And RBAC 允许访问该 tenant 下的 repo
```

**场景 2：无效 API key → 401**
```gherkin
Given 用户发送 Authorization: Basic base64(user:invalid-key)
When 请求 /repos/{id}/info/refs
Then 返回 401 Unauthorized
```

**场景 3：无认证 → 401（非公开）**
```gherkin
Given 请求 /repos/{id}/info/refs 不携带任何 Authorization 头
When 请求到达 RBAC 中间件
Then 返回 401 Unauthorized（NOT public path）
```

**场景 4：CSRF 豁免 → git POST 通过**
```gherkin
Given 用户已认证（Basic-Auth 有效）
When POST /repos/{id}/git-receive-pack 不携带 X-CSRF-Token
Then 请求通过 CSRF 检查（NOT 403 Forbidden）
```

## Validation Evidence Required

- Auth 单元/集成测试覆盖 Basic-Auth 解析（有效 key → 认证成功；无效 key → 401）。
- RBAC 测试验证 `/repos/` 路径不再公开。
- CSRF 测试验证 git POST 路径豁免。
- `cargo test --workspace` 全绿。
- `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

## Residual Work Destination

- EVO-106 scoped token 将提供专用 git token（B-1 使用 API key 作为过渡方案）；Basic-Auth → EVO-106 PAT 迁移记为未来 EVO。
- Smart HTTP 端点协议实现 → EVO-103-B-2。
- SSH server（Phase 5）。
- LFS（Phase 5）。

## Source Snapshot

- Source: EVO-103-B 拆分（2026-06-25），将 auth 基础设施与协议端点分离为独立可验收切片。
- Decision context: git 客户端使用 Basic-Auth 而非 Bearer/cookie；当前 RBAC catch-all 会将 `/repos/...` 视为公开（安全漏洞）；CSRF 中间件拦截 git POST。
- 关键依赖：现有 `ApiKeyRepository`、`extract_token()`（`rbac.rs`）、`actix-web-httpauth 0.8.2`（已声明未使用）、CSRF `exempt_paths`（`csrf.rs`）。
- 拆分日期：2026-06-25。
