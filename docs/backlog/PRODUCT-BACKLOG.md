# Product Backlog

> 状态维护规则见 [特性迭代工作流](../sop/ITERATION-WORKFLOW.md)。

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
| EVO-001 | 前后端 API 对齐 | bug | P0 | Ready | [开发计划 Phase A](../roadmap/DEVELOPMENT-PLAN.md#phase-a--现状校准与-api-对齐p0) | 修正 method、字段和不存在的前端 API 调用 |
| EVO-002 | 前端迁移到 React + Vite + Bun | tech-debt | P0 | Ready | [开发计划 Phase B](../roadmap/DEVELOPMENT-PLAN.md#phase-b--前端迁移到-react--vite--bunp0) | 去除 Next.js runtime，改静态 SPA |
| EVO-003 | 忘记密码与重置密码闭环 | feature | P0 | Ready | API 501 / 需求 F2.7.7 | 依赖 mailer |
| EVO-004 | 邀请接受 / Join 流程 | feature | P0 | Ready | API 501 / 需求 F2.5.5 | 租户成员闭环 |
| EVO-005 | MCP 工具真实执行 | feature | P0 | Ready | 需求 F1.1.3 | 当前 `tools/call` 返回 stub |
| EVO-006 | Skill 更新接口 | feature | P1 | Proposed | API 501 | `PUT /skills/{id}` |
| EVO-007 | Snippet 更新接口 | feature | P1 | Proposed | API 501 | `PUT /snippets/{id}` |
| EVO-008 | Snippet reference 格式增强 | feature | P1 | Proposed | 需求 F1.3.4 | direct / inline / with_deps |
| EVO-009 | SKILL.md 与 snippet frontmatter parser | feature | P1 | Proposed | 格式规范 | 为 CLI 和上传校验打基础 |
| EVO-010 | 租户 Members 页面接真实 API | feature | P1 | Proposed | 前端 TODO | 列表、邀请、移除 |
| EVO-011 | API Key 页面接真实 API | feature | P1 | Proposed | 前端 TODO | 创建、列表、revoke |
| EVO-012 | 租户设置保存 | feature | P2 | Proposed | 前端 TODO | tenant settings |
| EVO-013 | Audit log detail 接口 | feature | P2 | Proposed | API 501 | `GET /audit-logs/{log_id}` |
| EVO-014 | Stripe webhook 恢复 | feature | P2 | Proposed | routes TODO | 计费闭环 |
| EVO-015 | Rust CLI 子项目 | feature | P3 | Deferred | [规划](../planned/RUST-CLI.md) | API 稳定后启动 |
| EVO-016 | 前端嵌入后端发布物 | tech-debt | P3 | Deferred | [规划](../planned/EMBEDDED-FRONTEND.md) | Vite SPA 完成后启动 |

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

1. `EVO-001` 前后端 API 对齐。
2. `EVO-002` 前端迁移到 React + Vite + Bun。
3. `EVO-003` / `EVO-004` 认证与邀请闭环。

理由：先修正当前控制台可用性，再做前端工程迁移，随后补 SaaS 用户生命周期。
