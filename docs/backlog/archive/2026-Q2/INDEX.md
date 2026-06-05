# 2026 Q2 Backlog Archive

> Archive of non-active execution context compacted from the monolithic `PRODUCT-BACKLOG.md` on 2026-06-05. The active routing surface remains [Product Backlog](../../PRODUCT-BACKLOG.md).

| ID | Title | Type | Status | Priority | Source | Decision Context |
| --- | --- | --- | --- | --- | --- | --- |
| [EVO-001](EVO-001-前后端-api-对齐.md) | 前后端 API 对齐 | bug | Done | P0 | [实施路线图 Phase A](../../../roadmap/IMPLEMENTATION-ROADMAP.md#phase-a--现状校准与-api-对齐done) | 修正 method、字段和不存在的前端 API 调用；snippet 范围已转入 EVO-017 |
| [EVO-002](EVO-002-前端迁移到-react-+-vite-+-bun.md) | 前端迁移到 React + Vite + Bun | tech-debt | Done | P0 | [实施路线图 Phase B](../../../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0--高优先级) | EVO-021 至 EVO-025 全部完成 |
| [EVO-003](EVO-003-忘记密码与重置密码闭环.md) | 忘记密码与重置密码闭环 | feature | Done | P0 | Iteration 005 | handler 实现 + 前端 API 接入，Playwright 验证通过 |
| [EVO-004](EVO-004-邀请接受-join-流程.md) | 邀请接受 / Join 流程 | feature | Done | P0 | Iteration 005 | handler 实现，cargo test 通过 |
| [EVO-005](EVO-005-mcp-工具真实执行.md) | MCP 工具真实执行 | feature | Done | P0 | 需求 F1.1.3 / Iteration 006 | HTTP handler type 真实执行，Function 类型暂不支持 |
| [EVO-006](EVO-006-skill-更新接口.md) | Skill 更新接口 | feature | Done | P1 | API 501 / Iteration 010 | `PUT /skills/{id}` 已实现 |
| [EVO-007](EVO-007-snippet-更新接口.md) | Snippet 更新接口 | feature | Deferred | P1 | API 501 | 被 EVO-017 替代方向覆盖，暂停继续投入 |
| [EVO-008](EVO-008-snippet-reference-格式增强.md) | Snippet reference 格式增强 | feature | Deferred | P1 | 需求 F1.3.4 | 被 EVO-017 替代方向覆盖 |
| [EVO-009](EVO-009-skillmd-与-cli-interface-frontmatter-parser.md) | SKILL.md 与 CLI interface frontmatter parser | feature | Done | P1 | 格式规范 / EVO-017 / Iteration 010 | CLI interface parser 已有基线；SKILL.md parser 已实现 |
| [EVO-010](EVO-010-租户-members-页面接真实-api.md) | 租户 Members 页面接真实 API | feature | Done | P1 | Iteration 011 | 后端 find_by_tenant + remove_from_tenant + 前端接线 |
| [EVO-011](EVO-011-api-key-页面接真实-api.md) | API Key 页面接真实 API | feature | Done | P1 | Iteration 011 | types + api module + page 重写 |
| [EVO-015](EVO-015-rust-cli-子项目.md) | Rust CLI 子项目 | feature | Deferred | P3 | [提案](../../../proposals/RUST-CLI.md) | API 稳定后启动 |
| [EVO-016](EVO-016-前端嵌入后端发布物.md) | 前端嵌入后端发布物 | tech-debt | Done | P3 | [提案](../../../proposals/EMBEDDED-FRONTEND.md) | EVO-016-B 已由 Iteration 031 完成；Docker 单容器细化如需继续另拆 |
| [EVO-016-A](EVO-016-A-embedded-frontend-交付形态-refinement.md) | Embedded Frontend 交付形态 refinement | tech-debt | Deferred | P2 | EVO-016 split / Iteration 024 | 已被 Iteration 031 的 rust-embed-for-web 实施覆盖，不再单独激活 |
| [EVO-016-B](EVO-016-B-前端静态服务迁移到-rust-embed-for-web.md) | 前端静态服务迁移到 rust-embed-for-web | tech-debt | Done | P0 | EVO-016 split / 用户需求 / Iteration 031 | 替代 ZIP 方案：用 rust-embed-for-web 实现零拷贝 + 预压缩 + 自动缓存协商；14 项验收标准全部通过 |
| [EVO-017](EVO-017-snippet-迁移为-cli-友好接口.md) | Snippet 迁移为 CLI 友好接口 | product-change | Done | P0 | [ADR-0002](../../../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) | Iteration 002；replaces EVO-007/EVO-008；已建立 CLI interface 格式、API 兼容契约、parser 基线和迁移盘点 |
| [EVO-018](EVO-018-邮箱验证发送与确认闭环.md) | 邮箱验证发送与确认闭环 | feature | Done | P1 | Iteration 009 | Handler 与测试存在，Iteration 021 完成 contract/testing/roadmap 收口 |
| [EVO-021](EVO-021-前端路由适配层.md) | 前端路由适配层 | tech-debt | Done | P0 | EVO-002 split | Iteration 003；已新增 `frontend/src/lib/router.tsx`，页面和共享组件不再直接导入 Next 路由模块 |
| [EVO-022](EVO-022-vite-+-bun-构建骨架.md) | Vite + Bun 构建骨架 | tech-debt | Done | P0 | EVO-002 split | Iteration 004；新增 Vite 入口、React Router 根路由和并行构建脚本 |
| [EVO-023](EVO-023-前端运行时配置迁移.md) | 前端运行时配置迁移 | tech-debt | Done | P0 | EVO-002 split | Iteration 004；`src/lib/config.ts` 统一运行时环境变量，替换所有 `process.env` 引用 |
| [EVO-024](EVO-024-docker-nginx-切换到静态-spa.md) | Docker / Nginx 切换到静态 SPA | tech-debt | Done | P0 | EVO-002 split | Iteration 004；过渡部署形态，Dockerfile 改为 Bun + Vite build + Nginx 静态服务，SPA fallback |
| [EVO-025](EVO-025-移除-nextjs-依赖和遗留入口.md) | 移除 Next.js 依赖和遗留入口 | tech-debt | Done | P0 | EVO-002 split | Iteration 004；删除 next 依赖、App Router、middleware、config；router.tsx 改为 React Router |
| [EVO-026](EVO-026-前端-snippets-入口迁移为-cli-友好接口.md) | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | Done | P1 | EVO-017 / 页面残留 / Iteration 017 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| [EVO-030](EVO-030-github-cicd-重建.md) | GitHub CI/CD 重建 | tech-debt | Done | P2 | EVO-002 split / 工程收尾 | Iteration 029 收口（2026-06-01）：`.github/workflows/ci.yml` 建立（tag-only `v*.*.*` semver trigger / 单 job 后端+前端串联 / Swatinem/rust-cache + oven-sh/setup-bun 缓存 / postgres:16-alpine service 容器）。9 门禁全绿：fmt ✓ / check ✓ / clippy ✓ / cargo test 274 passed。TECH-STACK §4.2 + TESTING §5 同步。deploy workflow / PR trigger 显式 Deferred |
| [EVO-031](EVO-031-iteration-004005-质量修复与流程防呆.md) | Iteration 004/005 质量修复与流程防呆 | bug | Done | P0 | 质量审查 2026-05-17 | 修复静态资源反代、邀请接受闭环、公开接口放行、邮件公开 URL、sourcemap 默认关闭和流程规约 |
| [EVO-032](EVO-032-iteration-006-mcp-执行质量修复与流程防呆.md) | Iteration 006 MCP 执行质量修复与流程防呆 | bug | Done | P0 | 质量审查 2026-05-25 | Iteration 007；修复执行器初始化崩溃、工具调用鉴权、错误映射和验收证据失真 |
| [EVO-033](EVO-033-rustfmt-全量格式基线与-stable-配置清理.md) | Rustfmt 全量格式基线与 stable 配置清理 | tech-debt | Done | P2 | Iteration 007 验证残余 / Iteration 028 | 2026-06-01 完成；移除 7 个 nightly-only 配置，应用 stable rustfmt 重写 13 个文件；fmt/check/test 通过；clippy 1 个 pre-existing error 归口 EVO-059 |
| [EVO-034](EVO-034-epic-与子需求拆分治理规则.md) | Epic 与子需求拆分治理规则 | tech-debt | Done | P1 | 流程缺口 2026-05-26 | Iteration 008；已补齐父子编号、依赖、分层 DoR 与跨 Epic 选取约束 |
| [EVO-035](EVO-035-治理-skill-manifest-接入与一致性审计.md) | 治理 skill manifest 接入与一致性审计 | tech-debt | Done | P2 | Iteration 008 验证残余 | Iteration 023；已建立 manifest 并通过 bundled validator |
| [EVO-036](EVO-036-已发布迭代计划基线保护与改线防呆.md) | 已发布迭代计划基线保护与改线防呆 | tech-debt | Done | P1 | 计划覆写复盘 2026-05-27 | Iteration 013；修复 EVO-016 计划追踪并同步治理 skill |
| [EVO-037](EVO-037-治理-skill-弱模型闭环执行防呆.md) | 治理 skill 弱模型闭环执行防呆 | tech-debt | Done | P1 | 用户反馈 2026-05-27 | Iteration 014；为初始化、迁移和修复任务增加强制闭环协议 |
| [EVO-038](EVO-038-本项目实施任务闭环-sop-与完成声明门禁.md) | 本项目实施任务闭环 SOP 与完成声明门禁 | tech-debt | Done | P1 | 用户反馈 2026-05-27 | Iteration 015；将闭环协议落实到 Evolith 自身流程 |
| [EVO-039](EVO-039-迭代启动前库存盘点与既有计划优先规则.md) | 迭代启动前库存盘点与既有计划优先规则 | bug | Done | P1 | 流程缺口 2026-05-27 | Iteration 016；先处理在途/已规划迭代再选择新 story |
| [EVO-040](EVO-040-已实现接口完成声明与参考文档状态修复.md) | 已实现接口完成声明与参考文档状态修复 | bug | Done | P1 | 排期库存审计 2026-05-27 | Iteration 021；修复邮箱验证与 Skill 更新接口的收口漂移 |
| [EVO-041](EVO-041-敏捷实践与-bdd-验收格式适配规则.md) | 敏捷实践与 BDD 验收格式适配规则 | tech-debt | Done | P1 | 用户方法论反馈 2026-05-28 | Iteration 022；明确 Evolith iteration 与传统 Sprint、Story 与 BDD 的适配口径 |
| [EVO-042](EVO-042-编号保留位（缺号处置）.md) | 编号保留位（缺号处置） | governance | Dropped | — | Iteration 035 / 代码健康审查 2026-06-01 | 缺号处置：原编号未被任何 story 占用，确认为 2026-05-29 EVO-043~050 跨 Epic 集中入池时跳跃（041 → 043），并非遗漏；保留为占位以维持 EVO 编号连续性语义。如需新增可复用此编号并标注 `replaces <空>` |
| [EVO-043](EVO-043-后端依赖全量版本审计与迁移.md) | 后端依赖全量版本审计与迁移 | tech-debt | Done | P1 | Iteration 032 | 2026-06-01 完成：36 个 workspace 功能依赖 + 3 个 path dep + 3 个 crate 私有 dep 完整审计；cargo check 0 / cargo test 274 pass；22 个保留（caret 已覆盖 latest stable，无需修改 Cargo.toml）；14 个 workspace 大版本升级 + 2 个 crate 私有 deprecation → EVO-061~076（16 个新 backlog 项全部 P3 Proposed）；clippy 18 errors 归口 EVO-059 不并入 |
| [EVO-045-A](EVO-045-A-executionprovider-统一-trait-+-docker-容器池化.md) | ExecutionProvider 统一 trait + Docker 容器池化 | tech-debt | Done | P0 | EVO-045 split / EVO-048 输出 / Iteration 041 | 2026-06-04 完成；2026-06-05 回归修复：sandbox 默认关闭，lite/local 启动不依赖 Docker，显式启用仍 fail-fast |
| [EVO-048](EVO-048-serverless-执行架构设计-spike.md) | Serverless 执行架构设计 Spike | spike | Done | P0 | EVO-045/047 前置 / Iteration 033 | Iteration 033 收口（2026-06-03）：输出 `docs/proposals/SERVERLESS-RUNTIME.md`（11 节）；Phase 7 sandbox 复用结论=部分复用；统一 ExecutionProvider 接口设计；冷启动方法学（已缓存 ~350ms-1800ms / 池模式 ~50-200ms，实测待 Docker）；Vercel 演进路径 + 组件替换清单；EVO-045/047 依赖图 + 推荐实施顺序；本机无 Docker，冷启动实测 conditional |
| [EVO-049-A](EVO-049-A-skillcli-规范兼容数据模型基线.md) | Skill/CLI 规范兼容数据模型基线 | tech-debt | Done | P0 | EVO-049 split / Iteration 034 | 2026-06-04 完成：migration 006（skills +13 列 / snippets +8 列）；domain/DTO/repository 全量更新；SnippetRepository 新增 update 方法；282 tests passed（+8 新增） |
| [EVO-051](EVO-051-误导性注释、命名与后端死代码清理.md) | 误导性注释、命名与后端死代码清理 | tech-debt | Done | P2 | 代码健康审查 2026-06-01 | Iteration 039 收口（2026-06-04）：删除 10 个死代码文件 + 6 个 mod 声明清理 + TODO 注释移除 + NewUser.password → password_hash 全量重命名（21 处）；282 tests passed |
| [EVO-052](EVO-052-mysql-半接线收敛与快速失败.md) | MySQL 半接线收敛与快速失败 | tech-debt | Done | P2 | 代码健康审查 2026-06-01 / Iteration 038 | 2026-06-04 完成：config validate + create_pool 对 mysql 快速失败；main.rs 后置 MySql 分支移除；CONFIG/EVOLUTION 同步 |
| [EVO-054](EVO-054-backlog-状态漂移与编号一致性修复.md) | backlog 状态漂移与编号一致性修复 | bug | Done | P1 | 代码健康审查 2026-06-01 / Iteration 035 | 2026-06-01 完成：EVO-016-B 详情块 In Progress → Done、EVO-026 详情块 Ready → Done（额外漂移）、EVO-042 缺号登记 Dropped 行；详情块 100% 与总表一致 |
| [EVO-055](EVO-055-前后端-api-契约漂移修复.md) | 前后端 API 契约漂移修复 | bug | Done | P1 | 跨层一致性审查 2026-06-01 / Iteration 035 | 2026-06-01 完成：删除 `membersApi.acceptInvitation` 死分支 / `billing/page.tsx` 加 `BILLING_ENABLED` 闸门 + 「待计费」静态页 / `cliInterfacesApi.update` 改 `throw new Error` 指向 create+delete / `API-CONTRACT.md` Billing 段补「待计费」标注 |
| [EVO-056](EVO-056-沙箱降级静默成功修复.md) | 沙箱降级静默成功修复 | bug | Done | P2 | 跨层一致性审查 2026-06-01 / Iteration 038 | 2026-06-04 完成：沙箱启用时 Docker executor 初始化失败直接启动失败；DefaultSkillExecutor 返回 ConfigError，不再 exit_code:0 伪成功 |
| [EVO-058](EVO-058-前端死代码与类型卫生清理.md) | 前端死代码与类型卫生清理 | tech-debt | Done | P2 | 前端代码审查 2026-06-01 | Iteration 040 收口（2026-06-04）：删除 7 个死代码文件（-139 行）+ uiStore re-export 移除 + skillsApi.versions 假实现移除 + 重复 User 接口合并；bun run build + tsc 0 errors |
| [EVO-059](EVO-059-backend-clippy-历史-lint-升级修复.md) | Backend clippy 历史 lint 升级修复 | tech-debt | Done | P2 | Iteration 028 验证残余 / Iteration 029 配套 | 2026-06-01 Iteration 029 收口：21 个 `-D warnings` 错误归零（原估算 18，实际 13× unwrap_used + 3× dead_code + 4× unnecessary_min_or_max + 1× field_reassign_with_default）。修复策略：unwarp_used 在 7 个 test 文件加文件级 `#![allow(clippy::unwrap_used)]`（workspace deny 覆盖 clippy.toml 行为）；dead_code 移除未使用字段而非 `#[allow]`；unnecessary_min_or_max 移除 `.max(3)` 因 MIN_RPM=30 保障 rpm/10>=3；field_reassign_with_default 改 struct update syntax |
| [EVO-060](EVO-060-devsh-embeddedfrontendzip-死代码-+-关联-proposal-状态清理.md) | dev.sh EMBEDDED_FRONTEND/ZIP 死代码 + 关联 proposal 状态清理 | tech-debt | Done | P2 | 嵌入式模式验证 2026-06-01 | 2026-06-03 完成：删除 4 处死代码（EMBEDDED_FRONTEND / build_frontend_zip / --features embedded-frontend / ZIP 构建逻辑）；新增 build_frontend() 函数；lite/embedded 模式改为单端口（build + backend）；后端端口改为读 SERVER__PORT 环境变量；proposal 状态已晋升；SCRIPTS-RELEASE-NOTES.md 同步 |
| [EVO-077](EVO-077-governance-board-派生运营视图.md) | Governance board 派生运营视图 | governance | Done | P1 | 用户反馈 2026-06-03 / Iteration 036 | 按 agent-project-governance skill 标准新增 `docs/BOARD.md`，只汇总 owner docs 与 gate，不作为新状态源；验证通过并收口 |
| [EVO-078](EVO-078-最近开发任务治理漂移修复.md) | 最近开发任务治理漂移修复 | governance | Done | P1 | 用户反馈 2026-06-04 / Iteration 037 | 修复 Board / iterations README / Iteration 029 / Iteration 033 / 设计文档归类 / 文档断链漂移；补齐 EVO-045-A 子任务；记录近期 Figma 与脚本任务治理归口 |
| [EVO-079](EVO-079-sandbox-默认启用导致本地启动依赖-docker-回归修复.md) | sandbox 默认启用导致本地启动依赖 Docker 回归修复 | bug | Done | P1 | 用户反馈 2026-06-05 / Iteration 041 follow-up | 将 `sandbox.enabled`、`.env.development`、`.env.example` 默认改为 false；显式启用 sandbox 时仍保留 Docker fail-fast |
| [EVO-082](EVO-082-evolution-feedback-sop-缺失导致治理-validator-失败修复.md) | evolution feedback SOP 缺失导致治理 validator 失败修复 | governance | Done | P1 | 本轮治理验证 2026-06-05 | 补 `docs/sop/EVOLUTION-FEEDBACK.md` 并从 AGENTS/docs README 路由，恢复 governance validator |
| [EVO-083](EVO-083-skill-107-agent-redirect-入口纠偏.md) | skill 1.0.7 agent redirect 入口纠偏 | governance | Done | P1 | 用户反馈 2026-06-05 / skill 更新 | 补 `CLAUDE.md` / `GEMINI.md` 单行重定向入口，并同步 manifest 与文档地图 |
| [EVO-084](EVO-084-backlog-compaction-标准结构迁移.md) | Backlog compaction 标准结构迁移 | governance | Done | P1 | 用户反馈 2026-06-05 / skill backlog-compaction | 将 monolithic backlog 压缩为决策入口 + active item files + archive index；验证通过后收口 |
| [EVO-085](EVO-085-archive-index-二次压缩与-item-file-拆分纠偏.md) | Archive index 二次压缩与 item file 拆分纠偏 | governance | Done | P1 | 用户反馈 2026-06-05 / backlog compaction follow-up | `archive/2026-Q2/INDEX.md` 不应承载全部历史详情，已拆为 per-item archive files 并保留短索引 |

## Reading Rules

1. Use this file as the archive index only.
2. Open the linked item file for preserved execution context.
3. Return to `../../PRODUCT-BACKLOG.md` for current routing and prioritization.
