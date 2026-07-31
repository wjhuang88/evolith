# Iteration 052: HTTP Tool 出站安全与 SSRF 防护

> 文档状态：Planned
> 计划发布日期：2026-07-31
> 计划目标：完成 EVO-118-C，建立 HTTP Tool 的统一出站安全边界并关闭 SEC-02。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 0. 启动前库存处置

| Iteration | 当前状态 | 本轮处置 |
|-----------|----------|----------|
| 018 / 019 / 020 / 027 | Blocked for activation / Superseded direction | 继续保持，不激活旧 Registry / Skill 主线 |
| 025 / 026 | Blocked for activation | 继续保持，不抢占 EVO-118 S1 |
| 050 | Closed / Complete | 只读核对，无后续动作 |
| 051 | Closed / Complete | PR #3 / #4 已合并，SEC-01 已解除 |

启动结论：当前没有未处置的 Active / In Progress / Review Iteration；最高已使用编号为 051，Iteration 052 可承载下一 Ready Story EVO-118-C。激活时必须在同一治理变更中将 EVO-118-C 更新为 `In Progress`，并同步 Epic、Product Backlog、Board、文档入口和 Iteration 索引。

## 1. 发布计划基线：目标

关闭 HTTP Tool 的租户可控出站网络缺口：

- 所有 HTTP Tool 执行统一经过 `EgressPolicy` / `SafeHttpClient`，业务 Handler 和 Executor 不再直接信任租户 URL。
- 在创建、更新和每次真实连接前执行 Scheme、URL、DNS、IP、Metadata 与 Redirect 校验。
- DNS 解析结果与实际连接目标绑定，避免 DNS Rebinding 和解析后换址。
- 被拒绝的请求在网络层 hit count 为 0；错误和审计不泄露内部 URL、解析地址或网络拓扑。
- HTTP Tool 管理仅允许同租户 Owner/Admin JWT；Member、API Key caller 和跨租户调用 fail closed。
- 保持合法公网目标的现有结果投影、超时语义和 1 MiB 响应上限，并补充 Header、Redirect、并发和连接边界。

父 Epic 为 EVO-118；本轮唯一目标是解除 SEC-02，不把其他生产就绪 Gate 混入本 Iteration。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-118-C | HTTP Tool 出站安全与 SSRF 防护 | EVO-118 | P0 | EVO-118-A/B Done；PR #4 已合并；最新 `main` 为 `bc4a4a18153b575c47ef76a7a008ae20cf3836c3`；无其他 Active/In Progress/Review Iteration |

执行顺序：

1. 激活治理状态并建立失败测试。
2. 实现统一 URL/IP/DNS Policy 与可注入 Resolver。
3. 实现禁用自动 Redirect、逐跳验证且连接地址固定的 Safe HTTP Client。
4. 将生产真实路径 `HttpProxyProvider -> CompositeProvider -> ToolExecutorAdapter -> MCP tools/call` 接入安全客户端。
5. 收敛或改造 legacy `HttpToolExecutor`，避免保留第二条不受控 reqwest 路径。
6. 增加 Tool 管理角色门禁、跨租户隐藏语义、审计和错误脱敏。
7. 执行定向安全测试、全量门禁和 Navigator 安全复核。

## 3. 发布计划基线：威胁模型与失败模式

| 项目 | 内容 |
|------|------|
| 受保护资产 | Evolith 主机与容器网络、数据库/缓存/对象存储、云 Metadata、其他租户服务、出站凭证、审计记录 |
| 攻击者/调用者 | Tenant Member、API Key caller、被攻陷的 execute Key、恶意或误配置 HTTP Tool、跨租户调用者 |
| 入口 | `POST/PUT /api/v1/tools`、`POST /mcp` 的 `tools/call`、数据库中的 legacy Tool URL、Redirect Location、DNS A/AAAA 结果 |
| 信任边界 | 调用者 → Tool DTO；DB Tool 配置 → Executor；hostname → DNS；DNS 结果 → TCP/TLS 连接；上游响应 → Redirect / MCP 投影 |
| 已确认失败模式 | 创建/更新无角色与目标校验；主线 `HttpProxyProvider` 直接请求 raw URL；自动 Redirect 未逐跳验证；DNS 与连接未固定；错误回显 URL/网络细节；无 Egress 审计 |
| 安全默认 | fail closed；解析失败、任一禁止地址、超限、Redirect 异常或无法证明连接目标时拒绝；拒绝发生在网络访问前 |
| 验证证据 | Policy 单元测试、Resolver/连接边界测试、网络 hit count、API 角色矩阵、MCP E2E、审计与脱敏断言、workspace gates |

### 具体代码路径映射

| 失败模式 | 当前代码路径 | 计划处置 |
|----------|--------------|----------|
| raw URL 可直接持久化 | `api/src/dto/tool_dto.rs` → `api/src/handlers/tool_handlers.rs` → SQLite/PostgreSQL Tool Repository | 授权先于 DTO；统一 Handler 配置校验；创建/更新时执行 URL Policy；Repository 保持数据持久化职责 |
| Member / API Key 可管理 Tool | `tool_handlers.rs` 仅要求 `AuthenticatedUser`；API Key 被映射为 Member | 增加同租户 Owner/Admin JWT 管理门禁；API Key caller 明确拒绝并审计 |
| 主线 Executor 可访问私网 | `main.rs` → `HttpProxyProvider` → `CompositeProvider` → `ToolExecutorAdapter` → `mcp_handlers.rs` | `HttpProxyProvider` 只持有 `SafeHttpClient`；每次执行重新校验并固定解析结果 |
| 存在第二套 reqwest 客户端 | `service-tool/src/executor.rs::HttpToolExecutor` | 复用同一 Safe Client，或在不破坏 facade 合约的前提下移除独立网络实现 |
| Redirect 绕过 | reqwest 默认 Redirect Policy | 禁用自动 Redirect；显式逐跳处理，限制 hop 数，每跳重新验证 Scheme/DNS/IP 并重新固定连接 |
| DNS Rebinding | 系统 DNS 解析与 reqwest 连接未绑定 | 可注入 Resolver；校验全部 A/AAAA，任一禁止即拒绝；连接只使用本次已批准地址集合，不允许二次不受控解析 |
| 错误泄露目标信息 | `map_reqwest_error`、非 2xx stderr、MCP execution error | 对外仅返回稳定错误码/通用信息；日志与审计使用 reason code、tool_id 和脱敏目标标识 |
| 无执行审计 | `mcp_handlers.rs` 未写 HTTP Egress 审计 | 记录成功、策略拒绝和有界失败；归调用者 Tenant，不记录凭证、完整 URL、DNS 列表或响应体 |

## 4. 发布计划基线：不做事项

- 不实施 EVO-118-D 的 Git 持久卷、备份或恢复。
- 不实施 EVO-118-E 的 Docker、Compose、Embedded Frontend 或部署收敛。
- 不实现 Repo UI、Agent Session、Commit/Promote、PolicyEvaluator 或 Durable Outbox。
- 不实现 Webhook Out；只保证未来 Webhook 能复用本轮安全出站边界。
- 不建设通用企业 API Gateway、Service Mesh、Kafka、Kubernetes 网络架构或任意用户自带代理。
- 不重构支付等非租户可控 HTTP Client。
- 不顺手修复 Tool Repository 的事务性、全局按名称查询或 Repo 生命周期问题，除非它们直接阻止本 Story 的安全验收；发现则登记独立残余。

## 5. 发布计划基线：BDD 验收场景

### Scenario A：禁止 Scheme 和非法 URL 在网络前被拒绝

- **Given** HTTP Tool URL 使用非允许 Scheme、缺少 host、包含不允许的 userinfo，或无法规范化
- **When** Owner/Admin 创建、更新或 execute Key 执行 Tool
- **Then** 返回稳定的校验/安全错误，Resolver 与网络 hit count 均为 0，不回显完整目标

### Scenario B：本地、私网和非公网 IP 被拒绝

- **Given** 目标为 IPv4/IPv6 loopback、RFC1918/ULA、link-local、unspecified、multicast、reserved、IPv4-mapped IPv6 或 Cloud Metadata 地址
- **When** 创建、更新或执行 Tool
- **Then** fail closed，目标服务 hit count 为 0，并写入调用者 Tenant 的脱敏拒绝审计

### Scenario C：DNS 不能掩盖禁止地址

- **Given** hostname 解析出一个或多个 A/AAAA，其中任意地址属于禁止类别，或解析失败
- **When** 创建、更新或执行 Tool
- **Then** 整体拒绝，不挑选“看似安全”的记录继续连接，也不降级为 reqwest 自行解析

### Scenario D：DNS 解析结果与连接目标绑定

- **Given** 首次解析得到允许地址，而后续非受控解析可能返回禁止地址
- **When** 执行 Tool
- **Then** 实际连接仅使用本次已批准地址集合；连接层不重新进行不受控 DNS 查询

### Scenario E：Redirect 每一跳重新验证

- **Given** 初始 URL 合法，但 3xx Location 指向私网、Metadata、禁止 Scheme、不同 hostname 或相对地址
- **When** 执行 Tool
- **Then** 每一跳解析并重新执行完整 Policy；禁止目标 hit count 为 0；超过 Redirect 上限时有界失败

### Scenario F：受控公网目标保持兼容

- **Given** URL 满足 Egress Policy，调用 Key 具有 `execute` capability
- **When** 调用 HTTP Tool
- **Then** 返回原有 MCP 内容投影；请求超时、连接超时、响应 Header、1 MiB body、Redirect 和并发均受限，并记录成功审计

### Scenario G：Tool 管理权限 fail closed

- **Given** 调用者为同租户 Member、API Key caller、跨租户 Owner/Admin，或同租户 Owner/Admin JWT
- **When** 创建或更新 Tool
- **Then** 前三类在解析 DTO 和访问目标前被拒绝；跨租户资源与不存在资源保持相同隐藏语义；最后一类可在配置合法时成功

### Scenario H：错误与审计不泄露网络拓扑

- **Given** 请求被 Policy、DNS、连接、Redirect、超限或上游非 2xx 拒绝
- **When** 调用者读取 MCP/API 错误与审计记录
- **Then** 不包含凭证、完整 URL、解析 IP 列表、内部 hostname、完整响应体或底层 reqwest 诊断；审计 Tenant 归属正确

## 6. 发布计划基线：测试矩阵

| 类别 | 用例 | 最低断言 |
|------|------|----------|
| URL / Scheme | `https` 正常；按配置允许的 `http`；`file/gopher/ftp/data`；malformed；userinfo；无 host | 允许项进入 Resolver；拒绝项 Resolver=0、network=0 |
| IPv4 | `127/8`、`10/8`、`172.16/12`、`192.168/16`、`169.254/16`、`0/8`、CGNAT、benchmark、documentation、multicast、reserved | 全部拒绝，分类 reason 稳定 |
| IPv6 | `::1`、`::`、`fc00::/7`、`fe80::/10`、multicast、documentation、IPv4-mapped IPv6 | 全部拒绝，mapped 地址按 IPv4 再分类 |
| Metadata | `169.254.169.254`、常见 Metadata hostname、已知等价地址 | literal/hostname/解析结果均不能绕过 |
| DNS | 单一公网；多公网；公网+私网；仅私网；NXDOMAIN/timeout；地址集变化 | 任一禁止即拒绝；失败不降级；连接使用批准地址集 |
| Redirect | 公网→公网；公网→私网；公网→Metadata；相对 Location；scheme/host 改变；循环；超过上限 | 每跳完整校验；目标 hit count；最大 hop 生效 |
| Side effect | 创建/更新拒绝；执行拒绝；跨租户/无 execute | Repository/Resolver/Executor/网络调用次数符合“拒绝先于副作用” |
| Limits | request timeout、connect timeout、1 MiB+1 body、响应 Header 超限、并发耗尽、连接失败 | 有界失败，无大响应/内部细节回显 |
| 权限 | Owner/Admin/Member/API Key；同租户/跨租户；invalid DTO | 授权优先；Owner/Admin JWT 成功；其他 403/隐藏语义；拒绝审计正确 |
| MCP | 无 Key、无 execute、foreign Tool、missing Tool、合法 HTTP Tool、Policy 拒绝 | 保持 EVO-118-B execute 与资源隐藏语义；安全失败不调用 provider/network |
| 审计 | 管理拒绝、Egress 拒绝、成功、网络失败 | caller tenant、tool/action/reason 正确；无 URL/IP/response 泄露 |

网络零副作用证明采用可计数 Resolver/Transport 或计数 Mock Server，不只断言 HTTP/MCP 返回码。

## 7. 发布计划基线：计划实现边界

### 7.1 统一安全边界

计划在 `service-tool` 内新增：

- `EgressPolicy`：URL 规范化、Scheme、host、port、IP 分类、Metadata 与限制判定。
- `DnsResolver` 抽象及生产 Resolver：返回完整地址集，便于单元测试和 DNS Rebinding 证明。
- `SafeHttpClient`：禁用系统代理和自动 Redirect；每跳解析、校验、固定地址后连接；使用 Semaphore 和连接/响应限制。
- 类型化 `EgressError` / reason code：内部可审计，对外可稳定脱敏。

默认策略：

- `https` 允许；`http` 是否允许由明确配置决定，不能因开发环境自动放宽。
- 系统代理关闭。
- Redirect 显式处理且有最大 hop。
- DNS 任一结果禁止即整体拒绝。
- 仅连接本次校验通过的地址集合。
- 无“解析失败后继续请求”的 fallback。

### 7.2 API 管理边界

- `POST /api/v1/tools` 与 `PUT /api/v1/tools/{id}` 授权先于 DTO 解析。
- 仅同租户 Owner/Admin JWT 可管理；API Key caller 明确拒绝。
- foreign/missing Tool 返回相同资源隐藏语义。
- 安全校验失败不写 Tool、不触发目标网络连接。
- 管理拒绝和安全拒绝写调用者 Tenant 审计。

### 7.3 执行与投影边界

- 生产真实执行路径与 legacy facade 共用同一 Safe Client。
- `tools/call` 继续先验证有效 Key、`execute` capability 和 Tool tenant ownership。
- 执行前始终重新验证数据库中的 URL，防止 legacy 数据或绕过 API 的记录。
- 合法成功响应保持当前 JSON/text 投影；失败只返回稳定安全错误。

## 8. 发布计划基线：计划验证

```bash
# frontend required gates（预计无前端业务改动，仍执行）
cd frontend
bun install --frozen-lockfile
bun run type-check
bun run build

# backend formatting / compile / lint / test
cd ../backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p service-tool
cargo test -p api --test http_tool_egress_security_tests
cargo test -p api --test mcp_tool_execution_tests
cargo test -p api --test api_key_authorization_security_tests
cargo test --workspace

# governance / diff
python3 - <<'PY'
from pathlib import Path
import re
missing=[]
for path in list(Path('docs').rglob('*.md'))+[Path('README.md'),Path('AGENTS.md'),Path('EVOLUTION.md')]:
    if not path.exists():
        continue
    text=path.read_text(encoding='utf-8')
    for m in re.finditer(r'\[[^\]]+\]\(([^)]+\.md)(?:#[^)]+)?\)', text):
        target=m.group(1)
        if '://' in target:
            continue
        if not (path.parent/target).resolve().exists():
            missing.append((str(path),target))
if missing:
    raise SystemExit('\n'.join(f'{src} -> {target}' for src,target in missing))
print('all markdown links exist')
PY

git diff --check
```

Frontend lint 与 PostgreSQL 扩展测试继续保持 tag/manual-only，除非实际实现证明本 Story 必须改变该契约；不得顺手扩大 CI。

## 9. 发布计划基线：风险与回滚

| 风险 | 处理 / 回滚 |
|------|-------------|
| reqwest 自动解析或 Redirect 绕过已校验地址 | 自动 Redirect 必须关闭；显式逐跳；连接使用批准地址；无法证明时不合并 |
| DNS 多记录或 rebinding 产生 TOCTOU | 校验全部记录，任一禁止即拒绝；校验结果与连接绑定；测试地址集变化 |
| 安全策略误伤合法公网服务 | 明确 reason code、配置上限和受控公网测试；不提供任意私网 bypass；必要时回滚本 Story 提交并保持 SEC-02 blocked |
| 本地 mock 依赖 loopback 与新策略冲突 | 通过可注入 Resolver/Transport 或严格 test-only 受控目标测试，不在生产默认策略中开放 localhost |
| 两套 HTTP 实现只修一套 | 以 `main.rs` 真实注入链为验收 owner；legacy facade 必须复用同一边界或移除独立 client |
| 权限检查晚于 DTO 解析 | 采用 raw body + auth-first 模式，测试 invalid body 仍返回 403 且审计 |
| 错误脱敏破坏现有调用者诊断 | 保留稳定错误分类、status 和 request/tool correlation；敏感网络细节仅限安全内部日志且仍不记录凭证/完整 URL |
| 新配置默认不安全或环境漂移 | 安全默认 fail closed；配置验证范围和上限；同步 CONFIG / API Contract；生产缺失关键配置时不得自动放宽 |
| 审计写入失败 | 记录结构化错误并保持主安全动作拒绝；不得因审计失败转为允许 |
| 并发限制造成拒绝风暴 | 有界 Semaphore、明确 busy error 和测试；不引入分布式限流或跨 Story 平台工程 |

回滚策略：本 Story 在合并前通过单一分支/Draft PR 迭代；若安全边界或兼容性无法证明，回滚运行时代码提交，EVO-118-C 保持 `In Progress/Review`，SEC-02 保持开放，HTTP Tool 继续仅限受控开发环境。

## 10. 预计修改文件

### 计划新增

- `backend/crates/service-tool/src/egress.rs`
- `backend/crates/api/tests/http_tool_egress_security_tests.rs`
- `docs/iterations/ITERATION-052.md`

### 计划修改

- `backend/crates/service-tool/src/lib.rs`
- `backend/crates/service-tool/src/http_proxy_provider.rs`
- `backend/crates/service-tool/src/executor.rs`
- `backend/crates/service-tool/Cargo.toml`
- `backend/Cargo.toml` / `backend/Cargo.lock`（仅在安全实现确需新增 URL/IP 辅助依赖时）
- `backend/crates/common/src/execution.rs`（仅补直接服务 HTTP Egress 的约束/context；不重构其他 provider）
- `backend/crates/infra/src/config.rs`
- `backend/crates/api/src/dto/tool_dto.rs`
- `backend/crates/api/src/handlers/tool_handlers.rs`
- `backend/crates/api/src/handlers/mcp_handlers.rs`
- `backend/src/main.rs`
- `backend/crates/api/tests/mcp_tool_execution_tests.rs`
- `backend/crates/api/tests/api_key_authorization_security_tests.rs`（复用角色、审计和 hit-count 基础设施时）
- `docs/reference/CONFIG.md`
- `docs/reference/API-CONTRACT.md`
- `docs/reference/PRODUCTION-READINESS-BASELINE.md`（仅在 SEC-02 实际关闭时）
- `docs/backlog/active/EVO-118-C-http-tool-egress-security.md`
- `docs/backlog/active/EVO-118-production-readiness-and-security-hardening.md`
- `docs/backlog/PRODUCT-BACKLOG.md`
- `docs/BOARD.md`
- `docs/README.md`
- `docs/iterations/README.md`

数据库 schema、migration、部署文件和 CI workflow 预计不修改；若实现证明必须改变，先按 Change Control 记录，不静默扩展。

## 11. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | EVO-118-C 形成可验证的统一 HTTP Egress 安全边界，关闭 SEC-02，合法公网 HTTP Tool 保持可用 |
| 产物 | Egress Policy / Resolver / Safe Client、真实执行链接线、管理权限与审计、SSRF 负向测试、治理和 Reference 同步 |
| 状态同步归口 | EVO-118-C Story、EVO-118 Epic、Product Backlog、Board、docs 入口、Iteration 索引；完成后同步 Baseline |
| Story/BDD 归口 | 本文件 BDD 场景 A~H 与 Story 四个验收场景；行为类 Story，BDD 适用 |
| 验证证据 | service-tool 单元/集成、API/MCP E2E、网络零 hit、全 workspace gates、frontend type-check/build、CI required gates、Navigator 安全结论 |
| 残余工作归口 | Webhook 复用归 EVO-107；Git 耐久性归 EVO-118-D；部署归 EVO-118-E；Repo 生命周期归 EVO-118-F；CI/readiness 归 EVO-118-G |

## 12. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-07-31 | planning | 核对 PR #4 merged、最新 `main`=`bc4a4a18153b575c47ef76a7a008ae20cf3836c3`；无开放 PR、无 052 或 EVO-118-C 同名分支；完成治理/Security SOP 与真实 HTTP Tool 调用链只读审查。 |
| 2026-07-31 | planning | 创建分支 `agent/evo-118-c-http-tool-egress-security`；先发布本 Planned 基线。运行时代码尚未修改。 |
| 2026-07-31 | planning | 激活门禁：必须以单一治理变更同步 Story/Epic/Backlog/Board/docs/Iteration index 后，才把状态改为 Active/In Progress 并开始运行时代码。 |

## 13. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-07-31 | clarification | 接受 | 用户明确本轮只做 EVO-118-C，不混入 D/E、Repo UI、Agent 写入、Durable Outbox 或架构改造 | 无运行时代码半成品 |

## 14. Review

- 完成：待实施。
- 未完成：全部运行时代码、安全测试、实际验证、Navigator 审查和 SEC-02 状态同步。
- 验证结果：尚未运行；计划基线不以历史 CI 代替本轮证据。
- 闭环状态：`Partial`
- 残余归口：按闭环台账和本 Iteration 后续执行记录。

## 15. Retrospective

- 做得好的：待迭代结束填写。
- 需要调整的：待迭代结束填写。
- 写入 EVOLUTION：如实现发现新的稳定陷阱，再按 SOP 判断写回。