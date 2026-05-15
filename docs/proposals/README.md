# 远期提案

本目录用于存放尚未满足 Backlog 进入条件的想法和方案。

## 为什么需要独立的提案区

Evolith 的执行者是 AI Agent。Agent 从 Backlog 选取任务时会直接开工，无法像人类那样自行判断"这个需求还不够成熟，先不动"。因此：

- **Backlog** = 可执行指令源，所有条目必须满足 DoR，Agent 可以直接拿起来做。
- **Proposals** = 想法暂存区，Agent 不从中选取任务。只有经过人工确认和细化后，才能晋升到 Backlog。

这对应敏捷实践中的 "holding tank" 模式（Mike Cohn）——保持活跃 Backlog 精简可执行，未准备好的项暂存在别处。

## 晋升流程：Proposal → Backlog

一个提案满足以下所有条件时，可以晋升为 Backlog 条目：

1. **有明确的用户价值或技术目标**：能回答"为什么要做"。
2. **范围足够小**：能在一个迭代（0.5-2 天）内完成；过大的提案需先拆分。
3. **有至少一条验收标准**：可验证、可判断完成。
4. **依赖已识别**：知道前置条件是什么（即使尚未满足）。
5. **需要改哪些层已明确**：backend / frontend / db / docs / deploy。

晋升操作：

1. 在 `docs/backlog/PRODUCT-BACKLOG.md` 新增条目，分配 EVO-ID，填写完整字段。
2. 在本目录提案中标注晋升状态（如"已晋升为 EVO-XXX"）或直接移除条目行。
3. 提案文档本身保留在 `docs/proposals/` 作为背景参考，由 Backlog 条目链接引用。

## 提案状态

| 状态 | 含义 |
|------|------|
| 远期想法 | 刚提出，方向性描述，尚未展开 |
| 待整理 | 需要补充技术方案或细化范围 |
| 远期目标 | 方向已确认，等待前置条件成熟 |
| 已晋升 | 已拆分为 Backlog 条目（注明 EVO-ID） |
| 已废弃 | 明确不再推进 |

## 当前提案

| 事项 | 状态 | 下一步 |
|------|------|--------|
| API gap checkpoint | 待整理 | 汇总 API 合约中的 501、TODO 和前端占位动作 |
| Database migration SOP | 待整理 | 固化 SQLite/PostgreSQL 双轨 migration 流程 |
| Security model reference | 待整理 | 集中说明 JWT cookie、CSRF、RBAC、CORS 和安全头 |
| [Evolith Rust CLI](RUST-CLI.md) | 远期目标 | API 合约稳定后启动 CLI 子项目 |
| [前端嵌入后端](EMBEDDED-FRONTEND.md) | 远期目标 | 完成 React + Vite + Bun 迁移后实施 |
| [Artifact Repository](ARTIFACT-REPOSITORY.md) | 远期目标 | snippet/skill 稳定后再定义制品元数据和存储模型 |
| [AI Gateway](AI-GATEWAY.md) | 远期想法 | 核心平台稳定后展开技术方案评审 |
| [Agent Runtime](AGENT-RUNTIME.md) | 远期想法 | 核心功能稳定后定义工作空间模型和执行方案 |
