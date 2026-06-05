# EVO-030 GitHub CI/CD 重建

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: EVO-002 split / 工程收尾
- Decision Context: Iteration 029 收口（2026-06-01）：`.github/workflows/ci.yml` 建立（tag-only `v*.*.*` semver trigger / 单 job 后端+前端串联 / Swatinem/rust-cache + oven-sh/setup-bun 缓存 / postgres:16-alpine service 容器）。9 门禁全绿：fmt ✓ / check ✓ / clippy ✓ / cargo test 274 passed。TECH-STACK §4.2 + TESTING §5 同步。deploy workflow / PR trigger 显式 Deferred

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Proposed
- 用户价值或技术目标：在前端迁移、部署形态和核心项目结构稳定后，基于最终命令重建 GitHub CI/CD，避免在迁移中反复维护过时 workflow。
- 验收标准：
  - [ ] 新建 `.github/workflows/ci.yml`，覆盖后端 fmt/clippy/test、前端 type-check/build、必要的安全扫描。
  - [ ] 如仍需要部署自动化，新建 `.github/workflows/deploy.yml` 或明确替代方案。
  - [ ] CI 中使用最终前端命令，不再引用 Next standalone 构建路径。
  - [ ] PostgreSQL/Redis 或容器依赖的验证策略明确。
  - [ ] 更新测试与发布相关参考文档。
- 依赖或阻塞：EVO-024、EVO-025，以及项目主线功能稳定后统一排期。
- 影响范围：deploy / docs
- 最小验证方式：workflow lint 或一次 GitHub Actions dry run / 手动触发记录；本地执行对应命令。
