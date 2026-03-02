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

#### 测试模块位置

| Crate | 测试文件 | 测试内容 |
|-------|---------|---------|
| service-auth | `src/jwt.rs` | JWT 令牌生成与验证 |
| service-auth | `src/password.rs` | 密码哈希与验证 |
| common | `src/error.rs` | 错误类型定义 |
| common | `src/log.rs` | 日志配置 |
| api | `src/middleware/tenant.rs` | 多租户中间件 |

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

| 邮箱 | 密码 | 角色 |
|-----|------|-----|
| demo@evolith.io | demo123! | admin |

### 5.2 测试工具

| 名称 | 描述 | 分类 |
|-----|------|-----|
| filesystem_read | Read files from the filesystem with safety checks | filesystem |
| http_request | Make HTTP requests to external APIs | network |
| database_query | Execute SQL queries against connected databases | database |
| web_scraper | Extract structured data from web pages | network |
| json_transform | Transform JSON data using JSONPath expressions | data |

## 6. 持续集成

### 6.1 本地检查

```bash
# 格式化检查
cargo fmt --check

# Lint 检查
cargo clippy -- -D warnings

# 运行测试
cargo test --workspace

# 构建
cargo build --release
```

### 6.2 前端检查

```bash
# 安装依赖
cd frontend && npm install

# Lint 检查
npm run lint

# 类型检查
npm run type-check

# 构建
npm run build
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
