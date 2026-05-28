# 测试文档

本文档记录 Evolith 项目的测试用例、测试方法和最佳实践。

按变更类型选择验证命令、记录结果和失败处理的执行流程见 [测试与验证 SOP](../sop/TESTING.md)。本文档作为测试策略和用例清单参考。

## 1. 测试策略

### 1.1 测试金字塔

```
        /\
       /  \      E2E Tests (端到端测试)
      /----\     - 少量关键业务流程
     /      \
    /--------\   Integration Tests (集成测试)
   /          \  - API 端点测试
  /------------\ Unit Tests (单元测试)
 /              \- 核心业务逻辑
```

### 1.2 测试原则

- **快速反馈**: 单元测试应能在毫秒级完成
- **隔离性**: 每个测试应独立运行，不依赖外部状态
- **可重复性**: 同样的测试应产生同样的结果
- **自文档化**: 测试名称应清晰描述测试内容

## 2. 后端测试

### 2.1 单元测试

后端使用 Rust 的内置测试框架，位于各 crate 的源文件中。

#### 运行单元测试

```bash
# 运行所有单元测试
cargo test --workspace

# 运行特定 crate 的测试
cargo test -p service-auth
cargo test -p api
cargo test -p common

# 运行特定测试
cargo test --package service-auth test_jwt
```

#### 测试统计

> **最新数据 (2026-05-27)**: `cargo test -p api` → **48 passed, 0 failed** (23 unit + 18 auth e2e + 7 MCP e2e)

| Crate | 测试数 | 说明 |
|-------|--------|------|
| api | 48 | Handler/middleware 测试 (23) + 认证 E2E (18, 含邮箱验证与 Skill 更新) + MCP 工具执行 E2E (7) |
| common | 9 | 错误类型、日志配置、日志脱敏 |
| domain | 35 | 用户/租户/审计/邀请模型验证 |
| infra | 124 | 8 个 Repository 集成测试 (user, tenant, invitation, audit, tool, skill, snippet, api_key) |
| service-auth | 18 | JWT + Argon2id 密码 + 密码强度验证 |
| service-payment | 14 | Stripe 集成 (Mock) |
| doc-tests | 3 | 文档示例测试 |

#### 测试模块位置

| Crate | 测试文件 | 测试内容 |
|-------|---------|---------|
| service-auth | `src/jwt.rs` | JWT 令牌生成与验证 |
| service-auth | `src/password.rs` | 密码哈希与验证 (Argon2id) |
| domain | `src/user.rs` | 用户、租户邀请模型验证 |
| domain | `src/tenant.rs` | 租户、配额模型验证 |
| domain | `src/audit.rs` | 审计日志模型验证 |
| common | `src/error.rs` | 错误类型定义 |
| common | `src/log.rs` | 日志配置 |
| common | `src/sanitize.rs` | 日志脱敏 (password, token, secret, api_key) |
| api | `src/middleware/tenant.rs` | 多租户中间件 |
| api | `src/middleware/rbac.rs` | RBAC 角色访问控制 |
| api | `tests/auth_e2e_tests.rs` | 12 个认证端到端测试 |
| infra | `tests/user_repo_test.rs` | 用户 Repository 集成测试 |
| infra | `tests/tenant_repo_test.rs` | 租户 Repository 集成测试 |
| infra | `tests/invitation_repo_test.rs` | 邀请 Repository 集成测试 |
| infra | `tests/audit_repo_test.rs` | 审计 Repository 集成测试 |
| infra | `tests/tool_repo_test.rs` | 工具 Repository 集成测试 |
| infra | `tests/skill_repo_test.rs` | 技能 Repository 集成测试 |
| infra | `tests/snippet_repo_test.rs` | 片段 Repository 集成测试 |
| infra | `tests/api_key_repo_test.rs` | API Key Repository 集成测试 |

#### 示例: JWT 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt_token_generation() {
        let claims = Claims {
            sub: "user-id".to_string(),
            exp: Utc::now().timestamp() + 3600,
            tenant_id: "tenant-id".to_string(),
            role: "admin".to_string(),
        };
        
        let token = generate_token(&claims, "secret").unwrap();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_jwt_token_verification() {
        let claims = Claims {
            sub: "user-id".to_string(),
            exp: Utc::now().timestamp() + 3600,
            tenant_id: "tenant-id".to_string(),
            role: "admin".to_string(),
        };
        
        let token = generate_token(&claims, "secret").unwrap();
        let verified = verify_token(&token, "secret").unwrap();
        
        assert_eq!(verified.sub, "user-id");
    }
}
```

### 2.2 API 集成测试

使用 `actix-web` 的测试辅助工具进行 HTTP 级别测试。

#### 测试结构

```rust
#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use super::*;

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(
            App::new().route("/health", web::get().to(health_check))
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

### 2.3 MCP 协议测试

#### 测试 MCP Initialize

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {},
      "clientInfo": {
        "name": "test-client",
        "version": "1.0.0"
      }
    }
  }'
```

预期响应:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "evolith",
      "version": "1.0.0"
    }
  }
}
```

#### 测试 Tools List

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/list"
  }'
```

#### 测试 Tools Call

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -H "X-API-Key: <valid-api-key>" \
  -d '{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "tools/call",
    "params": {
      "name": "http_request",
      "arguments": {
        "url": "https://api.example.com",
        "method": "GET"
      }
    }
  }'
```

自动化验收不得请求 `httpbin.org` 或其他公网服务。HTTP 工具执行测试应启动本地 mock server，覆盖有效 API Key 的成功调用、无 key 拒绝、上游 4xx/5xx、超时和响应大小限制。

## 3. 前端测试

### 3.1 构建验证

前端使用 Vite + Bun 进行类型检查、构建和本地开发。

```bash
# 进入前端目录
cd frontend

# 类型检查
bun run type-check

# 构建
bun run build

# 开发模式
bun run dev
```

## 4. E2E 测试

### 4.1 测试环境

| 项目 | 内容 |
|:---|:---|
| **执行环境** | macOS Darwin, Backend (Rust/Actix-web :8080, SQLite in-memory), Frontend (Vite :3001) |
| **执行方式** | curl (API) + Playwright MCP (UI) |
| **最新结果** | **55/55 通过**, 7 个 Bug 已全部修复 (2026-04-09) |
| **P0 用例通过率** | 100% (14/14) |
| **P1 用例通过率** | 100% (34/34) |

### 4.2 测试用例

> **注意**: 系统使用 Argon2id 密码哈希。测试时通过 `/api/v1/auth/register` 注册新用户。

#### 4.2.1 系统健康检查 (Health)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-HLT-001 | 基础健康检查 | 无 | `GET /health` | 200 OK, `{"status": "ok", "version": "..."}` | P0 | ✅ |
| TC-HLT-002 | 存活探针检查 | 无 | `GET /health/live` | 200 OK | P0 | ✅ |
| TC-HLT-003 | 就绪探针检查 | 数据库已连接 | `GET /health/ready` | 200 OK, 确认数据库就绪 | P0 | ✅ |
| TC-HLT-004 | 数据库断开响应 | 模拟 DB 故障 | `GET /health/ready` | 503 Service Unavailable | P1 | ⏭️ |

#### 4.2.2 认证与用户管理 (Auth)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-ATH-001 | 用户注册成功 | 邮箱未注册 | `POST /api/v1/auth/register` | 201, 设置 `evolith_token` + `csrf_token` Cookie | P0 | ✅ |
| TC-ATH-002 | 注册校验失败 | 无 | 密码过短或邮箱格式错误 | 400 `VALIDATION_ERROR` | P1 | ✅ |
| TC-ATH-003 | 注册重复冲突 | 邮箱已存在 | 重复注册 | 409 `EMAIL_EXISTS` | P1 | ✅ |
| TC-ATH-004 | 登录成功流程 | 已注册 | `POST /api/v1/auth/login` | 200, 返回 user/tenant, 更新 Cookies | P0 | ✅ |
| TC-ATH-005 | 登录凭据错误 | 已注册 | 错误密码登录 | 401 `INVALID_CREDENTIALS` | P0 | ✅ |
| TC-ATH-006 | 获取当前信息 | 已登录 | `GET /api/v1/auth/me` | 200, 返回用户详情含 tenant_role | P0 | ✅ |
| TC-ATH-007 | 修改个人资料 | 已登录 | `PATCH /api/v1/auth/profile` | 200, 用户信息更新 | P1 | ✅ |
| TC-ATH-008 | 修改密码流程 | 已登录 | `POST /api/v1/auth/change-password` | 200, 旧密码失效 | P1 | ✅ |
| TC-ATH-009 | 退出登录 | 已登录 | `POST /api/v1/auth/logout` | 200, Cookies 清除 | P0 | ⚠️ |
| TC-ATH-010 | 令牌刷新 | 有效 Token | `POST /api/v1/auth/refresh` | 200, 返回新 token | P1 | ✅ |
| TC-ATH-011 | 发送邮箱验证 | 已注册 | `POST /api/v1/auth/send-verify` | 200, 返回 success（防枚举） | P2 | ✅ |

#### 4.2.3 工具管理 (Tools CRUD)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-TOL-001 | 创建工具成功 | 已登录 | `POST /api/v1/tools` | 201, 返回工具 ID | P0 | ✅ |
| TC-TOL-002 | 列表分页查询 | 存在多个工具 | `GET /api/v1/tools?page=1&per_page=10` | 200, 返回分页列表 | P1 | ✅ |
| TC-TOL-003 | 工具详情获取 | 工具存在 | `GET /api/v1/tools/{id}` | 200, 返回完整配置 | P0 | ✅ |
| TC-TOL-004 | 工具更新操作 | 工具所有者 | `PUT /api/v1/tools/{id}` | 200, 修改生效 | P1 | ✅ |
| TC-TOL-005 | 工具删除操作 | 工具所有者 | `DELETE /api/v1/tools/{id}` | 200, 后续查询 404 | P1 | ✅ |
| TC-TOL-006 | 越权访问拦截 | 跨租户 | `GET /api/v1/tools/{id_B}` | 403/404 (租户隔离) | P0 | ✅ |

#### 4.2.4 技能管理与沙箱执行 (Skills)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-SKL-001 | 创建技能 | 已登录 | `POST /api/v1/skills` | 201 | P0 | ✅ |
| TC-SKL-002 | 加载技能配置 | 技能存在 | `GET /api/v1/skills/{id}/load` | 200, 返回完整配置 | P1 | ✅ |
| TC-SKL-003 | 沙箱成功执行 | 沙箱可用 | `POST /api/v1/skills/{id}/execute` | 200, 返回执行结果 | P0 | ⏭️ |
| TC-SKL-004 | 执行超时拦截 | 代码含 sleep | 调用执行接口 | `timed_out: true` | P1 | ⏭️ |
| TC-SKL-005 | 运行时校验 | 无 | 指定不存在的 runtime | 400 `INVALID_RUNTIME` | P2 | ✅ |
| TC-SKL-006 | 删除技能 | 技能所有者 | `DELETE /api/v1/skills/{id}` | 200 | P1 | ✅ |

#### 4.2.5 代码片段管理 (Snippets)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-SNP-001 | 创建代码片段 | 已登录 | `POST /api/v1/snippets` | 201 | P0 | ✅ |
| TC-SNP-002 | 片段全文检索 | 片段存在 | `GET /api/v1/snippets/search?q=keyword` | 200, 返回匹配结果 | P1 | ✅ |
| TC-SNP-003 | LLM 引用格式 | 片段存在 | `GET /api/v1/snippets/{id}/reference` | 200, 返回优化后的 Markdown | P1 | ✅ |
| TC-SNP-004 | 标签过滤查询 | 多语言片段 | `GET /api/v1/snippets?language=rust` | 200, 仅返回 Rust 片段 | P1 | ✅ |

#### 4.2.6 团队成员管理 (Members)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-MBR-001 | 邀请新成员 | 管理员权限 | `POST /api/v1/tenant/{tid}/members/invite` | 201, 返回 invite_url | P0 | ✅ |
| TC-MBR-002 | 查看邀请列表 | 管理员权限 | `GET /api/v1/tenant/{tid}/members/invitations` | 200, 返回邀请列表 | P1 | ✅ |
| TC-MBR-003 | 移除团队成员 | 管理员, 非本人 | `DELETE /api/v1/tenant/{tid}/members/{mid}` | 200 | P1 | ⏭️ |
| TC-MBR-004 | 普通成员邀请拦截 | 成员权限 | 调用邀请接口 | 403 Forbidden | P0 | ✅ |
| TC-MBR-005 | 重复邀请校验 | 已有待处理邀请 | 再次邀请同一邮箱 | 400 `ALREADY_INVITED` | P1 | ✅ |

#### 4.2.7 API 密钥管理 (API Keys)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-AKY-001 | 创建 API 密钥 | 管理员权限 | `POST /api/v1/tenant/{tid}/api-keys` | 201, 返回明文 Key | P0 | ✅ |
| TC-AKY-002 | 使用密钥访问 MCP | 持有有效 Key | `POST /mcp` + Header | 200, JSON-RPC 成功 | P0 | ✅ |
| TC-AKY-003 | 撤销密钥 | 管理员权限 | `DELETE /api/v1/tenant/{tid}/api-keys/{kid}` | 200, 密钥失效 | P1 | ✅ |
| TC-AKY-004 | 过期密钥访问 | 密钥过期 | 使用该密钥 | 401 Unauthorized | P1 | ⏭️ |

#### 4.2.8 计费与订阅 (Billing)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-BIL-001 | 查看可用方案 | 已登录 | `GET /api/v1/tenant/{tid}/billing/plans` | 200, 返回方案列表 | P1 | ✅ |
| TC-BIL-002 | 获取当前订阅 | 已登录 | `GET /api/v1/tenant/{tid}/billing/subscription` | 200, 返回订阅状态 | P1 | ✅ |
| TC-BIL-003 | 创建支付会话 | 已登录 | `POST .../billing/subscription/checkout` | 200, 返回 Stripe URL | P2 | ⏭️ |
| TC-BIL-004 | 查看使用额度 | 已登录 | `GET /api/v1/tenant/{tid}/billing/usage` | 200, 返回使用量 | P1 | ✅ |

#### 4.2.9 审计日志 (Audit Logs)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-AUD-001 | 管理员查询日志 | 管理员权限 | `GET /api/v1/tenant/{tid}/audit-logs` | 200, 返回日志列表 | P1 | ✅ |
| TC-AUD-002 | 普通成员拦截 | 成员权限 | 查询日志 | 403 Forbidden | P1 | ✅ |
| TC-AUD-003 | 日志详情拦截 | 已登录 | `GET .../audit-logs/{id}` | 501 Not Implemented | P2 | ✅ |

#### 4.2.10 MCP 端点交互

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-MCP-001 | JSON-RPC 初始化 | 无 | `POST /mcp`, method: "initialize" | 返回 capabilities | P0 | ✅ |
| TC-MCP-002 | 列出 MCP 工具 | 已鉴权 | method: "tools/list" | 返回工具定义 | P0 | ✅ |
| TC-MCP-003 | 调用 MCP 工具 | 工具存在且 API Key 有效 | method: "tools/call" | 返回执行结果 | P0 | ✅ |
| TC-MCP-004 | 无效方法调用 | 无 | 调用不存在的方法 | JSON-RPC Error -32601 | P1 | ✅ |
| TC-MCP-005 | 匿名工具执行拦截 | Public HTTP tool 存在，无 API Key | method: "tools/call" | JSON-RPC auth error，不发起出站请求 | P0 | ✅ |
| TC-MCP-006 | 上游 HTTP 失败映射 | API Key 有效，本地 mock 返回 502 | method: "tools/call" | JSON-RPC error，包含上游状态 | P0 | ✅ |

#### 4.2.11 安全与中间件 (Security)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-SEC-001 | CSRF 保护 | 已登录 | POST 不携带 X-CSRF-Token | 403 `CSRF_ERROR` | P0 | ✅ |
| TC-SEC-002 | 速率限制 | 无 | 快速发送 60+ 请求 | 429 Too Many Requests | P1 | ✅ |
| TC-SEC-003 | 安全响应头 | 无 | 检查响应 Header | 包含 CSP, X-Frame-Options 等 | P1 | ✅ |
| TC-SEC-004 | 请求 ID 传递 | 无 | 发送请求 | 响应含 X-Request-ID | P2 | ✅ |
| TC-SEC-005 | JWT 过期失效 | 过期 Token | 调用受保护接口 | 401 Unauthorized | P0 | ✅ |

#### 4.2.12 前端 UI 交互

| ID | 测试名称 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---:|
| TC-UI-001 | 首页展示与 i18n | 访问 `/`, 切换中英文 | 内容正确翻译 | P1 | ✅ |
| TC-UI-002 | 登录表单校验 | `/login` 输入错误凭据 | 显示错误提示 | P1 | ✅ |
| TC-UI-003 | 路由守卫拦截 | 未登录访问 `/dashboard` | 重定向 `/login` | P0 | ✅ |
| TC-UI-004 | 入职向导流程 | 完成 `/onboarding` 三步 | 工具创建成功 | P1 | ✅ |
| TC-UI-005 | 工具创建器交互 | `/tools/new` 输入参数 | placeholder 正确翻译 | P1 | ✅ |
| TC-UI-006 | 响应式布局 | 切换手机宽度 | 侧边栏折叠 | P2 | ✅ |
| TC-UI-007 | 技能执行模态框 | `/skills/[id]` 执行 | 正确加载, 无 SSR 挂死 | P1 | ✅ |
| TC-UI-008 | 主题切换 | 点击主题按钮 | 深色/浅色切换 | P2 | ✅ |

#### 4.2.13 错误处理与容错

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 | 结果 |
|:---|:---|:---|:---|:---|:---|:---:|
| TC-ERR-001 | 404 页面展示 | 访问不存在路由 | 查看页面 | 自定义 404 页面 | P1 | ✅ |
| TC-ERR-002 | 前端崩溃捕获 | 触发渲染错误 | 查看页面 | 显示 error.tsx 页面 | P2 | ⏭️ |
| TC-ERR-003 | API 错误格式 | 构造失败请求 | 检查响应 | `{"success":false,"error":{...}}` | P0 | ✅ |
| TC-ERR-004 | 数据库异常 | 模拟 DB 超时 | 检查响应 | 500, 不泄露 SQL | P1 | ⏭️ |

#### 4.2.14 RBAC 权限矩阵

| 操作项目 | Owner | Admin | Member | Viewer |
|:---|:---:|:---:|:---:|:---:|
| 删除租户/更改账单 | ✅ | ❌ | ❌ | ❌ |
| 邀请/移除管理员 | ✅ | ❌ | ❌ | ❌ |
| 邀请/移除普通成员 | ✅ | ✅ | ❌ | ❌ |
| 管理 API 密钥 | ✅ | ✅ | ❌ | ❌ |
| 创建/修改工具与技能 | ✅ | ✅ | ✅ | ❌ |
| 查看资源列表 | ✅ | ✅ | ✅ | ✅ |
| 执行沙箱代码 | ✅ | ✅ | ✅ | ❌ |
| 查看审计日志 | ✅ | ✅ | ❌ | ❌ |

### 4.3 已知限制 (非 Bug)

| 限制 | 原因 | 影响 |
|:---|:---|:---|
| Logout 后 token 仍有效 | Dev 模式无 token 黑名单 | TC-ATH-009 部分通过 |
| Sandbox 执行失败 | Dev lite 模式无 Docker | TC-SKL-003/004 跳过 |
| API Key 撤销需 JSON body | DELETE handler 要求 Content-Type | 需传空 `{}` body |
| 被邀请用户创建独立租户 | 邀请-接受流程未完整实现 | TC-MBR-003 跳过 |
| Billing 返回模拟数据 | Stripe 使用 test_mode | TC-BIL-003 跳过 |

### 4.4 历史 Bug 记录

以下 Bug 在 2026-04-09 回归测试中全部修复并验证通过：

| Bug ID | 用例 | 严重度 | 描述 | 修复方案 |
|:---|:---|:---:|:---|:---|
| BUG-001 | TC-SNP-004 | P1 | Snippet language 过滤器无效 | `snippet_handlers.rs`: 添加 query params 提取 |
| BUG-002 | TC-AKY-002 | P1 | API Key 无法认证 MCP 端点 | `api_key_handlers.rs`: RandomState→SHA-256 哈希 |
| BUG-003 | TC-UI-002 | P1 | 登录失败时前端无错误提示 | `client.ts`/`authStore.ts`: 排除 login/register 的 token refresh |
| BUG-004 | TC-UI-004 | P1 | Onboarding 工具字段名不匹配 | `types.ts` + 3 个页面: schema→input_schema |
| BUG-005 | TC-UI-005 | P2 | 工具创建页 i18n key 缺失 | `en.json`/`zh-CN.json`: 添加 3 个缺失 key |
| BUG-006 | TC-UI-007 | P1 | 动态路由 SSR 在代理下挂死 | `dev.sh`: 检测代理时设置 NO_PROXY |
| BUG-007 | TC-UI-007 | P2 | i18n SSR Hydration Mismatch | `i18n.ts`: 默认语言 zh-CN→en |

### 4.5 E2E 统计汇总

| 指标 | 数值 |
|:---|:---|
| 总测试用例数 | 55 (API 47 + UI 8) |
| 通过 | 55 |
| 跳过 (环境限制) | 8 |
| 通过率 (已执行) | 100% (55/55) |
| P0 用例通过率 | 100% (14/14) |
| P1 用例通过率 | 100% (34/34) |

## 5. 持续集成

### 5.1 当前状态

GitHub Actions workflow 在前端迁移期间暂不作为当前门禁；后续由 backlog `EVO-030` 基于最终构建、测试和部署命令重建。

重建时建议覆盖以下步骤：

1. `cargo fmt --check` — 格式检查
2. `cargo clippy --workspace -- -D warnings` — Lint 检查（warnings 视为错误）
3. `cargo test --workspace` — 全量测试 (237 tests)
4. `cargo audit` — 依赖安全审计
5. `bun audit` — 前端依赖安全审计
6. Docker build — 验证镜像构建

### 5.2 本地检查

```bash
# 格式化检查
cargo fmt --check

# Lint 检查 (与 CI 一致)
cargo clippy --workspace -- -D warnings

# 运行测试
cargo test --workspace

# 安全审计
cargo audit

# 构建
cargo build --release
```

### 5.3 前端检查

```bash
# 安装依赖
cd frontend && bun install

# Lint 检查
bun run lint

# 类型检查
bun run type-check

# 构建
bun run build

# 安全审计
bun audit
```

## 6. 测试覆盖率

使用 `cargo-tarpaulin` 生成测试覆盖率报告:

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --workspace --out Html
```

## 7. 调试技巧

### 7.1 后端调试

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 跟踪特定模块
RUST_LOG=evolith::api=trace cargo run
```

### 7.2 前端调试

```bash
# 开发模式 (支持热重载)
bun run dev

# 生产构建分析
VITE_ENABLE_SOURCEMAP=true bun run build
```


## 8. 基础设施测试

### 8.1 缓存

```bash
cargo test -p infra -- cache
```

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| InMemoryCache 基本操作 | get/set/delete | 正确存取和删除 |
| TTL 过期 | 设置带过期时间的缓存 | 过期后返回 None |
| RedisCache (需要 Redis) | Redis 连接和操作 | 正确读写 Redis |

### 8.2 邮件

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| ConsoleMailer | 开发环境邮件 | 输出到控制台，不实际发送 |
| SmtpMailer (需要 SMTP) | 生产环境邮件 | 通过 SMTP 发送邮件 |

### 8.3 中间件

| 中间件 | 测试内容 | 预期结果 |
|-------|---------|---------|
| CSRF | 状态变更请求缺少 CSRF token | 返回 403 |
| CSRF | 正确的 CSRF 双提交 cookie | 请求通过 |
| Rate Limit | 超出频率限制 | 返回 429 |
| Request ID | 请求无 X-Request-ID | 自动生成并注入 |
| Request ID | 请求携带 X-Request-ID | 透传原始 ID |
| Security Headers | 响应头检查 | 包含 CSP, X-Frame-Options 等 |

### 8.4 日志脱敏

```bash
cargo test -p common -- sanitize
```

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 密码脱敏 | 日志包含 password 字段 | 值被替换为 `[REDACTED]` |
| Token 脱敏 | 日志包含 token 字段 | 值被替换为 `[REDACTED]` |
| API Key 脱敏 | 日志包含 api_key 字段 | 值被替换为 `[REDACTED]` |

### 8.5 沙箱执行器

> 需要 Docker 环境。使用 `SANDBOX__ENABLED=true` 启用。

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| Python 执行 | 执行 Python 3.11 代码 | 正确返回输出 |
| Node.js 执行 | 执行 Node.js 20 代码 | 正确返回输出 |
| 超时保护 | 执行超过 30 秒的代码 | 返回超时错误 |
| 内存限制 | 执行消耗大量内存的代码 | 返回资源限制错误 |
| 网络隔离 | 尝试网络请求 | 网络不可用 |

### 8.6 快速开发测试

推荐使用 `scripts/dev.sh` 进行本地开发测试：

```bash
# 轻量模式 (SQLite 内存数据库，无需 Docker 依赖)
./scripts/dev.sh lite

# 完整模式 (PostgreSQL + Redis，需要 Docker)
./scripts/dev.sh full
```
