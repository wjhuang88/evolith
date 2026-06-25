# EVO-087 前端 Bundle 体积优化

## Required Reads

- Product Backlog: `../PRODUCT-BACKLOG.md`
- Frontend package.json: `../../../frontend/package.json`
- Vite build output analysis

## Summary

- 类型：tech-debt
- 优先级：P1
- 状态：Proposed
- 父 Epic：无

## Problem Or Outcome

前端构建产物 `dist/assets/index-DYoZ8lmb.js` 达到 599.69 kB（gzip 后 172.29 kB），超过 Vite 默认 500 KB chunk 大小警告阈值。单一大 bundle 会：
- 增加首次加载时间，影响用户体验
- 降低代码缓存效率
- 增加移动端用户流量消耗

## Goal And Non-Goals

**目标**：
- 将 JS bundle 拆分成多个合理大小的 chunk（建议每个 < 300 KB）
- 利用 dynamic import 实现路由级代码分割
- 保持构建时间和开发体验不变

**不做**：
- 不引入复杂的微前端架构
- 不为了拆分而拆分（无明显加载收益的页面不单独拆）
- 不减少现有功能

## Dependencies And Blockers

无硬依赖。可独立启动。

## Governing ADRs, Specs Or Decisions

暂无 ADR，实施后如有重大取舍需补 ADR。

## Acceptance Criteria

- [ ] `bun run build` 无 chunk size warning（或明确调整 warning limit 并有合理理由）
- [ ] 主 JS chunk gzip 后 < 150 KB
- [ ] 路由级 dynamic import 至少覆盖 3 个非首页路由
- [ ] 首页加载 JS 总量（gzip）减少 ≥ 30%
- [ ] `bun run build` 和 `bun run type-check` 仍 0 errors

## Validation Evidence Required

- `bun run build` 输出截图（chunk 大小对比）
- Lighthouse 或 Chrome DevTools 首页加载 JS 总量对比
- 代码分割路由清单

## Residual Work Destination

- 优化结果写入 `docs/reference/TECH-STACK.md` 前端构建策略段
- 如有取舍写入 `docs/decisions/`

## Source Snapshot

- 当前构建产物：`dist/assets/index-DYoZ8lmb.js` 599.69 kB (gzip: 172.29 kB)
- 当前依赖：React 19.2.7、React Router 7.17.0、TanStack Query 5.101.0