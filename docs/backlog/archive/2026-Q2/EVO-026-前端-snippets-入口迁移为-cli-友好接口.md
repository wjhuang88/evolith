# EVO-026 前端 Snippets 入口迁移为 CLI 友好接口

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: product-change
- Status: Done
- Priority: P1
- Source: EVO-017 / 页面残留 / Iteration 017
- Decision Context: Vite 迁移后统一替换导航、路由文案、API client 和 i18n 旧 snippet 概念

#### Source Detail Snapshot

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
