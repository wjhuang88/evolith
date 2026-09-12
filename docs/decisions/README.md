# Decisions 目录

本目录保存重要技术和产品决策记录，采用轻量 ADR（Architecture Decision Record）格式。

## 何时新增决策记录

- 技术栈变更，例如 Next.js -> React + Vite + Bun。
- 架构边界变化，例如前端嵌入后端。
- 数据库、认证、安全、部署或 Git Storage 策略发生重大取舍。
- 选择了一个有明显替代方案的设计。

## 命名

```text
ADR-0001-react-vite-bun-frontend.md
ADR-0002-cli-friendly-interface-replaces-snippet.md
```

## 当前决策

| ADR | 状态 | 说明 |
|-----|------|------|
| [ADR-0001](ADR-0001-react-vite-bun-frontend.md) | Accepted | 前端采用 React + Vite + Bun |
| [ADR-0002](ADR-0002-cli-friendly-interface-replaces-snippet.md) | Accepted | CLI 友好接口替代旧 snippet 主线 |
| [ADR-0003](ADR-0003-embedded-frontend-rust-embed-for-web.md) | Accepted | 前端静态文件嵌入从 ZIP 方案迁移到 rust-embed-for-web |
| [ADR-0004](ADR-0004-git-centric-storage.md) | Accepted / amended by ADR-0011 | Git 是代码与历史事实源；durable storage 实现由 ADR-0011 演进为 object-store-backed WalGit data plane |
| [ADR-0005](ADR-0005-deprecate-sandbox-runtime.md) | Accepted | 废弃 Skill 沙箱执行，仅保留 FaaS 形态（2026-06-23 方向调整） |
| [ADR-0006](ADR-0006-smart-http-via-git-subprocess.md) | Accepted historical / superseded as target by ADR-0011 | 解释当前 Alpha 的 `git` subprocess Smart HTTP；长期目标已改为 WalGit primitives + Evolith service-git v2 |
| [ADR-0007](ADR-0007-agent-write-and-production-delivery-boundaries.md) | Accepted | Agent 写入 Policy/Scoped Token 边界与单一 Embedded Frontend 生产交付 |
| [ADR-0008](ADR-0008-repo-centric-interaction-architecture.md) | Accepted | Repo-centric 主流程、首次使用、Repo 内导航与能力发现的信息架构 |
| [ADR-0009](ADR-0009-no-prelaunch-registry-compatibility.md) | Accepted | 未上线阶段取消旧 Registry 双写/迁移兼容，改为 Repo-derived 承接后直接删除 |
| [ADR-0010](ADR-0010-final-production-convergence-after-product-completion.md) | Accepted | EVO-118-E 在目标产品开发与清理完成后执行；DEPLOY-01 只阻止上线，不阻止继续开发 |
| [ADR-0011](ADR-0011-walgit-backed-git-data-plane.md) | **Accepted** | `service-git` 重构为 WalGit-backed object-store/WAL Git Data Plane；Evolith 保持唯一产品控制面；新增 GIT-DP-01 |

## 当前架构决策入口

2026-09-10 起，Git Data Plane 相关的新实现先读：

1. [Project Status Baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md) — 区分当前实现与已接受目标。
2. [ADR-0011](ADR-0011-walgit-backed-git-data-plane.md) — 长期决策与 supersede 关系。
3. [WalGit Refactor Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md) — 实施级边界。
4. [Current Execution Plan](../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md) — 当前激活顺序。
5. [EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md) — 可执行 Epic/Story。

在 EVO-126-H cutover 前，ADR-0006 描述的 subprocess/filesystem 仍是**当前运行事实**；ADR-0011 描述的是**已接受目标**，不得混淆。

## 模板

```markdown
# ADR-000X: <标题>

## 状态

Proposed / Accepted / Superseded

## 背景

## 选项

## 决策

## 后果

## 相关链接
```
