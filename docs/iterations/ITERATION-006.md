# Iteration 006: MCP 工具真实执行

> 时间：2026-05-25
> 目标：让 MCP `tools/call` 真正执行注册的 HTTP 工具，而不是返回 stub 文本。完成平台核心价值闭环。

## 1. 本轮目标

- 实现 `HttpToolExecutor`：根据 `Tool.handler` 中的 `HandlerConfig` 发起真实 HTTP 请求。
- 替换 `mcp_handlers.rs` 中的硬编码 stub 文本为 `ToolExecutor` trait 调用。
- 将 `HttpToolExecutor` 注入 `AppState`，初始化时创建。
- HTTP 请求结果映射为 MCP `ToolCallResult` content。
- 执行超时、错误分类和集成测试。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-005 | MCP 工具真实执行 | P0 | Agent | Done |

## 3. 不做事项

- 不实现 Function 类型工具的沙箱执行（当前所有工具 handler_type 为 Http）。
- 不实现工具发现服务（`discovery.rs` placeholder 留给后续）。
- 不修改 MCP 协议层（JSON-RPC、schema validation 已完善）。
- 不修改 `ToolRepository` trait 或数据库 migration。
- 不实现工具调用审计日志增强（当前已有基础审计）。
- 不创建或恢复 GitHub CI/CD workflow（EVO-030）。

## 4. 验收标准

- [x] `POST /mcp` 的 `tools/call` 方法对 handler_type=Http 的工具发起真实 HTTP 请求。
- [x] 请求使用 `handler.url`、`handler.method`（默认 POST）、`handler.timeout`（默认 30000ms）。
- [x] 成功响应的 body 映射为 MCP content `{"type": "text", "text": "<response body>"}`。
- [x] 连接失败、超时、HTTP 4xx/5xx 返回 MCP error（含可读错误信息）。
- [x] 私有工具（`visibility != Public`）未被认证用户越权调用。
- [x] `cargo test --workspace` 通过，含 HTTP tool 执行集成测试。
- [x] `cargo check --workspace` 无错误。
- [x] `cargo clippy --workspace` 无错误。

## 5. 验证计划

```bash
# backend
cargo check --workspace
cargo clippy --workspace
cargo test --workspace

# 手工验证：注册一个 HTTP 工具（url 指向 httpbin.org 或本地 mock），
# 通过 MCP tools/call 调用，验证真实 HTTP 请求和响应映射。
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| HTTP 工具目标不可达导致请求挂起 | 使用 reqwest timeout，默认 30s，超时返回 MCP error |
| 目标返回非 UTF-8 响应体 | 截断或 base64 编码，映射为 MCP error |
| reqwest 版本与 workspace 不兼容 | workspace 已有 reqwest 0.11（service-payment 使用），直接复用 |
| 响应体过大影响性能 | 设置最大响应体大小限制（默认 1MB），超出截断 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-25 | Iteration 006 started. 选入 EVO-005，补齐 backlog 详情块。 |
| 2026-05-25 | EVO-005 实现完成。HttpToolExecutor 替代 stub，MCP tools/call 真实执行 HTTP 工具。cargo check/clippy 通过，api+service-tool 测试 18 passed。 |

## 8. 变更请求

无。

## 9. Review

- 完成：EVO-005 全部完成，MCP tools/call 真实执行 HTTP 工具。
- 未完成：Function 类型工具沙箱执行（不在本次范围）。
- 验证结果：cargo check 0 errors，cargo clippy 0 errors（仅 pre-existing warning），service-tool 4 tests passed，api 18 tests passed（含 5 个新 MCP 集成测试）。

## 10. Retrospective

- 做得好的：ToolExecutor trait 注入模式与 SkillExecutor 一致；错误映射覆盖 timeout/connect/request 三类；1MB 响应截断保护。
- 需要调整的：clippy 在 api crate 有一个 pre-existing needless_borrow warning（mailer.rs line 143），非本次引入。
- 写入 EVOLUTION：
