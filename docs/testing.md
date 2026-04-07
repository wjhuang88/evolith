# 测试文档

本文档记录 Evolith 项目的测试用例、测试方法和最佳实践。

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

> **最新数据 (2026-04-08)**: `cargo test --workspace` → **237 passed, 0 failed**

| Crate | 测试数 | 说明 |
|-------|--------|------|
| api | 34 | Handler 测试 + E2E 认证测试 (12 个) |
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

## 3. 前端测试

### 3.1 构建验证

前端使用 Next.js 的构建系统进行类型检查和构建验证。

```bash
# 进入前端目录
cd frontend

# 类型检查
npm run type-check

# 构建
npm run build

# 开发模式
npm run dev
```

### 3.2 手动测试清单

#### 认证流程

| 测试项 | 预期结果 |
|-------|---------|
| 访问 `/login` | 显示登录表单 |
| 使用正确凭证登录 | 重定向到 `/dashboard` |
| 使用错误凭证登录 | 显示错误信息 |
| 访问 `/dashboard` (未登录) | 重定向到 `/login` |

#### 工具管理

| 测试项 | 预期结果 |
|-------|---------|
| 访问 `/tools` | 显示工具列表 |
| 点击 "Create Tool" | 显示创建表单 |
| 填写表单并提交 | 新工具出现在列表中 |
| 点击 "Delete" | 工具从列表中移除 |

#### 技能管理

| 测试项 | 预期结果 |
|-------|---------|
| 访问 `/skills` | 显示技能列表 |
| 点击 "Delete" | 技能从列表中移除 |

#### 代码片段

| 测试项 | 预期结果 |
|-------|---------|
| 访问 `/snippets` | 显示片段列表 |
| 输入搜索关键词 | 显示匹配结果 |
| 点击 "Delete" | 片段从列表中移除 |

#### MCP 协议

| 测试项 | 预期结果 |
|-------|---------|
| POST `/mcp` (initialize) | 返回服务器信息 |
| POST `/mcp` (tools/list) | 返回工具列表 |
| POST `/mcp` (tools/call) | 返回工具执行结果 |

## 4. API 测试

### 4.1 健康检查

```bash
curl http://localhost:8080/health
```

预期响应:
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### 4.2 认证 API

#### 登录

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "demo@evolith.io",
    "password": "demo123!"
  }'
```

#### 注册

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@example.com",
    "username": "newuser",
    "password": "password123",
    "tenant_name": "My Company",
    "tenant_slug": "my-company"
  }'
```

### 4.3 工具 API

| 方法 | 端点 | 描述 |
|-----|------|-----|
| GET | `/api/v1/tools` | 列出所有工具 |
| GET | `/api/v1/tools/{id}` | 获取单个工具 |
| POST | `/api/v1/tools` | 创建工具 |
| PUT | `/api/v1/tools/{id}` | 更新工具 |
| DELETE | `/api/v1/tools/{id}` | 删除工具 |

### 4.4 技能 API

| 方法 | 端点 | 描述 |
|-----|------|-----|
| GET | `/api/v1/skills` | 列出所有技能 |
| GET | `/api/v1/skills/{id}` | 获取单个技能 |
| POST | `/api/v1/skills` | 创建技能 |
| PUT | `/api/v1/skills/{id}` | 更新技能 |
| DELETE | `/api/v1/skills/{id}` | 删除技能 |
| POST | `/api/v1/skills/{id}/execute` | 执行技能 |

### 4.5 片段 API

| 方法 | 端点 | 描述 |
|-----|------|-----|
| GET | `/api/v1/snippets` | 列出所有片段 |
| GET | `/api/v1/snippets/{id}` | 获取单个片段 |
| POST | `/api/v1/snippets` | 创建片段 |
| PUT | `/api/v1/snippets/{id}` | 更新片段 |
| DELETE | `/api/v1/snippets/{id}` | 删除片段 |
| GET | `/api/v1/snippets/search?q={query}` | 搜索片段 |

## 5. 测试数据

### 5.1 测试用户

> **注意**: 系统现使用 Argon2id 密码哈希。预置的 demo 用户 (`demo@evolith.io` / `demo123!`) 的明文密码已不再有效。测试时请通过 `/api/v1/auth/register` 注册新用户，注册时密码会自动哈希。

| 邮箱 | 密码 | 角色 | 备注 |
|-----|------|-----|------|
| *(通过注册创建)* | *(符合密码强度要求)* | admin (注册者自动为 owner) | 使用 `./scripts/dev.sh lite` 启动后注册 |

### 5.2 测试工具

| 名称 | 描述 | 分类 |
|-----|------|-----|
| filesystem_read | Read files from the filesystem with safety checks | filesystem |
| http_request | Make HTTP requests to external APIs | network |
| database_query | Execute SQL queries against connected databases | database |
| web_scraper | Extract structured data from web pages | network |
| json_transform | Transform JSON data using JSONPath expressions | data |

## 6. 持续集成

### 6.1 CI Pipeline (`.github/workflows/ci.yml`)

CI 在每次 push/PR 时自动运行以下步骤：

1. `cargo fmt --check` — 格式检查
2. `cargo clippy --workspace -- -D warnings` — Lint 检查（warnings 视为错误）
3. `cargo test --workspace` — 全量测试 (237 tests)
4. `cargo audit` — 依赖安全审计
5. `npm audit` — 前端依赖安全审计
6. Docker build — 验证镜像构建

### 6.2 本地检查

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

### 6.3 前端检查

```bash
# 安装依赖
cd frontend && npm install

# Lint 检查
npm run lint

# 类型检查
npm run type-check

# 构建
npm run build

# 安全审计
npm audit
```

## 7. 测试覆盖率

使用 `cargo-tarpaulin` 生成测试覆盖率报告:

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --workspace --out Html
```

## 8. 调试技巧

### 8.1 后端调试

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 跟踪特定模块
RUST_LOG=evolith::api=trace cargo run
```

### 8.2 前端调试

```bash
# 开发模式 (支持热重载)
npm run dev

# 生产构建分析
ANALYZE=true npm run build
```


## 9. Phase 6 测试用例 (用户与权限系统)

### 9.1 用户注册与登录

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 用户注册 | 提交有效用户信息 | 创建用户并自动创建租户 |
| 用户注册-无效邮箱 | 提交无效邮箱格式 | 验证失败，返回错误 |
| 用户注册-密码过短 | 密码少于8字符 | 验证失败，返回错误 |
| 用户登录 | 使用正确凭证登录 | 返回JWT token |
| 用户登录-错误密码 | 使用错误密码 | 认证失败，返回错误 |
| JWT token验证 | 验证token包含正确声明 | token包含user_id, tenant_id, role |
| Token过期处理 | 使用过期token | 返回认证错误 |

### 9.2 密码安全

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 密码哈希 | 验证Argon2id哈希 | 哈希以$argon2id$开头 |
| 密码验证 | 验证正确密码 | 返回true |
| 密码验证-错误 | 验证错误密码 | 返回false |
| 唯一哈希 | 相同密码多次哈希 | 产生不同哈希(随机salt) |
| 密码强度-有效 | 符合所有复杂度要求 | 验证通过 |
| 密码强度-无大写 | 缺少大写字母 | 验证失败 |
| 密码强度-无小写 | 缺少小写字母 | 验证失败 |
| 密码强度-无数字 | 缺少数字 | 验证失败 |
| 密码强度-无特殊字符 | 缺少特殊字符 | 验证失败 |
| 密码强度-过长 | 超过128字符 | 验证失败 |
| Unicode密码 | 使用Unicode字符 | 哈希成功 |

### 9.3 RBAC (角色访问控制)

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 角色层级-Owner | Owner角色权限 | 可以执行所有操作 |
| 角色层级-Admin | Admin角色权限 | 不能执行owner级别操作 |
| 角色层级-Member | Member角色权限 | 只能执行member级别操作 |
| 租户访问-正确 | 访问自己租户资源 | 允许访问 |
| 租户访问-错误 | 访问其他租户资源 | 拒绝访问 |
| 角色不足-Owner所需 | Member尝试Owner操作 | 返回Forbidden |
| Bearer Token提取 | 从Authorization头提取 | 正确提取token |
| API Key提取 | 从X-API-Key头提取 | 正确提取key |
| 无Token访问 | 无认证信息访问 | 返回Unauthorized |

### 9.4 租户模型

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 默认计划 | TenantPlan默认值 | 返回Free |
| 计划显示 | 各计划to_string | 返回小写计划名 |
| Quotas-Free计划 | Free计划配额 | 3用户, 5工具, 10技能 |
| Quotas-Starter计划 | Starter计划配额 | 10用户, 20工具, 50技能 |
| Quotas-Pro计划 | Pro计划配额 | 50用户, 100工具, 200技能 |
| 租户邀请序列化 | Invitation转JSON | 正确序列化角色 |
| 邀请请求验证 | 验证有效邀请请求 | 验证通过 |
| 邀请请求-无效邮箱 | 验证无效邮箱 | 验证失败 |
| 创建租户请求-无效slug | slug包含下划线 | 验证失败 |

### 9.5 审计日志

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 审计日志创建 | 创建完整审计日志 | 所有字段正确填充 |
| 审计日志序列化 | 序列化AuditLog | 正确转为JSON |
| 系统级审计 | 创建无租户审计日志 | tenant_id为None |
| 复杂详情 | 嵌套JSON详情 | 正确序列化 |

### 9.6 运行 Phase 6 测试

```bash
# 运行所有后端测试
cargo test --workspace

# 仅运行 domain crate 测试
cargo test -p domain

# 仅运行 service-auth 测试
cargo test -p service-auth

# 仅运行 api 测试
cargo test -p api

# 运行特定测试
cargo test test_jwt
cargo test test_password
cargo test test_role_hierarchy
```

## 10. 基础设施测试 (Phase 3-8)

### 10.1 缓存 (Phase 3)

```bash
cargo test -p infra -- cache
```

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| InMemoryCache 基本操作 | get/set/delete | 正确存取和删除 |
| TTL 过期 | 设置带过期时间的缓存 | 过期后返回 None |
| RedisCache (需要 Redis) | Redis 连接和操作 | 正确读写 Redis |

### 10.2 邮件 (Phase 3)

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| ConsoleMailer | 开发环境邮件 | 输出到控制台，不实际发送 |
| SmtpMailer (需要 SMTP) | 生产环境邮件 | 通过 SMTP 发送邮件 |

### 10.3 中间件 (Phase 3/6/8)

| 中间件 | 测试内容 | 预期结果 |
|-------|---------|---------|
| CSRF | 状态变更请求缺少 CSRF token | 返回 403 |
| CSRF | 正确的 CSRF 双提交 cookie | 请求通过 |
| Rate Limit | 超出频率限制 | 返回 429 |
| Request ID | 请求无 X-Request-ID | 自动生成并注入 |
| Request ID | 请求携带 X-Request-ID | 透传原始 ID |
| Security Headers | 响应头检查 | 包含 CSP, X-Frame-Options 等 |

### 10.4 健康检查 (Phase 3)

```bash
# 基本健康检查
curl http://localhost:8080/health

# 存活探针 (Kubernetes)
curl http://localhost:8080/health/live

# 就绪探针 (Kubernetes)
curl http://localhost:8080/health/ready
```

### 10.5 日志脱敏 (Phase 8)

```bash
cargo test -p common -- sanitize
```

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| 密码脱敏 | 日志包含 password 字段 | 值被替换为 `[REDACTED]` |
| Token 脱敏 | 日志包含 token 字段 | 值被替换为 `[REDACTED]` |
| API Key 脱敏 | 日志包含 api_key 字段 | 值被替换为 `[REDACTED]` |

### 10.6 沙箱执行器 (Phase 7)

> 需要 Docker 环境。使用 `SANDBOX__ENABLED=true` 启用。

| 测试项 | 测试内容 | 预期结果 |
|-------|---------|---------|
| Python 执行 | 执行 Python 3.11 代码 | 正确返回输出 |
| Node.js 执行 | 执行 Node.js 20 代码 | 正确返回输出 |
| 超时保护 | 执行超过 30 秒的代码 | 返回超时错误 |
| 内存限制 | 执行消耗大量内存的代码 | 返回资源限制错误 |
| 网络隔离 | 尝试网络请求 | 网络不可用 |

### 10.7 快速开发测试

推荐使用 `scripts/dev.sh` 进行本地开发测试：

```bash
# 轻量模式 (SQLite 内存数据库，无需 Docker 依赖)
./scripts/dev.sh lite

# 完整模式 (PostgreSQL + Redis，需要 Docker)
./scripts/dev.sh full
```
