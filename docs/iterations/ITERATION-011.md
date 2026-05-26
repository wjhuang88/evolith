# Iteration 011: Embedded Frontend 交付边界确认

> 状态：Planned，需先 refinement
> 计划目标：把 EVO-016 从远期方向细化为可以安全实施的 embedded frontend 交付切片，确定生产运行时配置与后端静态服务边界。

## 1. 计划边界

`EVO-016` 已具备前置条件，但当前仍是 `Deferred` 的大项。按 Epic / Story 方法论，本轮不能
直接选入裸 Epic；启动前需先将 `EVO-016` 明确为 Epic，并在 backlog 建立下述候选子 Story。

## 2. 候选拆分与选入 Story

| 候选 ID | 标题 | 父 Epic | 当前状态 | 依赖/顺序 |
|---------|------|---------|----------|-----------|
| EVO-016-A | Embedded Frontend 交付边界与运行时配置确认 | EVO-016 | 待建项 | EVO-022 至 EVO-025 已 Done |

> `EVO-016-A` 是规划编号，尚未进入 backlog；只有完成 refinement、晋升为 `Ready` 后才能启动本轮。

## 3. 目标范围

- 确认第一阶段采用“同一后端容器内文件服务”还是编译期嵌入，并记录重大取舍。
- 明确 `/api/v1`、`/health`、`/mcp`、`/assets` 与 SPA fallback 的路由优先级。
- 明确 embedded 模式下同源 API base 与 `/config.js` 的处理方式。
- 将后续实现拆成可验证子 Story，并给出依赖次序。

## 4. 不做事项

- 不在本轮直接完成 Rust 静态文件服务或 Docker 镜像切换。
- 不实现 `include_dir` 单二进制实验。
- 不提前重建 GitHub CI/CD；EVO-030 仍必须等待最终构建形态明确。

## 5. 计划验收标准

- [ ] `EVO-016` 已按 Epic 规则补齐完成条件、子 Story 表和依赖顺序。
- [ ] `EVO-016-A` 已成为满足 DoR 的可执行 Story。
- [ ] 部署形态、运行时配置与 SPA/API 路由边界有明确决策或实施约束。
- [ ] 后续文件服务、容器切换与可选单二进制实验被拆成独立候选 Story。

## 6. 计划验证

```bash
git diff --check
# 启动实施后再加入与选定方案匹配的 backend/frontend/deploy 验证命令。
```

## 7. 风险与进入条件

| 风险 | 计划控制 |
|------|----------|
| 将过渡 Nginx 形态误写为终局 | 以 EVO-016 proposal 和 roadmap 的终局约束为依据 |
| 直接对 Epic 开工导致范围失控 | 启动前必须建 `EVO-016-A` 并通过 Story DoR |
| 运行时配置在同源部署下失效 | 把 `/config.js` 或替代方案纳入边界决策 |

## 8. 计划记录

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Future iteration planned only. EVO-016 满足前置但尚未 refinement；规划候选子 Story `EVO-016-A`，未变更 backlog。 |
