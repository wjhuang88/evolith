# Iteration 004: 前端技术栈迁移 React + Vite + Bun（完成）

> 时间：2026-05-17
> 目标：完成 EVO-022 至 EVO-025，将前端从 Next.js 完整迁移到 React + Vite + Bun 静态 SPA。

## 1. 本轮目标

- 在 `frontend/` 中新增 Vite 构建配置和 SPA 入口。
- 使用 React Router v6 建立完整路由树，复用 Iteration 003 的路由适配层。
- 迁移运行时环境变量到 `src/lib/config.ts` 统一抽象。
- 更新 Docker/Nginx 配置适配静态 SPA 产物。
- 移除 Next.js 全部依赖和遗留入口。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-022 | Vite + Bun 构建骨架 | P0 | Agent | Done |
| EVO-023 | 前端运行时配置迁移 | P0 | Agent | Done |
| EVO-024 | Docker / Nginx 切换到静态 SPA | P0 | Agent | Done |
| EVO-025 | 移除 Next.js 依赖和遗留入口 | P0 | Agent | Done |

## 3. 不做事项

- 不创建或恢复 GitHub CI/CD workflow；归属项目后段的 EVO-030。
- 不引入 TanStack Query（保持 Zustand + Axios）。
- 不改变 API 合约或后端逻辑。

## 4. 验收标准

- [x] `vite.config.ts` 存在且配置了 React 插件、路径别名（`@/`）、Tailwind。
- [x] `index.html` SPA 入口可加载。
- [x] React Router 路由树覆盖当前所有 22 个页面路由。
- [x] `npm run dev` 启动 Vite dev server，SPA 可访问。
- [x] `npm run build` 产出 `dist/` 静态文件。
- [x] `npm run type-check` 通过，0 错误。
- [x] Playwright 验证：Landing → Login → Dashboard → Tools 导航正常。
- [x] Dockerfile 改为 Vite build + Nginx 静态服务。
- [x] Nginx 配置包含 SPA fallback `try_files $uri /index.html`。
- [x] Next.js 依赖全部移除，`'use client'` 指令清理完毕。

## 5. 验证结果

```bash
npm run type-check   → 0 errors
npm run build        → ✓ built in 807ms, dist/ 产出正常
Playwright           → Landing/Login/Dashboard/Tools 页面渲染正确，0 console errors
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| Vite 路径别名与 Next 的 `@/` 冲突 | 已解决：Vite alias 使用数组形式优先于 tsconfig paths |
| React Router Link 属性差异（href vs to） | 已解决：批量替换 44 处 `href=` → `to=` |
| `import.meta` 类型兼容 | 已解决：`as unknown as` 中间转换 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-17 | Iteration 004 started. 选入 EVO-022，补齐 backlog 详情块，创建迭代文件。 |
| 2026-05-17 | 前置清理：移除 `.github/workflows/ci.yml` 和 `deploy.yml`。 |
| 2026-05-17 | EVO-022 Done：vite.config.ts、index.html、main-spa.tsx、router-vite.tsx。Playwright 验证 Landing/Login/Dashboard。 |
| 2026-05-17 | EVO-023 Done：`src/lib/config.ts` 运行时配置抽象，替换 4 处 process.env。 |
| 2026-05-17 | EVO-025 Done：删除 next 依赖/配置/middleware/layout，router.tsx 改为 React Router，Link href→to 44 处，useSearchParams 修复，'use client' 42 文件清理。 |
| 2026-05-17 | EVO-024 Done：Dockerfile 改为 Vite build + Nginx，新增 spa-nginx.conf（SPA fallback），docker-compose.prod.yml 移除 Next 环境变量，生产 nginx 配置更新。 |
| 2026-05-17 | Playwright 全流程验证通过：Landing → Login → Dashboard → Tools，0 console errors。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-17 | scope-change | EVO-024 拆出 CI/CD 为 EVO-030 | 不包含 GitHub workflow | N/A |

## 9. Review

- 完成：EVO-022/023/024/025 全部完成，Next.js 完全移除，Vite 静态 SPA 构建正常。
- 未完成：bundle size 偏大（529KB gzip 153KB），后续可通过 code-splitting 优化。
- 验证结果：type-check ✅、build ✅、Playwright 全流程 ✅。

## 10. Retrospective

- 做得好的：Vite alias 数组形式解决了 tsconfig paths 覆盖问题；Playwright 逐步验证保证质量。
- 需要调整的：批量替换 Link 属性时应一次性覆盖模板字符串和对象属性形式。
- 写入 EVOLUTION：Vite alias 必须用数组形式；React Router `Link` 用 `to` 不是 `href`。
