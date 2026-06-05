# EVO-005 MCP 工具真实执行

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: feature
- Status: Done
- Priority: P0
- Source: 需求 F1.1.3 / Iteration 006
- Decision Context: HTTP handler type 真实执行，Function 类型暂不支持

#### Source Detail Snapshot

- 类型：feature
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：让 MCP `tools/call` 真正执行注册的 HTTP 工具，而不是返回 stub 文本。这是平台核心价值闭环——工具注册后可被外部 AI Agent 通过 MCP 协议真实调用。
- 范围：
  - 实现 `HttpToolExecutor`：根据 `Tool.handler` 中的 `HandlerConfig`（type=Http, url, method, timeout）发起真实 HTTP 请求。
  - 将 `DefaultToolExecutor` 替换为基于 `HandlerType` 分发的执行器。
  - `handle_tools_call` 中调用 `ToolExecutor` trait 而非硬编码 stub 文本。
  - HTTP 请求结果映射为 MCP `ToolCallResult`（content type: text，body 为响应体或错误信息）。
  - 执行超时保护：使用 `handler.timeout`（毫秒，默认 30s）。
  - 错误分类：连接失败、超时、HTTP 错误状态码、响应体过大等。
  - 集成测试覆盖 HTTP 工具真实调用。
- 不做：
  - 不实现 Function 类型工具的沙箱执行（当前所有工具 handler_type 为 Http）。
  - 不实现工具发现服务（`discovery.rs` placeholder 留给后续）。
  - 不修改 MCP 协议层（JSON-RPC、schema validation 已完善）。
  - 不修改 `ToolRepository` trait 或数据库 migration。
  - 不实现工具调用审计日志增强（当前已有基础审计）。
  - 不实现工具调用限流（当前 API key 级别限流已覆盖）。
- 验收标准：
  - [x] `POST /mcp` 的 `tools/call` 方法对 handler_type=Http 的工具发起真实 HTTP 请求。
  - [x] 请求使用 `handler.url`、`handler.method`（默认 POST）、`handler.timeout`（默认 30000ms）。
  - [x] 成功响应的 body 映射为 MCP content `{"type": "text", "text": "<response body>"}`。
  - [x] 连接失败、超时、HTTP 4xx/5xx 返回 MCP error（含可读错误信息）。
  - [x] 私有工具（`visibility != Public`）未被认证用户越权调用。
  - [x] `cargo test --workspace` 通过，含 HTTP tool 执行集成测试。
  - [x] `cargo check --workspace` 无错误。
  - [x] `cargo clippy --workspace` 无错误。
- 技术备注：
  - `reqwest` 已在 workspace（`service-payment` 使用），需要添加到 `service-tool/Cargo.toml`。
  - `Tool` 域模型已有 `HandlerConfig { handler_type, url, method, timeout }`，无需修改 domain 层。
  - `service-tool/src/executor.rs` 当前 `DefaultToolExecutor` 返回 stub，需要替换为 `HttpToolExecutor`。
  - `mcp_handlers.rs` line 250 硬编码 stub 文本，需改为调用 `ToolExecutor::execute`。
  - `AppState` 需要添加 `tool_executor: Arc<dyn ToolExecutor>` 字段。
- 依赖：无外部依赖阻塞。`reqwest` 已在 workspace。
- 影响范围：backend
- 最小验证方式：`cargo test --workspace`；手工 curl 测试 MCP `tools/call` 对 HTTP 工具的真实调用。
