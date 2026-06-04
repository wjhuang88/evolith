# Iteration 038: 假可用快速失败修复

> 文档状态：Closed（2026-06-04）
> 计划发布日期：2026-06-04
> 计划目标：收敛两个会把不可用状态伪装成可用的后端路径：MySQL 半接线和沙箱初始化失败静默降级。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- `EVO-052`：MySQL 未实现 repository 的事实在配置/连接池边界快速失败，不再连接后才在 `main.rs` 拒绝。
- `EVO-056`：沙箱启用时 Docker executor 初始化失败必须显式失败；默认 executor 不再返回 `exit_code:0` 的伪成功。
- 保持 `SANDBOX__ENABLED=false` 的显式禁用语义不变。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-052 | MySQL 半接线收敛与快速失败 | 无 | P2 | 无 |
| EVO-056 | 沙箱降级静默成功修复 | 无 | P2 | 无 |

## 3. 发布计划基线：不做事项

- 不实现 MySQL repository。
- 不移除 Cargo workspace 的 `sqlx/mysql` feature；依赖层清理另行评估。
- 不实现新的非 Docker skill executor。
- 不改 skill 执行 API contract 的成功响应结构。
- 不处理 CORS 配置化（EVO-057）或前端清理（EVO-058）。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical / Bug；BDD 不适用，使用配置失败态、执行失败态和命令级验证。
- [x] `AppConfig::validate()` 拒绝 `database.database_type=mysql`，错误信息明确说明 MySQL repositories 未实现。
- [x] `create_pool()` 对 `mysql` 快速返回配置错误，不建立 MySQL 连接池。
- [x] `main.rs` 不再包含 `DatabasePool::MySql` 后置拒绝分支。
- [x] `SANDBOX__ENABLED=true` 且 Docker executor 初始化失败时，服务启动返回配置错误。
- [x] `DefaultSkillExecutor::execute()` 返回明确配置错误，不返回 `exit_code:0` 伪成功。
- [x] 针对配置、连接池与默认执行器失败态新增测试。

## 5. 发布计划基线：计划验证

```bash
cargo test -p infra config
cargo test -p infra mysql
cargo test -p service-skill executor
cargo check --workspace
cargo test --workspace
git diff --check
sh /Users/GHuang/WorkSpace/AiProjects/skill-sources/agent-project-governance/skills/agent-project-governance/scripts/validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 误伤显式禁用 sandbox 的开发路径 | 保留 `SANDBOX__ENABLED=false` 时使用 default executor，但执行接口仍由 handler 先返回 disabled |
| MySQL feature 仍存在导致误读 | 配置文档和 backlog 记录明确主服务拒绝 MySQL；依赖 feature 是否移除另建任务 |
| Docker 不可用导致本地默认启动失败 | 这是本 story 的目标语义；本地无需执行 skill 时应设 `SANDBOX__ENABLED=false` |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 选择不放心外包的风险任务并亲自处理：EVO-052 / EVO-056 |
| 产物 | 配置校验、连接池分支、默认执行器、main 初始化逻辑、测试、CONFIG/BOARD/backlog/iteration 同步 |
| 状态同步归口 | EVO-052 / EVO-056、Iteration 038、docs/BOARD.md、docs/reference/CONFIG.md |
| Story/BDD 归口 | 技术/缺陷 story；BDD 不适用，使用错误态与命令级验证 |
| 验证证据 | `cargo test -p infra config`、`cargo test -p service-skill executor`、`cargo check --workspace`、governance validator、`git diff --check` |
| 残余工作归口 | EVO-057 CORS 配置化；EVO-058 前端类型卫生；MySQL 依赖 feature 深度移除如需要另建 story |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-04 | inventory | 当前无 Active / Review iteration；Iteration 034 为 Planned / Ready，但本轮选择 P2 风险微迭代插队，原因是两项均为"假可用/假成功"防呆，适合先收敛；Iterations 018-020、025-027 继续 Blocked；Iteration 028 保留 Ready。 |
| 2026-06-04 | activation | EVO-052 / EVO-056 状态 `Ready` → `In Progress`；创建 Iteration 038。 |
| 2026-06-04 | implementation | EVO-052：`config.rs` 拒绝 MySQL，`db/pool.rs` 不再打开 MySQL 连接池，`main.rs` 后置 MySql 分支移除。EVO-056：沙箱启用时 Docker executor 初始化失败返回 `ConfigError`，`DefaultSkillExecutor` 执行返回 `ConfigError`。 |
| 2026-06-04 | validation | `cargo fmt --all -- --check` 通过；`cargo test -p infra config` 2 passed；`cargo test -p infra mysql` 2 passed；`cargo test -p service-skill executor` 1 passed；`cargo check --workspace` 通过；`cargo test --workspace` 在普通沙箱因本地 mock HTTP server bind 权限失败，提权重跑后全量通过。 |
| 2026-06-04 | closure | EVO-052 / EVO-056 状态 → Done；Iteration 038 → Closed。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- EVO-052：
  - `DATABASE__DATABASE_TYPE=mysql` 在 `AppConfig::validate()` 直接失败。
  - `create_pool()` 对 `mysql` 返回 `ConfigError`，不建立连接池。
  - `DatabasePool::MySql` 和 `main.rs` 后置拒绝分支移除。
  - `docs/reference/CONFIG.md`、`AGENTS.md` 和 `EVOLUTION.md` 同步。
- EVO-056：
  - `SANDBOX__ENABLED=true` 时 Docker executor 初始化失败导致服务启动失败。
  - `DefaultSkillExecutor::execute()` 返回 `ConfigError`，不返回 `exit_code:0` 伪成功。
  - 新增默认 executor 失败态测试。
- 未完成：
- 未移除 Cargo workspace 的 `sqlx/mysql` feature；如要彻底裁剪依赖，另建任务评估。
- 验证结果：
- `cargo fmt --all -- --check`：通过。
- `cargo test -p infra config`：2 passed。
- `cargo test -p infra mysql`：2 passed。
- `cargo test -p service-skill executor`：1 passed。
- `cargo check --workspace`：通过。
- `cargo test --workspace`：普通沙箱下 4 个 MCP mock server bind 测试 PermissionDenied；提权重跑后全量通过。
- 闭环状态：`Complete`
- 残余归口：
- EVO-057：生产 CORS Origin 可配置化。
- EVO-058：前端死代码与类型卫生清理。
- 可选后续：MySQL `sqlx` feature / `db/mysql.rs` 深度移除，如维护者希望完全裁剪依赖。

## 11. Retrospective

- 做得好的：
- 把两个“假可用”问题合并成一个小批次，但没有扩大到 CORS/前端清理。
- 验证时发现 workspace 测试的本地端口绑定需要提权，保留了失败原因和重跑证据。
- 需要调整的：
- 后续如果完全移除 MySQL feature，需要先评估 `sqlx` feature 组合和 lockfile 影响，不能在本轮顺手裁剪。
- 写入 EVOLUTION：
- 已更新 MySQL 不支持与 sandbox fail-fast 的速查表口径。
