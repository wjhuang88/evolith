# EVO-117 当前架构与依赖文档基线校准

- **类型**：Governance / Documentation
- **状态**：Done
- **优先级**：P1
- **触发**：2026-07-29 仓库现状审查
- **完成**：2026-07-29

## 工程目标

修复入口与 reference 文档中的事实漂移，使开发者和 Agent 不再依据旧的 Registry / Sandbox 架构、过期工具链版本或错误的部署形态开展工作。

## 已确认的问题

1. `README.md` 声明 Rust 1.75 / 1.82，但 Workspace 与 Builder 镜像要求 Rust 1.88。
2. `README.md` 的 clone 示例仍使用占位地址。
3. `ARCHITECTURE.md` 将当前系统描述为微服务，并以旧 Tool / Skill / Snippet Registry 与 Sandbox 为主体。
4. `TECH-STACK.md` 中 React、TypeScript、Tailwind、ESLint、SQLx 等版本与实际 Manifest 不一致。
5. `AGENTS.md` 前半段已标记 Sandbox 为 legacy，后半段仍把服务端 Sandbox 执行描述为当前 Skill 主路径，并保留过期阶段状态。
6. `PROJECT-MAP.md` 未列出主线 `service-git` 和 Git 文件系统部署边界。

## 实施范围

- 校准 `README.md` 的定位、成熟度、版本、架构和 clone 示例。
- 重写 `docs/reference/ARCHITECTURE.md`，以模块化单体和 Git-centric 当前实现为基线。
- 重写 `docs/reference/TECH-STACK.md`，以 Manifest / Lockfile / Dockerfile / CI 为版本事实源。
- 精简并校准 `AGENTS.md`，保留流程硬约束与 Task Router，移除过期 Phase 状态。
- 更新 `docs/reference/PROJECT-MAP.md`，补齐 `service-git`、Smart HTTP / gix 分工和文件系统边界。
- 在 `docs/backlog/PRODUCT-BACKLOG.md` 增加 EVO-117 Done 路由行。
- 不修改依赖版本、运行时代码、数据库、脚本行为或 API 合约。

## 验收结果

- [x] Rust 基线统一为 1.88，与 `backend/Cargo.toml` 和 `backend/Dockerfile` 一致。
- [x] clone 示例改为 `wjhuang88/evolith` 实际地址。
- [x] 当前部署形态统一描述为模块化单体，而非多个独立微服务。
- [x] Git Smart HTTP subprocess 与 `gix` Repo Context 的职责明确分离。
- [x] Git-centric 主线、已落地能力、在建能力与 legacy Sandbox 边界明确区分。
- [x] 前后端版本与 `Cargo.toml` / `package.json` 同步。
- [x] `AGENTS.md` 不再把 Sandbox 执行描述为新功能主路径。
- [x] Product Backlog 只新增 EVO-117 一行，未改写历史索引。

## 验证记录

| 检查 | 结果 |
|------|------|
| `backend/Cargo.toml` / `backend/Dockerfile` 对照 | Rust 1.88 一致 |
| `frontend/package.json` 对照 | React 19.2.7、Vite 8.0.16、TypeScript 6.0.3、Tailwind 4.3.0、Bun 1.3.14 已同步 |
| `service-git/src/lib.rs` 对照 | Smart HTTP subprocess、gix Context、资源上限与文件系统路径已同步 |
| ADR-0004 / ADR-0005 / ADR-0006 | 三项均为 Accepted，文档表述与决策一致 |
| `main...branch` compare | 仅文档和 Backlog 变更；Product Backlog 为 `+1/-0` |
| 相对链接检查 | 新增链接目标已通过仓库文件读取确认；本环境无本地 checkout，未运行 DOC-CHECK 的 Python 脚本 |
| 代码测试 | 不适用；本轮没有代码、配置、脚本、数据库或 API 行为变更 |

## 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 修复仓库审查发现的确定性文档问题 |
| 产物 | README、AGENTS、Architecture、Project Map、Tech Stack、Product Backlog 与本 item |
| 状态同步归口 | Product Backlog + 本 item，均为 Done |
| 验证证据 | Manifest / Dockerfile / CI / service-git / ADR 对照与分支 compare |
| 残余工作归口 | 代码层 Sandbox 删除仍归 EVO-111；产品功能仍按 EVO-112/105/106/107/104/108 推进 |

## 完成声明

**Complete**：授权范围内的文档事实漂移已修复，Backlog 已同步，适用验证已记录；没有修改运行时代码或产品行为。
