# SOP: 新增功能

## 触发条件

- 新增 API、页面、业务实体、后台服务或跨模块能力。
- 扩展现有工具、技能、代码片段、租户、计费或审计能力。

## 前置确认

- [ ] 功能归属：tool / skill / snippet / auth / tenant / billing / infra / frontend。
- [ ] 是否需要数据库字段或新表。
- [ ] 是否影响 SQLite 和 PostgreSQL 两套实现。
- [ ] 是否需要 API 合约和前端类型同步。
- [ ] 是否需要 RBAC、CSRF、审计日志或限流规则。

## 标准落地顺序

1. 在 `docs/` 中补充或更新需求、API 合约或设计说明。
2. 修改 domain model 和 repository trait。
3. 同步 SQLite/PostgreSQL migration。
4. 实现 SQLite/PostgreSQL repository。
5. 增加或更新 service crate 逻辑。
6. 增加 DTO、handler、route。
7. 接入 RBAC、CSRF、审计和错误响应。
8. 更新前端 API client、types、store 和页面。
9. 增加测试。
10. 更新 SOP/reference/roadmap 中受影响的文档。

## 后端检查点

- Repository trait 不应泄漏具体数据库类型。
- handler 返回统一 `ApiResponse`。
- 新的状态变更接口必须考虑 CSRF。
- 租户资源必须检查 `tenant_id` 和角色。
- 生产路径禁止引入 `unwrap()`；workspace clippy 已配置 `unwrap_used = deny`。
- 数据库变更必须同时覆盖 `migrations/sqlite` 和 `migrations/postgres`。

## 前端检查点

- API 路径应基于 `apiClient`，不要硬编码完整 host。
- 状态变更请求依赖 `apiClient` 自动带 CSRF header。
- 新页面默认受 `AuthGuard` 保护，公开页面需要同步更新 `LayoutWrapper` 和 `middleware.ts`。
- 新文案需要同步 `zh-CN.json` 和 `en.json`。
- 表单错误应通过 `parseApiError` 或现有 toast 模式展示。

## 测试建议

| 改动类型 | 最小验证 |
|----------|----------|
| Rust domain/service | `cargo test -p <crate>` |
| API handler/middleware | `cargo test -p api` |
| repository/migration | `cargo test -p infra`，必要时用 PostgreSQL full 模式验证 |
| 前端类型或页面 | `bun run type-check`，必要时 `bun run build` |
| 跨端功能 | 后端 crate 测试 + 前端 build + 手工流程 |

## 完成后 Checklist

- [ ] API 合约和前端调用一致。
- [ ] 双数据库实现一致。
- [ ] 错误响应、权限和审计符合现有模式。
- [ ] 测试覆盖了成功路径和关键失败路径。
- [ ] 新增流程或踩坑已写入 SOP 或 `EVOLUTION.md`。
