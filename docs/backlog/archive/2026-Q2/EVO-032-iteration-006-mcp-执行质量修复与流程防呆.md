# EVO-032 Iteration 006 MCP 执行质量修复与流程防呆

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: bug
- Status: Done
- Priority: P0
- Source: 质量审查 2026-05-25
- Decision Context: Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真

#### Source Detail Snapshot

- 类型：bug
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：恢复 MCP HTTP 工具执行的可运行性和访问边界，确保迭代完成状态只能建立在可复现验证证据上。
- 范围：
  - 修复 `HttpToolExecutor` 初始化时读取系统代理导致的启动/测试 panic。
  - 要求 MCP `tools/call` 持有有效 API Key，禁止匿名触发真实 HTTP 工具执行。
  - 将上游 HTTP 4xx/5xx 映射为 MCP error，而不是成功 result。
  - 使用本地 mock HTTP 服务覆盖真实调用、鉴权、错误状态和超时测试，不依赖公网服务。
  - 修复全量测试暴露的日志敏感字段脱敏无限循环，恢复 workspace 测试门禁。
  - 更新迭代与验证流程，禁止未执行或失败的门禁被勾选为完成。
- 不做：
  - 不扩展 Function 类型工具执行。
  - 不设计通用代理配置能力；若后续需要受控出站代理，另行进入 backlog。
  - 不重写 Iteration 006 历史记录，只记录其审查结论和本次补救。
- 验收标准：
  - [x] `HttpToolExecutor::new()` 在本地测试和应用状态初始化中不触发系统代理初始化 panic。
  - [x] 未带有效 API Key 的 `tools/call` 不能触发公开或私有 HTTP 工具执行。
  - [x] 有效 API Key 可调用本租户 HTTP 工具并取得 MCP text content。
  - [x] 上游 HTTP 4xx/5xx 与超时均返回 MCP error。
  - [x] MCP 集成测试完全使用本地可控服务，并覆盖上述路径。
  - [x] 日志脱敏包含敏感字段时能完成并覆盖多个字段，不再阻塞全量测试。
  - [x] `cargo check --workspace`、`cargo clippy --workspace -- -D warnings`、`cargo test --workspace` 通过。
  - [x] `cargo fmt --all -- --check` 的结果如实记录；历史格式债已登记为 EVO-033，未虚报通过。
  - [x] SOP 与 `EVOLUTION.md` 已写回造成误验收的流程缺口。
- 依赖或阻塞：Iteration 006 / EVO-005 已合入；历史 workspace rustfmt 基线需要单独处理或明确记录。
- 影响范围：backend / docs
- 最小验证方式：`cargo fmt --all -- --check`；`cargo check --workspace`；`cargo clippy --workspace -- -D warnings`；`cargo test --workspace`。
