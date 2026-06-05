# EVO-045-A ExecutionProvider 统一 trait + Docker 容器池化

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P0
- Source: EVO-045 split / EVO-048 输出 / Iteration 041
- Decision Context: 2026-06-04 完成；2026-06-05 回归修复：sandbox 默认关闭，lite/local 启动不依赖 Docker，显式启用仍 fail-fast

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P0
- 状态：Done
- 父 Epic：EVO-045
- Story 形态：Technical
- 用户价值或技术目标：把 EVO-048 Spike 输出的统一执行边界落成可测试的后端基础设施，先解锁 CLI/MCP 共用执行层，再进入具体 CLI endpoint。
- 范围：
  - 定义统一 `ExecutionProvider` trait 与请求/响应结构，覆盖 Code / Command / HttpProxy 三类执行载荷。
  - 将 Phase 7 Docker sandbox 可复用配置接入新执行接口。
  - 建立本地 Docker 容器池生命周期：预热、借用、归还、超时回收和错误状态。
  - 保留现有 `SkillExecutor` / `ToolExecutor` 行为，通过 facade 或适配层迁移。
- 不做：
  - 不实现 CLI 执行 endpoint。
  - 不实现 MCP serverless tool。
  - 不实现 Vercel 或远程函数部署。
  - 不改数据库 schema，除非后续 CLI endpoint story 明确需要。
- 验收标准：
  - 非行为类：
    - [ ] 后端存在统一执行接口，现有 skill/tool executor 可通过适配层编译接入或明确保持兼容边界。
    - [ ] 容器池实现有资源限制、超时和失败清理路径。
    - [ ] 单元测试覆盖成功执行、超时、容器借还、池耗尽和 Docker 不可用错误态。
    - [ ] `cargo test --workspace` 或风险匹配的 crate 级测试通过。
    - [ ] 如本机无 Docker，必须记录未跑实测的原因，并保留 mock / unit 证据。
- 技术备注：
  - 直接承接 `docs/proposals/SERVERLESS-RUNTIME.md` §4 / §5 / §8。
  - EVO-061（bollard 升级）如在实施中阻塞，需要先提升优先级或在本 story 中显式处理。
- 依赖或阻塞：EVO-048 Done；建议先评估 EVO-061 是否必须前置。
- 解锁内容：EVO-045 后续 CLI endpoint、EVO-047 MCP serverless 执行。
- 影响范围：backend service-skill / service-tool / domain 或 common execution 边界 / tests / docs。
- 最小验证方式：执行接口与容器池单元测试；必要时 Docker 集成测试；`cargo test` 相关 crate。
