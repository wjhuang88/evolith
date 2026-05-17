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
| EVO-005 | MCP 工具真实执行 | feature | P0 | Ready | 需求 F1.1.3 | 当前 `tools/call` 返回 stub |
| EVO-006 | Skill 更新接口 | feature | P1 | Proposed | API 501 | `PUT /skills/{id}` |
| EVO-007 | Snippet 更新接口 | feature | P1 | Deferred | API 501 | 被 EVO-017 替代方向覆盖，暂停继续投入 |
| EVO-008 | Snippet reference 格式增强 | feature | P1 | Deferred | 需求 F1.3.4 | 被 EVO-017 替代方向覆盖 |
| EVO-009 | SKILL.md 与 CLI interface frontmatter parser | feature | P1 | Proposed | 格式规范 / EVO-017 | 为 CLI 友好接口和上传校验打基础 |
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
| EVO-024 | Docker / Nginx 切换到静态 SPA | tech-debt | P0 | Done | EVO-002 split | Iteration 004；Dockerfile 改为 Vite build + Nginx 静态服务，SPA fallback |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；删除 next 依赖、App Router、middleware、config；router.tsx 改为 React Router |
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | P1 | Proposed | EVO-017 / 页面残留 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| EVO-027 | Skill 多来源创建 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 |
| EVO-028 | Skill 版本管理与正确性验证 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 |
| EVO-029 | Skill 专业描述与发现质量提升 | feature | P2 | Proposed | Agent Skills spec | 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 |
| EVO-030 | GitHub CI/CD 重建 | tech-debt | P2 | Proposed | EVO-002 split / 工程收尾 | 放到项目后段统一做；基于最终构建、测试、部署命令重建 workflow |

## 故事模板

```markdown
### EVO-XXX <标题>

- 类型：
- 优先级：
- 状态：
- 用户价值：
- 范围：
- 不做：
- 验收标准：
  - [ ] ...
- 技术备注：
- 依赖：
```

## 下一批建议

优先选择：

1. `EVO-003` 忘记密码与重置密码闭环。
2. `EVO-004` 邀请接受 / Join 流程。

理由：Phase B（前端迁移）已完成；Phase C（认证闭环）是下一阶段。EVO-003 和 EVO-004 都是 P0 Ready，实现后用户生命周期关键流程（注册→登录→改密→忘记密码→邀请加入）基本闭环。EVO-018 邮箱验证可与 EVO-003 共用 token 生成逻辑，但优先级 P1，不在本次迭代。

## 已细化故事

### EVO-022 Vite + Bun 构建骨架

- 类型：tech-debt
- 优先级：P0
- 状态：In Progress
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
  - [ ] `vite.config.ts` 存在且配置了 React 插件、路径别名（`@/`）、Tailwind。
  - [ ] `index.html` SPA 入口可加载。
  - [ ] React Router 路由树覆盖当前所有 22 个页面路由。
  - [ ] `bun run dev:spa` 启动 Vite dev server，SPA 可访问。
  - [ ] `bun run build:spa` 产出 `dist/` 静态文件。
  - [ ] 现有 `npm run build` / `npm run type-check` 不受影响。
  - [ ] 路由适配层 `router.tsx` 在 Vite 环境使用 React Router 实现。
- 技术备注：
  - 路由适配层在 Iteration 003 已建立（`frontend/src/lib/router.tsx`），当前委托 Next；本故事需要让该层在 Vite 环境下使用 React Router 实现。
  - TanStack Query 暂不在本故事引入；当前项目使用 Zustand + Axios，保持不变。
  - Bun 作为包管理和脚本运行时，Vite 作为构建工具。
- 依赖：EVO-021（路由适配层）已完成。
- 影响范围：frontend
- 最小验证方式：`bun run build:spa` 成功产出 `dist/`；`npm run build` 不报错；手动访问 SPA 验证路由。

### EVO-024 Docker / Nginx 切换到静态 SPA

- 类型：tech-debt
- 优先级：P0
- 状态：Ready
- 用户价值或技术目标：让生产部署形态匹配 Vite 静态 SPA，避免继续依赖 Next standalone runtime，为后续移除 Next.js 依赖提供部署侧前置条件。
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
  - [ ] 生产前端镜像不再依赖 Next standalone server。
  - [ ] Nginx 能托管 `dist/` 并对 SPA 路由返回入口 HTML。
  - [ ] `/api/v1` 请求仍代理到后端，且前端 API base URL 保持 `/api/v1` 约束。
  - [ ] Docker production stack 可启动到前端静态页面和后端 health endpoint。
  - [ ] 文档说明 GitHub CI/CD 已拆到 EVO-030，避免误以为 EVO-024 包含 workflow。
- 依赖或阻塞：EVO-022、EVO-023。
- 影响范围：frontend / deploy / docs
- 最小验证方式：`bun run build:spa`；本地或容器内验证 Nginx 静态托管和 `/api/v1` 代理；`git diff --check`。

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
- 状态：Ready
- 用户价值或技术目标：用户忘记密码时能通过邮箱收到重置链接，点击后设置新密码，完成用户生命周期闭环。当前 `POST /auth/forgot-password` 和 `POST /auth/reset-password` 返回 501。
- 范围：
  - 实现 `forgot_password` handler：查找用户 → 生成随机 token → 存储到 password_reset_tokens 表 → 通过 Mailer 发送重置邮件。
  - 实现 `reset_password` handler：验证 token 有效性和过期时间 → 更新用户密码（Argon2id 哈希）→ 删除已用 token → 返回成功。
  - 新增 SQLite 和 PostgreSQL migration：`password_reset_tokens` 表（id, user_id, token_hash, expires_at, created_at）。
  - 在 `domain::repository` 新增 `PasswordResetTokenRepository` trait（create, find_by_token, delete）。
  - 在 `infra::db` 实现 SQLite 和 PostgreSQL 版本。
  - AppState 新增 `password_reset_repo: Arc<dyn PasswordResetTokenRepository>`。
  - handler 注入 Mailer 和 Repository，不再返回 501。
- 不做：
  - 不实现邮箱验证发送（EVO-018）。
  - 不修改前端页面逻辑（前端已有 forgot-password 和 reset-password 页面和表单）。
  - 不改变现有 DTO 结构（`ForgotPasswordRequest`、`ResetPasswordRequest` 已定义）。
- 验收标准：
  - [ ] `POST /api/v1/auth/forgot-password` 接受邮箱，存在时发送重置邮件（ConsoleMailer 打印 token），不存在时静默返回成功（防枚举）。
  - [ ] `POST /api/v1/auth/reset-password` 接受 token + 新密码，验证通过后更新密码，token 失效。
  - [ ] Token 有效期 1 小时，过期返回明确错误。
  - [ ] SQLite 和 PostgreSQL migration 都有。
  - [ ] `cargo test --workspace` 通过。
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
- 状态：Ready
- 用户价值或技术目标：被邀请的用户可以通过邮件中的邀请链接注册账号并自动加入租户，完成邀请闭环。当前 `POST /tenant/{id}/members/join` 返回 501。
- 范围：
  - 实现 `accept_invitation` handler：验证 token → 检查过期 → 检查邮箱是否已注册 → 创建用户（Argon2id）→ 标记邀请为 accepted → 分配租户成员角色 → 返回 JWT token。
  - 利用已有的 `InvitationRepository::find_by_token` 查找邀请。
  - 利用已有的 `UserRepository::create` 创建用户。
  - 利用已有的 `MemberRepository` 添加租户成员关系。
  - 利用 Mailer 发送欢迎邮件（可选，非阻塞）。
- 不做：
  - 不修改邀请创建逻辑（`invite_member` handler 已实现）。
  - 不修改前端页面（前端已有 accept-invite 流程）。
  - 不修改 DTO（`AcceptInviteRequest { token, password, username }` 已定义）。
- 验收标准：
  - [ ] `POST /api/v1/tenant/{tenant_id}/members/join` 接受 token + username + password，创建用户并加入租户。
  - [ ] Token 过期返回明确错误。
  - [ ] 同邮箱已注册时返回明确错误（建议提示登录后接受邀请）。
  - [ ] 成功后返回 JWT token（用户可直接使用系统）。
  - [ ] 邀请被标记为 accepted，不可重复使用。
  - [ ] `cargo test --workspace` 通过。
- 技术备注：
  - `InvitationRepository` trait 已有 `find_by_token` 方法。
  - `AcceptInviteRequest` DTO 已定义（token, password, username）。
  - `invite_member` handler 已实现，会生成 token 并通过 Mailer 发送邀请邮件。
  - 路由已注册：`/tenant/{tenant_id}/members/join`。
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
