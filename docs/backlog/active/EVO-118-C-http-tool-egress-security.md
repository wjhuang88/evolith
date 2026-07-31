# EVO-118-C HTTP Tool 出站安全与 SSRF 防护

- **类型**：Security / Network Boundary
- **状态**：Review
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A/B Done
- **所属迭代**：[Iteration 052](../../iterations/ITERATION-052.md)
- **影响范围**：backend / docs / tests / CI
- **PR**：#5 `security: harden HTTP tool egress and SSRF boundaries`（Draft）

## 用户价值

作为平台管理员，我需要租户配置的 HTTP Tool 只能访问允许的外部目标，不能探测或调用 Evolith 所在主机、容器网络、云 Metadata 或其他内部服务。

## 已确认失败模式

- Tool URL 由租户提供，创建/更新时没有目标地址安全校验。
- HTTP Executor 可直接请求 localhost 和私网地址。
- 缺少 DNS Rebinding、Redirect 重新校验、Metadata 地址和受控 Egress 边界。
- 普通成员可以创建/更新 HTTP Tool，扩大 SSRF 攻击面。
- 请求 deadline 未覆盖 DNS 与完整 Redirect 链，历史超大 timeout 可绕过 API 校验。
- 测试 Cargo feature 可通过 feature unification 弱化生产默认策略。
- 外租户 Tool 的真实 UUID 可经调用者审计流泄露。
- IPv6 特殊用途地址分类过宽。

## 验收场景

### Scenario 1：本地和私网地址被拒绝

- **Given** Tool URL 指向 loopback、RFC1918、link-local、multicast、documentation、benchmark 或 reserved 地址
- **When** 创建/更新或执行 Tool
- **Then** 请求被拒绝，目标服务未收到连接

### Scenario 2：DNS/Redirect 不能绕过

- **Given** 公网域名解析到私网，或公网 URL Redirect 到私网
- **When** 执行 Tool
- **Then** 每次连接前重新校验最终地址并拒绝

### Scenario 3：受控公网目标可执行

- **Given** 目标满足 Egress Policy
- **When** 使用具有 `execute` capability 的 Key 调用
- **Then** 请求成功，超时、响应大小、Header、Redirect 和并发均受限并写入审计

### Scenario 4：低权限用户不能创建任意出站 Tool

- **Given** 当前用户为 Member、API Key caller 或跨租户 Owner/Admin
- **When** 创建或修改 HTTP Tool handler URL
- **Then** 授权先于 DTO 解析和目标访问，统一 fail closed；仅同租户 Owner/Admin JWT 可管理

## 工程要求与实施状态

- [x] 建立统一 `EgressPolicy` / `SafeHttpClient` 边界，生产 HTTP Tool 路径不再直接信任租户 URL。
- [x] 仅允许 HTTP/HTTPS，拒绝 userinfo、非法 URL、Metadata hostname 和特殊用途 IP。
- [x] DNS 解析后校验全部 A/AAAA；任一禁止地址即整体拒绝，不降级为 reqwest 自行解析。
- [x] 将批准地址集合固定到连接层；禁用系统代理和自动 Redirect。
- [x] Redirect 显式逐跳处理，每跳重新执行 URL、DNS 和 IP 校验。
- [x] 一个总 deadline 覆盖 DNS、连接、Redirect、Header 和响应体读取；运行时再次强制 `1..=30000 ms`。
- [x] 保留 1 MiB 响应上限，增加 Header、Redirect、连接和并发边界。
- [x] `HttpProxyProvider::new()`、`HttpToolExecutor::new()` 和 `SafeHttpClient::default()` 永久使用严格生产策略。
- [x] 删除可经 `--all-features` 弱化默认构造器的 `test-egress` feature；localhost 测试显式注入测试策略。
- [x] Tool 创建/更新限定为同租户 Owner/Admin JWT；Member、API Key caller 和跨租户调用先鉴权后解析。
- [x] MCP 执行保持 `execute` capability 与 tenant ownership 门禁。
- [x] foreign/missing Tool 使用相同响应，并且不在调用者审计中记录外租户 resource UUID。
- [x] 错误与审计不回显完整 URL、DNS 列表、内部 IP 或大响应体。
- [x] 覆盖 IPv4/IPv6 loopback、私网、link-local、mapped IPv4、multicast、benchmark、ORCHID/ORCHIDv2、documentation、6to4 和 reserved 地址。
- [x] 补齐 Navigator 点名的 public→private Redirect 网络前拒绝、慢 Redirect 链共享 deadline、Header/并发上限、合法成功投影和 audit-visible foreign/missing 对照证据。
- [ ] 获得最新 head 的 Navigator 安全复核结论。
- [ ] Navigator 通过后再同步 SEC-02 为解除并将 Story 标记 Done。

## 不做事项

- 不构建通用企业级 API Gateway。
- 不支持任意自定义代理或用户自带网络插件。
- 不在本 Story 实现 Webhook Out；Webhook 后续必须复用同一 Egress Policy。
- 不关闭 DATA-01、DEPLOY-01 或其他 EVO-118 Gate。

## 实际验证

实现与最终定向安全证据锚点：`f8d78dedf71829bf4e35c72a57701d4142def4da`。

GitHub Actions run `30643832546`（CI #115）实际结果：

- `bun install --frozen-lockfile`：通过。
- `bun run type-check`：通过。
- `bun run build`：通过。
- `cargo fmt --all -- --check`：通过。
- `cargo check --workspace --all-targets`：通过。
- `cargo clippy --workspace --all-targets -- -D warnings`：通过。
- `cargo test --workspace`（SQLite required gate）：通过。
- Frontend lint 与 PostgreSQL 扩展测试为 tag/manual gate，本次 PR required run 正常跳过。

CI 修复过程中验证并关闭了以下实际回归：

- 测试夹具不能隐式依赖生产 localhost 放行，必须显式注入测试策略。
- `2001:db8::/32` 必须显式归入 IPv6 documentation 拒绝范围。
- 总 timeout 必须跨越多个 Redirect hop，而不是每跳重置。
- Header 与并发限制必须在安全边界内 fail closed。
- foreign 与 missing Tool 在调用者可见响应及审计数据中均不可区分。

本次治理-only 收口提交之后，PR 仍需以新的 exact head 再跑一次 required CI；最终 run 记录在 PR 描述和 Navigator 复核评论中，避免为写入自身未来 SHA 而制造无限文档提交。

## Navigator 评论处理

| 阻塞项 | 当前处理 |
|--------|----------|
| 治理未激活、PR 描述过期 | Story / Iteration / Epic / Backlog / Board / docs 入口均同步到 Review，PR 描述已改为真实实现与验证状态 |
| DNS 与 Redirect 不在统一 deadline 内 | 外层 `tokio::time::timeout` 覆盖完整执行；配置校验 DNS 受 connect timeout 限制；历史 timeout 在 provider 内再次校验；慢 Redirect 链测试证明不会逐跳重置 |
| `test-egress` 可弱化生产默认 | 已删除 feature；生产默认构造器永久 fail closed，测试显式注入 permissive policy |
| 外租户 Tool UUID 经审计泄露 | foreign/missing 均记录 `resource_id=None`、相同 action/reason/details，并写入调用者 Tenant，不泄露 foreign UUID |
| IPv6 特殊用途地址放行 | 已补保守拒绝表和回归矩阵，包括 benchmark、ORCHID/ORCHIDv2、documentation、6to4 与 `3fff::/20` |

## 定向安全证据

| 风险 | 证据 |
|------|------|
| DNS mixed result / fallback | `mixed_dns_results_fail_closed_before_request`、`dns_failure_does_not_fall_back_to_reqwest_resolution` |
| DNS / Redirect 总 deadline | `execution_deadline_includes_dns_resolution`、`configuration_validation_bounds_hanging_dns`、`total_deadline_is_shared_across_redirect_hops` |
| public→private Redirect | `private_redirect_is_rejected_before_dns_or_connection`，禁止目标在后续 DNS/连接前拒绝 |
| 响应与并发限制 | `test_http_executor_rejects_oversized_response`、`response_header_limit_is_enforced`、`concurrency_limit_fails_closed_without_waiting` |
| 生产安全默认 | `production_default_rejects_loopback_even_with_test_feature`、`direct_executor_default_rejects_loopback`，且 permissive Cargo feature 已删除 |
| 合法成功投影 | `test_mcp_tools_call_executes_authenticated_http_tool` 保持 MCP text/JSON 投影 |
| foreign/missing 隐藏语义 | `execute_key_cannot_discover_or_distinguish_foreign_tenant_tool` 同时断言响应、网络 hit、audit action/reason/details/resource_id 与 tenant stream |

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 建立可验证的统一 HTTP Egress 安全边界，并在 Navigator 通过后解除 SEC-02 |
| 产物 | Egress Policy / Resolver / Safe Client、真实执行链接线、管理权限与审计、SSRF 负向测试、治理同步 |
| 状态同步归口 | EVO-118-C Story、EVO-118 Epic、Product Backlog、Board、docs 入口、Iteration 052、PR #5 |
| 验证证据 | 实现/测试 head `f8d78de...` 的 CI #115 / run `30643832546` required gates 全绿；治理-only 最终 head 的 run 与 Navigator 结论记录在 PR |
| 残余工作归口 | 仅剩 Navigator 最新结论和 SEC-02 状态关闭；Webhook 复用归 EVO-107；Git 耐久性归 EVO-118-D；部署归 EVO-118-E |

## 当前结论

- 已实施：统一 egress、总 deadline、严格生产默认、Tool 管理门禁、审计脱敏和 IPv6 特殊用途拒绝。
- 已验证：实现与全部定向安全证据通过 required CI。
- 未完成：治理-only 最终 head CI、Navigator 最新安全结论及 SEC-02 状态关闭。
- 闭环状态：`Partial`。

## 解锁内容

仅在最终 exact-head CI 与 Navigator 安全复核通过、SEC-02 正式解除后，才允许 MCP HTTP Tool 在 Internal/External Alpha 中按受控策略启用，并为 EVO-107 Webhook 复用安全出站边界。
