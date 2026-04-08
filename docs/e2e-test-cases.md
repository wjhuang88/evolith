# Evolith 平台 E2E 测试用例规范

本文档详细说明了 Evolith 平台的端到端（E2E）测试用例，涵盖 API 接口、业务逻辑、安全边界及前端 UI 交互。

## 1. 系统健康检查 (Health)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-HLT-001 | 基础健康检查 | 无 | 发送 `GET /health` 请求 | 返回 200 OK，内容为 `{"status": "ok", "version": "..."}` | P0 |
| TC-HLT-002 | 存活探针检查 | 无 | 发送 `GET /health/live` 请求 | 返回 200 OK，确认服务进程运行正常 | P0 |
| TC-HLT-003 | 就绪探针检查 | 数据库已连接 | 发送 `GET /health/ready` 请求 | 返回 200 OK，确认数据库及关键依赖已就绪 | P0 |
| TC-HLT-004 | 数据库断开响应 | 模拟数据库连接失败 | 发送 `GET /health/ready` 请求 | 返回 503 Service Unavailable 或包含错误信息的 500 | P1 |

## 2. 认证与用户管理 (Auth)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-ATH-001 | 用户注册成功 | 邮箱未被注册 | `POST /api/v1/auth/register`，包含有效邮箱、用户名、8位以上强密码及租户信息 | 201 Created，设置 `evolith_token` 和 `csrf_token` Cookie，创建租户并设为 Owner | P0 |
| TC-ATH-002 | 注册校验失败 | 无 | 提交密码过短（<8位）或邮箱格式错误的注册请求 | 400 Bad Request，返回 `VALIDATION_ERROR` | P1 |
| TC-ATH-003 | 注册重复冲突 | 邮箱已存在 | 使用已注册邮箱调用注册接口 | 409 Conflict，返回 `EMAIL_EXISTS` | P1 |
| TC-ATH-004 | 登录成功流程 | 用户已注册 | `POST /api/v1/auth/login`，提供正确凭据 | 200 OK，返回用户信息和租户信息，更新 Auth Cookies | P0 |
| TC-ATH-005 | 登录凭据错误 | 用户已注册 | 使用错误密码调用登录接口 | 401 Unauthorized，返回 `INVALID_CREDENTIALS` | P0 |
| TC-ATH-006 | 获取当前信息 | 已登录 | 发送 `GET /api/v1/auth/me` | 200 OK，返回当前用户的详细配置与权限 | P0 |
| TC-ATH-007 | 修改个人资料 | 已登录 | `PATCH /api/v1/auth/profile`，修改用户名 | 200 OK，数据库中用户信息同步更新 | P1 |
| TC-ATH-008 | 修改密码流程 | 已登录 | `POST /api/v1/auth/change-password`，提供旧密码与符合强度的新密码 | 200 OK，旧密码失效，下次登录需用新密码 | P1 |
| TC-ATH-009 | 退出登录 | 已登录 | 发送 `POST /api/v1/auth/logout` | 200 OK，客户端 Auth Cookies 被清除 | P0 |
| TC-ATH-010 | 令牌刷新 | 持有有效 Refresh Token | 发送 `POST /api/v1/auth/refresh` | 200 OK，设置新的 JWT Cookie | P1 |
| TC-ATH-011 | 未实现功能拦截 | 已登录 | 调用 `POST /api/v1/auth/send-verify` 等未开发接口 | 501 Not Implemented | P2 |

## 3. 工具管理 (Tools CRUD)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-TOL-001 | 创建工具成功 | 已登录 | `POST /api/v1/tools`，提供名称、JSON Schema 格式的 input_schema 和 handler_url | 201 Created，返回工具 ID | P0 |
| TC-TOL-002 | 列表分页查询 | 存在多个工具 | `GET /api/v1/tools?page=1&per_page=10` | 200 OK，返回分页后的工具列表及元数据 | P1 |
| TC-TOL-003 | 工具详情获取 | 工具 ID 存在 | `GET /api/v1/tools/{id}` | 200 OK，返回完整工具配置信息 | P0 |
| TC-TOL-004 | 工具更新操作 | 工具 ID 存在且为所有者 | `PUT /api/v1/tools/{id}`，修改描述或 Schema | 200 OK，修改内容生效 | P1 |
| TC-TOL-005 | 工具删除操作 | 工具 ID 存在且为所有者 | `DELETE /api/v1/tools/{id}` | 200 OK 或 204 No Content，后续查询该 ID 返回 404 | P1 |
| TC-TOL-006 | 越权访问拦截 | 登录用户 A 访问用户 B 的工具 | 使用用户 A 的 Token 调用 `GET /api/v1/tools/{id_B}` | 403 Forbidden 或 404 Not Found (租户隔离) | P0 |

## 4. 技能管理与沙箱执行 (Skills)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-SKL-001 | 创建技能 | 已登录 | `POST /api/v1/skills`，包含 SKILL.md 内容和指定 runtime (python311) | 201 Created | P0 |
| TC-SKL-002 | 加载技能配置 | 技能 ID 存在 | `GET /api/v1/skills/{id}/load` | 200 OK，返回用于执行的完整 Markdown 配置 | P1 |
| TC-SKL-003 | 沙箱成功执行 | 技能已创建，沙箱可用 | `POST /api/v1/skills/{id}/execute`，提供 JSON 参数 | 200 OK，返回 `success` 状态、标准输出及执行耗时 | P0 |
| TC-SKL-004 | 执行超时拦截 | 技能代码包含长时间 sleep | 调用执行接口 | 返回结果中 `timed_out` 为 true，状态为 `timeout` | P1 |
| TC-SKL-005 | 运行时校验 | 无 | 创建技能时指定不存在的运行时 | 400 Bad Request，返回 `INVALID_RUNTIME` | P2 |
| TC-SKL-006 | 删除技能 | 技能所有者 | `DELETE /api/v1/skills/{id}` | 200 OK，技能及其版本记录被清除 | P1 |

## 5. 代码片段管理 (Snippets)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-SNP-001 | 创建代码片段 | 已登录 | `POST /api/v1/snippets`，提供代码块、语言和框架标签 | 201 Created | P0 |
| TC-SNP-002 | 片段全文检索 | 存在相关片段 | `GET /api/v1/snippets/search?q=keyword` | 200 OK，返回匹配搜索条件的片段列表 | P1 |
| TC-SNP-003 | LLM 引用格式 | 片段 ID 存在 | `GET /api/v1/snippets/{id}/reference` | 200 OK，返回经过 Prompt 优化的 Markdown 文本 | P1 |
| TC-SNP-004 | 标签过滤查询 | 存在多语言片段 | `GET /api/v1/snippets?language=rust` | 200 OK，仅返回 Rust 语言的片段 | P1 |

## 6. 团队成员管理 (Members)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-MBR-001 | 邀请新成员 | 管理员权限 | `POST /api/v1/tenant/{tid}/members/invite`，指定邮箱和 role=member | 201 Created，创建邀请记录 | P0 |
| TC-MBR-002 | 查看邀请列表 | 管理员权限 | `GET /api/v1/tenant/{tid}/members/invitations` | 200 OK，显示所有待处理邀请 | P1 |
| TC-MBR-003 | 移除团队成员 | 管理员权限，成员非本人 | `DELETE /api/v1/tenant/{tid}/members/{mid}` | 200 OK，该成员失去租户访问权限 | P1 |
| TC-MBR-004 | 普通成员邀请拦截 | 成员权限 | 调用邀请接口 | 403 Forbidden | P0 |
| TC-MBR-005 | 重复邀请校验 | 已存在待处理邀请 | 再次对同一邮箱发起邀请 | 400 Bad Request，返回 `ALREADY_INVITED` | P1 |

## 7. API 密钥管理 (API Keys)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-AKY-001 | 创建 API 密钥 | 管理员权限 | `POST /api/v1/tenant/{tid}/api-keys`，设置过期时间和权限 | 201 Created，返回明文 Key（仅限一次） | P0 |
| TC-AKY-002 | 使用密钥访问 MCP | 持有有效 Key | 调用 `POST /mcp`，在 Header 中携带密钥 | 200 OK，JSON-RPC 调用成功 | P0 |
| TC-AKY-003 | 撤销密钥 | 管理员权限 | `DELETE /api/v1/tenant/{tid}/api-keys/{kid}` | 200 OK，该密钥立即失效 | P1 |
| TC-AKY-004 | 过期密钥访问 | 密钥已过有效期 | 使用该密钥调用 API | 401 Unauthorized | P1 |

## 8. 计费与订阅 (Billing)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-BIL-001 | 查看可用方案 | 已登录 | `GET /api/v1/tenant/{tid}/billing/plans` | 200 OK，返回 Free/Pro/Enterprise 方案列表 | P1 |
| TC-BIL-002 | 获取当前订阅 | 已登录 | `GET /api/v1/tenant/{tid}/billing/subscription` | 200 OK，返回当前租户的订阅状态和周期 | P1 |
| TC-BIL-003 | 创建支付会话 | 已登录 | `POST /api/v1/tenant/{tid}/billing/subscription/checkout` | 200 OK，返回 Stripe Checkout URL | P2 |
| TC-BIL-004 | 查看使用额度 | 已登录 | `GET /api/v1/tenant/{tid}/billing/usage` | 200 OK，返回工具调用次数等指标 | P1 |

## 9. 审计日志 (Audit Logs)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-AUD-001 | 管理员查询日志 | 管理员权限 | `GET /api/v1/tenant/{tid}/audit-logs` | 200 OK，返回带分页的操作记录列表 | P1 |
| TC-AUD-002 | 普通成员查询拦截 | 成员权限 | 发送查询日志请求 | 403 Forbidden | P1 |
| TC-AUD-003 | 审计日志详情拦截 | 已登录 | 调用 `GET /api/v1/tenant/{tid}/audit-logs/{id}` | 501 Not Implemented | P2 |

## 10. MCP 端点交互

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-MCP-001 | JSON-RPC 初始化 | 无 | 调用 `POST /mcp`，method: "initialize" | 返回 JSON-RPC 响应，包含服务器 capabilities | P0 |
| TC-MCP-002 | 列出 MCP 工具 | 已鉴权 | 调用 `POST /mcp`，method: "tools/list" 或 `GET /mcp/tools` | 返回租户内可用的所有工具定义 | P0 |
| TC-MCP-003 | 调用 MCP 工具 | 工具存在且参数正确 | 调用 `POST /mcp`，method: "tools/call"，提供工具名和 arguments | 返回工具执行结果 content | P0 |
| TC-MCP-004 | 无效方法调用 | 无 | 调用不存在的 JSON-RPC 方法 | 返回标准的 JSON-RPC Error 对象 | P1 |

## 11. 安全与中间件 (Security)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-SEC-001 | CSRF 保护有效性 | 已登录 | 发送 `POST /api/v1/tools` 但不携带 `X-CSRF-Token` Header | 403 Forbidden，返回 `CSRF_ERROR` | P0 |
| TC-SEC-002 | 速率限制触发 | 无 | 快速连续发送超过 60 次请求 | 返回 429 Too Many Requests | P1 |
| TC-SEC-003 | 安全响应头校验 | 无 | 查看任一 API 响应 Header | 包含 X-Content-Type-Options: nosniff, X-Frame-Options: DENY 等 | P1 |
| TC-SEC-004 | 请求 ID 传递 | 无 | 发送请求并检查响应 Header | 包含 `X-Request-ID`，且与后续日志关联 | P2 |
| TC-SEC-005 | JWT 过期失效 | 持有过期 Token | 调用受保护接口 | 401 Unauthorized | P0 |

## 12. 前端 UI 交互 (Frontend UI)

| ID | 测试名称 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-UI-001 | 首页展示与 i18n | 访问 `/`，切换中英文语言 | 页面内容正确翻译，核心功能点可见 | P1 |
| TC-UI-002 | 登录表单校验 | 在 `/login` 输入错误格式邮箱或空密码 | 实时显示前端校验错误提示 | P1 |
| TC-UI-003 | 路由守卫拦截 | 未登录状态直接访问 `/dashboard` | 自动重定向至 `/login?redirect=/dashboard` | P0 |
| TC-UI-004 | 入职向导流程 | 首次登录进入 `/onboarding`，完成三步操作 | 成功创建首个工具并跳转至 Dashboard | P1 |
| TC-UI-005 | 工具创建器交互 | 在 `/tools/new` 使用 JSON 编辑器输入非法 JSON | 编辑器标记语法错误，提交按钮禁用 | P1 |
| TC-UI-006 | 响应式布局测试 | 切换至手机屏幕宽度访问页面 | 侧边栏折叠为汉堡菜单，列表自动适配宽度 | P2 |
| TC-UI-007 | 技能执行模态框 | 在 `/skills/[id]` 点击执行，输入参数并提交 | 弹出 Loading 状态，执行完成后在当前页显示结果 | P1 |
| TC-UI-008 | 主题切换 | 点击主题切换按钮 | 页面在深色/浅色模式间无缝切换，持久化至 LocalStorage | P2 |

## 13. 错误处理与容错 (Error Handling)

| ID | 测试名称 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|:---|:---|:---|:---|:---|:---|
| TC-ERR-001 | 404 页面展示 | 访问不存在的 UI 路由 | 展示自定义的 `not-found.tsx` 友好页面 | P1 |
| TC-ERR-002 | 前端崩溃捕获 | 触发一个组件渲染错误 | 展示 `error.tsx` 错误边界页面，提供重试按钮 | P2 |
| TC-ERR-003 | 标准 API 错误格式 | 构造任一失败请求 | 返回格式固定为 `{"success": false, "error": {"code": "...", "message": "..."}}` | P0 |
| TC-ERR-004 | 数据库异常处理 | 构造导致数据库操作超时的请求 | 返回 500 `DATABASE_ERROR`，不泄露内部 SQL 信息 | P1 |

## 14. RBAC 权限矩阵 (Role Based Access Control)

| 操作项目 | Owner (所有者) | Admin (管理员) | Member (成员) | Viewer (观察者) |
|:---|:---:|:---:|:---:|:---:|
| 删除租户/更改账单 | ✅ | ❌ | ❌ | ❌ |
| 邀请/移除管理员 | ✅ | ❌ | ❌ | ❌ |
| 邀请/移除普通成员 | ✅ | ✅ | ❌ | ❌ |
| 管理 API 密钥 | ✅ | ✅ | ❌ | ❌ |
| 创建/修改工具与技能 | ✅ | ✅ | ✅ | ❌ |
| 查看资源列表 | ✅ | ✅ | ✅ | ✅ |
| 执行沙箱代码 | ✅ | ✅ | ✅ | ❌ |
| 查看审计日志 | ✅ | ✅ | ❌ | ❌ |

---

> **测试执行报告**: 参见 [e2e-test-report.md](./e2e-test-report.md)

---
**文档版本**: 1.0.0
**最后更新**: 2026-04-08
**维护人**: Sisyphus
