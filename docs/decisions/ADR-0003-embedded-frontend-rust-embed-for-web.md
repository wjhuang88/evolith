# ADR-0003: 前端静态文件嵌入方案从 ZIP 迁移到 rust-embed-for-web

## 状态

Accepted

## 背景

Evolith 需要将前端 Vite SPA 产物嵌入后端 Rust 二进制，实现单容器部署（消除 Nginx 静态托管依赖）。

### 已完成的前置工作

- **Iteration 024 / EVO-016-A**：使用 `zip` crate + `include_bytes!("frontend.zip")` 实现了基础 ZIP 嵌入方案。
- **Iteration 030**：对 ZIP 方案进行了深入优化研究——`OnceLock` 静态索引、`Arc<ZipArchiveMetadata>` 零重解析、`web::block()` 同步桥接、16KB chunk 流式解压、ETag/Cache-Control 缓存头。
- ZIP 方案已验证可行，clippy/test 全部通过。

### 触发改线的发现

在深入实现 ZIP 流式方案时，对可选方案进行了广度调研（多方交叉验证），发现 ZIP 方案在 `rust-embed-for-web` 面前是过度工程：

1. ZIP 方案每请求需要 Deflate 解压 + `Vec<u8>` 堆分配 + `spawn_blocking` 线程池占用。
2. `rust-embed-for-web` 通过编译时嵌入 `&'static [u8]` 字面量，运行时零分配、零解压、零 CPU。
3. `actix-web-rust-embed-responder` 自动处理 ETag（SHA-256）、Last-Modified、304 条件请求、Content-Encoding 协商。
4. 生产级参考：OpenObserve 使用相同方案（`rust-embed-for-web` + axum）。

## 选项

### 选项 A：ZIP 流式方案（Iteration 030 原方向）

- **架构**：`include_bytes!("frontend.zip")` → `OnceLock<FrontendIndex>` → `web::block()` + Deflate 解压 → `HttpResponse::streaming()`
- **每请求内存**：`Vec<u8>` 堆分配（文件全量） + channel 缓冲
- **每请求 CPU**：Deflate 解压 + spawn_blocking 调度
- **代码量**：~280 行（手动 ETag、Content-Type、缓存头、SPA fallback、索引管理）
- **优势**：ZIP 压缩减少嵌入体积；团队已熟悉实现细节
- **劣势**：复杂度高；每请求有 CPU 和内存开销；channel/spawn_blocking 引入调度延迟

### 选项 B：rust-embed-for-web（最终选择）

- **架构**：`#[derive(RustEmbed)] #[folder = "../frontend/dist/"]` → 编译时嵌入 `&'static [u8]` 字面量 + 预压缩 → `actix-web-rust-embed-responder` 自动响应
- **每请求内存**：零分配（`&'static [u8]` 切片）
- **每请求 CPU**：零（无解压、无调度）
- **代码量**：~50-80 行
- **优势**：极简；零运行时开销；自动 ETag/304/Content-Encoding；Debug 模式读文件系统（前端热更新无需重编译）
- **劣势**：二进制体积增加 2-3 倍（原始 + gzip + brotli 预压缩变体）；新增两个外部依赖

### 选项 C：rust-embed（原版，不含预压缩）

- **架构**：同 B 但无预压缩，无 ETag 预计算，无 Content-Encoding 协商
- **劣势**：需要手动实现所有 HTTP 缓存逻辑；不如 `-for-web` 变体

### 选项 D：无压缩 + 扁平索引

- **架构**：`include_bytes!` 每个文件 + 自定义索引
- **劣势**：实现复杂度高；无预压缩；体积最大

### 选项 E：include_dir / rust-embed（无预压缩、无 Web 优化）

- **劣势**：同 C，且不支持 `actix-web-rust-embed-responder` 集成

## 决策

**选择选项 B：rust-embed-for-web + actix-web-rust-embed-responder。**

### 关键决策依据

#### 1. 运行时内存行为（源码验证）

基于对 `rust-embed-for-web` v11.3.0 源码的完整分析：

- **编译时**：proc macro 为每个文件生成独立的 `&[u8]` 字节字面量（`impl/src/embed.rs:15-21`），所有预压缩变体（gzip/brotli）同样作为 `&'static [u8]` 嵌入
- **运行时**：`EmbeddedFile` 结构体（`utils/src/file/embed.rs:12-25`）所有字段为 `&'static` 引用，`data()` 直接返回 `self.data` 字段（零拷贝）
- **ELF 加载**：数据放入 `.rodata` 段，OS 通过 `mmap` + demand paging 按需加载，未访问的文件不在物理 RAM 中
- **结论**：不存在"全量读入堆内存"问题

#### 2. 与 ZIP 方案的量化对比

| 维度 | ZIP 方案 | rust-embed-for-web |
|------|---------|-------------------|
| 每请求堆分配 | `Vec<u8>`（文件全量） | 零 |
| 每请求 CPU | Deflate 解压 + 线程调度 | 零 |
| 每请求延迟 | 解压 + channel + spawn_blocking | 直接指针返回 |
| ETag 强度 | CRC32 或文件名 hash | SHA-256 + Base85 |
| Content-Encoding | 无 | 自动 br/gzip 协商 |
| Debug 热更新 | 需替换 frontend.zip | 从 dist/ 读文件系统 |
| 代码量 | ~280 行 | ~50-80 行 |
| 二进制体积 | 压缩后 ~2-3MB | 原始 + 预压缩 ~6-10MB |

#### 3. 移除 `embedded-frontend` feature flag

`rust-embed-for-web` 内置 Debug/Release 行为切换：
- Debug：`DynamicFile` 从 `frontend/dist/` 读文件系统，前端改动无需重编译
- Release：`EmbeddedFile` 零拷贝嵌入

因此 `embedded-frontend` feature flag 不再需要。路由注册变为无条件。

#### 4. 生产参考

OpenObserve（生产级可观测平台）使用 `rust-embed-for-web` + axum 提供前端静态文件服务。

### 同时决策

- **移除** `zip` crate 依赖和 `frontend.zip` 占位文件
- **移除** `embedded-frontend` feature flag 和 `maybe_configure_frontend!` 宏
- **保留** `/config.js` 动态生成（运行时环境变量注入）
- **保留** `Cache-Control` 手动设置（responder 不处理此项）
- **不启用** zstd 预压缩（需 C 绑定，gzip + brotli 覆盖主流浏览器）

## 后果

### 正面

- 前端静态服务代码从 ~280 行降到 ~50-80 行，维护成本大幅降低
- 每请求零 CPU、零堆分配，高并发下内存稳定
- 自动获得预压缩（gzip + brotli）和强 ETag（SHA-256）
- Debug 模式下前端改动无需重编译 Rust
- 消除了 spawn_blocking 线程池占用和 channel 开销

### 负面

- 二进制体积增加 2-3 倍（典型 SPA 约 6-10MB 额外），但 `.rodata` 按需分页，不影响 RSS
- 新增两个外部依赖（`rust-embed-for-web`、`actix-web-rust-embed-responder`）
- 需要确保 CI/CD 中 `frontend/dist/` 在 Rust 编译前已构建
- 大文件（>1MB）的预压缩变体也全量嵌入二进制，但 smart compression 阈值（5%）会跳过不值得压缩的文件

### 风险缓解

| 风险 | 缓解措施 |
|------|---------|
| 二进制体积过大 | 可通过 `#[gzip = false]` / `#[br = false]` 关闭部分预压缩 |
| `rust-embed-for-web` 停止维护 | 上游是 `rust-embed` 的 fork；API 稳定；最坏情况 fork |
| `actix-web-rust-embed-responder` 版本不兼容 | API 极简（`EmbedResponse` + `into_response()`），必要时手动实现 Responder |
| `deny(clippy::unwrap_used)` 冲突 | responder 内部已处理；如冲突则在模块级别 allow |

## 相关链接

- [EVO-016-B Backlog 条目](../backlog/PRODUCT-BACKLOG.md)
- [Iteration 030（Superseded：ZIP 流式方案）](../iterations/ITERATION-030.md)
- [Iteration 031（Active：rust-embed-for-web 迁移）](../iterations/ITERATION-031.md)
- [rust-embed-for-web GitHub](https://github.com/SeriousBug/rust-embed-for-web)
- [actix-web-rust-embed-responder GitHub](https://github.com/SeriousBug/actix-web-rust-embed-responder)
- [rust-embed-for-web API 文档](https://docs.rs/rust-embed-for-web/latest/rust_embed_for_web/)
- [OpenObserve 前端嵌入实现参考](https://github.com/openobserve/openobserve/blob/main/src/handler/http/router/ui.rs)
- [EVOLUTION.md 教训记录](../../EVOLUTION.md)
