# EVO-118-C HTTP Tool 出站安全与 SSRF 防护

- **类型**：Security / Network Boundary
- **状态**：Ready
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A merge
- **影响范围**：backend / config / deploy / docs / tests

## 用户价值

作为平台管理员，我需要租户配置的 HTTP Tool 只能访问允许的外部目标，不能探测或调用 Evolith 所在主机、容器网络、云 Metadata 或其他内部服务。

## 已确认失败模式

- Tool URL 由租户提供，创建/更新时没有目标地址安全校验。
- HTTP Executor 可直接请求 localhost 和私网地址。
- 缺少 DNS Rebinding、Redirect 重新校验、Metadata 地址和受控 Egress 边界。
- 普通成员可以创建/更新 HTTP Tool，扩大 SSRF 攻击面。

## 验收场景

### Scenario 1：本地和私网地址被拒绝

- **Given** Tool URL 指向 loopback、RFC1918、link-local、multicast 或 reserved 地址
- **When** 创建/更新或执行 Tool
- **Then** 请求被拒绝，目标服务未收到连接

### Scenario 2：DNS/Redirect 不能绕过

- **Given** 公网域名解析到私网，或公网 URL Redirect 到私网
- **When** 执行 Tool
- **Then** 每次连接前重新校验最终地址并拒绝

### Scenario 3：受控公网目标可执行

- **Given** 目标命中 allowlist 或满足 Egress Policy
- **When** 使用具有 `execute` capability 的 Key 调用
- **Then** 请求成功，超时/响应大小/并发均受限并写入审计

### Scenario 4：低权限用户不能创建任意出站 Tool

- **Given** 当前用户为普通 Member
- **When** 创建或修改 HTTP Tool handler URL
- **Then** 按角色矩阵拒绝或进入明确审核流程

## 工程要求

- 建立统一 `EgressPolicy` / `SafeHttpClient` 边界，业务代码不直接使用通用 `reqwest::Client` 请求租户 URL。
- 仅允许明确 Scheme；默认优先 HTTPS。
- DNS 解析后检查最终 IP，并处理 IPv4/IPv6、Redirect 和 DNS Rebinding。
- 默认关闭系统代理；如使用代理，必须是受控 Egress Proxy。
- 保留并测试超时、1 MiB 响应上限，新增 Header、Redirect、并发和连接上限。
- 日志不回显凭证、完整内部 URL 或大响应体。
- Tool 创建/更新权限与 EVO-118-B 的 Typed Capability 对齐。

## 不做事项

- 不构建通用企业级 API Gateway。
- 不支持任意自定义代理或用户自带网络插件。
- 不在本 Story 实现 Webhook Out；Webhook 也必须复用同一 Egress Policy。

## 最小验证

- localhost、RFC1918、Metadata、IPv6 loopback、DNS→私网、Redirect→私网测试。
- 公网 Mock 成功、超时、超大响应、连接失败测试。
- Tool 创建/更新角色矩阵测试。
- 审计和错误脱敏检查。
- `cargo test -p service-tool`、相关 API integration tests、workspace clippy/test。

## 解锁内容

解除 SEC-02 Gate；允许 MCP HTTP Tool 在 Internal/External Alpha 中受控启用，并为 EVO-107 Webhook 复用安全出站边界。
