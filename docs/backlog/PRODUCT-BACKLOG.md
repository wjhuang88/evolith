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
| EVO-006 | Skill 更新接口 | feature | P1 | Done | API 501 / Iteration 010 | `PUT /skills/{id}` 已实现 |
| EVO-007 | Snippet 更新接口 | feature | P1 | Deferred | API 501 | 被 EVO-017 替代方向覆盖，暂停继续投入 |
| EVO-008 | Snippet reference 格式增强 | feature | P1 | Deferred | 需求 F1.3.4 | 被 EVO-017 替代方向覆盖 |
| EVO-009 | SKILL.md 与 CLI interface frontmatter parser | feature | P1 | Done | 格式规范 / EVO-017 / Iteration 010 | CLI interface parser 已有基线；SKILL.md parser 已实现 |
| EVO-010 | 租户 Members 页面接真实 API | feature | P1 | Done | Iteration 011 | 后端 find_by_tenant + remove_from_tenant + 前端接线 |
| EVO-011 | API Key 页面接真实 API | feature | P1 | Done | Iteration 011 | types + api module + page 重写 |
| EVO-012 | 租户设置保存 | feature | P2 | Proposed | 前端 TODO | tenant settings |
| EVO-013 | Audit log detail 接口 | feature | P2 | Proposed | API 501 | `GET /audit-logs/{log_id}` |
| EVO-014 | Stripe webhook 恢复 | feature | P2 | Proposed | routes TODO | 计费闭环 |
| EVO-015 | Rust CLI 子项目 | feature | P3 | Deferred | [提案](../proposals/RUST-CLI.md) | API 稳定后启动 |
| EVO-016 | 前端嵌入后端发布物 | tech-debt | P3 | Deferred | [提案](../proposals/EMBEDDED-FRONTEND.md) | Vite SPA 完成后启动 |
| EVO-016-A | Embedded Frontend 交付形态 refinement | tech-debt | P2 | Ready | EVO-016 split / Iteration 024 | 先确认后端静态服务、单容器边界和 Nginx 终局角色，解除 Iteration 012 前置 |
| EVO-016-B | 前端静态服务迁移到 rust-embed-for-web | tech-debt | P0 | Done | EVO-016 split / 用户需求 / Iteration 031 | 替代 ZIP 方案：用 rust-embed-for-web 实现零拷贝 + 预压缩 + 自动缓存协商；14 项验收标准全部通过 |
| EVO-017 | Snippet 迁移为 CLI 友好接口 | product-change | P0 | Done | [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) | Iteration 002；replaces EVO-007/EVO-008；已建立 CLI interface 格式、API 兼容契约、parser 基线和迁移盘点 |
| EVO-018 | 邮箱验证发送与确认闭环 | feature | P1 | Done | Iteration 009 | Handler 与测试存在，Iteration 021 完成 contract/testing/roadmap 收口 |
| EVO-019 | Skill registry 服务化 | tech-debt | P2 | Proposed | Phase E placeholder | 将 `service-skill/src/registry.rs` 从 placeholder 补成可复用注册能力 |
| EVO-020 | Storage 能力落地 | feature | P2 | Proposed | Phase E placeholder | 实现对象存储基础能力，支撑技能包和附件 |
| EVO-021 | 前端路由适配层 | tech-debt | P0 | Done | EVO-002 split | Iteration 003；已新增 `frontend/src/lib/router.tsx`，页面和共享组件不再直接导入 Next 路由模块 |
| EVO-022 | Vite + Bun 构建骨架 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；新增 Vite 入口、React Router 根路由和并行构建脚本 |
| EVO-023 | 前端运行时配置迁移 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；`src/lib/config.ts` 统一运行时环境变量，替换所有 `process.env` 引用 |
| EVO-024 | Docker / Nginx 切换到静态 SPA | tech-debt | P0 | Done | EVO-002 split | Iteration 004；过渡部署形态，Dockerfile 改为 Bun + Vite build + Nginx 静态服务，SPA fallback |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | tech-debt | P0 | Done | EVO-002 split | Iteration 004；删除 next 依赖、App Router、middleware、config；router.tsx 改为 React Router |
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | P1 | Ready | EVO-017 / 页面残留 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| EVO-027 | Skill 多来源创建 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 |
| EVO-028 | Skill 版本管理与正确性验证 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 |
| EVO-029 | Skill 专业描述与发现质量提升 | feature | P2 | Proposed | Agent Skills spec | 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 |
| EVO-030 | GitHub CI/CD 重建 | tech-debt | P2 | Proposed | EVO-002 split / 工程收尾 | 放到项目后段统一做；基于最终构建、测试、部署命令重建 workflow |
| EVO-031 | Iteration 004/005 质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-17 | 修复静态资源反代、邀请接受闭环、公开接口放行、邮件公开 URL、sourcemap 默认关闭和流程规约 |
| EVO-032 | Iteration 006 MCP 执行质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-25 | Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真 |
| EVO-033 | Rustfmt 全量格式基线与 stable 配置清理 | tech-debt | P2 | Proposed | Iteration 007 验证残余 | 独立处理历史格式差异和 nightly-only 配置告警，避免混入功能修复 |
| EVO-034 | Epic 与子需求拆分治理规则 | tech-debt | P1 | Done | 流程缺口 2026-05-26 | Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束 |
| EVO-035 | 治理 skill manifest 接入与一致性审计 | tech-debt | P2 | Done | Iteration 008 验证残余 | Iteration 023；已建立 manifest 并通过 bundled validator |
| EVO-036 | 已发布迭代计划基线保护与改线防呆 | tech-debt | P1 | Done | 计划覆写复盘 2026-05-27 | Iteration 013；修复 EVO-016 计划追踪并同步治理 skill |
| EVO-037 | 治理 skill 弱模型闭环执行防呆 | tech-debt | P1 | Done | 用户反馈 2026-05-27 | Iteration 014；为初始化、迁移和修复任务增加强制闭环协议 |
| EVO-038 | 本项目实施任务闭环 SOP 与完成声明门禁 | tech-debt | P1 | Done | 用户反馈 2026-05-27 | Iteration 015；将闭环协议落实到 Evolith 自身流程 |
| EVO-039 | 迭代启动前库存盘点与既有计划优先规则 | bug | P1 | Done | 流程缺口 2026-05-27 | Iteration 016；先处理在途/已规划迭代再选择新 story |
| EVO-040 | 已实现接口完成声明与参考文档状态修复 | bug | P1 | Done | 排期库存审计 2026-05-27 | Iteration 021；修复邮箱验证与 Skill 更新接口的收口漂移 |
| EVO-041 | 敏捷实践与 BDD 验收格式适配规则 | tech-debt | P1 | Done | 用户方法论反馈 2026-05-28 | Iteration 022；明确 Evolith iteration 与传统 Sprint、Story 与 BDD 的适配口径 |
| EVO-043 | 后端依赖全量版本审计与迁移 | tech-debt | P1 | Ready | 用户需求 / Iteration 030 | 审计 backend workspace 所有 crate 依赖，升级到最新稳定版并验证编译/测试通过 |


## 故事模板

行为类 Story 使用用户故事格式；技术、治理和 Spike 使用等价格式，但必须保留价值、
范围、不做、验收、依赖和验证字段。详细规则见
[需求进入与 Backlog 整理](../sop/REQUIREMENT-INTAKE.md#story-格式规范与-bdd-验收)。

```markdown
### EVO-XXX <标题>

- 类型：
- 优先级：
- 状态：
- 父 Epic：（非子 Story 填无）
- Story 形态：Product / API / Technical / Governance / Spike
- 用户故事或技术目标：
  - 作为/为了：
  - 我希望/需要：
  - 以便：
- 范围：
- 不做：
- 验收标准：
  - 行为类：
    - Given ...
      When ...
      Then ...
  - 非行为类：
    - [ ] <命令或人工检查> 证明 <结果>
- 技术备注：
- 依赖或阻塞：
- 解锁内容：
- 最小验证方式：
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

### EVO-016-A Embedded Frontend 交付形态 refinement

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：EVO-016
- Story 形态：Technical
- 用户故事或技术目标：
  - 作为/为了：维护者需要明确前端嵌入后端发布物的交付边界。
  - 我希望/需要：形成后端静态服务、SPA fallback、Nginx 角色、Docker 构建和验证方案。
  - 以便：Iteration 012 的 `EVO-016-B` 能按清晰方案实施，不再依赖过渡 Nginx 静态托管。
- 范围：
  - 明确前端静态产物由后端服务的目录、路由优先级和 SPA fallback 行为。
  - 明确 `/api/v1`、`/health`、`/mcp` 与 `/assets/` 的优先级和验证矩阵。
  - 明确生产 Docker 形态：是否单 backend 容器承载 API + 静态资源，Nginx 是否仅保留为可选网关。
  - 明确 Bun/Vite 构建产物如何进入后端发布物或镜像。
  - 输出 `EVO-016-B` 的实施边界、风险和验证命令。
- 不做：
  - 不在 refinement 中改业务代码或切换生产部署。
  - 不实现单二进制 `include_dir` 实验；可作为后续候选。
  - 不重建 GitHub CI/CD；归 EVO-030。
- 验收标准：
  - [ ] 写清 embedded frontend 的目标部署形态与当前 Nginx 过渡策略差异。
  - [ ] 写清路由优先级、SPA fallback 和静态资源缓存/路径策略。
  - [ ] 写清 Docker/build 输入输出和本地/容器验证矩阵。
  - [ ] 更新 Iteration 012 的激活条件或确认仍阻塞。
  - [ ] 将实施切片 `EVO-016-B` 的范围、依赖和验证方式补齐。
- 依赖或阻塞：Vite SPA 与 Bun 构建已完成；需要基于当前部署文档和代码确认最终策略。
- 影响范围：docs / deploy / backend / frontend
- 最小验证方式：Markdown 链接检查；`git diff --check`；必要时只读检查当前 Docker/Nginx/backend route 配置。

### EVO-016-B 前端静态服务迁移到 rust-embed-for-web

- 类型：tech-debt
- 优先级：P0
- 状态：In Progress
- 父 Epic：EVO-016
- Story 形态：Technical
- 用户故事或技术目标：
  - 作为/为了：后端单二进制部署的前端静态文件服务。
  - 我希望/需要：将当前基于 ZIP 解压的前端静态服务方案替换为 `rust-embed-for-web` + `actix-web-rust-embed-responder`，实现零拷贝、预压缩和自动 HTTP 缓存协商。
  - 以便：彻底消除每请求 ZIP 解压 CPU 开销和 spawn_blocking 线程池占用；利用构建时预压缩（gzip + brotli）实现零运行时 CPU 的 Content-Encoding 协商；利用构建时预计算 SHA-256 ETag 和 Last-Modified 实现自动 304 条件响应。
- 方案决策背景：
  - 经过多方调研（librarian 交叉验证、OpenObserve 生产实现参考、zip crate 源码分析），确认 ZIP 方案的 spawn_blocking + channel + 流式解压是过度工程。
  - `rust-embed-for-web` 直接消除整个问题域：`&'static [u8]` 零拷贝访问，无需解压、无需 channel、无需线程池桥接。
  - `actix-web-rust-embed-responder` 自动处理 ETag（SHA-256 + Base85）、Last-Modified、If-None-Match 304、Accept-Encoding 协商（gzip/brotli 预压缩数据）。
  - Debug 模式自动从文件系统读取 `frontend/dist/`，前端改动无需重编译 Rust。
  - 唯一代价：二进制体积增加约 2-4 倍（一个典型 SPA 约 5-15MB 额外），对服务端二进制可接受。
  - 参考：OpenObserve（生产级可观测平台）使用相同方案，但未启用预压缩和自动缓存协商。
- 范围：
  - 重写 `backend/src/frontend.rs`，用 `rust-embed-for-web` 替代当前 ZIP 实现：
  - **移除 ZIP 依赖**：删除 `zip` crate 依赖、`frontend.zip` 占位文件、`ZipArchive`/`ZipArchiveMetadata`/`FileEntry`/`FrontendIndex` 等数据结构、`build_index()`/`read_file_from_zip()` 等函数。
  - **移除 `embedded-frontend` feature flag**：`rust-embed-for-web` 的 Debug/Release 行为切换由 crate 内置处理（Debug 读文件系统、Release 零拷贝嵌入），不再需要外部 feature flag。删除 `maybe_configure_frontend!` 宏，路由注册变为无条件。
  - **引入 `rust-embed-for-web`**：`#[derive(RustEmbed)] #[folder = "../frontend/dist/"]` 编译时嵌入前端产物。
  - **引入 `actix-web-rust-embed-responder`**：用 `EmbedResponse` 作为 handler 返回类型，自动处理 ETag、Last-Modified、304、Content-Encoding 协商。
  - **SPA fallback**：`Embed::get(path).or_else(|| Embed::get("index.html")).into_response()`。
  - **保留 `/config.js` 动态生成**：运行时环境变量注入行为不变，不在 `RustEmbed` 嵌入范围内。
  - **保留 `Cache-Control` 手动设置**：responder 不自动处理 Cache-Control。对 Vite 哈希文件名资源设 `immutable`，对 `index.html` 设 `no-cache`。
  - **Content-Type**：resender 自动根据文件扩展名推断 MIME 类型（替代当前手动 match）。
- 不做：
  - 不修改 `/config.js` 动态生成逻辑。
  - 不修改 API 路由、中间件或业务逻辑。
  - 不重建 GitHub CI/CD（归 EVO-030）。
  - 不修改前端构建流程或 Dockerfile。
  - 不启用 zstd 预压缩（需 C 绑定，gzip + brotli 足够覆盖主流浏览器）。
- 验收标准：
  - 非行为类：
    - [ ] `zip` crate 依赖和 `frontend.zip` 已从 `backend/` 移除。
    - [ ] `embedded-frontend` feature flag 已移除；`cargo check --workspace` 无需 `--features` 即可通过。
    - [ ] `rust-embed-for-web` 和 `actix-web-rust-embed-responder` 已添加为 `backend/Cargo.toml` 依赖。
    - [ ] `cargo test --workspace` 通过。
    - [ ] `cargo clippy --workspace -- -D warnings` 通过。
    - [ ] Release 模式下 `Assets::get("index.html")` 返回 `Some(EmbeddedFile)`，`data()` 为 `&'static [u8]`（零拷贝）。
    - [ ] Debug 模式下前端改动后刷新浏览器即可生效（无需重编译 Rust）。
    - [ ] 响应包含 ETag header（SHA-256 + Base85）。
    - [ ] 支持 `If-None-Match` 条件请求，返回 304 Not Modified。
    - [ ] 哈希文件名资源返回 `Cache-Control: public, max-age=31536000, immutable`。
    - [ ] `index.html` 返回 `Cache-Control: no-cache`。
    - [ ] 客户端支持 brotli 时，响应包含 `Content-Encoding: br`（零 CPU 预压缩数据）。
    - [ ] 客户端支持 gzip 但不支持 brotli 时，响应包含 `Content-Encoding: gzip`（零 CPU 预压缩数据）。
    - [ ] SPA fallback：未知路径返回 `index.html` 的 200 响应。
    - [ ] `/config.js` 仍然由环境变量动态生成。
    - [ ] `frontend.rs` 代码量 ≤ 80 行（从当前 ~280 行大幅缩减）。
- 技术备注：
  - 关键 crate：
    - `rust-embed-for-web = "11"` — 编译时嵌入 + 预压缩（gzip + brotli）+ SHA-256 ETag + Base85 编码 + MIME 推断。Debug 模式从文件系统读取（`DynamicFile`），Release 模式零拷贝嵌入（`EmbeddedFile`）。
    - `actix-web-rust-embed-responder = "2"` — actix-web Responder 实现，自动处理 ETag/Last-Modified/304/Content-Encoding 协商。`Compress::IfPrecompressed` 模式仅使用预压缩数据，零运行时 CPU。
  - Debug/Release 自动切换：
    - Debug：`DynamicFile`，从 `frontend/dist/` 读文件系统，前端改动无需重编译。
    - Release：`EmbeddedFile`，`include_bytes!` 编译时嵌入，`data()` 返回 `&'static [u8]`。
  - 与当前 ZIP 方案对比：
    - 每请求 CPU：ZIP 方案（deflate 解压 + spawn_blocking）→ 新方案（零 CPU）。
    - 每请求内存：ZIP 方案（Vec<u8> + channel 缓冲）→ 新方案（零分配，`&'static [u8]` 切片）。
    - 代码复杂度：ZIP 方案（~280 行）→ 新方案（~50-80 行）。
    - ETag 强度：ZIP 方案（CRC32 或文件名 hash）→ 新方案（SHA-256 + Base85）。
    - Content-Encoding：ZIP 方案（无）→ 新方案（自动 br/gzip 协商）。
  - workspace 有 `#![deny(clippy::unwrap_used)]`，所有错误处理使用 `?` 或显式 match。
  - 生产参考：OpenObserve 使用 `rust-embed-for-web` + axum 提供前端静态文件。
- 依赖或阻塞：EVO-016-A 基础嵌入实现已完成（可被本故事完全替换）。
- 解锁内容：完成后进入 Iteration 012 单容器 Dockerfile 集成；消除 Nginx 静态托管依赖。
- 影响范围：backend（`frontend.rs` 重写、`Cargo.toml` 依赖变更、`main.rs` 移除 feature flag 和宏）
- 最小验证方式：`cargo test --workspace`；`cargo clippy --workspace -- -D warnings`；Release 模式 `curl -v localhost:8080/index.html` 检查 ETag/Cache-Control/Content-Encoding；`curl -H "If-None-Match: <etag>"` 验证 304。

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
- 状态：Done
- 用户价值或技术目标：让 Evolith 已有治理文档能被 `agent-project-governance` skill 明确识别为初始化/采用状态，并通过一致性审计发现后续漂移。
- 范围：
  - 按当前项目治理现状建立 `.agent-governance/manifest.yaml`。
  - 核对 capability 状态、标准入口与已有 SOP / reference / backlog / iteration 映射。
  - 运行 skill bundled validator，并将真实残余写回 backlog 或治理文档。
- 不做：
  - 不在 EVO-034 中补造 manifest 以掩盖附加审计失败。
  - 不顺带改业务逻辑或重写既有迭代历史。
- 验收标准：
  - [x] manifest 能准确表达 Evolith 的治理 profile、入口和能力状态。
  - [x] `validate_project_governance.py /Users/GHuang/WorkSpace/AiProjects/evolith` 通过，或将剩余问题拆为明确事项。
- 依赖或阻塞：EVO-034 已完成；本轮按当前治理状态采用 `high-risk / conformant` profile。
- 影响范围：docs
- 最小验证方式：运行 `agent-project-governance/scripts/validate_project_governance.py`。

### EVO-036 已发布迭代计划基线保护与改线防呆

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：防止已发布的 future iteration 被另一组工作直接覆盖，确保计划、
  实际执行、依赖阻塞和改线决策均可追溯。
- 范围：
  - 在入口约束、迭代/变更/文档/Git SOP 与模板中定义已发布计划基线保护规则。
  - 对 Iteration 011 补回原 EVO-016 计划基线和流程偏差记录，不撤销 EVO-010/EVO-011
    已发生的实际执行事实。
  - 标记 Iteration 012 暂不可激活；原 EVO-016 refinement 需使用新的 iteration 编号
    重新排期后再解除阻塞。
  - 将可复用的计划基线保护方法同步到 `agent-project-governance` skill。
- 不做：
  - 不回滚外部实施的业务代码、验证记录或提交历史。
  - 不在本故事中实施 EVO-016，也不重排全部远期事项。
- 验收标准：
  - [x] `AGENTS.md`、相关 SOP 与 iteration 模板禁止以不同目标就地覆写已发布计划。
  - [x] Iteration 011 可追溯到原 EVO-016 计划，Iteration 012 显式记录激活阻塞。
  - [x] `agent-project-governance` skill 体现同类初始化、审计与恢复规则。
  - [x] Markdown 链接检查、`git diff --check` 和 skill 结构校验通过。
- 依赖或阻塞：外部完成的 Iteration 011 实际工作保留；skill 仓库存在未提交修改，
  同步时不得覆盖无关文件。
- 影响范围：docs / external skill
- 最小验证方式：执行文档链接检查；`git diff --check`；运行 skill 的
  `quick_validate.py`。

### EVO-037 治理 skill 弱模型闭环执行防呆

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让能力较弱的模型使用治理 skill 时也必须完成状态同步、验证、
  残余登记和可继续操作说明，减少“文件生成了但问题没有闭环”的交付。
- 范围：
  - 在 `agent-project-governance` 的主工作流中加入不可跳过的闭环契约和完成判定。
  - 提供低自由度的执行清单、闭环台账与最终输出模板，区分完成、部分完成和受阻。
  - 增加针对只创建文件却漏掉 manifest/状态/验证/残余登记的评估场景。
  - 将本次发现的过程教训写回 Evolith 的经验记录。
- 不做：
  - 不试图通过文档保证所有低能力模型都能完成复杂代码实现。
  - 不改业务代码、部署形态或尚未启动的产品故事。
  - 不提交 skill 目录已有未提交修改。
- 验收标准：
  - [x] skill 入口明确要求实施任务按“建账、执行、核验、同步、交付”完成闭环。
  - [x] 专门参考文档给出可机械执行的闭环协议和部分完成/阻塞输出要求。
  - [x] 评估用例可识别生成骨架后过早宣布完成的行为。
  - [x] 项目文档检查、`git diff --check` 和 skill 结构校验通过。
- 依赖或阻塞：skill 仓库已有未提交治理更新，必须基于现状增量写入，不覆盖无关文件。
- 影响范围：docs / external skill
- 最小验证方式：执行 Markdown 链接检查；`git diff --check`；运行 skill 的
  `quick_validate.py`。

### EVO-038 本项目实施任务闭环 SOP 与完成声明门禁

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让 Evolith 自身的 Agent 执行流程具备统一、可机械遵循的
  收口门禁，避免实现或文档更新完成一部分后遗漏状态、验证、残余登记就宣称完成。
- 范围：
  - 新增通用任务收口 SOP，定义闭环台账、执行阶段和 `Complete / Partial / Blocked`
    完成声明条件。
  - 将入口约束、迭代推进、变更控制、结对审查、Git 与文档检查路由到收口门禁。
  - 更新 iteration 模板，使迭代记录显式承载闭环责任和完成结论。
  - 将已写入的闭环经验扩展到本项目 SOP 落地结论。
- 不做：
  - 不改业务代码、测试实现或部署配置。
  - 不重复实现专业测试、发布、数据库或 API SOP 已拥有的检查细节。
  - 不继续修改外部 skill；该部分已在 EVO-037 完成。
- 验收标准：
  - [x] `TASK-CLOSURE.md` 定义实施任务开始前建账、结束前核验/同步/交付的固定步骤。
  - [x] `AGENTS.md` 与相关 SOP/模板均能将 Agent 引导到通用收口门禁。
  - [x] 完成声明不能绕过验证结果、状态同步或残余工作登记。
  - [x] Markdown 链接检查和 `git diff --check` 通过。
- 依赖或阻塞：EVO-037 已完成外部 skill 的闭环协议；本轮将同类规则本地化。
- 影响范围：docs
- 最小验证方式：执行文档相对链接检查；`git diff --check`。

### EVO-039 迭代启动前库存盘点与既有计划优先规则

- 类型：bug
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：当用户要求开始迭代时，Agent 必须先发现并处理仍在推进、
  待收口或已规划的 iteration，不因直接扫描 backlog 而跳过在途目标或制造状态漂移。
- 范围：
  - 在迭代启动与迭代工作流中加入库存盘点门禁，定义 `Active / In Progress / Review /
    Planned / Blocked` 的处置优先级。
  - 规定仅在已有迭代均完成 disposition 后，才可从 backlog 选择新的 `Ready` story。
  - 修复已发现的 Iteration 010 状态漂移：保留其未核验项并转为 `Review`，不得伪造关闭。
  - 将可迁移的“iteration inventory before backlog selection”规则同步到治理 skill。
- 不做：
  - 不在本故事核验或补做 Iteration 010 尚未证明完成的格式依赖文档内容。
  - 不解除 Iteration 012 的 EVO-016 前置阻塞，也不实施 embedded frontend。
  - 不将外部 skill 仓库的独立修改纳入 Evolith 仓库提交。
- 验收标准：
  - [x] `START-ITERATION.md` 明确迭代库存盘点先于 backlog story 选择。
  - [x] `AGENTS.md`、迭代工作流、目录说明与文档检查均能防止绕过现有未完成计划。
  - [x] Iteration 010 的故事完成事实与迭代收口缺口均可追溯，Iteration 012 的阻塞保留。
  - [x] `agent-project-governance` skill 包含同类生成/审计与评估规则。
  - [x] 项目文档检查、`git diff --check` 与 skill 结构校验通过。
- 依赖或阻塞：本故事为治理缺陷修复，可在产品迭代重新选取前实施；Iteration 010
  收口证据缺口和 Iteration 012 阻塞仍需分别处置。
- 影响范围：docs / external skill
- 最小验证方式：执行文档相对链接检查；`git diff --check`；运行 skill 的
  `quick_validate.py`。

### EVO-040 已实现接口完成声明与参考文档状态修复

- 类型：bug
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：使已实现接口事实、API 合约、测试/路线图参考和迭代完成声明
  一致，避免 Agent 依据冲突文档规划或回归错误行为。
- 范围：
  - 复核 `send-verify` / `verify-email` 与 `PUT /skills/{id}` 的代码及已存在测试证据。
  - 将 API contract、testing reference 与 roadmap 中仍标为未实现或过期状态的内容
    更新为真实接口行为和验证口径。
  - 核验证据后同步 EVO-018 / Iteration 009 与 Iteration 010 的最终收口状态。
- 不做：
  - 不在本故事新增邮箱验证或 Skill 更新业务能力，也不改变认证策略。
  - 不混入 Phase E、embedded frontend 或 CI/CD 实施。
- 验收标准：
  - [x] API contract 不再将已实现的邮箱验证与 Skill 更新端点标为未实现。
  - [x] testing reference 与 roadmap 对实际接口路径、状态和验证口径保持一致。
  - [x] 必需验证重新执行并记录后，Iteration 009 / 010 状态一致可追溯。
- 依赖或阻塞：当前代码与历史测试记录可作为复核起点；完成前 Iteration 009 / 010
  保持 `Review`，不作为新产品迭代已处置依据。
- 影响范围：docs / backend validation
- 最小验证方式：`cargo test -p api`；Markdown 相对链接检查；`git diff --check`。

### EVO-041 敏捷实践与 BDD 验收格式适配规则

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 父 Epic：无
- 用户价值或技术目标：让 Evolith 的 Agent 迭代治理能吸收传统敏捷和 BDD 的可验证性，
  同时保留本项目“计划基线、库存盘点、命令级证据、闭环归口”的执行边界，避免后续
  Agent 机械套用 Scrum 或把所有任务都写成不合适的用户故事。
- 范围：
  - 定义 Evolith `iteration` 与传统 Scrum `Sprint` 的关系、差异和适用节奏。
  - 为产品故事、API/权限/状态故事、技术故事、治理文档故事和 spike 定义不同表述方式。
  - 明确哪些任务必须使用 Given/When/Then BDD 场景，哪些任务可用等价技术验收。
  - 将 Story / BDD 质量检查接入 DoR、迭代计划、文档一致性检查和模板。
  - 写回本次方法论经验，作为后续 Agent 判断依据。
- 不做：
  - 不引入完整 Scrum 仪式、团队容量统计或固定冲刺承诺。
  - 不修改业务代码、测试代码、CI/CD 或部署策略。
  - 不提交外部 `agent-project-governance` skill；仅按用户要求同步内容，提交由用户另行处理。
- 验收标准：
  - [x] `REQUIREMENT-INTAKE.md` 定义 Story 类型、BDD 适用规则与等价技术验收规则。
  - [x] `ITERATION-WORKFLOW.md` 说明 Evolith iteration 与传统 Sprint 的映射和差异。
  - [x] `DOC-CHECK.md` 能检查 Story / BDD / 技术验收的一致性。
  - [x] `ITERATION-TEMPLATE.md` 在计划验收和闭环台账中承接 BDD 适用性。
  - [x] `AGENTS.md` 入口约束和 `EVOLUTION.md` 经验记录覆盖该方法论。
  - [x] `agent-project-governance` skill 同步体现 Sprint / iteration / Story / BDD 适配方法。
  - [x] Markdown 链接检查和 `git diff --check` 通过。
- 依赖或阻塞：无；本故事为治理改进，可在不激活产品 planned iteration 的情况下实施。
- 影响范围：docs / external skill
- 最小验证方式：执行 Markdown 相对链接检查；`git diff --check`；运行 skill 结构校验。

### EVO-043 后端依赖全量版本审计与迁移

- 类型：tech-debt
- 优先级：P1
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：保持后端 workspace 依赖的时效性和安全性，减少技术债积累。
  - 我希望/需要：审计 `backend/Cargo.toml` 中 `[workspace.dependencies]` 的所有 crate，找到各依赖的最新稳定版本并迁移，确保编译和测试通过。
  - 以便：后续功能开发在最新依赖基线上进行，避免安全漏洞和 API 过时问题累积。
- 范围：
  - 审计 `[workspace.dependencies]` 中所有依赖的当前版本和最新稳定版本。
  - 逐个或分批升级依赖到最新稳定版本（优先升级补丁和小版本；大版本升级需确认破坏性变更）。
  - 重点关注：`actix-web`、`sqlx`、`tokio`、`serde`、`reqwest`、`argon2`、`jsonwebtoken`、`bollard`、`lettre`、`zip`（已升级到 8.x）等核心依赖。
  - 每次升级后运行 `cargo check --workspace`、`cargo clippy --workspace -- -D warnings`、`cargo test --workspace` 验证。
  - 记录每个依赖的升级决策：直接升级、需要适配、或暂缓（附原因）。
- 不做：
  - 不升级 Rust edition 或 MSRV。
  - 不升级前端依赖（npm/bun 包）。
  - 不修改业务逻辑以适配新 API（除非必要且改动最小）。
  - 不升级已 Deferred 的依赖（如 `mysql` 相关，当前不支持）。
- 验收标准：
  - 非行为类：
    - [ ] 所有 workspace 依赖已审计，版本对比表已记录。
    - [ ] 可安全升级的依赖已升级到最新稳定版。
    - [ ] `cargo check --workspace` 通过。
    - [ ] `cargo clippy --workspace -- -D warnings` 通过。
    - [ ] `cargo test --workspace` 通过。
    - [ ] 暂缓升级的依赖有明确的版本限制和原因记录。
- 技术备注：
  - 使用 `cargo outdated` 或手动 `cargo update` + `Cargo.lock` 检查。
  - 大版本升级（如 actix-web 4→5、sqlx 0.7→0.8）需先查阅 CHANGELOG 确认破坏性变更。
  - `zip` 已在 Iteration 030 中从 2.4.2 升级到 8.6.0。
  - `sqlx` 0.7 的 future-incompat 警告需要在升级时一并处理。
- 依赖或阻塞：无。
- 影响范围：backend（Cargo.toml + 可能的代码适配）
- 最小验证方式：`cargo test --workspace`；`cargo clippy --workspace -- -D warnings`。

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

### EVO-018 邮箱验证发送与确认闭环

- 类型：feature
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：完成用户生命周期最后一块——注册后可验证邮箱，确认邮箱真实性。与 forgot/reset password、invite/join 形成完整的认证闭环。
- 范围：
  - 实现 `send_verification_email` handler：查找用户 → 生成 crypto-random token → `set_verify_token` → 通过 Mailer 发送验证邮件（链接使用 `APP__PUBLIC_URL`）。
  - 实现 `verify_email` handler：`find_by_verify_token` → 校验 token → `verify_email`（设置 email_verified=true, 清除 token）→ 返回成功。
  - 防枚举：`send_verification_email` 无论邮箱是否存在都返回成功（与 forgot-password 一致）。
  - 已验证的邮箱不重复发送。
- 不做：
  - 不修改 DTO（`SendVerifyEmailRequest { email }`、`VerifyEmailRequest { token }` 已定义）。
  - 不修改 Repository trait 或数据库 migration（`verify_email`/`set_verify_token`/`find_by_verify_token` 已实现）。
  - 不修改 Mailer trait（`send_verification_email` 已实现）。
  - 不创建前端页面（验证链接指向已有或待建的 `/verify-email` 路由）。
- 验收标准：
  - [ ] `POST /api/v1/auth/send-verify` 接受邮箱，存在时发送验证邮件，不存在时静默返回成功。
  - [ ] `POST /api/v1/auth/verify-email` 接受 token，验证通过后设置 `email_verified=true`。
  - [ ] 验证邮件使用 `APP__PUBLIC_URL` 生成公开前端链接。
  - [ ] `cargo test -p api` 通过，含 send-verify / verify-email 集成测试。
  - [ ] `cargo check --workspace` 无错误。
- 技术备注：
  - DTO 已定义：`SendVerifyEmailRequest { email }`、`VerifyEmailRequest { token }`。
  - Repository 已实现：`verify_email(id)`、`set_verify_token(id, token)`、`find_by_verify_token(token)`。
  - Mailer 已实现：`send_verification_email(to, username, token, base_url)`。
  - 参考实现：`forgot_password` handler（同模式：查找用户→生成 token→发邮件→防枚举）。
- 依赖：Mailer（已有）。
- 影响范围：backend
- 最小验证方式：`cargo test -p api`；ConsoleMailer 输出 token 用于手工验证。
- 状态审计（2026-05-27）：代码与 Iteration 009 记录显示 handler / e2e 已存在，但
  `API-CONTRACT.md`、`TESTING.md` 与 roadmap 仍存在未实现/过期状态描述；按 EVO-040
  修复并重新核验之前，不恢复 `Done`。

### EVO-026 前端 Snippets 入口迁移为 CLI 友好接口

- 类型：product-change
- 优先级：P1
- 状态：Ready
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
