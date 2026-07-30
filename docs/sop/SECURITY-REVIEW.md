# SOP: 安全敏感变更审查

> 本 SOP 适用于认证、授权、API Key、Agent Token、公开入口、Git 写入、出站网络、Webhook、Sandbox、数据耐久性和生产部署变更。

## 触发条件

出现以下任一情况时，实施前必须读取本文：

- 修改登录、JWT、Cookie、Session、RBAC、租户隔离或 API Key。
- 新增或修改 MCP `tools/call`、HTTP Tool、Webhook 或任何出站请求。
- 新增 Git Push、Commit、Promote、Force Push、Ref/Branch/Path scope。
- 修改 Repo 创建/删除、Git 存储路径、备份、恢复或多实例部署。
- 修改 Dockerfile、Compose、K8s、Nginx、生产配置或关键依赖降级策略。
- 新增 public route、CSRF exemption、匿名发现接口或跨租户查询。
- Navigator、代码审查或事故复盘发现权限提升、SSRF、数据损坏或失效门禁。

## 前置检查

- [ ] 已读取 [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)。
- [ ] 变更已进入 Backlog，且安全/数据损坏问题标为 P0 或说明为何不是 P0。
- [ ] 已列出调用者：匿名用户、普通成员、管理员、Owner、API Key、Agent Token、内部服务。
- [ ] 已列出资源：Tenant、Repo、Branch/Ref、Path、Tool、Credential、Session、Webhook、Git Storage。
- [ ] 已区分认证（是谁）与授权（能对哪个资源做什么）。
- [ ] 已明确默认行为是 fail closed 还是 fail open；安全边界默认必须 fail closed。
- [ ] 已为高风险跨层改动安排 Driver 实现与 Navigator 审查。

## 1. 威胁模型最小模板

在 Story 或 Iteration 中填写：

```markdown
| 项目 | 内容 |
|------|------|
| 受保护资产 | 凭证、Git 对象、租户数据、内网服务、审计记录等 |
| 攻击者/调用者 | 匿名、Member、Admin、API Key、Agent、被攻陷 Tool |
| 入口 | HTTP route、Git Smart HTTP、MCP、Webhook、配置、文件系统 |
| 信任边界 | Browser→API、Tenant→Tool URL、API→Git、DB→Worker 等 |
| 失败模式 | 越权、SSRF、Ref 越界、数据分裂、事件丢失、静默降级 |
| 安全默认 | 拒绝、回滚、标记 ERROR、进入重试队列 |
| 验证证据 | 负向测试、资源边界测试、恢复演练、clean build |
```

未完成该表的 P0/P1 安全变更不得进入 `In Progress`。

## 2. 认证与授权门禁

### 2.1 API Key / Token

必须满足：

1. 权限使用枚举或 Typed Capability，不接受未校验的任意字符串。
2. 创建者不能授予高于自身的权限。
3. API Key 管理必须有明确角色门禁；不能只检查 `tenant_id`。
4. 每个请求同时检查：调用者、Action、Resource、Tenant、Scope 和有效期。
5. `read`、`write`、`execute`、`promote`、`manage_keys` 不得互相隐式包含，除非权限矩阵明确规定。
6. API Key/Agent Token 不得管理其他凭证，除非存在独立且经过审查的 capability。
7. 撤销、过期、用户停用和角色变化必须有可验证生效路径。

最小负向测试：

- Member 创建 `admin` Key 被拒绝。
- Read-only Key 写 Repo 被拒绝。
- 无 `execute` 的 Key 调用 MCP Tool 被拒绝。
- A Tenant 的 Key 访问 B Tenant 资源被拒绝且不泄露资源存在性。
- 已撤销/过期 Key 在所有入口失效。

### 2.2 Git 写入

必须区分：

- 人类凭证与 Agent Scoped Token。
- Smart HTTP Push 与 Commit/Promote API。
- Repository、Ref/Branch、Path 和操作类型。

要求：

1. `commit:<scope>` 必须解析为真实 branch/path scope，不得以字符串前缀授予通用写权限。
2. Agent 默认不得直接使用不受 PolicyEvaluator 约束的 `git-receive-pack`。
3. Force Push、删除 Ref、写默认分支和写 protected path 必须显式授权。
4. Policy 的 `auto_merge / require_review / block` 必须由行为测试证明生效，不能只验证 YAML Parser。
5. 失败路径必须证明目标 Ref 未移动。

## 3. 出站 HTTP / SSRF 门禁

租户可配置 URL 或请求参数可影响目标地址时，必须经过统一 Egress Policy：

1. 仅允许明确支持的 Scheme，默认 `https`；如允许 `http`，必须记录环境和风险。
2. 解析 DNS 后拒绝 loopback、private、link-local、multicast、reserved 和云 Metadata 地址。
3. 禁止凭域名字符串判断安全；必须校验最终连接 IP。
4. Redirect 默认关闭；如允许，每一跳重新执行 Scheme/DNS/IP 校验。
5. 防止 DNS Rebinding：解析与连接策略必须可证明目标未切换到私网地址。
6. 配置连接、总请求、响应体、Header 和并发上限。
7. 不使用宿主机隐式代理绕过策略；需要代理时使用受控 Egress Proxy。
8. Tool 创建和更新需要角色/审核门禁，不能让普通成员任意创建内网探针。
9. 错误响应不得回显内部 URL、凭证、完整响应体或网络拓扑。

最小测试矩阵：

| 场景 | 预期 |
|------|------|
| `127.0.0.1` / `localhost` | 拒绝 |
| RFC1918 私网地址 | 拒绝 |
| `169.254.169.254` 或等价 Metadata | 拒绝 |
| 公网域名解析到私网 | 拒绝 |
| 公网 URL Redirect 到私网 | 拒绝 |
| 超时、超大响应、过多 Redirect | 有界失败 |
| 明确 allowlist 公网 Mock | 成功并记录审计 |

## 4. 数据耐久性与一致性门禁

涉及 PostgreSQL + Git 文件系统时：

1. 不把 best-effort warning 当作成功。
2. Repo 创建使用 `CREATING → ACTIVE / ERROR` 状态或等价补偿机制。
3. Repo 删除使用 `DELETING / trash / reconcile`，避免 DB 删除后留下不可追踪孤儿。
4. Production Git 路径必须挂载持久存储；容器层文件系统不得作为事实源。
5. 备份必须同时覆盖 PostgreSQL 和 Git 仓库，并定义一致性点。
6. 发布前至少完成一次从空环境恢复的演练。
7. Readiness 检查 Git 存储可读写和数据库可用；失败返回非 2xx。
8. 多实例前必须解决共享存储、一致路由或明确的 Repo affinity。

最小恢复证据：

```text
创建 repo → push commit/tag → 备份 → 清空环境 → 恢复 → clone → 校验 commit/tag/权限元数据
```

## 5. 生产配置与降级门禁

- 生产构建必须从干净 checkout 执行。
- Embedded Frontend 只保留一种事实交付路径；不得同时维护互相冲突的静态前端和嵌入式后端口径。
- JWT Secret、Git Storage、数据库和安全策略缺失时启动失败。
- SMTP 配置声明启用但初始化失败时不得静默切换 ConsoleMailer。
- Redis 如果承担安全计数、Session 或分布式锁，连接失败不得退化为进程内状态；普通缓存降级需明确说明。
- `/health/live` 只表示进程存活；`/health/ready` 检查关键依赖并在失败时返回 503。
- API Key 自身的 rate limit 和调用类型限流必须真实接入请求链，不能只存在配置字段。

## 6. 实施步骤

1. 在 Backlog/Iteration 写威胁模型、失败模式、验收和负向测试。
2. Driver 先实现最小安全边界，不顺手扩大产品功能。
3. 运行局部单元/集成测试，确认拒绝路径先于成功路径。
4. Navigator 独立检查授权矩阵、租户边界、网络边界、数据损坏和降级行为。
5. 修复 Navigator 发现的问题；未解决项进入明确 P0/P1 Story。
6. 执行风险匹配的全量门禁。
7. 更新 Reference、API Contract、Release SOP、Backlog 和 Iteration。
8. 按 `TASK-CLOSURE.md` 使用 `Complete / Partial / Blocked` 收口。

## 7. 验证要求

| 变更类型 | 最低验证 |
|----------|----------|
| API Key/RBAC | 角色矩阵 + 跨租户 + 无权限/过期/撤销负向集成测试 |
| MCP Tool | `execute` scope + Tool tenant ownership + SSRF 测试 |
| Git Write | Ref/Branch/Path scope + force push/protected path + Ref 不移动测试 |
| Repo lifecycle | DB/FS 失败注入 + Reconciler/补偿测试 |
| Git durability | 持久卷重建 + DB/Git 备份恢复演练 |
| Production build | `docker compose ... build --no-cache` + 启动 + Smoke Test |
| Readiness | 依赖故障时返回 503，恢复后返回 200 |
| Durable Event | 进程崩溃/重启后事件不丢、重试有界、幂等验证 |

治理或文档变更至少执行 DOC-CHECK；安全实现不得只运行单元测试后声明完成。

## 8. 发布阻断判定

出现以下任一情况，结论必须为 `Blocked` 或 `Partial`，不得发布：

- 可以由低权限调用者获得高权限凭证。
- 可通过租户可控 URL 访问私网、localhost 或 Metadata。
- Git 数据只保存在容器临时层。
- 备份无法恢复 Git Commit/Ref。
- clean production build 未通过。
- 安全配置失败后静默降级。
- 必需负向测试未运行或失败。
- 已知数据分裂路径没有补偿、状态或 Reconciler 归口。

## 9. 失败恢复

| 发现 | 处理 |
|------|------|
| 已合并但发现越权/SSRF | 立即登记 P0，禁用入口或回滚，轮换受影响凭证，补审计 |
| Repo DB/磁盘分裂 | 停止新建/删除，运行盘点，标记 ERROR，执行补偿或恢复 |
| 生产 Git 卷未持久化 | 停止重建容器，先导出 Git 目录，再修复挂载和备份 |
| 事件丢失 | 停止依赖内存事件推进后续功能，补 Outbox/Reconcile |
| 安全测试无法构造 | Story 保持 Proposed/Blocked，先做 Spike，不以人工判断替代 |

## 相关文档

- [生产就绪与项目完成度基线](../reference/PRODUCTION-READINESS-BASELINE.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- [分阶段结对开发](PAIRING-WORKFLOW.md)
- [发布与部署](RELEASE.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)
