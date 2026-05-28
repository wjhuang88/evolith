# Iteration 030: ZIP 嵌入前端流式响应与静态索引

> 文档状态：Active
> 计划发布日期：2026-05-28
> 计划目标：将 ZIP 嵌入前端服务从 Vec<u8> 全量读取改造为纯流式响应 + 静态索引，
> 利用 ZIP 流式解压和 Arc<ZipArchiveMetadata> 零重解析，使每个请求内存恒定。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 重写 `backend/src/frontend.rs`：纯流式 HTTP body + 启动时静态文件索引 + 零中央目录重解析。
- 每个请求堆内存峰值 ≤ 16KB + 元数据（不随文件大小增长）。
- 添加 HTTP 缓存头（Cache-Control、ETag、Content-Length）和 304 条件请求支持。
- 同步 IO 通过 `web::block()` 桥接，不阻塞异步运行时。
- 超越 oc-platform 参考实现（其每次请求重解析 ZIP + 全量读取）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-016-B | Embedded Frontend 流式响应与静态索引 | EVO-016 | P0 | EVO-016-A 基础实现已完成 |

### Iteration Inventory Disposition

| Iteration | 当前状态 | 处置结论 |
|-----------|----------|----------|
| Iteration 012 | Planned / Blocked | 继续阻塞，等待 EVO-016-B 完成后更新激活条件 |
| Iteration 017 | Planned / Ready for activation | 不激活；优先级低于 EVO-016-B (P0) |
| Iteration 024 | Planned / Ready for activation | 保留原 refinement 计划；本迭代目标为实现，使用新编号 030 |
| Iteration 028 | Planned / Ready for activation | 不激活；优先级低于 EVO-016-B (P0) |

## 3. 发布计划基线：不做事项

- 不预解压所有文件到内存。
- 不修改 `/config.js` 动态生成逻辑。
- 不修改 API 路由、中间件或业务逻辑。
- 不修改前端构建流程或 Dockerfile。
- 不重建 GitHub CI/CD（归 EVO-030 backlog EVO-030）。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] 候选为 Technical Story，使用等价技术验收。
  - [x] 行为类 BDD 不适用；验收以技术指标和命令级证据为准。
- [ ] `OnceLock<FrontendIndex>` 仅初始化一次；后续请求 O(1) 路径查找。
- [ ] 每个请求不再调用 `ZipArchive::new()` 重新解析中央目录。
- [ ] 响应 body 为 `HttpResponse::streaming()`，chunk size ≤ 16KB。
- [ ] 任何单个文件请求的堆内存峰值 ≤ 16KB + 元数据。
- [ ] 哈希文件名资源返回 `Cache-Control: public, max-age=31536000, immutable`。
- [ ] `index.html` 返回 `Cache-Control: no-cache`。
- [ ] 所有响应包含 `Content-Length` 头。
- [ ] 支持 `If-None-Match` 条件请求，返回 304 Not Modified。
- [ ] 同步 IO 通过 `web::block()` 执行。
- [ ] `cargo test --workspace --features embedded-frontend` 通过。
- [ ] `cargo clippy --workspace --features embedded-frontend -- -D warnings` 通过。

## 5. 发布计划基线：计划验证

```bash
# backend
cargo test --workspace --features embedded-frontend
cargo clippy --workspace --features embedded-frontend -- -D warnings
cargo check --workspace --features embedded-frontend

# 手工验证
# curl -v http://localhost:8080/index.html → 检查 Cache-Control, Content-Length, ETag
# curl -H "If-None-Match: <etag>" http://localhost:8080/index.html → 304
# curl -v http://localhost:8080/assets/index-xxx.js → immutable cache
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| `unsafe_new_with_metadata` API 限制或行为不符预期 | 先用 `OnceLock<Arc<ZipArchiveMetadata>>` + 每次新建 archive 方案兜底 |
| `web::block()` 中 ZIP 解压阻塞线程池 | 调整 actix-web 线程池大小或验证前端文件均较小（< 1MB） |
| 流式响应与 actix-web 中间件不兼容 | 回退到 `Vec<u8>` 方案但保留索引优化 |
| workspace `deny(clippy::unwrap_used)` 冲突 | 使用 `?` 或显式 match，不使用 `unwrap()` |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 纯流式 ZIP 前端服务，内存恒定，缓存策略完备 |
| 产物 | 重写的 `backend/src/frontend.rs` |
| 状态同步归口 | EVO-016-B backlog、Iteration 012 激活条件、Iteration 030 |
| Story/BDD 归口 | Technical Story；等价技术验收 |
| 验证证据 | cargo test/clippy、curl 响应头验证 |
| 残余工作归口 | Docker 单容器 Dockerfile 归 Iteration 012；CI/CD 归 EVO-030 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | activation | 用户要求将 EVO-016-B 放入迭代并作为下一个开始。新建 Iteration 030，盘点非终态 iteration 后确认无阻塞。 |
| 2026-05-28 | progress | 研究确认 zip crate 2.4.2 无 `metadata()`/`unsafe_new_with_metadata()` API（仅在 zip 8.x 中存在）。采用 OnceLock 静态索引 + 每次请求新建 ZipArchive + by_index() 定位方案。 |
| 2026-05-28 | progress | 重写 `frontend.rs` 完成：OnceLock\<FrontendIndex\> 静态索引、web::block() 同步桥接、16KB chunk 分块读取、HttpResponse::streaming() 流式响应、Cache-Control/ETag/Content-Length/304 条件请求、SPA fallback。 |
| 2026-05-28 | validation | `cargo clippy --workspace --features embedded-frontend -- -D warnings` 通过。`cargo test --workspace --features embedded-frontend` 全部通过（含 21 个 frontend 模块测试）。不带 feature 时 clippy 也通过。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Blocked`（刚激活，尚未开始实现）
- 残余归口：

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
