# EVO-048 Serverless 执行架构设计 Spike

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: spike
- Status: Done
- Priority: P0
- Source: EVO-045/047 前置 / Iteration 033
- Decision Context: Iteration 033 收口（2026-06-03）：输出 `docs/proposals/SERVERLESS-RUNTIME.md`（11 节）；Phase 7 sandbox 复用结论=部分复用；统一 ExecutionProvider 接口设计；冷启动方法学（已缓存 ~350ms-1800ms / 池模式 ~50-200ms，实测待 Docker）；Vercel 演进路径 + 组件替换清单；EVO-045/047 依赖图 + 推荐实施顺序；本机无 Docker，冷启动实测 conditional

#### Source Detail Snapshot

- 类型：spike
- 优先级：P0
- 状态：Done
- 用户价值或技术目标：为 CLI（EVO-045）和 MCP（EVO-047）设计统一的 serverless 执行架构，确定本地版实现方案和远期 Vercel 模式的演进路径。
- 调研范围：
  1. **Phase 7 sandbox 复用评估**：当前 Docker sandbox（bollard + Python/Node runtime）能否作为 serverless 基础？冷启动延迟、资源开销、并发能力
  2. **本地 serverless runtime 设计**：
     - 函数部署模型：用户上传代码 → 构建镜像 → 按需启动容器
     - 冷启动优化：预热池、容器复用、快照恢复
     - 资源限制：CPU/内存/超时/PIDs（复用现有 sandbox 限制）
     - 并发模型：请求路由、实例扩缩
  3. **外部执行代理设计**：统一抽象层，内部 serverless 和外部 HTTP 调用使用相同接口
  4. **远期 Vercel 模式演进**：哪些组件需要替换（Docker → 真实 serverless 平台），哪些可以保留
- 验收标准：
  - [ ] 输出架构设计文档（ADR 或 proposal），包含本地版实现方案和演进路径
  - [ ] Phase 7 sandbox 复用可行性结论（复用 / 部分复用 / 替换）
  - [ ] 冷启动延迟基准测试（目标 < 2s）
  - [ ] 外部执行代理接口设计
  - [ ] 远期 Vercel 模式的组件替换清单
- 依赖或阻塞：无
- 影响范围：docs（ADR/proposal）
- 最小验证方式：设计文档通过 review；冷启动基准测试有数据
- 不做：不实现 serverless runtime（仅设计）；不实现 Vercel 完整模式
