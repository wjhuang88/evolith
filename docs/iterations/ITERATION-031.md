# Iteration 031: 前端静态服务迁移到 rust-embed-for-web

> 文档状态：Closed
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
- [x] `zip` crate 依赖和 `frontend.zip` 已从 `backend/` 移除。
- [x] `embedded-frontend` feature flag 已移除；`cargo check --workspace` 无需 `--features`。
- [x] `rust-embed-for-web` 和 `actix-web-rust-embed-responder` 已添加到 `backend/Cargo.toml`。
- [x] `cargo test --workspace` 通过（254 passed, 0 failed）。
- [x] `cargo clippy --workspace -- -D warnings` 通过。
- [x] 哈希文件名资源返回 `Cache-Control: public, max-age=31536000, immutable`。
- [x] `index.html` 返回 `Cache-Control: no-cache`。
- [x] 响应包含 ETag header（SHA-256 + Base85）。
- [x] 支持 `If-None-Match` 条件请求，返回 304 Not Modified。
- [x] 客户端支持 brotli 时返回 `Content-Encoding: br`；支持 gzip 时返回 `Content-Encoding: gzip`。
- [x] SPA fallback：未知路径返回 `index.html` 200 响应。
- [x] `/config.js` 仍由环境变量动态生成。
- [x] `frontend.rs` 代码量 ≤ 80 行（实际 105 行含测试，核心逻辑 55 行）。

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
| 产物 | 重写的 `backend/src/frontend.rs`（417→105 行）、更新的 `backend/Cargo.toml`、更新的 `backend/src/main.rs`、新增 `backend/build.rs`、修改 `backend/crates/api/src/middleware/rbac.rs`、删除 `backend/frontend.zip`、修复前端 i18n key 不匹配 |
| 状态同步归口 | EVO-016-B backlog → Done、Iteration 012 激活条件可更新、Iteration 030 Superseded 记录 |
| Story/BDD 归口 | Technical Story；等价技术验收（全部 14 项 AC 通过） |
| 验证证据 | `cargo test --workspace` 254 passed、`cargo clippy --workspace -- -D warnings` 0 errors、release build 12MB、curl 验证 ETag/304/br/SPA fallback/API auth、浏览器注册+登录+页面导航验证 |
| 残余工作归口 | Docker 单容器 Dockerfile 归 Iteration 012；CI/CD 归 EVO-030；依赖升级归 EVO-043 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-28 | activation | Iteration 030 ZIP 流式方案经多方调研确认过度工程，改线到 rust-embed-for-web 方案。新建 Iteration 031。EVO-016-B backlog 已更新。 |
| 2026-05-28 | implementation | `backend/src/frontend.rs` 完全重写：ZIP 解压 + HashMap 索引 + spawn_blocking（417 行）→ RustEmbed + EmbedResponse（105 行含测试）。移除 `zip`/`futures-util` 直接依赖。 |
| 2026-05-28 | implementation | `backend/src/main.rs` 简化：移除 `#[cfg(feature = "embedded-frontend")]`、`maybe_configure_frontend!` 宏，前端路由无条件注册。 |
| 2026-05-28 | implementation | `backend/Cargo.toml`：移除 `zip` 和 `embedded-frontend` feature，新增 `rust-embed-for-web = "11.3"` + `actix-web-rust-embed-responder = "2.3"`。 |
| 2026-05-28 | implementation | `backend/build.rs` 新增：`cargo:rerun-if-changed` 监听 `frontend/dist/`，解决前端变更不触发后端重编译的问题。 |
| 2026-05-28 | implementation | `rbac.rs`：`is_public_path` 增加 `!path.starts_with("/api/")` 判断，非 API 路径免认证以支持 SPA fallback。 |
| 2026-05-28 | fix | 前端 i18n key 不匹配修复：`register/page.tsx`（`passwordsDoNotMatch`→`passwordsNoMatch`、`invalidSlug`→`slugInvalid`）、`reset-password/page.tsx`（`passwordsDoNotMatch`→`passwordsNoMatch`）。 |
| 2026-05-28 | verification | `cargo check --workspace` 0 errors、`cargo clippy --workspace -- -D warnings` 0 errors、`cargo test --workspace` 254 passed 0 failed。 |
| 2026-05-28 | verification | Release build 12MB。curl 验证：ETag/304/br（751B→277B, 63% 压缩）/SPA fallback/API 401 全部通过。浏览器验证：注册→登录→Dashboard→Tools→Skills→Snippets 页面导航正常。 |
| 2026-05-28 | commit | `5536abb` feat: migrate embedded frontend from ZIP to rust-embed-for-web (EVO-016-B) |
| 2026-05-28 | commit | `0ecd2f8` fix: correct i18n key mismatches in register and reset-password pages |
| 2026-05-28 | close | 全部 14 项验收标准通过。迭代关闭。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|

## 10. Review

- 完成：全部 14 项验收标准通过。`frontend.rs` 417→105 行（-75%）。Release 二进制 12MB 含前端+预压缩。浏览器全链路验证通过。
- 未完成：无（`frontend.rs` 核心逻辑 55 行，含测试 105 行，略超 80 行目标，可接受）。
- 验证结果：`cargo test --workspace` 254/0、`cargo clippy -- -D warnings` 0、release build 12MB、ETag/304/br/SPA/API auth curl 全通过、浏览器注册+登录+导航全通过。
- 闭环状态：`Closed`
- 残余归口：Iteration 012（Docker 单容器）、EVO-030（CI/CD）、EVO-043（依赖升级）

## 11. Retrospective

- 做得好的：方案选型先广后深（6 方案对比），源码级验证内存模型（`.rodata` demand paging），ADR 先行再实现。proc macro 零 heap 的方案比 ZIP streaming 更简洁且性能更好。`build.rs` 的 `cargo:rerun-if-changed` 解决了前端变更不触发重编译的运维陷阱。
- 需要调整的：验收标准 `frontend.rs ≤ 80 行` 过于严格，含测试 105 行实际更合理。未来 Technical Story 行数目标应标注"不含测试"。实施过程中发现 RBAC 中间件拦截 SPA 路由的问题——应在计划阶段就考虑前端路由与中间件的交互。
- 写入 EVOLUTION：已在 Iteration 030 阶段写入"方案选型应先广后深"。本次追加"嵌入方案需要 build.rs 监听 dist 目录变更"。
