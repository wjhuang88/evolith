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
