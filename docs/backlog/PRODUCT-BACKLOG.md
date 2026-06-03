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
| EVO-016 | 前端嵌入后端发布物 | tech-debt | P3 | Done | [提案](../proposals/EMBEDDED-FRONTEND.md) | EVO-016-B 已由 Iteration 031 完成；Docker 单容器细化如需继续另拆 |
| EVO-016-A | Embedded Frontend 交付形态 refinement | tech-debt | P2 | Deferred | EVO-016 split / Iteration 024 | 已被 Iteration 031 的 rust-embed-for-web 实施覆盖，不再单独激活 |
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
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | P1 | Done | EVO-017 / 页面残留 / Iteration 017 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| EVO-027 | Skill 多来源创建 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 |
| EVO-028 | Skill 版本管理与正确性验证 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 |
| EVO-029 | Skill 专业描述与发现质量提升 | feature | P2 | Proposed | Agent Skills spec | 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 |
| EVO-030 | GitHub CI/CD 重建 | tech-debt | P2 | Done | EVO-002 split / 工程收尾 | Iteration 029 收口（2026-06-01）：`.github/workflows/ci.yml` 建立（tag-only `v*.*.*` semver trigger / 单 job 后端+前端串联 / Swatinem/rust-cache + oven-sh/setup-bun 缓存 / postgres:16-alpine service 容器）。9 门禁全绿：fmt ✓ / check ✓ / clippy ✓ / cargo test 274 passed。TECH-STACK §4.2 + TESTING §5 同步。deploy workflow / PR trigger 显式 Deferred |
| EVO-031 | Iteration 004/005 质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-17 | 修复静态资源反代、邀请接受闭环、公开接口放行、邮件公开 URL、sourcemap 默认关闭和流程规约 |
| EVO-032 | Iteration 006 MCP 执行质量修复与流程防呆 | bug | P0 | Done | 质量审查 2026-05-25 | Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真 |
| EVO-033 | Rustfmt 全量格式基线与 stable 配置清理 | tech-debt | P2 | Done | Iteration 007 验证残余 / Iteration 028 | 2026-06-01 完成；移除 7 个 nightly-only 配置，应用 stable rustfmt 重写 13 个文件；fmt/check/test 通过；clippy 1 个 pre-existing error 归口 EVO-059 |
| EVO-034 | Epic 与子需求拆分治理规则 | tech-debt | P1 | Done | 流程缺口 2026-05-26 | Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束 |
| EVO-035 | 治理 skill manifest 接入与一致性审计 | tech-debt | P2 | Done | Iteration 008 验证残余 | Iteration 023；已建立 manifest 并通过 bundled validator |
| EVO-036 | 已发布迭代计划基线保护与改线防呆 | tech-debt | P1 | Done | 计划覆写复盘 2026-05-27 | Iteration 013；修复 EVO-016 计划追踪并同步治理 skill |
| EVO-037 | 治理 skill 弱模型闭环执行防呆 | tech-debt | P1 | Done | 用户反馈 2026-05-27 | Iteration 014；为初始化、迁移和修复任务增加强制闭环协议 |
| EVO-038 | 本项目实施任务闭环 SOP 与完成声明门禁 | tech-debt | P1 | Done | 用户反馈 2026-05-27 | Iteration 015；将闭环协议落实到 Evolith 自身流程 |
| EVO-039 | 迭代启动前库存盘点与既有计划优先规则 | bug | P1 | Done | 流程缺口 2026-05-27 | Iteration 016；先处理在途/已规划迭代再选择新 story |
| EVO-040 | 已实现接口完成声明与参考文档状态修复 | bug | P1 | Done | 排期库存审计 2026-05-27 | Iteration 021；修复邮箱验证与 Skill 更新接口的收口漂移 |
| EVO-041 | 敏捷实践与 BDD 验收格式适配规则 | tech-debt | P1 | Done | 用户方法论反馈 2026-05-28 | Iteration 022；明确 Evolith iteration 与传统 Sprint、Story 与 BDD 的适配口径 |
| EVO-042 | 编号保留位（缺号处置） | governance | — | Dropped | Iteration 035 / 代码健康审查 2026-06-01 | 缺号处置：原编号未被任何 story 占用，确认为 2026-05-29 EVO-043~050 跨 Epic 集中入池时跳跃（041 → 043），并非遗漏；保留为占位以维持 EVO 编号连续性语义。如需新增可复用此编号并标注 `replaces <空>` |
| EVO-043 | 后端依赖全量版本审计与迁移 | tech-debt | P1 | Done | Iteration 032 | 2026-06-01 完成：36 个 workspace 功能依赖 + 3 个 path dep + 3 个 crate 私有 dep 完整审计；cargo check 0 / cargo test 274 pass；22 个保留（caret 已覆盖 latest stable，无需修改 Cargo.toml）；14 个 workspace 大版本升级 + 2 个 crate 私有 deprecation → EVO-061~076（16 个新 backlog 项全部 P3 Proposed）；clippy 18 errors 归口 EVO-059 不并入 |
| EVO-044 | 前端 CLI 命名简化 | product-change | P1 | Proposed | 用户反馈 2026-05-29 | 导航、页面、i18n 中 "CLI Interfaces" / "CLI 接口" 统一简化为 "CLI" |
| EVO-045 | CLI 命令执行引擎（Serverless） | feature | P0 | Proposed | 用户反馈 2026-05-29 | CLI 从纯文本记录升级为可执行命令接口，支持 serverless 执行环境或外部执行信息记录 |
| EVO-046 | Skill 可下载制品与 Agent 一键安装 | product-change | P1 | Proposed | 用户反馈 2026-05-29 | Skill 从服务端执行改为可下载制品（ClawHub 模式），支持搜索、下载和一键安装到 Agent 工作空间 |
| EVO-047 | MCP 工具 Serverless 执行 | feature | P1 | Proposed | 用户反馈 2026-05-29 | MCP 工具支持 serverless 执行环境或外部执行信息记录，与 CLI 共享执行基础设施 |
| EVO-048 | Serverless 执行架构设计 Spike | spike | P0 | In Progress | EVO-045/047 前置 / Iteration 033 | Iteration 033 激活（2026-06-01）：设计本地版 serverless runtime 架构，复用 Phase 7 sandbox 基础设施，为远期 Vercel 完整模式铺路；本机无 Docker，冷启动实测 conditional，方法学必出、实测值标注为待 Docker 环境执行 |
| EVO-049 | Skill/CLI 生态兼容与行业标准对齐 | feature | P0 | Proposed | 用户反馈 2026-05-29 | Epic；不直接进迭代，先执行 EVO-049-A/B 子 Story |
| EVO-049-A | Skill/CLI 规范兼容数据模型基线 | tech-debt | P0 | Ready | EVO-049 split / Iteration 034 | SQLite/PostgreSQL、domain、DTO、repository 对齐 Agent Skills 与 CLI 结构化字段 |
| EVO-049-B | Skill/CLI parser 接线与校验报告 | feature | P0 | Blocked | EVO-049 split | 依赖 EVO-049-A；将 parser 接入创建/更新并输出 blocking error / warning |
| EVO-050 | Skill/CLI 评分与质量体系 | feature | P1 | Proposed | 用户反馈 2026-05-29 | 平台提供 Skill/CLI 组件的评分能力：用户评分、使用统计、质量评估，支撑生态发现和信任 |
| EVO-051 | 误导性注释、命名与后端死代码清理 | tech-debt | P2 | Ready | 代码健康审查 2026-06-01 | 删除 `// TODO: Hash password` 误导注释、`NewUser.password`→`password_hash`、修正 db/sqlite.rs 与 db/postgres.rs 失效占位注释，并删除未接线的 guards.rs/registry/reference/search/error.rs 后端死代码；纯清晰度，无行为变更 |
| EVO-052 | MySQL 半接线收敛与快速失败 | tech-debt | P2 | Ready | 代码健康审查 2026-06-01 | config/pool 接受 `mysql` 但 main.rs 连接池后才拒绝；改为配置解析期快速失败或移除半接线，消除"看似可用"陷阱 |
| EVO-053 | 测试盲区补齐 | tech-debt | P2 | Proposed | 代码健康审查 2026-06-01 | service-audit 零测试、8 个 repo 集成测试仅覆盖 SQLite，PostgreSQL repository 无集成测试；PG 测试基础设施待与 EVO-030 CI 协调 |
| EVO-054 | backlog 状态漂移与编号一致性修复 | bug | P1 | Done | 代码健康审查 2026-06-01 / Iteration 035 | 2026-06-01 完成：EVO-016-B 详情块 In Progress → Done、EVO-026 详情块 Ready → Done（额外漂移）、EVO-042 缺号登记 Dropped 行；详情块 100% 与总表一致 |
| EVO-055 | 前后端 API 契约漂移修复 | bug | P1 | Done | 跨层一致性审查 2026-06-01 / Iteration 035 | 2026-06-01 完成：删除 `membersApi.acceptInvitation` 死分支 / `billing/page.tsx` 加 `BILLING_ENABLED` 闸门 + 「待计费」静态页 / `cliInterfacesApi.update` 改 `throw new Error` 指向 create+delete / `API-CONTRACT.md` Billing 段补「待计费」标注 |
| EVO-056 | 沙箱降级静默成功修复 | bug | P2 | Ready | 跨层一致性审查 2026-06-01 | Docker executor 初始化失败降级 DefaultSkillExecutor 返回 exit_code:0+空输出，与成功 no-op 无法区分；改为明确错误态或启动期 fail-fast |
| EVO-057 | 生产 CORS Origin 可配置化 | tech-debt | P2 | Ready | 跨层一致性审查 2026-06-01 | docker-compose.prod.yml 的 `CORS__ALLOWED_ORIGINS` 被忽略，main.rs 硬编码 origin；新增 CorsConfig 使其可配置 |
| EVO-058 | 前端死代码与类型卫生清理 | tech-debt | P2 | Ready | 前端代码审查 2026-06-01 | 删除 src/types/ 重复死类型、uiStore、accept-invitation 冗余 re-export；修不安全 as cast、root! 断言、skillsApi.versions 假实现；纯清晰度 |
| EVO-059 | Backend clippy 历史 lint 升级修复 | tech-debt | P2 | Done | Iteration 028 验证残余 / Iteration 029 配套 | 2026-06-01 Iteration 029 收口：21 个 `-D warnings` 错误归零（原估算 18，实际 13× unwrap_used + 3× dead_code + 4× unnecessary_min_or_max + 1× field_reassign_with_default）。修复策略：unwarp_used 在 7 个 test 文件加文件级 `#![allow(clippy::unwrap_used)]`（workspace deny 覆盖 clippy.toml 行为）；dead_code 移除未使用字段而非 `#[allow]`；unnecessary_min_or_max 移除 `.max(3)` 因 MIN_RPM=30 保障 rpm/10>=3；field_reassign_with_default 改 struct update syntax |
| EVO-060 | dev.sh EMBEDDED_FRONTEND/ZIP 死代码 + 关联 proposal 状态清理 | tech-debt | P2 | Ready | 嵌入式模式验证 2026-06-01 | `scripts/dev.sh` 残留 4 处 `EMBEDDED_FRONTEND` / `build_frontend_zip` / `frontend.zip` / `--features embedded-frontend` 死代码（实际后端用 `#[folder]` 嵌入，无需 zip/feature flag）；`docs/proposals/EMBEDDED-FRONTEND.md` 状态 `远期目标` → `已晋升`（Iteration 031 已用 rust-embed-for-web 实现等价目标）；`--features embedded-frontend` 在 Cargo.toml 中未定义，调用会报 unknown feature |
| EVO-061 | bollard 0.17→0.21 Docker Engine API 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | bollard 跨 4 个 minor 的 Docker Engine API 演进（0.18 起 `Docker::connect_with_*` API 调整）；当前 `bollard 0.17.1` + Phase 7 sandbox（service-skill）零运行时问题；迁移需重写 `service-skill/src/executor.rs` + 验证 sandbox 镜像兼容性 |
| EVO-062 | sqlx 0.7→0.8/0.9 迁移（解决 future-incompat） | tech-debt | P1 | Proposed | Iteration 032 依赖审计暂缓项 | **19 个 repo 文件 / ~120 call sites** 依赖 `query!` / `query_as!` 宏与 `FromRow` derive；0.7→0.8 是 macro 与 `Database` trait 跨主版本破坏；已观测 `sqlx-postgres 0.7.4` never-type-fallback warning（Rust 2024 edition 强制前不阻断），是 Iteration 028 显式归口 |
| EVO-063 | config 0.14→0.15 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.14→0.15 改 `ConfigBuilder` API（`with_source` → `add_source` 合并/语义变化）；影响 `infra/src/config.rs`；caret 范围不变 |
| EVO-064 | validator 0.16→0.18+ 升级（trait 迁移） | tech-debt | P2 | Proposed | Iteration 032 依赖审计暂缓项 | 0.18 起 `validate` 方法改成 `#[derive(Validate)]` + `validator::Validate` trait；9 个 domain 模型 + DTO 需适配；breaking 跨多个 minor |
| EVO-065 | jsonwebtoken 9→10 升级 | tech-debt | P2 | Proposed | Iteration 032 依赖审计暂缓项 | 9→10 改 `Header` / `EncodingKey` / `decode` 签名（with/without validation 合并）；影响 `service-auth/src/jwt.rs`；10.x 已稳定（10.4.0） |
| EVO-066 | thiserror 1→2 升级（需 edition 2024） | tech-debt | P2 | Proposed | Iteration 032 依赖审计暂缓项 | 1→2 需 `edition = "2024"`；影响 `common/src/error.rs` + 8 个 service error 类型；与 EVO-043 「不升级 Rust edition」约束冲突，须先解除 edition 升级前置 |
| EVO-067 | jsonschema 0.17→0.46 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 跨 28 个 minor 的 schema draft / API 演进；`service-tool` 内部使用 0.17 即可；升级涉及 `validator` / `draft-7` / `draft-2020` API 切换 |
| EVO-068 | reqwest 0.11→0.12/0.13 升级 | tech-debt | P2 | Proposed | Iteration 032 依赖审计暂缓项 | 0.12 起移除 `blocking` 特性独立 crate；0.13 进一步改 `ClientBuilder` API；影响 `service-tool` HTTP executor + `service-payment` Stripe webhook |
| EVO-069 | hmac 0.12→0.13 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.13 改 `Mac::new_from_slice` 签名 + 移除 `SimpleHMac`；影响 `service-payment` Stripe webhook 验签 |
| EVO-070 | sha2 0.10→0.11 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.11 改 `Sha256::new()` 返回 `CtOutput`（const-time）；`service-payment` + `infra` 影响 |
| EVO-071 | redis 0.27→1.x 命名空间重置 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.x→1.0 是显式 breaking（trait 完全重写）；`service-payment` 暂未真实使用（infra 缓存层预留） |
| EVO-072 | actix-governor 0.6→0.7+ 升级 | tech-debt | P2 | Proposed | Iteration 032 依赖审计暂缓项 | 0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；影响 `api/src/middleware/rate_limit.rs`；0.10 latest |
| EVO-073 | actix-web-prom 0.8→0.9+ 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.9 起改 `PrometheusMetricsBuilder` API；metrics endpoint 路径需调整；0.10 latest |
| EVO-074 | prometheus 0.13→0.14 升级 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 0.14 改 `Encoder` trait 签名；与 actix-web-prom 强耦合（EVO-073 同源） |
| EVO-075 | rand 0.8→0.9/0.10 升级（service-auth） | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | rand 0.9/0.10 是 `Rng` trait 重组（major breaking）；影响 `service-auth`（crate 私有 dep，非 workspace） |
| EVO-076 | serde_yaml 0.9 → serde_yml / serde_norway 迁移 | tech-debt | P3 | Proposed | Iteration 032 依赖审计暂缓项 | 上游 serde_yaml 0.9 已 deprecate；建议迁移到 `serde_yml`（社区 fork）或 `serde_norway`（纯 Rust 替代）；影响 `service-skill` + `service-snippet` 的 YAML 解析路径 |
| EVO-077 | Governance board 派生运营视图 | governance | P1 | Done | 用户反馈 2026-06-03 / Iteration 036 | 按 agent-project-governance skill 标准新增 `docs/BOARD.md`，只汇总 owner docs 与 gate，不作为新状态源；验证通过并收口 |


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
- 状态：Deferred
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
- 依赖或阻塞：已被 EVO-016-B / Iteration 031 的 rust-embed-for-web 实施覆盖；不再单独激活。
- 影响范围：docs / deploy / backend / frontend
- 最小验证方式：Markdown 链接检查；`git diff --check`；必要时只读检查当前 Docker/Nginx/backend route 配置。

### EVO-016-B 前端静态服务迁移到 rust-embed-for-web

- 类型：tech-debt
- 优先级：P0
- 状态：Done
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
  - 当时不解除 Iteration 012 的 EVO-016 前置阻塞，也不实施 embedded frontend。
  - 不将外部 skill 仓库的独立修改纳入 Evolith 仓库提交。
- 验收标准：
  - [x] `START-ITERATION.md` 明确迭代库存盘点先于 backlog story 选择。
  - [x] `AGENTS.md`、迭代工作流、目录说明与文档检查均能防止绕过现有未完成计划。
  - [x] Iteration 010 的故事完成事实与迭代收口缺口均可追溯，Iteration 012 的当时阻塞保留。
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

1. `EVO-033` Rustfmt 全量格式基线与 stable 配置清理（Iteration 028）。
2. `EVO-043` 后端依赖全量版本审计与迁移（Iteration 032）。
3. `EVO-030` GitHub CI/CD 重建（Iteration 029，依赖 028；建议在 043 后执行以减少 CI 返工）。
4. `EVO-048` Serverless 执行架构设计 Spike（Iteration 033）。
5. `EVO-049-A` Skill/CLI 规范兼容数据模型基线（Iteration 034）。

理由：先稳定工程门禁和依赖基线，再重建 CI；随后进入 Serverless 与 Skill/CLI 生态主线，避免在执行引擎和制品化前过早固定数据模型。

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
- 用户价值或技术目标：让当时生产部署形态匹配 Vite 静态 SPA，避免继续依赖 Next standalone runtime。该 Nginx 托管静态资源方案是 EVO-016 前的过渡策略；当前已由 EVO-016-B / Iteration 031 的 `rust-embed-for-web` 嵌入方案替代。
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
- 状态：Done
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

### EVO-044 前端 CLI 命名简化

- 类型：product-change
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：缩短导航和页面中的 "CLI Interfaces" / "CLI 接口" 为 "CLI"，降低认知负担，与 Tools / Skills 保持同级简洁度。
- 验收标准：
  - [ ] 导航栏、页面标题、面包屑、空状态、按钮中 "CLI Interfaces" / "CLI 接口" 统一为 "CLI"
  - [ ] i18n 两个 locale 文件同步更新
  - [ ] `bun run build` 通过，无回归
- 依赖或阻塞：无
- 影响范围：frontend（i18n、Header、页面组件）
- 最小验证方式：`bun run build` 0 errors；视觉回归导航和页面标题
- 不做：不改后端 API 路径、不改数据模型字段名

### EVO-045 CLI 命令执行引擎（Serverless）

- 类型：feature
- 优先级：P0
- 状态：Proposed
- 用户价值或技术目标：将 CLI 从纯代码文本记录升级为可执行的命令接口，让用户在 Agent 工作空间中发现、调用和管理 CLI 命令，获得结构化输入输出和错误语义。
- 核心设计：
  1. **结构化命令定义**：command、subcommands、参数 schema（类型、必填、默认值、枚举）、output schema、error model
  2. **Serverless 执行引擎**：
     - 本地版（近期）：复用 Phase 7 Docker sandbox 基础设施，按需启动容器执行命令，支持冷启动优化
     - Vercel 完整模式（远期）：函数部署、自动扩缩、按调用计费
  3. **外部执行支持**：CLI 也可仅记录外部执行 URL（如用户自建的 serverless 函数），平台做代理调用
  4. **Agent 集成**：CLI 命令可被 MCP tool 系统发现和调用，或通过独立 CLI endpoint 暴露
- 验收标准：
  - [ ] CLI 数据模型支持结构化命令定义（command、parameters、output schema、error model）
  - [ ] 后端提供 CLI 执行 endpoint（输入命令+参数 → 结构化结果 stdout/stderr/exit code）
  - [ ] 执行环境有资源限制（CPU/内存/超时）和安全隔离
  - [ ] 支持两种模式：平台 serverless 执行 + 外部执行 URL 代理
  - [ ] Agent 可通过 MCP tool 或 CLI endpoint 发现和调用已注册的 CLI 命令
  - [ ] 前端创建/编辑页支持结构化命令定义（不只是代码编辑器）
- 依赖或阻塞：EVO-048 Serverless 架构设计 Spike 完成；EVO-049 Phase 2（CLI 数据模型结构化字段就绪后执行引擎才能正确路由命令）
- 影响范围：backend（domain、service、API）/ frontend / db migration / docs
- 最小验证方式：后端集成测试覆盖命令注册+执行+调用全链路；前端创建页手工验证
- 不做：不实现本地 CLI 客户端（Rust CLI 另建 story）；不实现命令版本管理（EVO-028 范围）

### EVO-046 Skill 可下载制品与 Agent 一键安装

- 类型：product-change
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：Skill 从服务端 Docker sandbox 执行改为可下载制品（参考 ClawHub 模式），用户可搜索、下载 Skill 并一键安装到自己的 Agent 工作空间中使用。
- 核心设计：
  1. **制品市场**：Skill 作为可搜索、可下载的制品包（SKILL.md + 代码 + 资源文件），类似 npm package
  2. **一键安装**：用户选择 Skill → 安装到自己的 Agent 工作空间 → Agent 可在本地使用该 Skill
  3. **版本与来源追踪**：记录安装来源、版本、安装时间，支持更新检查
  4. **Docker sandbox 迁移**：Phase 7 的 sandbox 执行器从 Skill 服务中移除，挪给 CLI/MCP 使用
- 验收标准：
  - [ ] Skill 列表页支持搜索、筛选、预览制品详情
  - [ ] 提供 Skill 下载 API（打包为 ZIP 或 tar.gz）
  - [ ] 一键安装到 Agent 工作空间（记录安装关系，Agent 可引用已安装 Skill）
  - [ ] 已安装 Skill 列表管理（查看、更新、卸载）
  - [ ] Skill 服务移除 Docker sandbox 执行逻辑，sandbox 代码迁移到 CLI/MCP 服务
- 依赖或阻塞：EVO-049 Phase 3（Skill 制品打包 API 就绪后才能实现下载和安装）
- 影响范围：backend（service-skill 重构、新增安装服务）/ frontend / db migration / docs
- 最小验证方式：下载 API 返回有效制品包；安装后 Agent 工作空间可见已安装 Skill
- 不做：不实现 Skill 评分/评论（远期市场功能）；不实现 Skill 自动执行

### EVO-047 MCP 工具 Serverless 执行

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：MCP 工具支持 serverless 执行环境（与 CLI 共享基础设施），或仅记录外部执行 URL 由平台代理调用。
- 核心设计：
  1. **Serverless 执行**：MCP 工具处理函数部署到 serverless runtime，按需调用
  2. **外部执行代理**：MCP 工具记录外部 HTTP endpoint，平台做代理调用（当前 HTTP executor 增强）
  3. **与 CLI 共享基础设施**：复用 EVO-045/048 的 serverless runtime
- 验收标准：
  - [ ] MCP 工具支持 serverless 执行模式（工具处理函数部署到 runtime）
  - [ ] MCP 工具支持外部执行模式（记录 URL，代理调用）
  - [ ] 执行环境有资源限制和安全隔离
  - [ ] 与 CLI 共享 serverless runtime 基础设施
- 依赖或阻塞：EVO-045（CLI serverless）+ EVO-048（架构设计）
- 影响范围：backend（service-tool 增强）/ db migration / docs
- 最小验证方式：MCP tool 注册 + serverless 执行 + 调用全链路测试
- 不做：不实现 MCP 协议完整服务端（当前只做 tool 执行层）

### EVO-048 Serverless 执行架构设计 Spike

- 类型：spike
- 优先级：P0
- 状态：Ready
- 用户价值或技术目标：为 CLI（EVO-045）和 MCP（EVO-047）设计统一的 serverless 执行架构，确定本地版实现方案和远期 Vercel 模式的演进路径。
- 调研范围：
  1. **Phase 7 sandbox 复用评估**：当前 Docker sandbox（bollard + Python/Node runtime）能否作为 serverless 基础？冷启动延迟、资源开销、并发能力
  2. **本地 serverless runtime 设计**：
     - 函数部署模型：用户上传代码 → 构建镜像 → 按需启动容器
     - 冷启动优化：预热池、容器复用、快照恢复
     - 资源限制：CPU/内存/超时/PIDs（复用现有 sandbox 限制）
     - 并发模型：请求路由、实例扩缩
  3. **外部执行代理设计**：统一抽象层，内部 serverless 和外部 HTTP 调用使用相同接口
  4. **远期 Vercel 模式演进**：哪些组件需要替换（Docker → 真实 serverless 平台），哪些可以保留
- 验收标准：
  - [ ] 输出架构设计文档（ADR 或 proposal），包含本地版实现方案和演进路径
  - [ ] Phase 7 sandbox 复用可行性结论（复用 / 部分复用 / 替换）
  - [ ] 冷启动延迟基准测试（目标 < 2s）
  - [ ] 外部执行代理接口设计
  - [ ] 远期 Vercel 模式的组件替换清单
- 依赖或阻塞：无
- 影响范围：docs（ADR/proposal）
- 最小验证方式：设计文档通过 review；冷启动基准测试有数据
- 不做：不实现 serverless runtime（仅设计）；不实现 Vercel 完整模式

### EVO-049 Skill/CLI 生态兼容与行业标准对齐

- 类型：feature / Epic
- 优先级：P0
- 状态：Proposed
- 父 Epic：无
- Story 形态：Epic
- 用户价值或技术目标：让 Evolith 平台的 Skill 和 CLI 完全兼容 Agent Skills 行业规范（agentskills.io），使任何支持该规范的 Agent 工具（Claude Codex、OpenAI Codex 等）都能使用 Evolith 上的 Skill 和 CLI，成为大生态的一部分。

#### 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| EVO-049-A | Skill/CLI 规范兼容数据模型基线 | Ready | 无 | Iteration 034 |
| EVO-049-B | Skill/CLI parser 接线与校验报告 | Blocked | EVO-049-A | - |
| 后续切片 | 制品打包、导入、前端编辑体验 | Proposed | EVO-049-B / EVO-020 | - |

#### 行业标准兼容性现状

Evolith SKILL-FORMAT.md 在 Agent Skills 规范 6 个标准字段（name/description/license/compatibility/metadata/allowed-tools）上完全匹配。主要差距：

| 项目 | 当前状态 | 目标 |
|------|----------|------|
| `version`/`author`/`tags` | 顶层 frontmatter 字段（非标准） | 移入 `metadata` 下，顶层保留兼容别名 |
| `when_to_use` | 缺失 | 新增，Claude Code 扩展字段 |
| `arguments` | 缺失 | 新增，支持 `$0`/`$name` 位置参数替换 |
| 目录结构（scripts/references/assets） | 格式文档定义了，但存储只存 `skill_md` 单文本 | 支持完整目录打包和对象存储 |
| Frontmatter 解析器 | 已实现（`service-skill/parser.rs`、`service-snippet/parser.rs`）但未接线到 handler | 在创建/导入/更新流程中调用 |
| CLI 结构化字段 | 格式文档定义了（command/inputs/output/error_model），但模型只有 name/language/code/content | 数据模型新增所有 CLI 格式字段 |
| 制品打包/下载 | 无 | ZIP 打包下载 API，外部工具可直接使用 |

#### 实施子任务与顺序

**Phase 1：数据模型升级（前后端对齐）**

1. **DB migration — skills 表新增列**
   - SQLite + PostgreSQL 双轨 migration
   - 新增列：`author TEXT`、`tags JSON`、`skill_type TEXT DEFAULT 'instruction'`、`execution TEXT DEFAULT 'client'`、`entrypoint TEXT`、`timeout INTEGER DEFAULT 30`、`memory_mb INTEGER DEFAULT 256`、`permissions JSON`、`license TEXT`、`compatibility TEXT`、`disable_model_invocation BOOLEAN DEFAULT 0`、`user_invocable BOOLEAN DEFAULT 1`、`argument_hint TEXT`、`skill_package_path TEXT`
   - 现有 `skill_md` 列保留，存放完整 SKILL.md 原文；新列存放解析后的结构化字段

2. **DB migration — snippets 表新增列（CLI 字段）**
   - 新增列：`version TEXT`、`summary TEXT`、`command TEXT`、`subcommands JSON`、`inputs JSON`、`output JSON`、`examples JSON`、`error_model JSON`
   - 现有 `language`/`framework`/`code` 列保留兼容；`command` 替代 `code` 的语义角色

3. **Domain model 更新**
   - `Skill` 结构体：新增所有 migration 列对应的 Rust 字段
   - `NewSkill`/`UpdateSkill`：同步新增可选字段
   - `Snippet` 结构体：新增 CLI Interface 字段（version/summary/command/subcommands/inputs/output/examples/error_model）
   - `NewSnippet`/`UpdateSnippet`：同步新增

4. **DTO 更新**
   - `CreateSkillRequest`/`SkillResponse`：新增 author/tags/skill_type/execution/entrypoint/timeout/memory_mb/permissions/license/compatibility 等字段
   - `CreateSnippetRequest`/`SnippetResponse`：新增 version/summary/command/subcommands/inputs/output/examples/error_model
   - `SkillResponse`：移除硬编码空字符串的 `category`/`tags`，改为真实字段映射

5. **Repository 实现**
   - 4 个 repo 实现（SQLite skill/snippet + PostgreSQL skill/snippet）的 SQL 和映射同步更新

**Phase 2：Parser 接线与校验**

6. **Skill handler 接线 SkillParser**
   - `create_skill`：接收 content → `SkillParser::parse()` → 提取 metadata 字段 → 存入结构化列 + `skill_md` 存原文
   - `update_skill`：同上
   - `load_skill`/`execute_skill`：使用 `entrypoint` + `skill_package_path` 定位代码，不再把 `skill_md` 当代码执行

7. **Snippet handler 接线 CliInterfaceParser**
   - `create_snippet`：接收 content → `CliInterfaceParser::parse()` → 提取 metadata → 存入结构化列
   - `update_snippet`：实现（当前 501）

8. **Frontmatter 校验**
   - 创建和导入时校验：name 格式（小写+连字符，≤64字符，无连续/首尾连字符）、description 非空且 ≤1024字符、name 无 XML 标签、name 无保留词（anthropic/claude）
   - 对齐 OpenAI `quick_validate.py` 规则
   - 区分 blocking error 和 warning

**Phase 3：制品打包与导入**

9. **Skill 制品打包/下载 API**
   - `GET /api/v1/skills/{id}/package` → 返回 ZIP（SKILL.md + scripts/ + references/ + assets/ + src/）
   - ZIP 内目录名 = skill name，符合 Agent Skills 规范
   - 对象存储（MinIO/S3）存放 ZIP，DB 记录 `skill_package_path`

10. **Skill 导入 API**
    - `POST /api/v1/skills/import` — 接受 ZIP 上传或 Git URL
    - 解压/clone → 校验目录结构（必须含 SKILL.md）→ 解析 frontmatter → 存储
    - 返回导入报告（成功/校验问题/资源清单）

11. **CLI 制品打包/下载 API**
    - `GET /api/v1/snippets/{id}/package` → 返回 CLI Interface 描述文件（YAML frontmatter + Markdown body）

12. **CLI 导入 API**
    - `POST /api/v1/snippets/import` — 接受标准 CLI Interface 文件或 URL

**Phase 4：前端升级**

13. **Skill 创建/编辑表单升级**
    - 支持 runtime 选择（Python/Node/Wasm）
    - 支持 dependencies 编辑
    - 支持 SKILL.md 编辑器（带 frontmatter 高亮）
    - 支持 scripts/references/assets 文件管理（上传/预览）
    - 导入向导（ZIP 上传 / Git URL）

14. **CLI 创建/编辑表单升级**
    - 支持 command 输入
    - 支持 inputs schema 构建器（参数名、类型、必填、默认值、枚举值）
    - 支持 output schema 编辑
    - 支持 examples 编辑（command + input + output）
    - 支持 error_model 编辑（code + message + retryable）

#### 验收标准

- [ ] Skills 表包含 Agent Skills 规范所有标准字段的结构化列
- [ ] 创建 Skill 时 frontmatter 被解析并存入结构化字段（不再只存原文）
- [ ] Frontmatter 校验覆盖 name/description/license/compatibility 全部规则
- [ ] Skill 打包下载 API 返回符合 Agent Skills 规范的 ZIP
- [ ] 从 GitHub 导入 OpenAI skill-creator 和 cli-creator 成功
- [ ] Evolith 导出的 Skill 可被 Claude Codex / OpenAI Codex 正确识别
- [ ] CLI 数据模型包含 command、inputs、output、error_model、examples 结构化字段
- [ ] CLI 创建表单支持结构化命令定义（不只是代码编辑器）
- [ ] SQLite + PostgreSQL 双轨 migration 和 repository 同步更新
- [ ] `cargo test --workspace` 通过；`bun run build` 通过

- 依赖或阻塞：无（可与 EVO-046 并行推进 Phase 1/2）
- 影响范围：backend（domain model、DTO、service、handler、migration、storage）/ frontend（创建/编辑表单）/ docs（格式规范更新）
- 最小验证方式：导入 OpenAI skills 仓库中的 skill-creator 和 cli-creator 成功；导出 Skill 可被外部工具解析；全部测试通过
- 不做：不实现 Skill 执行环境迁移（EVO-046）；不实现 CLI 执行引擎（EVO-045）；不实现评分体系（EVO-050）

#### 依赖关系

```
EVO-049 Phase 1（数据模型）→ Phase 2（Parser 接线）→ Phase 3（打包/导入）→ Phase 4（前端）
                                                        ↗
                        EVO-046（Skill 制品化）           │
                        EVO-050（评分体系）────── 依赖 Phase 2 完成
```

### EVO-049-A Skill/CLI 规范兼容数据模型基线

- 类型：tech-debt
- 优先级：P0
- 状态：Ready
- 父 Epic：EVO-049
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：让 Skill 与 CLI interface 能承载 Agent Skills 规范和结构化 CLI 命令字段。
  - 我希望/需要：完成双数据库 migration、domain model、DTO 和 repository 映射的最小兼容基线。
  - 以便：后续 parser 接线、制品打包、导入和执行引擎不再建立在单文本字段或 legacy snippet 字段上。
- 范围：
  - SQLite 与 PostgreSQL 双轨 migration：为 skills 增加 Agent Skills 兼容字段，为 snippets/CLI interface 增加 command、inputs、output、examples、error_model 等结构化字段。
  - 更新 domain model、Create/Update 请求 DTO、Response DTO 和 SQLite/PostgreSQL repository 映射。
  - 保留 legacy `skill_md`、`language`、`framework`、`code` 等兼容字段，不在本轮删除旧 API。
  - 更新相关 reference 或 API contract 中的数据字段说明。
- 不做：
  - 不接线 parser 到 create/update handler；归 EVO-049-B。
  - 不实现 ZIP/Git 导入、制品下载或对象存储。
  - 不实现 CLI 执行引擎、MCP serverless 或评分体系。
  - 不改前端复杂编辑器，仅保证 API response 字段可承载后续 UI。
- 验收标准：
  - [ ] SQLite 与 PostgreSQL migration 字段语义一致，默认值和 nullable 策略明确。
  - [ ] `Skill` / `NewSkill` / `UpdateSkill` 与 CLI interface 域模型包含规范兼容字段。
  - [ ] API DTO 和 repository 映射能读写新增字段，旧字段兼容不破坏。
  - [ ] 双数据库 repository 测试覆盖新增字段的 create/update/read。
  - [ ] API contract 或 format reference 记录本轮新增字段和后续 parser 接线边界。
- 依赖或阻塞：无硬依赖；激活前需确认不与 EVO-027/EVO-028 已发布计划基线冲突。
- 解锁内容：EVO-049-B parser 接线与校验报告；EVO-045 CLI 执行引擎的数据路由前置。
- 影响范围：backend / db / docs
- 最小验证方式：`cargo test -p infra`；`cargo test -p domain`（如适用）；`cargo test -p api`；`cargo check --workspace`；`git diff --check`。

### EVO-049-B Skill/CLI parser 接线与校验报告

- 类型：feature
- 优先级：P0
- 状态：Blocked
- 父 Epic：EVO-049
- Story 形态：API / Technical
- 用户价值或技术目标：
  - 作为/为了：平台维护者和企业用户创建 Skill/CLI 组件时，需要及时得到结构兼容性反馈。
  - 我希望/需要：create/update/import 路径调用现有 parser，写入结构化字段，并返回 blocking error / warning 校验报告。
  - 以便：不合规组件不会静默进入可用状态，后续制品化和评分体系可复用同一报告。
- 范围：
  - `create_skill` / `update_skill` 接线 `SkillParser`，把 frontmatter 映射到 EVO-049-A 的结构化字段。
  - `create_snippet` / CLI interface update 路径接线 `CliInterfaceParser`。
  - 建立校验报告结构，区分 blocking error 与 warning。
  - 更新 API contract 和最小前端展示边界。
- 不做：
  - 不实现制品打包下载、ZIP/Git 导入或对象存储。
  - 不实现质量评分排序；归 EVO-050。
  - 不实现执行引擎；归 EVO-045。
- 验收标准：
  - [ ] 创建和更新 Skill 时 parser 被调用，结构化字段来自 frontmatter 而不是硬编码默认。
  - [ ] CLI interface 创建/更新能解析 command、inputs、output、examples、error_model。
  - [ ] 校验报告能返回 blocking error 和 warning，并在 API contract 中定义。
  - [ ] 不合规 name/description 返回明确错误；低质量描述只给 warning。
  - [ ] parser、handler 和 API 测试覆盖合法、非法、warning 样例。
- 依赖或阻塞：EVO-049-A 完成并验证。
- 解锁内容：EVO-049 后续制品打包/导入；EVO-050 质量评分；EVO-045 执行引擎。
- 影响范围：backend / frontend / docs
- 最小验证方式：`cargo test -p service-skill`；`cargo test -p service-snippet`；`cargo test -p api`；`cargo test --workspace`；`bun run type-check`（如前端有最小展示）。

### EVO-050 Skill/CLI 评分与质量体系

- 类型：feature
- 优先级：P1
- 状态：Proposed
- 用户价值或技术目标：平台提供 Skill/CLI 组件的评分和质量评估能力，支撑生态中的发现、信任和筛选。用户可对组件评分，平台自动评估质量指标。
- 核心设计：
  1. **用户评分**：1-5 星评分 + 可选文字评价，展示组件平均分和评价数量
  2. **使用统计**：下载量、安装量、调用频次，作为热门度排序信号
  3. **质量评估**（自动计算质量分数，0-100）：
     - **Frontmatter 完整性**（30 分）：必选字段 name+description 满分 20，每多一个可选字段（author/tags/license/compatibility）加 2.5
     - **Description 质量**（20 分）：非空 10 分，长度 > 50 字符 +5，同时包含"做什么"和"何时使用" +5
     - **资源丰富度**（30 分）：有 scripts/ +10，有 references/ +10，有 assets/ +5，有 examples +5
     - **结构合规**（20 分）：name 符合格式 +5，目录名匹配 +5，SKILL.md < 500 行 +5，文件引用正确 +5
  4. **信任信号**：是否经过验证、官方标记、安全扫描通过
  5. **排序/筛选**：列表页支持按评分、下载量、更新时间、质量分数排序
  6. **评分数据模型**：
     - `Rating`：id, user_id, component_id, component_type(skill/cli), score(1-5), comment, created_at
     - `ComponentStats`：component_id, component_type, avg_rating, rating_count, download_count, install_count, call_count, quality_score, updated_at
- 验收标准：
  - [ ] 组件详情页展示平均评分、评价数、下载量、质量分数
  - [ ] 用户可对已使用的组件提交 1-5 星评分和文字评价
  - [ ] 创建/导入时自动计算质量分数（frontmatter 完整性 + description 质量 + 资源丰富度 + 结构合规）
  - [ ] 列表页支持按评分/下载量/更新时间/质量分数排序
  - [ ] SQLite + PostgreSQL 双轨 migration
- 依赖或阻塞：EVO-049 Phase 2（Parser 接线后质量评估维度才准确）
- 影响范围：backend（新增 Rating/ComponentStats 模型、repository、handler）/ frontend（评分 UI、列表排序）/ db migration
- 最小验证方式：评分 API 创建和查询；质量评估对 OpenAI skill-creator 产出合理分数（预期 ≥ 80）
- 不做：不实现安全扫描（远期）；不实现付费推荐/置位（远期市场功能）

### EVO-051 误导性注释、命名与后端死代码清理

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：降低后续 Agent / 维护者误读代码语义的风险，并消除编译进产物但永不触达的死代码。
  - 维护者需要：清除会主动误导读者的注释和字段命名，并删除未接线的占位/桩模块，使代码表面与真实行为一致。
  - 以便：阅读 repository / 鉴权层时不会误判密码是否已哈希、不会误以为 SQLite/PostgreSQL repository 尚未实现、不会被永不调用的 registry/guards 桩误导。
- 范围（本次做）：
  1. 删除 `backend/crates/infra/src/db/user_repo.rs` 中 `// TODO: Hash password` 注释——密码已在 `api/src/handlers/auth/register.rs:67` 用 Argon2 哈希后传入。
  2. 将 `domain` 中 `NewUser.password` 重命名为 `password_hash`，同步所有引用（register/invite 等构造点与两套 repository 绑定）。
  3. 修正 `backend/crates/infra/src/db/sqlite.rs`、`db/postgres.rs` 中 "TODO: Implement ... repositories" 失效注释——repository 已作为独立文件实现。
  4. 删除未接线的后端死代码（grep 证实 api crate 零引用）：`api/src/guards.rs` 整模块（含硬编码 `dev_secret_key_for_testing_only` 回退，真实鉴权走 `middleware/rbac.rs`）、`service-skill/src/registry.rs`、`service-tool/src/registry.rs`、`service-snippet/src/repository.rs`、`service-snippet/src/reference.rs`、`service-snippet/src/search.rs`、`api/src/middleware/error.rs`；同步删除各 `lib.rs`/`mod.rs` 的对应 `pub mod` 声明。
- 不做：
  - 不动 `db/mysql.rs` 注释（MySQL 确实未实现，注释属实；接线收敛归 EVO-052）。
  - 不删除 `infra/src/storage.rs`、`service-auth/src/{rbac,session}.rs`、`service-tool/src/discovery.rs` 等"未来功能占位"（注释属实，非误导，保留）。
  - 不清理前端死代码（归 EVO-058）。
  - 不改任何运行时行为。
- 验收标准：
  - [ ] `rg "TODO: Hash password" backend/` 无结果。
  - [ ] `NewUser.password` 全仓无引用，`password_hash` 字段贯通构造点与 repository 绑定。
  - [ ] `db/sqlite.rs` / `db/postgres.rs` 不再出现 "not implemented / TODO: Implement repositories" 失效描述。
  - [ ] `guards.rs` / 三个 registry/repository 桩 / reference.rs / search.rs / error.rs 已删除，且对应 `mod` 声明清理干净。
  - [ ] `rg "dev_secret_key_for_testing_only" backend/` 无结果。
  - [ ] `cargo test --workspace` 与 `cargo clippy --workspace -- -D warnings` 通过（证明纯命名/注释/死代码删除未破坏行为）。
- 依赖或阻塞：无。
- 解锁内容：减少 EVO-049-A 数据模型改造时对密码字段语义的误读；缩小鉴权相关攻击面（删除硬编码 secret 死路径）。
- 影响范围：backend
- 最小验证方式：`cargo test --workspace`、`cargo clippy --workspace -- -D warnings`、`rg` 检查残留。

### EVO-052 MySQL 半接线收敛与快速失败

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：消除"配置接受、连接池建立成功、运行时才拒绝"的误导性接线陷阱。
  - 维护者/运维者需要：当 `DATABASE__DATABASE_TYPE=mysql` 时尽早得到清晰、确定的拒绝，而不是先连上池再 panic 式拒绝。
  - 以便：不会误以为 MySQL 已可用（`infra/src/config.rs`、`db/pool.rs` 均接受 mysql，但 `backend/src/main.rs:156` 才拒绝，无 repository 实现）。
- 范围（本次做）：
  1. 在配置解析或应用启动早期对 `mysql` 做 fail-fast，给出明确"MySQL 未实现，请使用 SQLite/PostgreSQL"的错误。
  2. 收敛 `db/pool.rs` 的 MySQL 分支与 `config.rs` 的接受逻辑二选一：要么移除半接线，要么集中到单一拒绝点。
  3. 同步 `EVOLUTION.md` 问题速查表（记录此陷阱）与 `docs/reference/CONFIG.md` MySQL 现状说明。
- 不做：
  - 不实现任何 MySQL repository。
  - 不改 SQLite / PostgreSQL 行为。
- 验收标准：
  - [ ] 设置 `DATABASE__DATABASE_TYPE=mysql` 启动时，在建立连接池之前即返回明确错误并退出。
  - [ ] 代码中不再存在"接受 mysql 但延后到 main.rs 才拒绝"的分裂路径。
  - [ ] `EVOLUTION.md` 速查表与 `CONFIG.md` 记录 MySQL 现状与处置。
  - [ ] `cargo test --workspace` 通过。
- 依赖或阻塞：无。
- 解锁内容：减少部署期对数据库支持范围的误判。
- 影响范围：backend / docs
- 最小验证方式：`DATABASE__DATABASE_TYPE=mysql ./target/.../evolith` 观察早期拒绝；`cargo test --workspace`。

### EVO-053 测试盲区补齐

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：降低无测试覆盖模块的回归风险，并让生产数据库（PostgreSQL）的 repository 行为获得集成验证。
  - 维护者需要：为 `service-audit`（当前零测试）补单元测试，并为 8 个 repository 增加 PostgreSQL 集成测试覆盖（当前 `infra/tests/` 全部走内存 SQLite）。
  - 以便：双数据库一致性不再仅靠 SQLite 侧验证。
- 范围（待 refinement 收敛）：
  1. `service-audit` 关键路径单元测试。
  2. PostgreSQL repository 集成测试基础设施（容器化 PG 或 CI service）与首批 repo 用例。
- 不做：
  - 不追求覆盖率数字指标，只补关键路径与双库差异点。
- 待澄清（阻止当前直接 Ready）：
  - PostgreSQL 集成测试运行形态（本地 testcontainers vs CI service）需与 EVO-030 CI 重建协调，避免重复搭建。
- 验收标准（草案，refinement 后定稿）：
  - [ ] `service-audit` 具备可执行单元测试且 `cargo test -p service-audit` 通过。
  - [ ] 至少核心 repository（user/tool/skill）具备针对 PostgreSQL 的集成测试。
- 依赖或阻塞：PostgreSQL 测试基础设施与 EVO-030（CI/CD 重建）协调。
- 解锁内容：提升双库改动（如 EVO-049-A）的回归信心。
- 影响范围：backend / db / deploy(CI)
- 最小验证方式：`cargo test -p service-audit`；PG 集成测试套件（基础设施确定后补全验证命令）。

### EVO-054 backlog 状态漂移与编号一致性修复

- 类型：bug
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance / Docs
- 用户价值或技术目标：
  - 为了：防止 backlog 总表与详情块状态不一致、编号断档导致 Agent 误判工作状态（本项目已有 EVO-040 同类漂移修复先例）。
  - 维护者需要：对齐 EVO-016-B 状态，并对 EVO-042 缺号给出明确处置记录。
  - 以便：backlog 作为可执行指令源保持自洽，符合 REQUIREMENT-INTAKE「总表 + 详情块一致」防呆规则。
- 范围（本次做）：
  1. 将 EVO-016-B 详情块（PRODUCT-BACKLOG.md 第 181 行）状态由 `In Progress` 改为 `Done`，与总表及 Iteration 031 收口结论一致。
  2. 登记 EVO-042 缺号处置：确认为有意跳过则在 backlog 加一行说明（status `Dropped`/说明），否则按需补占位。
  3. 按 DOC-CHECK 核对是否存在其他总表/详情块状态漂移。
- 不做：
  - 不改任何代码或运行时行为。
  - 不回填 EVO-042 为真实需求（仅做编号一致性处置）。
- 验收标准：
  - [ ] EVO-016-B 总表与详情块状态一致（均为 `Done`）。
  - [ ] EVO-042 在 backlog 有明确处置记录（跳过说明或占位）。
  - [ ] `rg "状态：In Progress" docs/backlog/PRODUCT-BACKLOG.md` 不再误标已完成项。
- 依赖或阻塞：无。
- 解锁内容：恢复 backlog 状态自洽，避免后续库存盘点误判。
- 影响范围：docs
- 最小验证方式：DOC-CHECK 一致性核对；总表与详情块逐项比对；`git diff --check`。

### EVO-055 前后端 API 契约漂移修复

- 类型：bug
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：API
- 用户故事：作为使用 SPA 的租户成员，我希望邀请接受、CLI 接口更新和计费相关操作不会因前端调用了后端不存在的路径而静默失败（404/501），以便这些功能真实可用。
- 背景（跨层一致性审查 2026-06-01 确认）：
  - `frontend/src/lib/api/members.ts:42` 调用 `POST /auth/accept-invite`，后端真实路由为 `POST /api/v1/invitations/accept`（`auth.ts:87` 已有正确实现，`membersApi.acceptInvitation` 为死分支）。
  - `frontend/src/lib/api/billing.ts` 与 `components/billing/PaymentMethods.tsx` 调用 8 个 billing/payment-method 端点（create/update/cancel subscription、invoices、payment-methods 全套），后端无对应路由。
  - `frontend/src/lib/api/cli-interfaces.ts:78` 调用 `PUT /snippets/{id}`，后端返回 501（`SnippetRepository` trait 无 `update`）。
- 范围（本次做）：
  1. 删除/修正 `membersApi.acceptInvitation` 死分支，统一走 `/invitations/accept`。
  2. 对无后端实现的 billing/payment-method 端点：在前端禁用入口并标注「待计费迭代」，或补后端 stub 路由返回结构化「未实现」——二选一，refinement 时定。
  3. CLI 接口更新：前端在 `PUT /snippets/{id}` 返回 501 时给出明确不可用提示；真实 update 能力归 EVO-049/后续。
  4. 同步 `docs/reference/API-CONTRACT.md`，纠正 skills list 等响应 shape 描述与真实路由。
- 不做：
  - 不实现真实计费业务逻辑（billing 仍为已知 stub，归计费迭代）。
  - 不实现 `SnippetRepository::update`（归 EVO-049/后续）。
- 验收标准：
  - [ ] 前端不存在调用 `/auth/accept-invite` 的代码路径；邀请接受走 `/invitations/accept` 并成功。
  - [ ] 前端不再向无后端实现的 billing/payment-method 端点发起请求（禁用或补 stub），无新增 404。
  - [ ] CLI 接口更新在不支持时给出明确提示，不再静默 501。
  - [ ] `docs/reference/API-CONTRACT.md` 与真实路由/响应 shape 一致。
- 依赖或阻塞：billing 端点处置方向需与计费迭代规划协调。
- 解锁内容：消除 SPA 关键流程的功能性 404，恢复邀请/接口可用性。
- 影响范围：frontend / backend(api) / docs
- 最小验证方式：手动走通邀请接受流程；`rg "/auth/accept-invite" frontend/` 无结果；前端构建通过；契约文档比对。

### EVO-056 沙箱降级静默成功修复

- 类型：bug
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：消除「Docker 沙箱初始化失败后静默降级，却返回成功态」的误导（AGENTS.md 陷阱 #4）。
  - 调用者需要：当 `SANDBOX__ENABLED=true` 但 Docker executor 不可用时，skill 执行返回明确的「沙箱不可用」错误，而非 `exit_code:0` + 空输出。
  - 以便：不会把基础设施失败误判为执行成功的 no-op。
- 背景：`backend/src/main.rs:67` 初始化失败仅 `warn!` 降级到 `DefaultSkillExecutor`；后者 `executor.rs:48` 返回 `exit_code:0`、空 stdout/stderr、`{"message":"...not implemented"}`，HTTP 200。
- 范围（本次做）：
  1. 二选一（refinement 定）：(a) `SANDBOX__ENABLED=true` 且 Docker 不可用时启动期 fail-fast；或 (b) 保留降级但让执行响应返回明确非成功状态（非 0 退出码 / 结构化错误 / 可观测告警）。
  2. 确保降级路径在日志与响应层都可区分于真实成功执行。
- 不做：
  - 不实现新的非 Docker 执行后端。
  - 不改 `SANDBOX__ENABLED=false` 的显式禁用语义。
- 验收标准：
  - [ ] Docker 不可用且沙箱启用时，skill 执行请求返回可被调用方识别的失败/不可用态（非伪成功 exit_code:0）。
  - [ ] 降级发生时有明确日志且响应可区分。
  - [ ] `cargo test --workspace` 通过；新增针对降级路径的测试。
- 依赖或阻塞：无。
- 解锁内容：提升 skill 执行结果可信度，避免沙箱故障被掩盖。
- 影响范围：backend(service-skill / main)
- 最小验证方式：模拟 Docker 不可用启动并触发 skill 执行，断言非伪成功响应；`cargo test -p service-skill`。

### EVO-057 生产 CORS Origin 可配置化

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：让生产 CORS 允许来源可通过环境变量配置，而非硬编码。
  - 运维者需要：`docker-compose.prod.yml:69` 设置的 `CORS__ALLOWED_ORIGINS` 真正生效。
  - 以便：部署到非 `evolith.io`/`app.evolith.io` 域名时无需改代码。
- 背景：`config.rs` 无 `CorsConfig` 结构；`main.rs:222` 生产 origin 硬编码为两个域名，环境变量被静默忽略。
- 范围（本次做）：
  1. 新增 `CorsConfig`（嵌套双下划线键 `CORS__ALLOWED_ORIGINS`，逗号分隔多 origin），并入 `AppConfig`。
  2. `main.rs` 生产分支按配置构建 CORS allowed origins；保留开发宽松策略。
  3. 同步 `docs/reference/CONFIG.md` 与 AGENTS.md 配置键说明（注意 AGENTS 现写的是单数 `CORS__ALLOWED_ORIGIN`，需统一）。
- 不做：
  - 不改 CSRF / 安全头中间件。
  - 不放宽开发模式策略。
- 验收标准：
  - [ ] 设置 `CORS__ALLOWED_ORIGINS` 后生产构建按其值放行，未设置时回退到安全默认。
  - [ ] `config.rs` 存在 `CorsConfig`，键格式为双下划线。
  - [ ] `CONFIG.md` 与 AGENTS.md 配置键描述一致（单复数统一）。
  - [ ] `cargo test --workspace` 通过。
- 依赖或阻塞：无。
- 解锁内容：生产多域名部署无需改代码。
- 影响范围：backend(infra/api) / docs
- 最小验证方式：设置不同 `CORS__ALLOWED_ORIGINS` 启动验证响应头；`cargo test --workspace`。

### EVO-058 前端死代码与类型卫生清理

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：消除前端未使用、与真实 API 类型冲突的死代码，并修复不安全的类型断言。
  - 维护者需要：删除从不被引用且与 `lib/api/types.ts` 定义冲突的类型，修正会在运行时抛错或静默产生非法值的断言。
  - 以便：类型系统反映真实 API 契约，减少误导与潜在运行时异常。
- 背景（前端代码审查 2026-06-01 确认）：
  - 死代码：`src/types/`（auth.ts/tool.ts/skill.ts/cli-interface.ts/index.ts，整目录未被引用且类型定义与真实 API 冲突）、`src/stores/uiStore.ts`（导出但无引用）、`src/app/accept-invitation/page.tsx`（冗余 re-export，路由已映射 JoinPage）。
  - 类型 holes：`lib/theme.tsx:18` 与 `hooks/usePermission.ts:20` 不安全 `as` cast、`main-spa.tsx:43` `getElementById('root')!` 非空断言、`lib/api/types.ts` 重复 `User` 接口。
  - 假实现：`lib/api/skills.ts:84` `versions()` 把单个结果包成数组冒充版本列表。
- 范围（本次做）：
  1. 删除 `src/types/` 死类型目录与 `uiStore.ts`、`accept-invitation/page.tsx` 冗余文件；清理引用与 barrel 导出。
  2. 修正不安全断言：theme/role 用类型守卫校验；root 元素加缺失守卫。
  3. 合并 `lib/api/types.ts` 重复 `User` 定义为单一真源。
  4. `skillsApi.versions` 改为明确「未实现」或对接真实端点（归 EVO 后续），不再伪造数组。
- 不做：
  - 不清理后端死代码（归 EVO-051）。
  - 不改 billing/i18n 缺口（随对应 feature 迭代处理）。
  - 不重写 API client 拦截器逻辑。
- 验收标准：
  - [ ] `src/types/`、`uiStore.ts`、`accept-invitation/page.tsx` 删除且无残留 import。
  - [ ] `rg "as Theme|as TokenRole|getElementById\('root'\)!" frontend/src` 无不安全用法（或已加守卫）。
  - [ ] `lib/api/types.ts` 仅有单一 `User` 定义。
  - [ ] `skillsApi.versions` 不再返回伪造数组。
  - [ ] 前端类型检查与构建通过（`bun run build` / `tsc` 0 错误）。
- 依赖或阻塞：无。
- 解锁内容：前端类型反映真实契约，降低维护误导。
- 影响范围：frontend
- 最小验证方式：`bun run build`；`rg` 检查死代码残留；类型检查 0 错误。

### EVO-059 Backend clippy 历史 lint 升级修复

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 来源：Iteration 028 验证残余 / 2026-06-01
- 用户/工程价值：消除 `cargo clippy --workspace -- -D warnings` 18 个错误，让 release 构建前可重启用 clippy 严格门禁。
- 背景（Iteration 028 rustfmt 基线收口时发现；Iteration 032 重新确认 18 个错误分布）：
  - 10× `unwrap_used`（`crates/infra/tests/{api_key,user,audit,snippet,skill}_repo_tests.rs`），工作区 `[workspace.lints.clippy] unwrap_used = "deny"` 应用到所有 target 含 tests（**注**：项目无 `clippy.toml` allow-unwrap-in-tests 配置）。
  - 3× `dead_code`（`crates/api/tests/auth_e2e_tests.rs:49/55/76`，三个 struct 字段 `message` / `expires_at` / `plan` 定义但未读）。
  - 4× `unnecessary_min_or_max`（`crates/api/src/middleware/rate_limit.rs:111-114`）。
  - 1× `field_reassign_with_default`（`crates/service-payment/src/config.rs:53`）。
  - 跨 8 个文件、18 个 `-D warnings` 错误，全部非业务问题。
- 范围（本次做）：
  1. `infra/tests/*_repo_tests.rs`：评估将 `[workspace.lints.clippy] unwrap_used` 从 `deny` 降为 `warn`；或对必要 `unwrap` 加 `#[expect(clippy::unwrap_used)]` + 解释。
  2. `api/tests/auth_e2e_tests.rs`：对未读取字段加 `#[allow(dead_code)]` 或 `#[expect(dead_code)]` + 注释；或删除未使用字段。
  3. `api/src/middleware/rate_limit.rs:111-114`：用 `.min(...).max(...)` 替换 `min(...).min(...)` 模式或反向比较。
  4. `service-payment/src/config.rs:53`：使用结构体更新语法或显式字段赋值替换。
- 不做：
  - 不改业务逻辑。
  - 不调整工作区 `deny(clippy::panic)` / `deny(clippy::expect_used)` / `deny(clippy::todo)` 等其他 `deny` 级别。
  - 不新增 suppress，不批量 `#[allow(...)]`。
- 验收标准：
  - [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 错误。
  - [ ] `cargo test --workspace` 仍 0 失败（验证非行为变更）。
  - [ ] 无新增 `#[allow(...)]` / `#[expect(...)]` 除非带注释说明。
  - [ ] `[workspace.lints.clippy]` 配置变化有注释说明。
- 依赖或阻塞：无。
- 解锁内容：Iteration 028 验证残余清零，release 流程可启用 clippy 严格门禁。
- 影响范围：backend（含 tests）
- 最小验证方式：`cargo clippy --workspace --all-targets -- -D warnings`；`cargo test --workspace`。

### EVO-060 dev.sh EMBEDDED_FRONTEND/ZIP 死代码 + 关联 proposal 状态清理

- 类型：tech-debt
- 优先级：P2
- 状态：Ready
- 来源：嵌入式模式验证 2026-06-01 / Iteration 035 后续
- 用户/工程价值：消除 dev.sh 与 proposal 中指向已废 ZIP 嵌入方案的孤儿代码，避免新成员按过时模式复用。
- 背景（嵌入式模式本地验证时发现）：
  - Iteration 031 改用 `rust-embed-for-web` 的 `#[folder = "../frontend/dist/"]` 直接嵌入目录（`backend/src/frontend.rs:7`），取代了原 ZIP 方案。
  - 但 `scripts/dev.sh` 仍保留 ZIP 方案的全部脚手架：`EMBEDDED_FRONTEND` 环境变量、`build_frontend_zip()` 函数、`embedded` 子命令入口。
  - `docs/proposals/EMBEDDED-FRONTEND.md` 状态仍为 `远期目标`，未反映 Iteration 031 已用替代方案实现。
  - `Cargo.toml` 中**未定义** `embedded-frontend` feature；调用 `cargo build --features embedded-frontend` 会报 unknown feature。
  - 历史记录保留：`docs/iterations/ITERATION-030.md`（Superseded → Iteration 031）、`EVOLUTION.md` ADR-0003 引用。
- 范围（本次做）：
  1. **`scripts/dev.sh` 清理**：
     - 删除 line 10 `EMBEDDED_FRONTEND="${EMBEDDED_FRONTEND:-false}"`。
     - 删除 line 126-140 `build_frontend_zip()` 函数。
     - 删除 line 152-168 `if [ "$EMBEDDED_FRONTEND" = "true" ]; then ...` 分支。
     - 删除 line 365-368 `embedded` 子命令入口。
     - 同步更新 `--help` / usage 列表。
  2. **`docs/proposals/EMBEDDED-FRONTEND.md` 状态更新**：
     - `远期目标` → `已晋升（Iteration 031 改用 rust-embed-for-web 替代 ZIP 方案）`。
     - 移除或注释 `--features embedded-frontend` 引用（line 75、149）。
     - 顶部加改线说明 + 链接到 Iteration 031 / ADR-0003。
  3. **`docs/iterations/ITERATION-030.md` 注释补全**（按 AGENTS.md「已发布 iteration 计划基线保护」原则，保留原计划不动）：
     - 在「已改线」段落中补充：`--features embedded-frontend` 在 Cargo.toml 中未定义，Iteration 031 实际改用 `#[folder]` 无 feature flag；如需重新启用 feature flag 形式参见 ADR-0003。
- 不做：
  - 不改 backend `frontend.rs` 的 `#[folder]` 嵌入实现（工作正常）。
  - 不改 `docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md`（决策已生效）。
  - 不重写 `docs/iterations/ITERATION-030.md` 的计划基线（按 AGENTS.md 规则仅追加说明）。
  - 不删 `EVOLUTION.md` 历史记录。
- 验收标准：
  - [ ] `rg "EMBEDDED_FRONTEND|frontend\.zip" scripts/dev.sh` 0 hits。
  - [ ] `rg "embedded-frontend" scripts/dev.sh docs/proposals/ backend/Cargo.toml backend/crates/*/Cargo.toml` 0 hits（确认 feature flag 未复活）。
  - [ ] `rg "embedded-frontend" docs/iterations/ITERATION-030.md` 仍保留（历史基线），但段落有改线说明。
  - [ ] `docs/proposals/EMBEDDED-FRONTEND.md` 顶部状态含「已晋升」+ Iteration 031 链接。
  - [ ] `bash -n scripts/dev.sh` 语法检查通过。
  - [ ] `scripts/dev.sh --help` / `bash scripts/dev.sh` 启动正常，无 `embedded` 子命令。
  - [ ] `scripts/dev.sh` 总行数减少约 50 行。
  - [ ] `scripts/dev.sh lite|start|stop|status|logs|clean|infra|backend|frontend` 子命令未受影响。
  - [ ] `docs/reference/SCRIPTS-RELEASE-NOTES.md` 同步记录 dev.sh 行为变更（删除 `embedded` 子命令 + `EMBEDDED_FRONTEND` 环境变量 + `build_frontend_zip` 函数）。
- 依赖或阻塞：无。
- 解锁内容：dev.sh 与 proposal 反映 Iteration 031 终局形态；新成员复用 dev.sh 时不会看到死代码。
- 影响范围：scripts/dev.sh、docs/proposals/EMBEDDED-FRONTEND.md、docs/iterations/ITERATION-030.md、docs/reference/SCRIPTS-RELEASE-NOTES.md
- 最小验证方式：`bash -n scripts/dev.sh`；`rg "EMBEDDED_FRONTEND|frontend\.zip|embedded-frontend" scripts/dev.sh docs/proposals/ backend/` 0 hits；`bash scripts/dev.sh lite` / `bash scripts/dev.sh status` 仍可执行。

### EVO-061 bollard 0.17→0.21 Docker Engine API 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：bollard 0.18 起 `Docker::connect_with_*` API 调整、0.19+ 引入 `exec` 异步流式 API、0.20+ 改 `ContainerStats` 返回类型；当前 0.17.1 在 Phase 7 sandbox 验证通过但 4 个 minor 的 bugfix 缺失。
- 范围：升级 `bollard = "0.17"` → `"0.21"`；重写 `service-skill/src/executor.rs` 的容器启动/停止/日志流；验证 `backend/sandbox/{python,node}/Dockerfile` 在新 bollard API 下行为不变。
- 不做：升级 `service-skill` Phase 7 之外的 bollard 使用方（实际仅 executor.rs）。
- 验收标准：`cargo check / clippy / test --workspace` 0 error；Phase 7 沙箱 Python/Node 示例技能可执行。
- 影响范围：`service-skill` + sandbox 镜像。
- 最小验证方式：起 Phase 7 测试环境跑 `python_skill:hello` + `node_skill:hello` 两类示例技能。

### EVO-062 sqlx 0.7→0.8/0.9 迁移（解决 future-incompat）

- 类型：tech-debt
- 优先级：P1
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / Iteration 028 显式归口 / 2026-06-01
- 用户/工程价值：解决 `sqlx-postgres 0.7.4` never-type-fallback future-incompat warning（Rust 2024 edition 强制前不阻断，但 1.95+ 已可见）；获取 0.8 引入的 `Database` trait 简化与新 `Executor` 抽象。
- 范围：升级 `sqlx = "0.7"` → `"0.8"`（暂不升 0.9，0.9 仍是 alpha）；19 个 repo 文件的 `query!` / `query_as!` 宏按 0.8 语法适配；`FromRow` derive 重新生成；SQLite + PostgreSQL 双轨 integration tests 全绿。
- 不做：不升级 sqlx 0.9（alpha）；不修改业务查询语义；不重新设计 repository trait。
- 验收标准：`cargo check / clippy / test --workspace` 0 error；`future-incompat` 报告 0 sqlx-postgres 警告。
- 影响范围：所有 8 个 `infra/src/db/{pg_,}*_repo.rs` + 2 个 integration test 文件。
- 最小验证方式：跑 `cargo test --workspace`（含 PG integration tests）；`cargo report future-incompatibilities` 无 sqlx 项。

### EVO-063 config 0.14→0.15 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.15 合并 `with_source` → `add_source` 并简化 `ConfigBuilder` API。
- 范围：升级 `config = "0.14"` → `"0.15"`；重写 `infra/src/config.rs` 中 `Config::builder().with_source(...).build()` 调用链。
- 不做：保留当前配置结构（`AppConfig` / 子 config 嵌套）。
- 验收标准：`cargo check / test --workspace` 0 error；应用启动能加载默认 config。
- 影响范围：`infra/src/config.rs`。
- 最小验证方式：`cargo test -p infra`；本地起服能读到 `JWT__SECRET` 等环境变量。

### EVO-064 validator 0.16→0.18+ 升级（trait 迁移）

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.18 起 `validate` 方法从 `#[validate(...)]` 改为 `#[derive(Validate)]` + `validator::Validate` trait 调用；与 serde 生态更协调，错误类型统一为 `ValidationErrors`。
- 范围：升级 `validator = "0.16"` → `"0.18"`（或最新 0.20）；重写 9 个 domain / DTO 的 `validate` 调用路径；handler 错误映射同步更新。
- 不做：不升 0.19+（API 仍在微调）；不改校验规则（regex 长度等）。
- 验收标准：`cargo check / test --workspace` 0 error；register / login / forgot_password 等 handler 校验路径在 DTO 失败时仍返 400。
- 影响范围：domain 模型 + 8 个 DTO + auth handler 错误映射。
- 最小验证方式：`cargo test --workspace`；手测注册接口 `POST /api/v1/auth/register` 传 `password=short` 返 400。

### EVO-065 jsonwebtoken 9→10 升级

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：9→10 合并 `decode` with/without validation、`Header` 改 `JoseHeader`、`EncodingKey::from_secret` 不变；与 openssl 0.10+ / RustCrypto 升级链对齐。
- 范围：升级 `jsonwebtoken = "9.2"` → `"10"`；重写 `service-auth/src/jwt.rs` 的 `encode` / `decode` 调用与 `Header` 构造。
- 不做：JWT payload 结构不变（`sub` / `exp` / `tenant_id` / `role`）；不改 token 有效期策略。
- 验收标准：`cargo check / test --workspace` 0 error；登录/登出/CsrfMiddleware token 解析路径不退化。
- 影响范围：`service-auth` + `api/src/middleware/csrf.rs` + `api/src/middleware/auth.rs`。
- 最小验证方式：`cargo test --workspace`；本地起服登录拿 token 访问 `/api/v1/auth/profile`。

### EVO-066 thiserror 1→2 升级（需 edition 2024）

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：thiserror 2.0 需 `edition = "2024"`；与 Rust 1.85+ 强制规则同步；`#[from]` 自动 trait 路径推断。
- 范围：升级 `thiserror = "1.0"` → `"2"`；同步把 `[workspace.package] edition = "2021"` 升到 `"2024"`；`common/src/error.rs` + 8 个 service error 类型迁移。
- 不做：不引入 thiserror 2 的新 feature；保持现有 `AppError` enum 形状。
- 前置依赖：Rust edition 升级（Iteration 032 plan §3 明确「不升级 Rust edition」，本项激活时需先解除该约束）。
- 验收标准：`cargo check / test --workspace` 0 error；所有 `AppError` 变体仍能正确显示。
- 影响范围：所有 backend crate 的 `thiserror::Error` derive + Rust edition。
- 最小验证方式：`cargo test --workspace`；`grep -r "edition = " backend/Cargo.toml backend/crates/*/Cargo.toml` 全部 2024。

### EVO-067 jsonschema 0.17→0.46 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：跨 28 个 minor 的 schema draft 支持演进（draft-07 → draft 2019-09 / 2020-12）；新 `Retrieval` / `Validator` trait 抽象。
- 范围：升级 `jsonschema = "0.17"` → `"0.46"`；重写 `service-tool` 的 schema 校验入口（如果有 trait 改动）。
- 不做：暂不升级 draft 语义（仍用 draft-07 校验 MCP tool 的 input_schema）。
- 验收标准：`cargo check / test --workspace` 0 error；MCP `tools/call` 校验路径不退化。
- 影响范围：`service-tool`。
- 最小验证方式：`cargo test -p service-tool`；MCP tools/call 测试用例 `mcp_tool_execution_tests` 仍 4 通过。

### EVO-068 reqwest 0.11→0.12/0.13 升级

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.12 移除 `blocking` 特性独立 crate + Rustls 默认；0.13 改 `ClientBuilder` 默认值（redirect 策略、超时）。
- 范围：升级 `reqwest = "0.11"` → `"0.13"`；`service-tool` HTTP executor 与 `service-payment` Stripe webhook 同步适配。
- 不做：不升 `reqwest` 的 `native-tls` 路径（项目统一 rustls）；不改 HTTP 行为语义。
- 验收标准：`cargo check / test --workspace` 0 error；MCP HTTP tool 执行成功；Stripe webhook 验签路径不退化。
- 影响范围：`service-tool` + `service-payment`。
- 最小验证方式：`cargo test --workspace`；手测 MCP tools/call 一个 HTTP 类型工具。

### EVO-069 hmac 0.12→0.13 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.13 改 `Mac::new_from_slice` 签名（返回 `CtOutput`）+ 移除 `SimpleHMac`；与 `sha2 0.11`（EVO-070）强耦合。
- 范围：升级 `hmac = "0.12"` → `"0.13"`；`service-payment` Stripe webhook 验签迁移。
- 不做：不改 webhook 验签算法（仍 HMAC-SHA256 + Stripe-Signature 头）。
- 验收标准：`cargo check / test --workspace` 0 error；Stripe webhook mock 测试通过。
- 影响范围：`service-payment`。
- 最小验证方式：`cargo test -p service-payment`。

### EVO-070 sha2 0.10→0.11 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.11 改 `Sha256::new()` 返回 `CtOutput`（const-time 防御时序攻击）；与 hmac 0.13 强耦合（EVO-069）。
- 范围：升级 `sha2 = "0.10"` → `"0.11"`；`service-payment` + `infra`（缓存 key 哈希）影响。
- 不做：不引入 `sha1` / `sha3` 等其他变体（项目只用 sha256）。
- 验收标准：`cargo check / test --workspace` 0 error。
- 影响范围：`service-payment` + `infra`。
- 最小验证方式：`cargo test --workspace`。

### EVO-071 redis 0.27→1.x 命名空间重置

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：redis crate 0.x→1.0 是显式 breaking（trait 完全重写、`AsyncCommands` 重命名、新 `Connection` 抽象）。
- 范围：升级 `redis = "0.27"` → `"1.2"`；重写 `infra/src/cache.rs` 的 `RedisCache` 实现（`service-payment` 暂未真实使用）。
- 不做：保留 `InMemoryCache` 作为 fallback；不引入 redis cluster / sentinel。
- 验收标准：`cargo check / test --workspace` 0 error；`RedisCache` 单元测试通过（如有）。
- 影响范围：`infra`。
- 最小验证方式：`cargo test -p infra`（mock 模式）；本地启动 docker compose redis 7-alpine 后集成测试。

### EVO-072 actix-governor 0.6→0.7+ 升级

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.7 起 `Governor` 中间件构造改 actix-web 4.x 新 `KeyExtractor` trait；0.10 latest 加入 `cfg` 模块化配置。
- 范围：升级 `actix-governor = "0.6"` → `"0.10"`；`api/src/middleware/rate_limit.rs` 迁移 KeyExtractor 构造。
- 不做：不调整 rate limit 阈值（保留当前 unauthenticated/authenticated/api_key 区分）。
- 验收标准：`cargo check / test --workspace` 0 error；rate limit middleware 单元测试通过。
- 影响范围：`api/src/middleware/rate_limit.rs`。
- 最小验证方式：`cargo test -p api`；`/api/v1/auth/login` 错误密码 30+ 次触发 429。

### EVO-073 actix-web-prom 0.8→0.9+ 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.9 起改 `PrometheusMetricsBuilder` API（命名空间、registry 配置变化）；0.10 latest。
- 范围：升级 `actix-web-prom = "0.8"` → `"0.10"`；`api/src/lib.rs` 构造 + metrics endpoint 路径同步。
- 不做：不引入 OpenMetrics exporter；不改 metric 标签体系。
- 验收标准：`cargo check / test --workspace` 0 error；`/metrics` endpoint 仍返回 Prometheus 格式。
- 影响范围：`api`。
- 最小验证方式：`cargo test -p api`；`curl /metrics` 返回非空 Prometheus 文本。

### EVO-074 prometheus 0.13→0.14 升级

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：0.14 改 `Encoder` trait 签名；与 `actix-web-prom`（EVO-073）同源。
- 范围：升级 `prometheus = "0.13"` → `"0.14"`；`api` metrics 注册路径同步。
- 不做：不改 metric 名称/标签。
- 验收标准：`cargo check / test --workspace` 0 error。
- 影响范围：`api`。
- 最小验证方式：`cargo test -p api`。

### EVO-075 rand 0.8→0.9/0.10 升级（service-auth）

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：rand 0.9/0.10 重整 `Rng` trait 层级（`RngCore` / `Rng` 分离）；与 chrono / argon2 生态对齐。
- 范围：升级 `service-auth/Cargo.toml` 的 `rand = "0.8"` → `"0.10"`；token 生成 / salt 随机化迁移。
- 不做：不引入 `rand_distr` / `rand_chacha` 等额外 crate。
- 验收标准：`cargo check / test --workspace` 0 error。
- 影响范围：`service-auth`。
- 最小验证方式：`cargo test -p service-auth`。

### EVO-076 serde_yaml 0.9 → serde_yml / serde_norway 迁移

- 类型：tech-debt
- 优先级：P3
- 状态：Proposed
- 来源：Iteration 032 依赖审计暂缓项 / 2026-06-01
- 用户/工程价值：serde_yaml 0.9 已 deprecate；社区 fork `serde_yml` 与纯 Rust 替代 `serde_norway` 提供活跃维护路径。
- 范围：`service-skill` + `service-snippet` 内的 `serde_yaml::from_str` / `to_string` 调用迁移到 `serde_yml`（推荐，社区活跃）或 `serde_norway`（纯 Rust 替代）。
- 不做：保留 YAML 格式本身（不切 JSON）；不改 SKILL.md / CLI 格式定义。
- 验收标准：`cargo check / test --workspace` 0 error；现有 SKILL.md parser 测试不退化。
- 影响范围：`service-skill` + `service-snippet`。
- 最小验证方式：`cargo test -p service-skill -p service-snippet`。

### EVO-077 Governance board 派生运营视图

- 类型：governance
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Governance
- 用户故事或技术目标：
  - 作为/为了：维护者和 Agent 需要一个快速判断当前工作流向的派生运营视图。
  - 我希望/需要：新增符合 agent-project-governance skill 的 `docs/BOARD.md`。
  - 以便：在不复制 backlog / iteration 状态源的前提下，快速回答 Now / Review / Blocked / Next / Later 和每项 gate。
- 范围：
  - 新增 `docs/BOARD.md`，标明派生视图规则。
  - 看板行仅包含 `Item / State / Owner Doc / Gate`。
  - 每行链接 owner doc，并写明 exit / resume / activation / deferral gate。
  - 同步 `docs/README.md` 文档地图和 `AGENTS.md` Session End Checklist。
  - 建立 `Iteration 036` 记录本次治理修复与迭代合理性评估。
- 不做：
  - 不创建前端看板页面或运行时代码。
  - 不把看板放进 `docs/backlog/`，不让看板成为第二个 backlog。
  - 不在看板里维护 story 详情、验收清单或执行日志。
  - 不关闭 `Iteration 033` 或改写 `Iteration 034` 的计划基线。
- 验收标准：
  - 非行为类：
    - [x] `docs/BOARD.md` 存在，并明确是 derived operating view。
    - [x] `docs/BOARD.md` 只使用 `Item / State / Owner Doc / Gate` 四列。
    - [x] 看板每条实际工作行都有 owner doc 链接和明确 gate。
    - [x] `docs/README.md` 链接 `docs/BOARD.md`。
    - [x] `AGENTS.md` Session End Checklist 包含 owner docs 先于 board 同步的检查项。
    - [x] `docs/BOARD.md` 不与 backlog / iteration README / active iteration 状态冲突。
    - [x] `Iteration 036` 记录本次插队治理修复和 Iteration 033/034 合理性评估。
    - [x] 文档链接检查、governance validator 和 `git diff --check` 通过。
- 技术备注：
  - Skill 标准结构将 board 定义为可选的 `docs/BOARD.md`，不是 backlog 子文档。
  - Board 只解决运营扫描问题；状态权威仍在 owner docs。
- 依赖或阻塞：无。
- 解锁内容：开始新迭代前可先扫 board，但仍必须按 START-ITERATION 扫描 owner docs。
- 影响范围：docs / AGENTS.md / .gitignore
- 最小验证方式：文档链接检查；`sh /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/skills/agent-project-governance/scripts/validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith`；`git diff --check`。
