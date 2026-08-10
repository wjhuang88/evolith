# Decisions 目录

本目录保存重要技术和产品决策记录，采用轻量 ADR（Architecture Decision Record）格式。

## 何时新增决策记录

- 技术栈变更，例如 Next.js -> React + Vite + Bun。
- 架构边界变化，例如前端嵌入后端。
- 数据库、认证、安全、部署策略发生重大取舍。
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
| [ADR-0004](ADR-0004-git-centric-storage.md) | Accepted | 内容存储从 DB 列迁移到 Git 仓库文件（2026-06-23 方向调整） |
| [ADR-0005](ADR-0005-deprecate-sandbox-runtime.md) | Accepted | 废弃 Skill 沙箱执行，仅保留 FaaS 形态（2026-06-23 方向调整） |
| [ADR-0006](ADR-0006-smart-http-via-git-subprocess.md) | Accepted | Smart HTTP 经 `git` CLI subprocess 实现（gix 已发布 crate 无服务端协议能力） |
| [ADR-0007](ADR-0007-agent-write-and-production-delivery-boundaries.md) | Accepted | Agent 写入 Policy/Scoped Token 边界与单一 Embedded Frontend 生产交付 |
| [ADR-0008](ADR-0008-repo-centric-interaction-architecture.md) | Accepted | Repo-centric 主流程、首次使用、Repo 内导航与能力发现的信息架构 |
| [ADR-0009](ADR-0009-no-prelaunch-registry-compatibility.md) | Accepted | 未上线阶段取消旧 Registry 双写/迁移兼容，改为 Repo-derived 承接后直接删除 |
| [ADR-0010](ADR-0010-final-production-convergence-after-product-completion.md) | Accepted | EVO-118-E 在目标产品开发与清理完成后执行；DEPLOY-01 只阻止上线，不阻止继续开发 |

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
