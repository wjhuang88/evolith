# Iterations 目录

本目录记录每轮迭代计划、执行和复盘。

## 文件

- [迭代模板](ITERATION-TEMPLATE.md)
- [Iteration 001](ITERATION-001.md) — 工程化文档体系与流程改造。
- [Iteration 002](ITERATION-002.md) — CLI 友好接口概念迁移。
- [Iteration 003](ITERATION-003.md) — 前端路由适配层。
- [Iteration 004](ITERATION-004.md) — 前端技术栈迁移 React + Vite + Bun（EVO-022~025）。
- [Iteration 005](ITERATION-005.md) — 认证闭环：忘记密码与邀请接受（EVO-003/004）。
- [Iteration 006](ITERATION-006.md) — MCP 工具真实执行（EVO-005）。
- [Iteration 007](ITERATION-007.md) — MCP 工具执行质量修复与流程防呆（EVO-032）。
- [Iteration 008](ITERATION-008.md) — Epic 与子需求拆分治理规则（EVO-034）。
- [Iteration 009](ITERATION-009.md) — 邮箱验证闭环（EVO-018；
  Review：代码/测试记录存在，contract/testing/roadmap reference 收口待修复）。
- [Iteration 010](ITERATION-010.md) — Skill 更新与 SKILL.md parser（EVO-006 / EVO-009；
  Review：stories Done，API contract 与迭代收口证据待核对）。
- [Iteration 011](ITERATION-011.md) — Members 与 API Keys 前端接真实 API
  （EVO-010 / EVO-011；替换了原 EVO-016 计划，已补偏差记录）。
- [Iteration 013](ITERATION-013.md) — 已发布迭代计划基线保护与改线防呆（EVO-036）。
- [Iteration 014](ITERATION-014.md) — 治理 skill 弱模型闭环执行防呆（EVO-037）。
- [Iteration 015](ITERATION-015.md) — 本项目实施任务闭环 SOP 与完成声明门禁（EVO-038）。
- [Iteration 016](ITERATION-016.md) — 迭代启动前库存盘点与既有计划优先规则（EVO-039）。
- [Iteration 021](ITERATION-021.md) — 已实现接口完成声明与参考文档状态修复（EVO-040；
  Closed：Iteration 009 / 010 收口完成）。
- [Iteration 022](ITERATION-022.md) — 敏捷实践与 BDD 验收格式适配规则（EVO-041）。
- [Iteration 023](ITERATION-023.md) — 治理 skill manifest 接入与一致性审计（EVO-035）。
- [Iteration 024](ITERATION-024.md) — Embedded Frontend 交付形态 refinement（EVO-016-A；Superseded）。
- [Iteration 025](ITERATION-025.md) — 租户设置与审计详情补齐（EVO-012 / EVO-013）。
- [Iteration 026](ITERATION-026.md) — Stripe Webhook 与计费闭环恢复（EVO-014）。
- [Iteration 027](ITERATION-027.md) — Skill 发现质量与描述治理（EVO-029）。
- [Iteration 028](ITERATION-028.md) — Rustfmt 基线与 CI 命令准备（EVO-033；Closed：fmt baseline 完成，clippy 1 pre-existing error 归口 EVO-059）。
- [Iteration 029](ITERATION-029.md) — GitHub CI/CD 重建（EVO-030；Closed：EVO-030 + EVO-059 双 Done；`.github/workflows/ci.yml` tag-only `v*.*.*` semver 触发 / 9 门禁全绿 / 274 测试通过）。
- [Iteration 030](ITERATION-030.md) — ZIP 嵌入前端流式响应与静态索引（Superseded：改线到 Iteration 031）。
- [Iteration 031](ITERATION-031.md) — 前端静态服务迁移到 rust-embed-for-web（EVO-016-B；Closed）。
- [Iteration 032](ITERATION-032.md) — 后端依赖全量版本审计与迁移（EVO-043；Closed：审计 39 dep + 3 crate 私有 dep；16 个大版本升级归口 EVO-061~076）。
- [Iteration 033](ITERATION-033.md) — Serverless 执行架构设计 Spike（EVO-048；Closed：输出 `docs/proposals/SERVERLESS-RUNTIME.md`；Phase 7 部分复用；统一 ExecutionProvider；冷启动方法学；Vercel 演进路径）。
- [Iteration 034](ITERATION-034.md) — Skill/CLI 规范兼容数据模型基线（EVO-049-A；Closed：migration 006 + domain/DTO/repository 全量更新 + 282 tests passed）。
- [Iteration 035](ITERATION-035.md) — P1 治理先行：backlog 状态漂移与前后端 API 契约漂移修复（EVO-054 / EVO-055；Closed）。
- [Iteration 036](ITERATION-036.md) — Governance board operating view（EVO-077；Closed：派生 Board 建立并验证通过）。
- [Iteration 037](ITERATION-037.md) — 最近开发任务治理漂移修复（EVO-078；Closed：Board / iteration 目录 / DESIGN 归类 / EVO-045-A 补齐）。
- [Iteration 038](ITERATION-038.md) — 假可用快速失败修复（EVO-052 / EVO-056；Closed：MySQL 快速失败 + sandbox fail-fast）。
- [Iteration 039](ITERATION-039.md) — 后端死代码与误导性注释清理（EVO-051；Closed：删除 10 个死代码文件 + NewUser.password → password_hash 重命名 + 282 tests passed）。
- [Iteration 040](ITERATION-040.md) — 前端死代码与类型卫生清理（EVO-058；Closed：删除 7 个死代码文件 + skillsApi.versions 假实现移除 + 重复 User 接口合并 + build/tsc 0 errors）。
- [Iteration 041](ITERATION-041.md) — ExecutionProvider 统一 trait + Docker 容器池化（EVO-045-A；Closed：创建统一执行抽象 + 容器池化 + HTTP 转发 + 适配器 facade + 309 tests passed）。
- [Iteration 042](ITERATION-042.md) — Phase E'-1 Git Service 基础（EVO-101 + EVO-102；Closed：2026-06-24，git_repos schema + policy.yaml parser + TenantQuotas 适配 + ExecutionProvider deprecated）。
- [Iteration 043](ITERATION-043.md) — Direction Pivot Review Remediation（EVO-113；Closed：2026-06-25，安全默认策略、状态同步、断链、sandbox legacy reference、manifest risk gates 修复）。
- [Iteration 044](ITERATION-044.md) — Phase E'-1b Repo CRUD（EVO-103-A；Closed：2026-06-25，EVO-103-A Repo CRUD 完成（cargo test 0 failures / clippy 0 errors））。
- [Iteration 045](ITERATION-045.md) — Phase E'-1b Git Client Auth Infra（EVO-103-B-1；Active：2026-06-25 启动，Basic-Auth + /repos/ RBAC + CSRF 豁免，解锁 git 客户端鉴权）。

## 未来计划

以下文档仅为仍可能激活的排期草案，未启动实施，也不代表候选事项已进入 `In Progress`。
已关闭或已被取代的计划仅保留在文件清单和库存记录中，不再列入未来候选：

- [Iteration 042](ITERATION-042.md) — Phase E'-1 Git Service 基础（EVO-101 + EVO-102；
  Closed：2026-06-24，314 tests passed）。
- [Iteration 025](ITERATION-025.md) — 租户设置与审计详情补齐（EVO-012 / EVO-013；
  Blocked for activation：候选需 Story/BDD refinement）。
- [Iteration 026](ITERATION-026.md) — Stripe Webhook 与计费闭环恢复（EVO-014；
  Blocked for activation：候选需 webhook 安全与 mock 验收 refinement）。
- [Iteration 018](ITERATION-018.md) — Skill 导入基础能力细化与基线（EVO-019 / EVO-020；
  Superseded by 2026-06-23 方向调整 → EVO-100 Phase E'-3 / EVO-108）。
- [Iteration 019](ITERATION-019.md) — Skill 多来源导入闭环（EVO-027；
  Superseded by 2026-06-23 方向调整 → EVO-100 Phase E'-3 / EVO-108）。
- [Iteration 020](ITERATION-020.md) — Skill 版本与正确性验证（EVO-028；
  Superseded by 2026-06-23 方向调整 → git 原生版本管理 + EVO-108）。
- [Iteration 027](ITERATION-027.md) — Skill 发现质量与描述治理（EVO-029；
  Superseded by 2026-06-23 方向调整 → EVO-109 Discovery API + Pages 式发现 UI）。

> **2026-06-24 文档漂移清理**（ITERATION-042 启动前）：
> Iteration 028（Rustfmt 基线 / EVO-033）与 Iteration 034（Skill/CLI 数据模型基线 / EVO-049-A）**实际已 Closed**（见非终态库存），不应再列入"Ready for activation"。本轮已从未来计划 section 移除。

## 非终态库存

启动任何新的产品迭代前，必须先按启动 SOP 处置：

- [Iteration 009](ITERATION-009.md) — `Closed`：EVO-040 完成收口。
- [Iteration 010](ITERATION-010.md) — `Closed`：EVO-040 完成收口。
- [Iteration 012](ITERATION-012.md) — `Superseded by Iteration 031`：EVO-016-B 已由
  Iteration 031 完成，本计划不再激活。
- [Iteration 017](ITERATION-017.md) — `Closed`：EVO-026 前端 CLI Interface 概念收口；
  前端代码符号 Snippet→CliInterface 重命名 + UI 迁移完成。
- [Iteration 018](ITERATION-018.md) 至 [Iteration 020](ITERATION-020.md) —
  `Superseded by 2026-06-23 方向调整 → EVO-100 Phase E'-3 / EVO-108`：原 Phase E Skill 导入 / 多来源 / 版本验证已由 git 原生能力 + indexer 覆盖。
- [Iteration 024](ITERATION-024.md) — `Superseded by Iteration 031`：EVO-016-A
  refinement 已被 rust-embed-for-web 实施覆盖。
- [Iteration 025](ITERATION-025.md) 至 [Iteration 026](ITERATION-026.md) —
  `Planned / Blocked`：Phase F 候选需 refinement，与 Phase E' 独立。
- [Iteration 027](ITERATION-027.md) — `Superseded by 2026-06-23 方向调整 → EVO-109 Discovery API + Pages 式发现 UI`。
- [Iteration 028](ITERATION-028.md) — `Closed`：EVO-033 Rustfmt 基线和 CI 命令准备；fmt/check/test 通过，clippy 1 pre-existing error 归口 EVO-059。
- [Iteration 029](ITERATION-029.md) — `Closed`：2026-06-01 EVO-030 + EVO-059 双 Done。`.github/workflows/ci.yml` 建立（tag-only `v*.*.*` semver trigger / 单 job 后端+前端串联 / Swatinem/rust-cache + oven-sh/setup-bun 缓存 / postgres:16-alpine service 容器）。9 门禁全绿：fmt / check / clippy / cargo test 274 passed。TECH-STACK §4.2 + TESTING §5 同步。deploy workflow + PR trigger 显式 Deferred。
- [Iteration 030](ITERATION-030.md) — `Superseded`：ZIP 流式方案改线到
  rust-embed-for-web（Iteration 031）。
- [Iteration 031](ITERATION-031.md) — `Closed`：EVO-016-B 前端静态服务迁移到
  rust-embed-for-web。全部 14 项验收标准通过。Iteration 012 激活条件已解除。
- [Iteration 032](ITERATION-032.md) — `Closed`：EVO-043 后端依赖审计与迁移。
  完整审计 39 workspace dep + 3 crate 私有 dep；16 个大版本升级归口
  EVO-061~076（全部 P3 Proposed）；clippy 18 pre-existing errors 维持 EVO-059
  P2 Ready；cargo check 0 / cargo test 274 passed / clippy `--all-targets`
  18 已知错误（仅 059）。
- [Iteration 033](ITERATION-033.md) — `Closed`：EVO-048 Serverless 架构 Spike。
  输出 `docs/proposals/SERVERLESS-RUNTIME.md`（Phase 7 部分复用 / 统一
  ExecutionProvider / 容器池架构 / 冷启动方法学 / Vercel 演进 / EVO-045/047 依赖图）。
  冷启动实测待 Docker 环境。
- [Iteration 034](ITERATION-034.md) — `Closed`：EVO-049-A Skill/CLI 规范兼容数据模型基线。
  Migration 006（skills +13 列 / snippets +8 列）；domain/DTO/repository 全量更新；
  SnippetRepository 新增 update 方法；282 tests passed（+8 新增）。
- [Iteration 035](ITERATION-035.md) — `Closed`：EVO-054 / EVO-055 完成。EVO-054 修复 3 处详情块漂移 + 1 处 Dropped 占位（EVO-042）+ 1 处额外漂移（EVO-026）；EVO-055 修复死分支 / billing 闸门 / 501 显式 throw / API-CONTRACT 同步。
- [Iteration 036](ITERATION-036.md) — `Closed`：EVO-077 治理看板文档；
  插队治理修复完成；当时未改变 Iteration 033，后续已由 Iteration 033 收口和
  Iteration 037 同步 Board 状态。
- [Iteration 037](ITERATION-037.md) — `Closed`：EVO-078 最近开发任务治理漂移修复；
  Board / iteration README / DESIGN 归类 / EVO-045-A 缺失记录已同步。
- [Iteration 038](ITERATION-038.md) — `Closed`：EVO-052 / EVO-056 完成。
  MySQL 不再被配置/连接池当成可用后端；sandbox 启用时 Docker executor 初始化失败
  不再降级为 `DefaultSkillExecutor` 伪成功。
- [Iteration 039](ITERATION-039.md) — `Closed`：EVO-051 后端死代码与误导性注释清理。
  删除 10 个死代码文件 + 6 个 mod 声明清理 + TODO 注释移除 +
  NewUser.password → password_hash 全量重命名（21 处）；282 tests passed。
- [Iteration 040](ITERATION-040.md) — `Closed`：EVO-058 前端死代码与类型卫生清理。
  删除 7 个死代码文件（-139 行）+ uiStore re-export 移除 +
  skillsApi.versions 假实现移除 + 重复 User 接口合并；
  bun run build + tsc 0 errors。
- [Iteration 041](ITERATION-041.md) — `Closed`：EVO-045-A
  ExecutionProvider 统一 trait + Docker 容器池化。2026-06-05 follow-up 修复 sandbox
  默认启用导致 lite/local 启动依赖 Docker 的回归。
- [Iteration 044](ITERATION-044.md) — `Closed`：EVO-103-A Repo CRUD（Phase E'-1b 首切片；2026-06-25 启动，cargo test 0 failures / clippy 0 errors）。
- [Iteration 045](ITERATION-045.md) — `Active`：EVO-103-B-1 Git Client Auth Infra（Phase E'-1b 第二切片；2026-06-25 启动，Basic-Auth + /repos/ RBAC + CSRF 豁免）。

## 下一周建议顺序

该顺序仅为排期建议，实际启动时仍必须重新执行 [开始一次迭代 SOP](../sop/START-ITERATION.md)
并记录库存 disposition：

1. **ITERATION-042 已 Closed**（Phase E'-1）：EVO-101 + EVO-102 完成（2026-06-24）。
2. **ITERATION-043 已 Closed**：EVO-113 Direction Pivot Review Remediation（方向变更评审缺口修复，2026-06-25）。
3. **ITERATION-044 已 Closed**（Phase E'-1b）：EVO-103-A Repo CRUD（gix::init + RBAC）Done。
4. **ITERATION-045 已激活**（Phase E'-1b）：EVO-103-B-1 Git Client Auth Infra（Basic-Auth + /repos/ RBAC + CSRF 豁免）。
5. **下一候选**（需 refinement 到 Ready）：
   - EVO-103-B-2（Smart HTTP 端点，依赖 B-1 完成）
   - EVO-103-C（Repo Context API，依赖 EVO-103-A Done 已满足；需 BDD refinement）
6. **新进 Ready 待评估**（独立微迭代候选）：
   - EVO-057（生产 CORS Origin 可配置化，可独立启动 deploy/config 微迭代）
   - EVO-080（Wasmer/WASI 替代 Docker sandbox 可行性 Spike）
   - EVO-081（内部文档页面基于独立 Markdown 目录渲染）

## 命名

```text
ITERATION-001.md
ITERATION-002.md
...
```

## 推荐节奏

- 常规迭代：1 周。
- Agent 微迭代：一次会话只完成一个 backlog story 或一个明确切片。
- Evolith iteration 借鉴 Sprint 的小批次、目标、验收和复盘，但本质是可审计工作批次；
  不强制完整 Scrum 仪式或团队容量统计。

## 发布计划基线规则

`Planned` iteration 一旦提交即为计划基线，不是可复用的编号占位符：

1. 实际执行仍属于原目标时，在同一文档追加激活、验证、Review 和复盘，不删除原计划。
2. 实际要做另一目标或另一组 story 时，原文档追加延期/阻塞说明，新工作创建新的
   iteration 编号。
3. 已发布的后续计划依赖被改线计划时，标注 `Blocked for activation`，直至新的前置
   计划完成。

## 状态同步

迭代开始：

1. 先按 [开始一次迭代 SOP](../sop/START-ITERATION.md) 执行固定检查。
2. 盘点本目录中的 `Active / In Progress / Review / Planned / Blocked` 文档并记录
   disposition；在途或待收口迭代优先处理。
3. 已规划迭代可以按原计划激活时优先激活；继续阻塞、延期或改线时先补记录。
4. 只有既有 iteration 已处置后，才从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
   选择新的 Ready 项并创建 `ITERATION-<N>.md`。
5. 明确本轮不做什么。

迭代结束：

1. 更新完成情况和验证结果。
2. 更新 backlog item 状态。
3. 新经验写入 `EVOLUTION.md`。
