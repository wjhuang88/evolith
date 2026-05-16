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
| EVO-002 | 前端迁移到 React + Vite + Bun | tech-debt | P0 | In Progress | [实施路线图 Phase B](../roadmap/IMPLEMENTATION-ROADMAP.md#phase-b--前端迁移到-react--vite--bunp0--高优先级) | 高优先级工程门禁；已拆分为 EVO-021 至 EVO-025 |
| EVO-003 | 忘记密码与重置密码闭环 | feature | P0 | Ready | API 501 / 需求 F2.7.7 | 依赖 mailer |
| EVO-004 | 邀请接受 / Join 流程 | feature | P0 | Ready | API 501 / 需求 F2.5.5 | 租户成员闭环 |
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
| EVO-022 | Vite + Bun 构建骨架 | tech-debt | P0 | Ready | EVO-002 split | 新增 Vite 入口、React Router 根路由和并行构建脚本 |
| EVO-023 | 前端运行时配置迁移 | tech-debt | P0 | Ready | EVO-002 split | 从 `NEXT_PUBLIC_*` 迁移到 Vite/runtime config，保留 `/api/v1` 约束 |
| EVO-024 | Docker / Nginx / CI 切换到静态 SPA | tech-debt | P0 | Ready | EVO-002 split | 用 `dist/` 静态产物替代 Next standalone runtime |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | tech-debt | P0 | Ready | EVO-002 split | 删除 App Router、middleware、next config 和 Next package 依赖 |
| EVO-026 | 前端 Snippets 入口迁移为 CLI 友好接口 | product-change | P1 | Proposed | EVO-017 / 页面残留 | Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念 |
| EVO-027 | Skill 多来源创建 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 支持 ZIP 上传、Git 仓库接入、SkillHub 同步三种创建入口 |
| EVO-028 | Skill 版本管理与正确性验证 | feature | P1 | Proposed | 用户需求 / Agent Skills spec | 建立版本历史、回滚、agentskills 规范校验、描述质量检查和导入报告 |
| EVO-029 | Skill 专业描述与发现质量提升 | feature | P2 | Proposed | Agent Skills spec | 提升 description、触发关键词、兼容性、资源索引和搜索排序质量 |

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

1. `EVO-022` Vite + Bun 构建骨架。
2. `EVO-023` 前端运行时配置迁移。
3. `EVO-024` Docker / Nginx / CI 切换到静态 SPA。

理由：EVO-001 和 EVO-017 已完成；Next.js 去除是企业级 harness 平台交付形态的高优先级前置工作，随后再补 SaaS 用户生命周期和核心工具执行闭环。

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
