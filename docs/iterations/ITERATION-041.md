# Iteration 041: ExecutionProvider 统一 trait + Docker 容器池化

> 文档状态：Closed（2026-06-04）
> 计划发布日期：2026-06-04
> 计划目标：完成 EVO-045-A，建立统一执行接口和容器池基础设施，解锁 CLI/MCP 共用执行层。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 定义统一 `ExecutionProvider` trait 与请求/响应结构，覆盖 Code / Command / HttpProxy 三类执行载荷。
- 将 Phase 7 Docker sandbox 可复用配置接入新执行接口。
- 建立本地 Docker 容器池生命周期：预热、借用、归还、超时回收和错误状态。
- 保留现有 `SkillExecutor` / `ToolExecutor` 行为，通过 facade 或适配层迁移。
- 解锁 EVO-045 后续 CLI endpoint、EVO-047 MCP serverless 执行。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-045-A | ExecutionProvider 统一 trait + Docker 容器池化 | EVO-045 | P0 | EVO-048 Done ✅ |

## 3. 发布计划基线：不做事项

- 不实现 CLI 执行 endpoint（归 EVO-045 后续 story）。
- 不实现 MCP serverless tool（归 EVO-047）。
- 不实现 Vercel 或远程函数部署（远期）。
- 不改数据库 schema（除非后续 CLI endpoint story 明确需要）。
- 不升级 bollard（EVO-061 独立处理，除非实施中阻塞）。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用命令级验收。
- [ ] 后端存在统一执行接口（`ExecutionProvider` trait），现有 skill/tool executor 可通过适配层编译接入或明确保持兼容边界。
- [ ] 容器池实现有资源限制、超时和失败清理路径。
- [ ] 单元测试覆盖成功执行、超时、容器借还、池耗尽和 Docker 不可用错误态。
- [ ] `cargo test --workspace` 或风险匹配的 crate 级测试通过。
- [ ] 如本机无 Docker，必须记录未跑实测的原因，并保留 mock / unit 证据。

## 5. 发布计划基线：计划验证

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p service-skill
cargo test -p service-tool
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| bollard 0.17 API 不支持池化 | 评估 EVO-061 是否必须前置；如阻塞则用 mock 实现池逻辑 |
| 容器池并发竞争 | 使用 `tokio::sync::Semaphore` 或 channel-based 调度 |
| 现有 handler 代码改动过大 | Phase 1 facade 策略：保持 SkillExecutor/ToolExecutor 不变 |
| 本机无 Docker 无法集成测试 | 用 mock Docker client + unit test 覆盖池逻辑 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 统一执行接口 + 容器池基础设施 |
| 产物 | ExecutionProvider trait + DockerSandboxProvider + HttpProxyProvider + 容器池 + 测试 |
| 状态同步归口 | EVO-045-A、Iteration 041、iterations/README.md |
| Story/BDD 归口 | Technical Story；命令级验收 |
| 验证证据 | cargo check/clippy/test + 单元测试覆盖 |
| 残余工作归口 | CLI endpoint 归 EVO-045 后续；MCP serverless 归 EVO-047；bollard 升级归 EVO-061 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-04 | activation | 状态 → Active / In Progress。前置：Iteration 040 Closed，无 Active/Review 迭代。EVO-045-A Ready → In Progress。委托 deep agent 实施。 |
| 2026-06-04 | execution | Deep agent 完成实施（17m 15s）：创建 `common/src/execution.rs`（410 行，统一 trait + 类型）+ `service-skill/src/docker_sandbox_provider.rs`（612 行，容器池化）+ `service-tool/src/http_proxy_provider.rs`（249 行，HTTP 转发）；修改 10 个文件（lib.rs/Cargo.toml/executor.rs/state.rs/main.rs/测试文件）；新增 27 个单元测试。 |
| 2026-06-04 | closure | 独立验证通过：cargo check 0 errors / clippy 0 errors 0 warnings / cargo test 309 passed（基线 282 + 新增 27）。状态 → Closed。 |
| 2026-06-05 | regression-fix | 用户指出本地启动依赖 Docker。根因：EVO-056/041 的 fail-fast 行为保留了 `sandbox.enabled=true` 默认值，导致默认/lite 启动也进入 Docker provider prewarm。修复：`sandbox.enabled`、`.env.development`、`.env.example` 默认改为 `false`；显式 `SANDBOX__ENABLED=true` 时仍 fail-fast。归口 EVO-079。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
  - 创建 `common/src/execution.rs`（410 行）：统一 `ExecutionProvider` trait + `ExecutionCaller`/`ExecutionPayload`/`ExecutionConstraints`/`ExecutionContext`/`ExecutionResponse` 类型 + `CompositeProvider` 路由
  - 创建 `service-skill/src/docker_sandbox_provider.rs`（612 行）：容器池化实现，使用 `OwnedSemaphorePermit` 实现 borrow/return 语义，支持预热/空闲超时/最大生命周期/冷启动回退
  - 创建 `service-tool/src/http_proxy_provider.rs`（249 行）：HTTP 转发实现，支持 7 种 HTTP 方法
  - 修改 10 个文件：lib.rs/Cargo.toml/executor.rs/state.rs/main.rs/测试文件
  - 新增 `SkillExecutorAdapter` 和 `ToolExecutorAdapter`（facade 模式，保持向后兼容）
  - `AppState` 添加 `execution_provider` 字段
  - `main.rs` 初始化 `DockerSandboxProvider` → `CompositeProvider` → 适配器连接
  - 新增 27 个单元测试（common 7 + docker_sandbox 8 + skill_adapter 3 + tool_adapter 3 + http_proxy 6）
  - 2026-06-05 follow-up：sandbox 默认关闭，恢复 lite/local 启动不依赖 Docker；显式启用仍 fail-fast
- 未完成：无
- 验证结果：
  - `cargo check --workspace` → 0 errors ✅
  - `cargo clippy --workspace --all-targets -- -D warnings` → 0 errors, 0 warnings ✅
  - `cargo test --workspace` → 309 passed, 0 failed（基线 282 + 新增 27）✅
  - 2026-06-05 follow-up：`cargo test -p infra config` / `cargo check --workspace` / governance validator / `git diff --check` 通过
- 闭环状态：`Complete`
- 残余归口：
  - 真实 Docker 集成测试 → 延期到 CI 环境（需要 Docker daemon）
  - 健康检查后台任务 → 延期到 EVO-045-B 或后续优化
  - `DockerExecutor`（per-request）清理 → 延期到 Phase 2（handler 直接迁移后删除）

## 11. Retrospective

- 做得好的：
  - Facade 模式迁移策略成功：保留现有 `SkillExecutor`/`ToolExecutor` trait，通过适配器委托，避免大规模重构
  - `OwnedSemaphorePermit` 解决了 `SemaphorePermit<'_>` 的生命周期问题，实现跨函数边界的 permit 传递
  - 容器池化设计完整：预热/borrow/return/超时/清理/冷启动回退，覆盖所有关键路径
  - 测试覆盖全面：27 个单元测试覆盖 trait contract、池生命周期、适配器转换、错误路径
  - 所有测试不需要 Docker daemon（全部 mock），本地开发友好
- 需要调整的：
  - Deep agent 耗时 17m 15s，超出预期（10-15m），主要因为 `SemaphorePermit` 生命周期问题的调试和修复
  - 健康检查后台任务未实现，当前依赖 borrow 时清理，可能不够及时
  - fail-fast 不能和默认启用混在一起；默认配置必须匹配 lite/local 启动不依赖 Docker 的开发约束
- 写入 EVOLUTION：2026-06-05 已更新 sandbox 速查口径。
