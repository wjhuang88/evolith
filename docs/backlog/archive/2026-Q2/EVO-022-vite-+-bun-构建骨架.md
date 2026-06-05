# EVO-022 Vite + Bun 构建骨架

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P0
- Source: EVO-002 split
- Decision Context: Iteration 004；新增 Vite 入口、React Router 根路由和并行构建脚本

#### Source Detail Snapshot

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
