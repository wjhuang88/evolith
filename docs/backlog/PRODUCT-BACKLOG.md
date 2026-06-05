# Product Backlog

> Compact routing and prioritization surface. Executable context lives in `docs/backlog/active/`; completed, deferred and dropped history lives in `docs/backlog/archive/`.
> Status and DoR rules: [Requirement Intake](../sop/REQUIREMENT-INTAKE.md). Completion rules: [Iteration Workflow](../sop/ITERATION-WORKFLOW.md). Compaction protocol: `agent-project-governance/references/backlog-compaction.md`.

## Current Priorities

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-080 | Spike: 验证 Wasmer/WASI 替代 Docker sandbox 可行性 | Ready | P1 | 用户反馈 2026-06-05 / Docker 依赖反思; 评估 Wasmer/Wasmtime/WASI 能否承接 Skill/CLI/MCP 执行；输出 ADR 或提案更新，不直接替换运行时 | [Item file](active/EVO-080-spike-验证-wasmer-wasi-替代-docker-sandbox-可行性.md)<br>[Serverless runtime proposal](../proposals/SERVERLESS-RUNTIME.md)<br>[Skill format](../reference/formats/SKILL-FORMAT.md)<br>[CLI interface format](../reference/formats/CLI-INTERFACE-FORMAT.md) |
| EVO-081 | 内部文档页面基于独立 Markdown 目录渲染 | Ready | P1 | 用户反馈 2026-06-05; 新增内部文档页面，文档源放独立 md 目录，前端根据目录渲染文档列表和详情 | [Item file](active/EVO-081-内部文档页面基于独立-markdown-目录渲染.md)<br>[Product requirements](../reference/product/REQUIREMENTS.md) |
| EVO-057 | 生产 CORS Origin 可配置化 | Ready | P2 | 跨层一致性审查 2026-06-01; docker-compose.prod.yml 的 `CORS__ALLOWED_ORIGINS` 被忽略，main.rs 硬编码 origin；新增 CorsConfig 使其可配置 | [Item file](active/EVO-057-生产-cors-origin-可配置化.md)<br>[Config reference](../reference/CONFIG.md) |

## Active Items

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-012 | 租户设置保存 | Proposed | P2 | 前端 TODO; tenant settings | [Item file](active/EVO-012-租户设置保存.md) |
| EVO-013 | Audit log detail 接口 | Proposed | P2 | API 501; `GET /audit-logs/{log_id}` | [Item file](active/EVO-013-audit-log-detail-接口.md) |
| EVO-014 | Stripe webhook 恢复 | Proposed | P2 | routes TODO; 计费闭环 | [Item file](active/EVO-014-stripe-webhook-恢复.md) |
| EVO-019 | Skill registry 服务化 | Proposed | P2 | Phase E placeholder; 将 `service-skill/src/registry.rs` 从 placeholder 补成可复用注册能力 | [Item file](active/EVO-019-skill-registry-服务化.md) |
| EVO-020 | Storage 能力落地 | Proposed | P2 | Phase E placeholder; 实现对象存储基础能力，支撑技能包和附件 | [Item file](active/EVO-020-storage-能力落地.md) |
| EVO-027 | Skill 多来源创建 | Proposed | P1 | 用户需求 / Agent Skills spec; 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 | [Item file](active/EVO-027-skill-多来源创建.md) |
| EVO-028 | Skill 版本管理与正确性验证 | Proposed | P1 | 用户需求 / Agent Skills spec; 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 | [Item file](active/EVO-028-skill-版本管理与正确性验证.md) |
| EVO-029 | Skill 专业描述与发现质量提升 | Proposed | P2 | Agent Skills spec; 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 | [Item file](active/EVO-029-skill-专业描述与发现质量提升.md) |
| EVO-044 | 前端 CLI 命名简化 | Proposed | P1 | 用户反馈 2026-05-29; 导航、页面、i18n 中 "CLI Interfaces" / "CLI 接口" 统一简化为 "CLI" | [Item file](active/EVO-044-前端-cli-命名简化.md) |
| EVO-045 | CLI 命令执行引擎（Serverless） | Proposed | P0 | 用户反馈 2026-05-29; CLI 从纯文本记录升级为可执行命令接口，支持 serverless 执行环境或外部执行信息记录 | [Item file](active/EVO-045-cli-命令执行引擎-serverless.md)<br>[Serverless runtime proposal](../proposals/SERVERLESS-RUNTIME.md) |
| EVO-046 | Skill 可下载制品与 Agent 一键安装 | Proposed | P1 | 用户反馈 2026-05-29; Skill 从服务端执行改为可下载制品（ClawHub 模式），支持搜索、下载和一键安装到 Agent 工作空间 | [Item file](active/EVO-046-skill-可下载制品与-agent-一键安装.md) |
| EVO-047 | MCP 工具 Serverless 执行 | Proposed | P1 | 用户反馈 2026-05-29; MCP 工具支持 serverless 执行环境或外部执行信息记录，与 CLI 共享执行基础设施 | [Item file](active/EVO-047-mcp-工具-serverless-执行.md)<br>[Serverless runtime proposal](../proposals/SERVERLESS-RUNTIME.md) |
| EVO-049 | Skill/CLI 生态兼容与行业标准对齐 | Proposed | P0 | 用户反馈 2026-05-29; Epic；不直接进迭代，先执行 EVO-049-A/B 子 Story | [Item file](active/EVO-049-skill-cli-生态兼容与行业标准对齐.md)<br>[Skill format](../reference/formats/SKILL-FORMAT.md)<br>[CLI interface format](../reference/formats/CLI-INTERFACE-FORMAT.md) |
| EVO-050 | Skill/CLI 评分与质量体系 | Proposed | P1 | 用户反馈 2026-05-29; 平台提供 Skill/CLI 组件的评分能力：用户评分、使用统计、质量评估，支撑生态发现和信任 | [Item file](active/EVO-050-skill-cli-评分与质量体系.md)<br>[Skill format](../reference/formats/SKILL-FORMAT.md)<br>[CLI interface format](../reference/formats/CLI-INTERFACE-FORMAT.md) |
| EVO-053 | 测试盲区补齐 | Proposed | P2 | 代码健康审查 2026-06-01; service-audit 零测试、8 个 repo 集成测试仅覆盖 SQLite，PostgreSQL repository 无集成测试；PG 测试基础设施待与 EVO-030 CI 协调 | [Item file](active/EVO-053-测试盲区补齐.md) |
| EVO-057 | 生产 CORS Origin 可配置化 | Ready | P2 | 跨层一致性审查 2026-06-01; docker-compose.prod.yml 的 `CORS__ALLOWED_ORIGINS` 被忽略，main.rs 硬编码 origin；新增 CorsConfig 使其可配置 | [Item file](active/EVO-057-生产-cors-origin-可配置化.md)<br>[Config reference](../reference/CONFIG.md) |
| EVO-061 | bollard 0.17→0.21 Docker Engine API 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; bollard 跨 4 个 minor 的 Docker Engine API 演进（0.18 起 `Docker::connect_with_*` API 调整）；当前 `bollard 0.17.1` + Phase 7 sandbox（service-skill）零运行时问题；迁移需重写 `service-skill/src/executor.rs` + 验证 sandbox 镜像兼容性 | [Item file](active/EVO-061-bollard-0-17-0-21-docker-engine-api.md) |
| EVO-062 | sqlx 0.7→0.8/0.9 迁移（解决 future-incompat） | Proposed | P1 | Iteration 032 依赖审计暂缓项; **19 个 repo 文件 / ~120 call sites** 依赖 `query!` / `query_as!` 宏与 `FromRow` derive；0.7→0.8 是 macro 与 `Database` trait 跨主版本破坏；已观测 `sqlx-postgres 0.7.4` never-type-fallback warning（Rust 2024 edition 强制前不阻断），是 Iteration 028 显式归口 | [Item file](active/EVO-062-sqlx-0-7-0-8-0-9-迁移.md) |
| EVO-063 | config 0.14→0.15 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.14→0.15 改 `ConfigBuilder` API（`with_source` → `add_source` 合并/语义变化）；影响 `infra/src/config.rs`；caret 范围不变 | [Item file](active/EVO-063-config-0-14-0-15-升级.md) |
| EVO-064 | validator 0.16→0.18+ 升级（trait 迁移） | Proposed | P2 | Iteration 032 依赖审计暂缓项; 0.18 起 `validate` 方法改成 `#[derive(Validate)]` + `validator::Validate` trait；9 个 domain 模型 + DTO 需适配；breaking 跨多个 minor | [Item file](active/EVO-064-validator-0-16-0-18-升级-trait-迁移.md) |
| EVO-065 | jsonwebtoken 9→10 升级 | Proposed | P2 | Iteration 032 依赖审计暂缓项; 9→10 改 `Header` / `EncodingKey` / `decode` 签名（with/without validation 合并）；影响 `service-auth/src/jwt.rs`；10.x 已稳定（10.4.0） | [Item file](active/EVO-065-jsonwebtoken-9-10-升级.md) |
| EVO-066 | thiserror 1→2 升级（需 edition 2024） | Proposed | P2 | Iteration 032 依赖审计暂缓项; 1→2 需 `edition = "2024"`；影响 `common/src/error.rs` + 8 个 service error 类型；与 EVO-043 「不升级 Rust edition」约束冲突，须先解除 edition 升级前置 | [Item file](active/EVO-066-thiserror-1-2-升级-需-edition-2024.md) |
| EVO-067 | jsonschema 0.17→0.46 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 跨 28 个 minor 的 schema draft / API 演进；`service-tool` 内部使用 0.17 即可；升级涉及 `validator` / `draft-7` / `draft-2020` API 切换 | [Item file](active/EVO-067-jsonschema-0-17-0-46-升级.md) |
| EVO-068 | reqwest 0.11→0.12/0.13 升级 | Proposed | P2 | Iteration 032 依赖审计暂缓项; 0.12 起移除 `blocking` 特性独立 crate；0.13 进一步改 `ClientBuilder` API；影响 `service-tool` HTTP executor + `service-payment` Stripe webhook | [Item file](active/EVO-068-reqwest-0-11-0-12-0-13-升级.md) |
| EVO-069 | hmac 0.12→0.13 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.13 改 `Mac::new_from_slice` 签名 + 移除 `SimpleHMac`；影响 `service-payment` Stripe webhook 验签 | [Item file](active/EVO-069-hmac-0-12-0-13-升级.md) |
| EVO-070 | sha2 0.10→0.11 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.11 改 `Sha256::new()` 返回 `CtOutput`（const-time）；`service-payment` + `infra` 影响 | [Item file](active/EVO-070-sha2-0-10-0-11-升级.md) |
| EVO-071 | redis 0.27→1.x 命名空间重置 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.x→1.0 是显式 breaking（trait 完全重写）；`service-payment` 暂未真实使用（infra 缓存层预留） | [Item file](active/EVO-071-redis-0-27-1-x-命名空间重置.md) |
| EVO-072 | actix-governor 0.6→0.7+ 升级 | Proposed | P2 | Iteration 032 依赖审计暂缓项; 0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；影响 `api/src/middleware/rate_limit.rs`；0.10 latest | [Item file](active/EVO-072-actix-governor-0-6-0-7-升级.md) |
| EVO-073 | actix-web-prom 0.8→0.9+ 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.9 起改 `PrometheusMetricsBuilder` API；metrics endpoint 路径需调整；0.10 latest | [Item file](active/EVO-073-actix-web-prom-0-8-0-9-升级.md) |
| EVO-074 | prometheus 0.13→0.14 升级 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 0.14 改 `Encoder` trait 签名；与 actix-web-prom 强耦合（EVO-073 同源） | [Item file](active/EVO-074-prometheus-0-13-0-14-升级.md) |
| EVO-075 | rand 0.8→0.9/0.10 升级（service-auth） | Proposed | P3 | Iteration 032 依赖审计暂缓项; rand 0.9/0.10 是 `Rng` trait 重组（major breaking）；影响 `service-auth`（crate 私有 dep，非 workspace） | [Item file](active/EVO-075-rand-0-8-0-9-0-10-升级.md) |
| EVO-076 | serde_yaml 0.9 → serde_yml / serde_norway 迁移 | Proposed | P3 | Iteration 032 依赖审计暂缓项; 上游 serde_yaml 0.9 已 deprecate；建议迁移到 `serde_yml`（社区 fork）或 `serde_norway`（纯 Rust 替代）；影响 `service-skill` + `service-snippet` 的 YAML 解析路径 | [Item file](active/EVO-076-serde-yaml-0-9-serde-yml-serde-norway.md) |
| EVO-080 | Spike: 验证 Wasmer/WASI 替代 Docker sandbox 可行性 | Ready | P1 | 用户反馈 2026-06-05 / Docker 依赖反思; 评估 Wasmer/Wasmtime/WASI 能否承接 Skill/CLI/MCP 执行；输出 ADR 或提案更新，不直接替换运行时 | [Item file](active/EVO-080-spike-验证-wasmer-wasi-替代-docker-sandbox-可行性.md)<br>[Serverless runtime proposal](../proposals/SERVERLESS-RUNTIME.md)<br>[Skill format](../reference/formats/SKILL-FORMAT.md)<br>[CLI interface format](../reference/formats/CLI-INTERFACE-FORMAT.md) |
| EVO-081 | 内部文档页面基于独立 Markdown 目录渲染 | Ready | P1 | 用户反馈 2026-06-05; 新增内部文档页面，文档源放独立 md 目录，前端根据目录渲染文档列表和详情 | [Item file](active/EVO-081-内部文档页面基于独立-markdown-目录渲染.md)<br>[Product requirements](../reference/product/REQUIREMENTS.md) |

## Blocked Items

| ID | Title | Status | Priority | Decision Context | Required Reads |
| --- | --- | --- | --- | --- | --- |
| EVO-049-B | Skill/CLI parser 接线与校验报告 | Blocked | P0 | EVO-049 split; 依赖 EVO-049-A；将 parser 接入创建/更新并输出 blocking error / warning | [Item file](active/EVO-049-B-skill-cli-parser-接线与校验报告.md)<br>[Skill format](../reference/formats/SKILL-FORMAT.md)<br>[CLI interface format](../reference/formats/CLI-INTERFACE-FORMAT.md) |

## Archived Index

| ID | Title | Status | Priority | Decision Context | Archive |
| --- | --- | --- | --- | --- | --- |
| EVO-001 | 前后端 API 对齐 | Done | P0 | [实施路线图 Phase A](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-a--现状校准与-api-对齐done); 修正 method、字段和不存在的前端 API 调用；snippet 范围已转入 EVO-017 | [2026-Q2](archive/2026-Q2/EVO-001-前后端-api-对齐.md) |
| EVO-002 | 前端迁移到 React + Vite + Bun | Done | P0 | [实施路线图 Phase B](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0--高优先级); EVO-021 至 EVO-025 全部完成 | [2026-Q2](archive/2026-Q2/EVO-002-前端迁移到-react-+-vite-+-bun.md) |
| EVO-003 | 忘记密码与重置密码闭环 | Done | P0 | Iteration 005; handler 实现 + 前端 API 接入，Playwright 验证通过 | [2026-Q2](archive/2026-Q2/EVO-003-忘记密码与重置密码闭环.md) |
| EVO-004 | 邀请接受 / Join 流程 | Done | P0 | Iteration 005; handler 实现，cargo test 通过 | [2026-Q2](archive/2026-Q2/EVO-004-邀请接受-join-流程.md) |
| EVO-005 | MCP 工具真实执行 | Done | P0 | 需求 F1.1.3 / Iteration 006; HTTP handler type 真实执行，Function 类型暂不支持 | [2026-Q2](archive/2026-Q2/EVO-005-mcp-工具真实执行.md) |
| EVO-006 | Skill 更新接口 | Done | P1 | API 501 / Iteration 010; `PUT /skills/{id}` 已实现 | [2026-Q2](archive/2026-Q2/EVO-006-skill-更新接口.md) |
| EVO-007 | Snippet 更新接口 | Deferred | P1 | API 501; 被 EVO-017 替代方向覆盖，暂停继续投入 | [2026-Q2](archive/2026-Q2/EVO-007-snippet-更新接口.md) |
| EVO-008 | Snippet reference 格式增强 | Deferred | P1 | 需求 F1.3.4; 被 EVO-017 替代方向覆盖 | [2026-Q2](archive/2026-Q2/EVO-008-snippet-reference-格式增强.md) |
| EVO-009 | SKILL.md 与 CLI interface frontmatter parser | Done | P1 | 格式规范 / EVO-017 / Iteration 010; CLI interface parser 已有基线；SKILL.md parser 已实现 | [2026-Q2](archive/2026-Q2/EVO-009-skillmd-与-cli-interface-frontmatter-parser.md) |
| EVO-010 | 租户 Members 页面接真实 API | Done | P1 | Iteration 011; 后端 find_by_tenant + remove_from_tenant + 前端接线 | [2026-Q2](archive/2026-Q2/EVO-010-租户-members-页面接真实-api.md) |
| EVO-011 | API Key 页面接真实 API | Done | P1 | Iteration 011; types + api module + page 重写 | [2026-Q2](archive/2026-Q2/EVO-011-api-key-页面接真实-api.md) |
| EVO-015 | Rust CLI 子项目 | Deferred | P3 | [提案](../proposals/RUST-CLI.md); API 稳定后启动 | [2026-Q2](archive/2026-Q2/EVO-015-rust-cli-子项目.md) |
| EVO-016 | 前端嵌入后端发布物 | Done | P3 | [提案](../proposals/EMBEDDED-FRONTEND.md); EVO-016-B 已由 Iteration 031 完成；Docker 单容器细化如需继续另拆 | [2026-Q2](archive/2026-Q2/EVO-016-前端嵌入后端发布物.md) |
| EVO-016-A | Embedded Frontend 交付形态 refinement | Deferred | P2 | EVO-016 split / Iteration 024; 已被 Iteration 031 的 rust-embed-for-web 实施覆盖，不再单独激活 | [2026-Q2](archive/2026-Q2/EVO-016-A-embedded-frontend-交付形态-refinement.md) |
| EVO-016-B | 前端静态服务迁移到 rust-embed-for-web | Done | P0 | EVO-016 split / 用户需求 / Iteration 031; 替代 ZIP 方案：用 rust-embed-for-web 实现零拷贝 + 预压缩 + 自动缓存协商；14 项验收标准全部通过 | [2026-Q2](archive/2026-Q2/EVO-016-B-前端静态服务迁移到-rust-embed-for-web.md) |
| EVO-017 | Snippet 迁移为 CLI 友好接口 | Done | P0 | [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md); Iteration 002；replaces EVO-007/EVO-008；已建立 CLI interface 格式、API 兼容契约、parser 基线和迁移盘点 | [2026-Q2](archive/2026-Q2/EVO-017-snippet-迁移为-cli-友好接口.md) |
| EVO-018 | 邮箱验证发送与确认闭环 | Done | P1 | Iteration 009; Handler 与测试存在，Iteration 021 完成 contract/testing/roadmap 收口 | [2026-Q2](archive/2026-Q2/EVO-018-邮箱验证发送与确认闭环.md) |
| EVO-021 | 前端路由适配层 | Done | P0 | EVO-002 split; Iteration 003；已新增 `frontend/src/lib/router.tsx`，页面和共享组件不再直接导入 Next 路由模块 | [2026-Q2](archive/2026-Q2/EVO-021-前端路由适配层.md) |
| EVO-022 | Vite + Bun 构建骨架 | Done | P0 | EVO-002 split; Iteration 004；新增 Vite 入口、React Router 根路由和并行构建脚本 | [2026-Q2](archive/2026-Q2/EVO-022-vite-+-bun-构建骨架.md) |
| EVO-023 | 前端运行时配置迁移 | Done | P0 | EVO-002 split; Iteration 004；`src/lib/config.ts` 统一运行时环境变量，替换所有 `process.env` 引用 | [2026-Q2](archive/2026-Q2/EVO-023-前端运行时配置迁移.md) |
| EVO-024 | Docker / Nginx 切换到静态 SPA | Done | P0 | EVO-002 split; Iteration 004；过渡部署形态，Dockerfile 改为 Bun + Vite build + Nginx 静态服务，SPA fallback | [2026-Q2](archive/2026-Q2/EVO-024-docker-nginx-切换到静态-spa.md) |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | Done | P0 | EVO-002 split; Iteration 004；删除 next 依赖、App Router、middleware、config；router.tsx 改为 React Router | [2026-Q2](archive/2026-Q2/EVO-025-移除-nextjs-依赖和遗留入口.md) |
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | Done | P1 | EVO-017 / 页面残留 / Iteration 017; Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 | [2026-Q2](archive/2026-Q2/EVO-026-前端-snippets-入口迁移为-cli-友好接口.md) |
| EVO-030 | GitHub CI/CD 重建 | Done | P2 | EVO-002 split / 工程收尾; Iteration 029 收口（2026-06-01）：`.github/workflows/ci.yml` 建立（tag-only `v*.*.*` semver trigger / 单 job 后端+前端串联 / Swatinem/rust-cache + oven-sh/setup-bun 缓存 / postgres:16-alpine service 容器）。9 门禁全绿：fmt ✓ / check ✓ / clippy ✓ / cargo test 274 passed。TECH-STACK §4.2 + TESTING §5 同步。deploy workflow / PR trigger 显式 Deferred | [2026-Q2](archive/2026-Q2/EVO-030-github-cicd-重建.md) |
| EVO-031 | Iteration 004/005 质量修复与流程防呆 | Done | P0 | 质量审查 2026-05-17; 修复静态资源反代、邀请接受闭环、公开接口放行、邮件公开 URL、sourcemap 默认关闭和流程规约 | [2026-Q2](archive/2026-Q2/EVO-031-iteration-004005-质量修复与流程防呆.md) |
| EVO-032 | Iteration 006 MCP 执行质量修复与流程防呆 | Done | P0 | 质量审查 2026-05-25; Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真 | [2026-Q2](archive/2026-Q2/EVO-032-iteration-006-mcp-执行质量修复与流程防呆.md) |
| EVO-033 | Rustfmt 全量格式基线与 stable 配置清理 | Done | P2 | Iteration 007 验证残余 / Iteration 028; 2026-06-01 完成；移除 7 个 nightly-only 配置，应用 stable rustfmt 重写 13 个文件；fmt/check/test 通过；clippy 1 个 pre-existing error 归口 EVO-059 | [2026-Q2](archive/2026-Q2/EVO-033-rustfmt-全量格式基线与-stable-配置清理.md) |
| EVO-034 | Epic 与子需求拆分治理规则 | Done | P1 | 流程缺口 2026-05-26; Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束 | [2026-Q2](archive/2026-Q2/EVO-034-epic-与子需求拆分治理规则.md) |
| EVO-035 | 治理 skill manifest 接入与一致性审计 | Done | P2 | Iteration 008 验证残余; Iteration 023；已建立 manifest 并通过 bundled validator | [2026-Q2](archive/2026-Q2/EVO-035-治理-skill-manifest-接入与一致性审计.md) |
| EVO-036 | 已发布迭代计划基线保护与改线防呆 | Done | P1 | 计划覆写复盘 2026-05-27; Iteration 013；修复 EVO-016 计划追踪并同步治理 skill | [2026-Q2](archive/2026-Q2/EVO-036-已发布迭代计划基线保护与改线防呆.md) |
| EVO-037 | 治理 skill 弱模型闭环执行防呆 | Done | P1 | 用户反馈 2026-05-27; Iteration 014；为初始化、迁移和修复任务增加强制闭环协议 | [2026-Q2](archive/2026-Q2/EVO-037-治理-skill-弱模型闭环执行防呆.md) |
| EVO-038 | 本项目实施任务闭环 SOP 与完成声明门禁 | Done | P1 | 用户反馈 2026-05-27; Iteration 015；将闭环协议落实到 Evolith 自身流程 | [2026-Q2](archive/2026-Q2/EVO-038-本项目实施任务闭环-sop-与完成声明门禁.md) |
| EVO-039 | 迭代启动前库存盘点与既有计划优先规则 | Done | P1 | 流程缺口 2026-05-27; Iteration 016；先处理在途/已规划迭代再选择新 story | [2026-Q2](archive/2026-Q2/EVO-039-迭代启动前库存盘点与既有计划优先规则.md) |
| EVO-040 | 已实现接口完成声明与参考文档状态修复 | Done | P1 | 排期库存审计 2026-05-27; Iteration 021；修复邮箱验证与 Skill 更新接口的收口漂移 | [2026-Q2](archive/2026-Q2/EVO-040-已实现接口完成声明与参考文档状态修复.md) |
| EVO-041 | 敏捷实践与 BDD 验收格式适配规则 | Done | P1 | 用户方法论反馈 2026-05-28; Iteration 022；明确 Evolith iteration 与传统 Sprint、Story 与 BDD 的适配口径 | [2026-Q2](archive/2026-Q2/EVO-041-敏捷实践与-bdd-验收格式适配规则.md) |
| EVO-042 | 编号保留位（缺号处置） | Dropped | — | Iteration 035 / 代码健康审查 2026-06-01; 缺号处置：原编号未被任何 story 占用，确认为 2026-05-29 EVO-043~050 跨 Epic 集中入池时跳跃（041 → 043），并非遗漏；保留为占位以维持 EVO 编号连续性语义。如需新增可复用此编号并标注 `replaces <空>` | [2026-Q2](archive/2026-Q2/EVO-042-编号保留位（缺号处置）.md) |
| EVO-043 | 后端依赖全量版本审计与迁移 | Done | P1 | Iteration 032; 2026-06-01 完成：36 个 workspace 功能依赖 + 3 个 path dep + 3 个 crate 私有 dep 完整审计；cargo check 0 / cargo test 274 pass；22 个保留（caret 已覆盖 latest stable，无需修改 Cargo.toml）；14 个 workspace 大版本升级 + 2 个 crate 私有 deprecation → EVO-061~076（16 个新 backlog 项全部 P3 Proposed）；clippy 18 errors 归口 EVO-059 不并入 | [2026-Q2](archive/2026-Q2/EVO-043-后端依赖全量版本审计与迁移.md) |
| EVO-045-A | ExecutionProvider 统一 trait + Docker 容器池化 | Done | P0 | EVO-045 split / EVO-048 输出 / Iteration 041; 2026-06-04 完成；2026-06-05 回归修复：sandbox 默认关闭，lite/local 启动不依赖 Docker，显式启用仍 fail-fast | [2026-Q2](archive/2026-Q2/EVO-045-A-executionprovider-统一-trait-+-docker-容器池化.md) |
| EVO-048 | Serverless 执行架构设计 Spike | Done | P0 | EVO-045/047 前置 / Iteration 033; Iteration 033 收口（2026-06-03）：输出 `docs/proposals/SERVERLESS-RUNTIME.md`（11 节）；Phase 7 sandbox 复用结论=部分复用；统一 ExecutionProvider 接口设计；冷启动方法学（已缓存 ~350ms-1800ms / 池模式 ~50-200ms，实测待 Docker）；Vercel 演进路径 + 组件替换清单；EVO-045/047 依赖图 + 推荐实施顺序；本机无 Docker，冷启动实测 conditional | [2026-Q2](archive/2026-Q2/EVO-048-serverless-执行架构设计-spike.md) |
| EVO-049-A | Skill/CLI 规范兼容数据模型基线 | Done | P0 | EVO-049 split / Iteration 034; 2026-06-04 完成：migration 006（skills +13 列 / snippets +8 列）；domain/DTO/repository 全量更新；SnippetRepository 新增 update 方法；282 tests passed（+8 新增） | [2026-Q2](archive/2026-Q2/EVO-049-A-skillcli-规范兼容数据模型基线.md) |
| EVO-051 | 误导性注释、命名与后端死代码清理 | Done | P2 | 代码健康审查 2026-06-01; Iteration 039 收口（2026-06-04）：删除 10 个死代码文件 + 6 个 mod 声明清理 + TODO 注释移除 + NewUser.password → password_hash 全量重命名（21 处）；282 tests passed | [2026-Q2](archive/2026-Q2/EVO-051-误导性注释、命名与后端死代码清理.md) |
| EVO-052 | MySQL 半接线收敛与快速失败 | Done | P2 | 代码健康审查 2026-06-01 / Iteration 038; 2026-06-04 完成：config validate + create_pool 对 mysql 快速失败；main.rs 后置 MySql 分支移除；CONFIG/EVOLUTION 同步 | [2026-Q2](archive/2026-Q2/EVO-052-mysql-半接线收敛与快速失败.md) |
| EVO-054 | backlog 状态漂移与编号一致性修复 | Done | P1 | 代码健康审查 2026-06-01 / Iteration 035; 2026-06-01 完成：EVO-016-B 详情块 In Progress → Done、EVO-026 详情块 Ready → Done（额外漂移）、EVO-042 缺号登记 Dropped 行；详情块 100% 与总表一致 | [2026-Q2](archive/2026-Q2/EVO-054-backlog-状态漂移与编号一致性修复.md) |
| EVO-055 | 前后端 API 契约漂移修复 | Done | P1 | 跨层一致性审查 2026-06-01 / Iteration 035; 2026-06-01 完成：删除 `membersApi.acceptInvitation` 死分支 / `billing/page.tsx` 加 `BILLING_ENABLED` 闸门 + 「待计费」静态页 / `cliInterfacesApi.update` 改 `throw new Error` 指向 create+delete / `API-CONTRACT.md` Billing 段补「待计费」标注 | [2026-Q2](archive/2026-Q2/EVO-055-前后端-api-契约漂移修复.md) |
| EVO-056 | 沙箱降级静默成功修复 | Done | P2 | 跨层一致性审查 2026-06-01 / Iteration 038; 2026-06-04 完成：沙箱启用时 Docker executor 初始化失败直接启动失败；DefaultSkillExecutor 返回 ConfigError，不再 exit_code:0 伪成功 | [2026-Q2](archive/2026-Q2/EVO-056-沙箱降级静默成功修复.md) |
| EVO-058 | 前端死代码与类型卫生清理 | Done | P2 | 前端代码审查 2026-06-01; Iteration 040 收口（2026-06-04）：删除 7 个死代码文件（-139 行）+ uiStore re-export 移除 + skillsApi.versions 假实现移除 + 重复 User 接口合并；bun run build + tsc 0 errors | [2026-Q2](archive/2026-Q2/EVO-058-前端死代码与类型卫生清理.md) |
| EVO-059 | Backend clippy 历史 lint 升级修复 | Done | P2 | Iteration 028 验证残余 / Iteration 029 配套; 2026-06-01 Iteration 029 收口：21 个 `-D warnings` 错误归零（原估算 18，实际 13× unwrap_used + 3× dead_code + 4× unnecessary_min_or_max + 1× field_reassign_with_default）。修复策略：unwarp_used 在 7 个 test 文件加文件级 `#![allow(clippy::unwrap_used)]`（workspace deny 覆盖 clippy.toml 行为）；dead_code 移除未使用字段而非 `#[allow]`；unnecessary_min_or_max 移除 `.max(3)` 因 MIN_RPM=30 保障 rpm/10>=3；field_reassign_with_default 改 struct update syntax | [2026-Q2](archive/2026-Q2/EVO-059-backend-clippy-历史-lint-升级修复.md) |
| EVO-060 | dev.sh EMBEDDED_FRONTEND/ZIP 死代码 + 关联 proposal 状态清理 | Done | P2 | 嵌入式模式验证 2026-06-01; 2026-06-03 完成：删除 4 处死代码（EMBEDDED_FRONTEND / build_frontend_zip / --features embedded-frontend / ZIP 构建逻辑）；新增 build_frontend() 函数；lite/embedded 模式改为单端口（build + backend）；后端端口改为读 SERVER__PORT 环境变量；proposal 状态已晋升；SCRIPTS-RELEASE-NOTES.md 同步 | [2026-Q2](archive/2026-Q2/EVO-060-devsh-embeddedfrontendzip-死代码-+-关联-proposal-状态清理.md) |
| EVO-077 | Governance board 派生运营视图 | Done | P1 | 用户反馈 2026-06-03 / Iteration 036; 按 agent-project-governance skill 标准新增 `docs/BOARD.md`，只汇总 owner docs 与 gate，不作为新状态源；验证通过并收口 | [2026-Q2](archive/2026-Q2/EVO-077-governance-board-派生运营视图.md) |
| EVO-078 | 最近开发任务治理漂移修复 | Done | P1 | 用户反馈 2026-06-04 / Iteration 037; 修复 Board / iterations README / Iteration 029 / Iteration 033 / 设计文档归类 / 文档断链漂移；补齐 EVO-045-A 子任务；记录近期 Figma 与脚本任务治理归口 | [2026-Q2](archive/2026-Q2/EVO-078-最近开发任务治理漂移修复.md) |
| EVO-079 | sandbox 默认启用导致本地启动依赖 Docker 回归修复 | Done | P1 | 用户反馈 2026-06-05 / Iteration 041 follow-up; 将 `sandbox.enabled`、`.env.development`、`.env.example` 默认改为 false；显式启用 sandbox 时仍保留 Docker fail-fast | [2026-Q2](archive/2026-Q2/EVO-079-sandbox-默认启用导致本地启动依赖-docker-回归修复.md) |
| EVO-082 | evolution feedback SOP 缺失导致治理 validator 失败修复 | Done | P1 | 本轮治理验证 2026-06-05; 补 `docs/sop/EVOLUTION-FEEDBACK.md` 并从 AGENTS/docs README 路由，恢复 governance validator | [2026-Q2](archive/2026-Q2/EVO-082-evolution-feedback-sop-缺失导致治理-validator-失败修复.md) |
| EVO-083 | skill 1.0.7 agent redirect 入口纠偏 | Done | P1 | 用户反馈 2026-06-05 / skill 更新; 补 `CLAUDE.md` / `GEMINI.md` 单行重定向入口，并同步 manifest 与文档地图 | [2026-Q2](archive/2026-Q2/EVO-083-skill-107-agent-redirect-入口纠偏.md) |
| EVO-084 | Backlog compaction 标准结构迁移 | Done | P1 | 用户反馈 2026-06-05 / skill backlog-compaction; 将 monolithic backlog 压缩为决策入口 + active item files + archive index；验证通过后收口 | [2026-Q2](archive/2026-Q2/EVO-084-backlog-compaction-标准结构迁移.md) |
| EVO-085 | Archive index 二次压缩与 item file 拆分纠偏 | Done | P1 | 用户反馈 2026-06-05; `archive/2026-Q2/INDEX.md` 不应承载全部历史详情，已拆为 per-item archive files 并保留短索引 | [2026-Q2](archive/2026-Q2/EVO-085-archive-index-二次压缩与-item-file-拆分纠偏.md) |

## Reading Rules

1. Read this file first for routing, priority, status and decision context.
2. Before implementation or prioritization, open every path in `Required Reads` for the target row.
3. Read archive files only when `Required Reads` points there, an item says it supersedes or depends on archive history, or the user asks for historical rationale.
4. Do not mark an item Ready when a governing ADR, spec, dependency or archive record is only implicit. Add it to `Required Reads` and the item file first.
5. Do not store long acceptance criteria or execution logs in this file; put them in item files or archive records.
