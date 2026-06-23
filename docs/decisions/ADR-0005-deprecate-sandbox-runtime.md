# ADR-0005: 废弃 Skill 沙箱执行，仅保留 FaaS 形态

## 状态

Accepted

## 背景

Evolith 当前实现 Skill 执行采用 Docker 容器沙箱：

- `service-skill/src/docker_executor.rs`（per-request 容器生命周期）
- `service-skill/src/docker_sandbox_provider.rs`（容器池化）
- `backend/sandbox/{python,node}/Dockerfile`（沙箱镜像）
- `bollard` crate（Docker API 客户端）
- `SANDBOX__ENABLED` 环境变量（默认 false，EVO-079 修复）

`common/src/execution.rs` 定义了统一 `ExecutionProvider` trait，意图把 Skill（Code payload）、CLI（Command payload）、MCP（HttpProxy payload）三种执行路径抽象到一个接口。

2026-06-23 用户反馈明确：

1. 不再提供 skill 执行能力。
2. 不提供智能体执行能力（agent engine 外部）。
3. 保留 mcp 作为 FaaS（HttpProxy）。
4. CLI 作为 FaaS 注册（无 sandbox 执行）。

详见 `docs/proposals/GIT-CENTRIC-PLATFORM.md`。

## 选项

| 选项 | 优点 | 缺点 |
|------|------|------|
| **A. 维持 sandbox 现状** | Phase 7 工作保留 | 与新方向冲突；运维负担（Docker daemon）；agent 自带执行环境后无意义 |
| **B. 删除 sandbox 与 ExecutionProvider 中 Skill 分支，保留 HttpProxy** | 与新方向一致；体积 / 启动时间改善；运维简化 | 已实现代码丢弃；部分集成测试需迁移或删除 |
| **C. sandbox 作为可选项（feature flag）** | 保留迁移路径 | 增加维护负担；与"明确删除"语义冲突 |

## 决策

采用 **B**：删除 skill 沙箱执行，ExecutionProvider 简化为仅 HttpProxy。

具体动作：

- 删除 `backend/crates/service-skill/src/executor.rs`（`SkillExecutor` trait + `DefaultSkillExecutor` + `SkillExecutorAdapter`）。
- 删除 `backend/crates/service-skill/src/docker_executor.rs`。
- 删除 `backend/crates/service-skill/src/docker_sandbox_provider.rs`。
- 删除 `backend/sandbox/` 目录及 Dockerfiles。
- 从 `service-skill/Cargo.toml` 移除 `bollard` 依赖。
- 从 `infra/config.rs` 移除 `SandboxConfig`；从环境变量移除 `SANDBOX__*` 全部字段。
- 从 `common/execution.rs` 移除 `ExecutionCaller::Skill` 变体；简化 `ExecutionProvider` 为扁平 HttpProxy 抽象（或直接内联到 mcp handler）。
- 从 `AppState` 移除 `skill_executor` 字段。
- 删除 `POST /api/v1/skills/{id}/execute` route + handler。
- 删除前端 `skillsApi.execute()` 与 "执行" 按钮 UI。
- Sandbox 相关集成测试归类 `[ignore]` 或删除。

## 后果

### 正面

- 与 2026-06-23 方向调整一致。
- 删除 bollard 依赖（compile time 改善 ~20s）。
- 删除 `backend/sandbox/` 镜像构建负担；CI / CD 简化。
- 启动时间缩短（无需 Docker daemon prewarm 检查）。
- `ExecutionProvider` trait 简化（HttpProxy 单一实现）。
- 用户运维负担降低：本地 / lite 启动不依赖 Docker。

### 负面

- 已投入 Phase 7 sandbox 工作丢弃（~500 行 service-skill 执行代码）。
- 旧 Skill 执行测试（`DockerExecutor`、`RuntimePool`）删除。
- CLI FaaS 执行必须找到 HttpProxy 替代（handler 注册 + URL 代理）。
- 已注册 skills 失去 "在 Evolith 内执行" 能力；用户需自行托管。

### 缓解

- Phase 4 排 EVO-111 专项清理；明确 git commit 标 `refactor:`。
- 旧 Skill 执行能力可用"用户在仓库里 vibe coding + agent 外部执行"替代；不丢失核心价值。
- 测试删除前用 `#[ignore = "sandbox removed per ADR-0005"]` 标注，便于追溯。
- EVOLUTION.md 记录执行环境变化与迁移路径。

## 风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 部分集成测试在 sandbox 删除后失败 | `cargo test --workspace` 红 | 删除前 [ignore] 标注，渐进清理 |
| 用户已有依赖 skill 执行的 workflow | 中断 | release note 强调；建议改用 agent 外部执行 |
| CLI FaaS 执行路径未实现 | 用户无法执行 CLI interface | EVO-105 中明确 CLI 注册为 HttpProxy handler URL，由用户自托管 |
| Sandbox fail-fast 行为（EVO-056/EVO-079）丢失 | 失去 Docker executor 失败的快速反馈 | 新方向无 Docker 依赖，问题自然消失 |

## 不做

- 不重新引入 sandbox 形态（除非 EVO-080 Wasmer/WASI Spike 结论明确支持）。
- 不保留 sandbox 作为 feature flag（明确删除）。
- 不为已删除 sandbox 创建 deprecation shim（直接删除，避免误导）。

## 相关链接

- [Git-Centric Platform Proposal](../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0004 Git-Centric Storage](ADR-0004-git-centric-storage.md)
- [EVO-111 废弃 Skill 沙箱执行](../backlog/active/EVO-111-废弃-skill-沙箱执行.md)
- [SERVERLESS-RUNTIME.md](../proposals/SERVERLESS-RUNTIME.md)（历史参考，简化后保留）
- [EVO-056 沙箱降级静默成功修复](../backlog/archive/2026-Q2/EVO-056-沙箱降级静默成功修复.md)
- [EVO-079 sandbox 默认启用导致本地启动依赖 Docker 回归修复](../backlog/archive/2026-Q2/EVO-079-sandbox-默认启用导致本地启动依赖-docker-回归修复.md)
- [EVOLUTION.md](../../EVOLUTION.md)（2026-06-23 方向变更记录）
