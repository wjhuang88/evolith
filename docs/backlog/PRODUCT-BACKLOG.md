# Product Backlog

> 状态维护和 DoR 规则见 [需求进入与 Backlog 整理](../sop/REQUIREMENT-INTAKE.md)；完成检查见 [特性迭代工作流](../sop/ITERATION-WORKFLOW.md)。

## 优先级说明

| 优先级 | 含义 |
|--------|------|
| P0 | 阻塞核心体验或主线价值，下一批优先处理 |
| P1 | 重要但不阻塞当前主线 |
| P2 | 增强体验、补齐管理能力 |
| P3 | 远期探索或可选优化 |

## 当前需求池

| ID | 标题 | 类型 | 优先级 | 状态 | 来源 | 备注 |
|----|------|------|--------|------|------|------|
| EVO-001 | 前后端 API 对齐 | bug | P0 | Done | [实施路线图 Phase A](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-a--现状校准与-api-对齐done) | 修正 method、字段和不存在的前端 API 调用；snippet 范围已转入 EVO-017 |
| EVO-002 | 前端迁移到 React + Vite + Bun | tech-debt | P0 | Done | [实施路线图 Phase B](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0--高优先级) | EVO-021 至 EVO-025 全部完成 |
| EVO-003 | 忘记密码与重置密码闭环 | feature | P0 | Done | Iteration 005 | handler 实现 + 前端 API 接入，Playwright 验证通过 |
| EVO-004 | 邀请接受 / Join 流程 | feature | P0 | Done | Iteration 005 | handler 实现，cargo test 通过 |
| EVO-005 | MCP 工具真实执行 | feature | P0 | Done | 需求 F1.1.3 / Iteration 006 | HTTP handler type 真实执行，Function 类型暂不支持 |
| EVO-006 | Skill 更新接口 | feature | P1 | Proposed | API 501 | `PUT /skills/{id}` |
| EVO-007 | Snippet 更新接口 | feature | P1 | Deferred | API 501 | 被 EVO-017 替代方向覆盖，暂停继续投入 |
| EVO-008 | Snippet reference 格式增强 | feature | P1 | Deferred | 需求 F1.3.4 | 被 EVO-017 替代方向覆盖 |
| EVO-009 | SKILL.md 与 CLI interface frontmatter parser | feature | P1 | Done | 格式规范 / EVO-017 / Iteration 010 | CLI interface parser 已有基线；SKILL.md parser 已实现 |
| EVO-010 | 租户 Members 页面接真实 API | feature | P1 | Proposed | 前端 TODO | 列表、邀请、移除 |
| EVO-011 | API Key 页面接真实 API | feature | P1 | Proposed | 前端 TODO | 创建、列表、revoke |
| EVO-012 | 租户设置保存 | feature | P2 | Proposed | 前端 TODO | tenant settings |
| EVO-013 | Audit log detail 接口 | feature | P2 | Proposed | API 501 | `GET /audit-logs/{log_id}` |
| EVO-014 | Stripe webhook 恢复 | feature | P2 | Proposed | routes TODO | 计费闭环 |
| EVO-015 | Rust CLI 子项目 | feature | P3 | Deferred | [提案](../proposals/RUST-CLI.md) | API 稳定后启动 |
| EVO-016 | 前端嵌入后端发布物 | tech-debt | P3 | Deferred | [提案](../proposals/EMBEDDED-FRONTEND.md) | Vite SPA 完成后启动 |
| EVO-017 | Snippet 迁移为 CLI 友好接口 | product-change | P0 | Done | [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) | Iteration 002；replaces EVO-007/EVO-008；已建立 CLI interface 格式、API 兼容契约、parser 基线和迁移盘点 |
| EVO-018 | 邮箱验证发送与确认闭环 | feature | P1 | Proposed | API 501 / Phase C | `send-verify`、`verify-email`，依赖 mailer |
| EVO-019 | Skill registry 服务化 | tech-debt | P2 | Proposed | Phase E placeholder | 将 `service-skill/src/registry.rs` 从 placeholder 补成可复用注册能力 |
| EVO-020 | Storage 能力落地 | feature | P2 | Proposed | Phase E placeholder | 实现对象存储基础能力，支撑技能包和附件 |
| EVO-021 | 前端路由适配层 | tech-debt | P0 | Done | EVO-002 split | Iteration 003；已新增 `frontend/src/lib/router.tsx`，页面和共享组件不再直接导入 Next 路由模块 |
| EVO-022 | Vite + Bun 构建骨架 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；新增 Vite 入口、React Router 根路由和并行构建脚本 |
| EVO-023 | 前端运行时配置迁移 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；`src/lib/config.ts` 统一运行时环境变量，替换所有 `process.env` 引用 |
| EVO-024 | Docker / Nginx 切换到静态 SPA | tech-debt | P0 | Done | EVO-002 split | Iteration 004；过渡部署形态，Dockerfile 改为 Bun + Vite build + Nginx 静态服务，SPA fallback |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；删除 next 依赖、App Router、middleware、config；router.tsx 改为 React Router |
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | P1 | Proposed | EVO-017 / 页面残留 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| EVO-027 | Skill 多来源创建 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 |
| EVO-028 | Skill 版本管理与正确性验证 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 |
| EVO-029 | Skill 专业描述与发现质量提升 | feature | P2 | Proposed | Agent Skills spec | 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 |
| EVO-030 | GitHub CI/CD 重建 | tech-debt | P2 | Proposed | EVO-002 split / 工程收尾 | 放到项目后段统一做；基于最终构建、测试、部署命令重建 workflow |
| EVO-031 | Iteration 004/005 质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-17 | 修复静态资源反代、邀请接受闭环、公开接口放行、邮件公开 URL、sourcemap 默认关闭和流程规约 |
| EVO-032 | Iteration 006 MCP 执行质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-25 | Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真 |
| EVO-033 | Rustfmt 全量格式基线与 stable 配置清理 | tech-debt | P2 | Proposed | Iteration 007 验证残余 | 独立处理历史格式差异和 nightly-only 配置告警，避免混入功能修复 |
| EVO-034 | Epic 与子需求拆分治理规则 | tech-debt | P1 | Done | 流程缺口 2026-05-26 | Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束 |
| EVO-035 | 治理 skill manifest 接入与一致性审计 | tech-debt | P2 | Proposed | Iteration 008 验证残余 | 建立 `.agent-governance/manifest.yaml` 后运行 bundled validator |

## 故事模板

```markdown
### EVO-XXX <标题>

- 类型：
- 优先级：
- 状态：
- 父 Epic：（非子 Story 填无）
- 用户价值：
- 范围：
- 不做：
- 验收标准：
  - [ ] ...
- 技术备注：
- 依赖或阻塞：
- 解锁内容：
```

### EVO-005 MCP 工具真实执行

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

### EVO-032 Iteration 006 MCP 执行质量修复与流程防呆

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

### EVO-034 Epic 与子需求拆分治理规则

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让维护者和 Agent 能把较大需求稳定地组织为 Epic 与可执行子 Story，避免把多阶段工作当成一个模糊任务，或在迭代中遗漏依赖和完成边界。
- 范围：
  - 明确 Epic 与普通 Story 的判定标准、拆分维度和子 Story 细度。
  - 为 Evolith 建立保留 `EVO-*` 前缀的父子编号、依赖记录、DoR 与迭代选取规则。
  - 在文档一致性检查与经验记录中加入相应防呆。
  - 将可迁移的方法论同步到 `agent-project-governance` skill，不覆盖该仓库已有外部改动。
- 不做：
  - 不批量重编号历史 `EVO-*` 事项；历史拆分关系继续保留。
  - 不改业务代码、构建流程或部署配置。
  - 不自动为现有 Proposed 事项建立新的 Epic 层级。
- 验收标准：
  - [x] `REQUIREMENT-INTAKE.md` 定义 Epic 判定、拆分维度/粒度、父子编号、依赖校验、Epic/子 Story DoR 和跨 Epic 迭代规则。
  - [x] `START-ITERATION.md`、`ITERATION-WORKFLOW.md` 与 `DOC-CHECK.md` 承接父子和依赖检查。
  - [x] `agent-project-governance` skill 提供可按项目编号前缀适配的 Epic/Story 方法论与评估场景。
  - [x] Markdown 链接校验和 `git diff --check` 通过，skill 结构校验通过。
- 依赖或阻塞：无；skill 目标目录存在用户未提交的 `README.md` 变更，实施时不得改写该文件。
- 影响范围：docs / external skill
- 最小验证方式：执行文档链接校验；`git diff --check`；运行 skill 的 `quick_validate.py`。

### EVO-035 治理 skill manifest 接入与一致性审计

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 用户价值或技术目标：让 Evolith 已有治理文档能被 `agent-project-governance` skill 明确识别为初始化/采用状态，并通过一致性审计发现后续漂移。
- 范围：
  - 按当前项目治理现状建立 `.agent-governance/manifest.yaml`。
  - 核对 capability 状态、标准入口与已有 SOP / reference / backlog / iteration 映射。
  - 运行 skill bundled validator，并将真实残余写回 backlog 或治理文档。
- 不做：
  - 不在 EVO-034 中补造 manifest 以掩盖附加审计失败。
  - 不顺带改业务逻辑或重写既有迭代历史。
- 验收标准：
  - [ ] manifest 能准确表达 Evolith 的治理 profile、入口和能力状态。
  - [ ] `validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith` 通过，或将剩余问题拆为明确事项。
- 依赖或阻塞：EVO-034 已完成；需要单独确认初始化/adoption 范围。
- 影响范围：docs
- 最小验证方式：运行 `agent-project-governance/scripts/validate_project_governance.py`。

## 下一批建议

优先选择：

1. `EVO-018` 邮箱验证发送与确认闭环。
2. `EVO-006` Skill 更新接口。
3. `EVO-027` Skill 多来源创建。

理由：EVO-005 MCP 工具真实执行已完成，EVO-009 parser 即将完成；下一步应补齐认证闭环（邮箱验证），再推进 Skill 生命周期。

## 已细化故事

### EVO-009 SKILL.md 与 CLI interface frontmatter parser

- 类型：feature
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让 SKILL.md parser 从 stub 升级为真正解析 YAML frontmatter + Markdown body 的生产级解析器，为 Skill 上传校验（EVO-027/028）和 CLI interface 创建打基础。CLI interface parser 已有基线（Iteration 002），本次补齐 SKILL.md 侧。
- 范围：
  - 重写 `service-skill/src/parser.rs`：解析 SKILL.md 的 YAML frontmatter，提取 `name`、`description`、`version`、`type`、`execution`、`runtime`、`entrypoint`、`timeout`、`memory`、`dependencies`、`tags`、`author` 等字段。
  - 扩展 `SkillMetadata` 结构体，覆盖 SKILL-FORMAT.md 中定义的字段。
  - 实现校验逻辑：`name` 必填且符合命名规则（小写字母/数字/连字符，不首尾连字符，无连续连字符）；`description` 必填且 1-1024 字符；`version` 如存在须为合法 semver。
  - Markdown body 提取为独立字段。
  - 单元测试覆盖：合法 SKILL.md、缺少 frontmatter、缺少必填字段、name 格式非法、description 过长。
  - CLI interface parser 如有遗漏字段也一并补齐（当前已较完整）。
- 不做：
  - 不实现 Skill 包目录扫描（只解析单个 SKILL.md 文本）。
  - 不实现 Skill 导入管线（EVO-027）。
  - 不实现版本管理和校验报告模型（EVO-028）。
  - 不实现描述质量评分（EVO-029）。
  - 不修改 Skill API handler 或数据库 schema。
- 验收标准：
  - [x] `SkillParser::parse()` 能解析包含完整 frontmatter 的 SKILL.md 文本，返回 `SkillMetadata` + body。
  - [x] `name` 校验：必填、1-64 字符、小写字母/数字/连字符、不首尾连字符、无连续连字符。
  - [x] `description` 校验：必填、1-1024 字符。
  - [x] 缺少 frontmatter 或缺少必填字段返回 `ValidationError`。
  - [x] 可选字段（version、author、tags、dependencies 等）缺失时不报错，使用合理默认值。
  - [x] `cargo test -p service-skill` 通过，parser 单元测试 >= 5 个。
  - [x] `cargo check --workspace` 无错误。
  - [x] `cargo clippy --workspace` 无错误。
- 技术备注：
  - CLI interface parser（`service-snippet/src/parser.rs`）已有成熟的 frontmatter 拆分 + YAML 解析 + 校验模式，SKILL.md parser 应复用相同模式。
  - `serde_yaml` 已在 `service-skill/Cargo.toml` 中。
  - SKILL.md 格式规范见 `docs/reference/formats/SKILL-FORMAT.md`。
  - 当前 `SkillMetadata` 只有 5 个字段（name/description/version/author/runtime），需要扩展。
- 依赖：无外部依赖阻塞。EVO-017（CLI interface 概念迁移）已完成。
- 影响范围：backend
- 最小验证方式：`cargo test -p service-skill`；手工构造 SKILL.md 文本验证解析和校验。

### EVO-022 Vite + Bun 构建骨架

- 类型：tech-debt
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：建立 Vite + Bun 构建入口，使用 React Router 替代 Next.js App Router 路由，实现与现有 Next 构建并行的双构建能力。这是前端迁移链的第一步，后续 EVO-023/024/025 依赖本故事的产物。
- 范围：
  - 在 `frontend/` 中新增 Vite 配置（`vite.config.ts`）。
  - 新增 Vite 入口 HTML（`index.html`）和 SPA 入口（`src/main-spa.tsx`）。
  - 用 React Router v6 建立 SPA 路由树，复用 Iteration 003 建立的路由适配层 `router.tsx`。
  - 新增 `package.json` scripts：`dev:spa`（Vite dev server）、`build:spa`（Vite 构建）、`preview:spa`（Vite preview）。
  - 保留现有 Next 构建不被破坏（双构建并行）。
- 不做：
  - 不迁移 `NEXT_PUBLIC_*` 环境变量；归属 EVO-023。
  - 不修改 Docker / Nginx 配置；归属 EVO-024。
  - 不创建或恢复 GitHub CI/CD workflow；归属 EVO-030。
  - 不删除 Next.js 依赖、App Router 或 middleware；归属 EVO-025。
  - 不改变 API client 或业务逻辑。
- 验收标准：
  - [x] `vite.config.ts` 存在且配置了 React 插件、路径别名（`@/`）、Tailwind。
  - [x] `index.html` SPA 入口可加载。
  - [x] React Router 路由树覆盖当前页面路由。
  - [x] `bun run dev` 启动 Vite dev server，SPA 可访问。
  - [x] `bun run build` 产出 `dist/` 静态文件。
  - [x] `bun run type-check` 通过。
  - [x] 路由适配层 `router.tsx` 在 Vite 环境使用 React Router 实现。
- 技术备注：
  - 路由适配层在 Iteration 003 已建立（`frontend/src/lib/router.tsx`），当前委托 Next；本故事需要让该层在 Vite 环境下使用 React Router 实现。
  - TanStack Query 暂不在本故事引入；当前项目使用 Zustand + Axios，保持不变。
  - Bun 作为包管理和脚本运行时，Vite 作为构建工具。
- 依赖：EVO-021（路由适配层）已完成。
- 影响范围：frontend
- 最小验证方式：`bun run build` 成功产出 `dist/`；`bun run type-check` 不报错；手动访问 SPA 验证路由。

### EVO-024 Docker / Nginx 切换到静态 SPA

- 类型：tech-debt
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：让当前生产部署形态匹配 Vite 静态 SPA，避免继续依赖 Next standalone runtime。该 Nginx 托管静态资源方案是 EVO-016 前的过渡策略，终局仍计划把前端静态产物嵌入后端发布物。
- 范围：
  - 更新前端 Dockerfile 或生产镜像构建流程，使用 Vite `dist/` 静态产物。
  - 更新 Nginx 配置，支持 SPA history fallback、静态资源缓存和 `/api/v1` 反向代理。
  - 更新 `docker-compose.prod.yml` 中与前端构建产物、服务启动命令、挂载路径相关的配置。
  - 保留或补充运行时配置 `/config.js` 的部署方式。
- 不做：
  - 不创建或恢复 `.github/workflows/*.yml`；归属 EVO-030。
  - 不删除 Next.js 依赖、App Router 或 middleware；归属 EVO-025。
  - 不改变后端 API 合约。
- 验收标准：
  - [x] 生产前端镜像不再依赖 Next standalone server。
  - [x] Nginx 能托管 `dist/` 并对 SPA 路由返回入口 HTML。
  - [x] `/api/v1` 请求仍代理到后端，且前端 API base URL 保持 `/api/v1` 约束。
  - [x] Docker production stack 可启动到前端静态页面和后端 health endpoint。
  - [x] 文档说明 GitHub CI/CD 已拆到 EVO-030，避免误以为 EVO-024 包含 workflow。
- 依赖或阻塞：EVO-022、EVO-023。
- 影响范围：frontend / deploy / docs
- 最小验证方式：`bun run build`；本地或容器内验证 Nginx 静态托管和 `/api/v1` 代理；`git diff --check`。

### EVO-030 GitHub CI/CD 重建

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 用户价值或技术目标：在前端迁移、部署形态和核心项目结构稳定后，基于最终命令重建 GitHub CI/CD，避免在迁移中反复维护过时 workflow。
- 验收标准：
  - [ ] 新建 `.github/workflows/ci.yml`，覆盖后端 fmt/clippy/test、前端 type-check/build、必要的安全扫描。
  - [ ] 如仍需要部署自动化，新建 `.github/workflows/deploy.yml` 或明确替代方案。
  - [ ] CI 中使用最终前端命令，不再引用 Next standalone 构建路径。
  - [ ] PostgreSQL/Redis 或容器依赖的验证策略明确。
  - [ ] 更新测试与发布相关参考文档。
- 依赖或阻塞：EVO-024、EVO-025，以及项目主线功能稳定后统一排期。
- 影响范围：deploy / docs
- 最小验证方式：workflow lint 或一次 GitHub Actions dry run / 手动触发记录；本地执行对应命令。

### EVO-003 忘记密码与重置密码闭环

- 类型：feature
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：用户忘记密码时能通过邮件中的公开前端链接设置新密码，完成用户生命周期闭环。
- 范围：
  - 实现 `forgot_password` handler：查找用户 → 生成 token → 使用已有 `UserRepository::set_reset_token` 存储 → 通过 Mailer 发送重置邮件。
  - 实现 `reset_password` handler：验证 token 与有效期 → 更新用户密码（Argon2id 哈希）→ 清除已用 token → 返回成功。
  - 密码重置邮件使用 `APP__PUBLIC_URL` 生成用户可访问的前端链接。
- 不做：
  - 不实现邮箱验证发送（EVO-018）。
  - 不改变现有 DTO 结构（`ForgotPasswordRequest`、`ResetPasswordRequest` 已定义）。
- 验收标准：
  - [x] `POST /api/v1/auth/forgot-password` 接受邮箱，存在时发送重置邮件，不存在时静默返回成功（防枚举）。
  - [x] `POST /api/v1/auth/reset-password` 接受 token + 新密码，验证通过后更新密码，token 失效。
  - [x] Token 有效期 1 小时，过期返回明确错误。
  - [x] 复用现有双数据库用户 reset token 字段和 repository 行为，无需新增 migration。
  - [x] `cargo test -p api` 通过，并有 Iteration 005 的前端流程验证记录。
- 技术备注：
  - Mailer trait 已有 `send_password_reset_email` 方法，SmtpMailer 和 ConsoleMailer 都已实现。
  - DTO 已定义（`ForgotPasswordRequest { email }`、`ResetPasswordRequest { token, password }`）。
  - Token 使用 crypto-random 生成，存储时只存 SHA-256 hash，原始 token 仅在邮件中传递。
  - 可复用 `service-auth` 中的 `hash_password`（Argon2id）。
- 依赖：Mailer（已有）。
- 影响范围：backend / db
- 最小验证方式：`cargo test -p api` 覆盖 forgot/reset 端到端；ConsoleMailer 输出 token 用于手工验证。

### EVO-004 邀请接受 / Join 流程

- 类型：feature
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：被邀请的用户可以通过邮件中的公开 `/join` 链接注册账号并自动加入租户，完成邀请闭环。
- 范围：
  - 实现规范化公开入口 `POST /api/v1/invitations/accept`，并保留 tenant-scoped join 兼容入口。
  - `accept_invitation` 验证 token → 检查过期/邮箱冲突 → 创建用户（Argon2id）→ 标记邀请为 accepted → 分配租户成员角色 → 返回 JWT token。
  - 利用已有的 `InvitationRepository::find_by_token` 查找邀请。
  - 利用已有的 `UserRepository::create` 创建用户。
  - `invite_member` 通过 Mailer 发送指向 `APP__PUBLIC_URL` 的 `/join` 邀请链接。
  - 增加 SPA `/join` 页面和中英文文案。
- 不做：
  - 不修改 DTO（`AcceptInviteRequest { token, password, username }` 已定义）。
- 验收标准：
  - [x] `POST /api/v1/invitations/accept` 接受 token + username + password，创建用户并加入租户。
  - [x] Token 过期或已使用返回明确错误。
  - [x] 同邮箱已注册时返回 `EMAIL_EXISTS` 冲突错误。
  - [x] 成功后返回 JWT token（用户可直接使用系统）。
  - [x] 前端公开 `/join` 路由和邀请邮件链接对齐。
  - [x] `cargo test -p api` 覆盖公开入口冲突路径、RBAC public path 和 CSRF exempt path。
- 技术备注：
  - `InvitationRepository` trait 已有 `find_by_token` 方法。
  - `AcceptInviteRequest` DTO 已定义（token, password, username）。
  - `invite_member` handler 已实现，会生成 token 并通过 Mailer 发送邀请邮件。
  - 规范化路由已注册：`/invitations/accept`；`/tenant/{tenant_id}/members/join` 保持兼容。
- 依赖：Mailer（已有）、EVO-003（可共用 token 验证模式，但无硬依赖）。
- 影响范围：backend
- 最小验证方式：`cargo test -p api` 覆盖 accept_invitation 端到端。

## 待细化故事

### EVO-026 前端 Snippets 入口迁移为 CLI 友好接口

- 类型：product-change
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：消除前端旧 Snippet 产品概念残留，让页面语言与 CLI 友好接口方向一致。
- 验收标准：
  - [ ] 导航、页面标题、空状态、按钮、详情页和新建页不再以 Snippet 作为用户可见主概念。
  - [ ] `snippetsApi` 的调用边界被替换为 CLI interface API client 或明确兼容层。
  - [ ] 中英文 i18n 文案同步迁移。
  - [ ] 旧 `/snippets` 路由的兼容、重定向或下线策略有记录。
- 依赖或阻塞：EVO-022 至 EVO-025 完成后实施，避免与前端迁移冲突。
- 影响范围：frontend / docs
- 最小验证方式：前端 type-check；搜索 `Snippet|snippet|snippets|代码片段` 确认仅剩兼容或历史文档。

### EVO-027 Skill 多来源创建

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：用户可以从本地 ZIP、Git 仓库和 SkillHub 同步创建技能，降低企业内部 Skill 沉淀和复用成本。
- 验收标准：
  - [ ] ZIP 上传支持标准 Skill 目录，必须包含根目录或单层目录下的 `SKILL.md`。
  - [ ] Git 接入支持仓库 URL、分支或 tag、子目录路径、凭据引用和导入预览。
  - [ ] SkillHub 同步支持按名称或来源 URL 拉取，并记录上游来源、同步时间和 upstream version。
  - [ ] 所有导入方式输出统一导入报告：解析成功、校验问题、资源清单、创建或更新结果。
  - [ ] 失败时不产生半成品 Skill，或半成品以 `draft` 状态可清理。
- 依赖或阻塞：EVO-020 Storage 能力；需要安全策略限制 ZIP 解压、Git clone 和外部网络访问。
- 影响范围：backend / frontend / db / docs / deploy
- 最小验证方式：后端集成测试覆盖 ZIP 导入和非法包；Git/SkillHub 可先用 mocked provider；前端导入向导手工验证。

### EVO-028 Skill 版本管理与正确性验证

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：Skill 的创建、更新和同步都有版本轨迹和正确性验证，避免无效技能进入企业级 harness 平台。
- 验收标准：
  - [ ] `name + version + tenant/source` 唯一性规则明确，支持版本列表、版本详情、设为默认版本和回滚。
  - [ ] 校验遵循 Agent Skills 规范：目录必须含 `SKILL.md`；frontmatter 必须含 `name`、`description`；`name` 使用小写字母、数字和连字符，不能首尾为连字符或包含连续连字符，且匹配目录名。
  - [ ] `description` 必须非空且不超过 1024 字符，并同时说明“做什么”和“何时使用”；低质量描述给出 warning。
  - [ ] `license`、`compatibility`、`metadata`、`allowed-tools`、`scripts/`、`references/`、`assets/` 有解析和兼容处理。
  - [ ] 校验报告区分 blocking error 与 warning，API 和前端都能展示。
- 依赖或阻塞：EVO-027 多来源导入；需要决定是否直接集成 `skills-ref validate` 或实现兼容校验器。
- 影响范围：backend / frontend / db / docs
- 最小验证方式：parser/validator 单测覆盖合法、非法和 warning 样例；repository 测试覆盖版本查询和回滚。

### EVO-029 Skill 专业描述与发现质量提升

- 类型：feature
- 优先级：P2
- 状态：Proposed
- 用户价值或技术目标：让技能描述更适合智能体自动发现和选择，提升企业知识资产的可检索性。
- 验收标准：
  - [ ] 创建和导入时对 description 给出专业度评分或检查项：能力、触发场景、关键词、边界条件。
  - [ ] 列表和搜索优先使用 name、description、tags、compatibility 和 metadata 中的发现信号。
  - [ ] 前端新建页提供符合 Agent Skills 建议的描述模板，不再使用泛泛示例。
  - [ ] 低质量描述不会阻塞保存，但必须在导入报告或编辑页提示。
- 依赖或阻塞：EVO-028 的校验报告模型。
- 影响范围：backend / frontend / docs
- 最小验证方式：描述质量检查单测；前端新建页文案检查；搜索结果基本回归。
