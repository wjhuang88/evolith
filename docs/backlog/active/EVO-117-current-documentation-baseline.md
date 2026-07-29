# EVO-117 当前架构与依赖文档基线校准

- **类型**：Governance / Documentation
- **状态**：In Progress
- **优先级**：P1
- **触发**：2026-07-29 仓库现状审查

## 工程目标

修复入口与 reference 文档中的事实漂移，使开发者和 Agent 不再依据旧的 Registry / Sandbox 架构、过期工具链版本或错误的部署形态开展工作。

## 已确认的问题

1. `README.md` 声明 Rust 1.75 / 1.82，但 Workspace 与 Builder 镜像要求 Rust 1.88。
2. `README.md` 的 clone 示例仍使用占位地址。
3. `ARCHITECTURE.md` 将当前系统描述为微服务，并以旧 Tool / Skill / Snippet Registry 与 Sandbox 为主体。
4. `TECH-STACK.md` 中 React、TypeScript、Tailwind、ESLint、SQLx 等版本与实际 manifest 不一致。
5. `AGENTS.md` 前半段已标记 Sandbox 为 legacy，后半段仍把服务端 Sandbox 执行描述为当前 Skill 主路径，并保留过期阶段状态。

## 范围

- 校准 `README.md`、`docs/reference/ARCHITECTURE.md`、`docs/reference/TECH-STACK.md`。
- 精简并校准 `AGENTS.md` 的当前项目事实，保留流程硬约束与 Task Router。
- 不修改依赖版本、运行时代码、数据库或脚本行为。

## 验收

- 文档中的 Rust 基线统一为 1.88。
- 当前部署形态统一描述为模块化单体，而非多个独立微服务。
- Git-centric 主线、已落地能力、在建能力与 legacy Sandbox 边界明确区分。
- 技术版本以 `Cargo.toml` / `package.json` / lockfile 为权威来源。
- Markdown 相对链接存在，变更无尾随空白或冲突标记。

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 修复仓库审查发现的确定性问题 |
| 产物 | README、Architecture、Tech Stack、AGENTS 与本条记录 |
| 状态同步归口 | Product Backlog + 本 item |
| 验证证据 | 文档链接检查、旧术语检查、diff review |
| 残余工作归口 | 代码层 Sandbox 删除仍归 EVO-111；产品功能仍按 EVO-112/105/106/107/104/108 推进 |
