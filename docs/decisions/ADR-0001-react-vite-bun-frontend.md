# ADR-0001: 前端采用 React + Vite + Bun

## 状态

Accepted

## 背景

Evolith 前端是 SaaS 控制台和管理后台，不依赖 SEO 或服务端渲染。当前 Next.js App Router 提供的 SSR、middleware 和 standalone runtime 对项目收益有限，但增加了部署和迁移复杂度。

## 选项

| 选项 | 优点 | 缺点 |
|------|------|------|
| 保留 Next.js | 改动小，沿用现有结构 | SSR/runtime 复杂度继续存在 |
| React + Vite + Bun | 保留 React 生态，静态部署简单 | 需要迁移路由和构建配置 |
| SolidJS + Bun | 性能好，bundle 小 | 需要重写组件，生态和团队熟悉度风险更高 |

## 决策

采用 `React + Vite + TypeScript + Tailwind + React Router + Zustand + TanStack Query + Bun`。

暂不采用 SolidJS。SolidJS 保留为远期实验方向。

## 后果

- 前端可作为纯静态 SPA 发布。
- 生产部署不需要 Node.js runtime。
- 保留现有 React 组件资产和生态。
- 需要替换 Next.js middleware、App Router 和环境变量前缀。

## 相关链接

- [开发计划](../roadmap/DEVELOPMENT-PLAN.md)
- [前端嵌入后端提案](../proposals/EMBEDDED-FRONTEND.md)
