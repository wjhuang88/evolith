# SOP: 测试与验证

稳定测试策略和测试用例索引见 [测试参考](../reference/TESTING.md)。本文档只保留执行流程。

## 按变更范围选择验证

| 改动类型 | 最小验证 |
|----------|----------|
| Rust domain/service | `cargo test -p <crate>` |
| API handler/middleware | `cargo test -p api` |
| repository/migration | `cargo test -p infra` |
| 前端类型/API client | `npm run type-check` |
| 前端页面 | `npm run type-check` + `npm run build` |
| 跨端功能 | 后端相关测试 + 前端 build + 手工流程 |
| 脚本/部署 | 对应 SOP 中的 dry run 或局部命令 |

## 后端验证

```bash
cd backend
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

局部验证：

```bash
cargo test -p api
cargo test -p infra
cargo test -p service-auth
```

## 前端验证

当前 Next.js 阶段：

```bash
cd frontend
npm run type-check
npm run build
```

迁移到 Vite+Bun 后：

```bash
cd frontend
bun run type-check
bun run build
```

## CI 对齐

CI 预期覆盖：

1. `cargo fmt --check`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo test --workspace`
4. `cargo audit`
5. 前端依赖审计
6. Docker build

## 调试技巧

后端详细日志：

```bash
RUST_LOG=debug cargo run
RUST_LOG=evolith::api=trace cargo run
```

前端开发模式：

```bash
npm run dev
```

或 Vite+Bun 迁移后：

```bash
bun run dev
```

## 测试覆盖率

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```
