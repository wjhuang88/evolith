# Iteration 031: 前端静态服务迁移到 rust-embed-for-web

> 文档状态：Active
> 计划发布日期：2026-05-28
> 计划目标：用 rust-embed-for-web + actix-web-rust-embed-responder 替换 ZIP 嵌入方案，
> 实现零拷贝、预压缩、自动 HTTP 缓存协商；移除 embedded-frontend feature flag。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。
> 改线来源：Iteration 030（ZIP 流式方案）经调研确认为过度工程，保留原计划基线。
> 架构决策：[ADR-0003](../decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md)。

## 1. 发布计划基线：目标

- 用 `rust-embed-for-web` 替换 `zip` crate，实现编译时嵌入 + 构建时预压缩 + 零拷贝访问。
- 用 `actix-web-rust-embed-responder` 替换手动 ETag/304/Content-Type 逻辑，自动处理 HTTP 缓存协商。
- 移除 `embedded-frontend` feature flag — Debug/Release 行为由 crate 内置处理。
- 移除 `frontend.zip` 占位文件和所有 ZIP 相关代码。
- 保留 `/config.js` 动态生成和 SPA fallback。
- 保留 `Cache-Control` 手动设置（responder 不处理此项）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-016-B | 前端静态服务迁移到 rust-embed-for-web | EVO-016 | P0 | EVO-016-A 已完成（将被替换） |

### Iteration Inventory Disposition

| Iteration | 当前状态 | 处置结论 |
|-----------|----------|----------|
| Iteration 012 | Planned / Blocked | 继续阻塞，等待 EVO-016-B 完成后更新激活条件 |
| Iteration 030 | Superseded | ZIP 流式方案改线到本迭代；保留原计划基线 |
| 其他 | 各自状态 | 不激活；优先级低于 EVO-016-B (P0) |

## 3. 发布计划基线：不做事项

- 不修改 `/config.js` 动态生成逻辑。
- 不修改 API 路由、中间件或业务逻辑。
- 不重建 GitHub CI/CD（归 EVO-030）。
- 不修改前端构建流程或 Dockerfile。
- 不启用 zstd 预压缩（需 C 绑定，gzip + brotli 覆盖主流浏览器）。
- 不升级其他后端依赖（归 EVO-043）。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] 候选为 Technical Story，使用等价技术验收。
  - [x] 行为类 BDD 不适用；验收以技术指标和命令级证据为准。
- [ ] `zip` crate 依赖和 `frontend.zip` 已从 `backend/` 移除。
- [ ] `embedded-frontend` feature flag 已移除；`cargo check --workspace` 无需 `--features`。
- [ ] `rust-embed-for-web` 和 `actix-web-rust-embed-responder` 已添加到 `backend/Cargo.toml`。
- [ ] `cargo test --workspace` 通过。
- [ ] `cargo clippy --workspace -- -D warnings` 通过。
- [ ] 哈希文件名资源返回 `Cache-Control: public, max-age=31536000, immutable`。
- [ ] `index.html` 返回 `Cache-Control: no-cache`。
- [ ] 响应包含 ETag header（SHA-256 + Base85）。
- [ ] 支持 `If-None-Match` 条件请求，返回 304 Not Modified。
- [ ] 客户端支持 brotli 时返回 `Content-Encoding: br`；支持 gzip 时返回 `Content-Encoding: gzip`。
- [ ] SPA fallback：未知路径返回 `index.html` 200 响应。
- [ ] `/config.js` 仍由环境变量动态生成。
- [ ] `frontend.rs` 代码量 ≤ 80 行。

## 5. 发布计划基线：计划验证

```bash
# backend
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo check --workspace

# 手工验证 (release build)
# curl -v http://localhost:8080/index.html
#   → 检查 Cache-Control: no-cache, ETag, Content-Type
# curl -v http://localhost:8080/assets/index-xxx.js
#   → 检查 Cache-Control: immutable, Content-Encoding: br/gzip
# curl -H "If-None-Match: <etag>" http://localhost:8080/index.html
#   → 304 Not Modified
# curl http://localhost:8080/config.js
#   → 动态生成的 JS 配置
# curl http://localhost:8080/unknown-spa-route
#   → 返回 index.html (SPA fallback)
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| `rust-embed-for-web` proc macro 在 workspace 中行为异常 | 先在独立小项目验证，再集成 |
| `actix-web-rust-embed-responder` 与项目 actix-web 版本不兼容 | 检查版本兼容矩阵；必要时手动实现 Responder |
| Debug 模式下 `frontend/dist/` 不存在导致编译失败 | 开发文档中注明：嵌入模式需要先 `bun run build` |
| 二进制体积增长过大 | 可通过 `#[gzip = false]` 或 `#[br = false]` 关闭部分预压缩 |
| `deny(clippy::unwrap_used)` 与 responder 内部冲突 | 检查 responder 是否有 unsafe/unwrap；必要时 allow |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 零拷贝 + 预压缩 + 自动缓存协商的前端静态服务 |
| 产物 | 重写的 `backend/src/frontend.rs`、更新的 `backend/Cargo.toml`、更新的 `backend/src/main.rs` |
| 状态同步归口 | EVO-016-B backlog、Iteration 012 激活条件、Iteration 030 Superseded 记录 |
| Story/BDD 归口 | Technical Story；等价技术验收 |
| 验证证据 | cargo test/clippy、curl 响应头验证 |
| 残余工作归口 | Docker 单容器 Dockerfile 归 Iteration 012；CI/CD 归 EVO-030；依赖升级归 EVO-043 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | activation | Iteration 030 ZIP 流式方案经多方调研确认过度工程，改线到 rust-embed-for-web 方案。新建 Iteration 031。EVO-016-B backlog 已更新。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：
- 未完成：
- 验证结果：
- 闭环状态：`Active`（刚激活）
- 残余归口：

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
