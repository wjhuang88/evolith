# EVO-126-A — WalGit dependency and toolchain boundary

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Dependency / Supply Chain
- **优先级**：P0
- **状态**：Ready
- **GitHub Issue**：[#13](https://github.com/wjhuang88/evolith/issues/13)
- **依赖或阻塞**：无
- **解锁内容**：EVO-126-B
- **影响范围**：backend / deploy / docs

## 工程目标

把 Evolith 的 toolchain 与 WalGit engine crates 放到可重复、可审计、不会随上游 `main` 漂移的编译基线上，不改变当前 runtime Git 行为。

## Scope

- Rust MSRV `1.88 -> 1.90`，同步 Cargo/Docker/CI/dev 权威版本。
- exact commit SHA pin `walgit-store/config/git/wal`；`walgit-bundle` 可在 G Story 再启用。
- 明确禁止以 `walgit-server` Router/AppState 作为运行时依赖。
- 添加第三方 MIT attribution / `THIRD_PARTY_NOTICES.md`。
- 记录 pinned upstream SHA 与升级流程。

## Acceptance

- [ ] Rust 1.90 下 `cargo fmt --all -- --check` 通过。
- [ ] `cargo check --workspace --all-targets` 通过。
- [ ] `cargo test --workspace` 通过。
- [ ] strict clippy 通过。
- [ ] `Cargo.lock` 对 WalGit 使用 exact pinned Git revision；不存在 branch=`main`/floating dependency。
- [ ] 当前 clone/push/pull 行为未切换，filesystem 引擎仍是 runtime current implementation。
- [ ] third-party notice 满足 WalGit MIT notice 保留条件。

## 失败模式

- Rust 升级导致既有依赖/CI 不兼容。
- WalGit crate 不能作为外部 Git dependency 编译或需要未声明 feature。
- dependency graph 引入重复 gix/tokio 等不可接受冲突。

任何阻塞必须形成明确结论；不能在无可重复 build 的情况下继续 B。

## 最小验证

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## 不做

不迁移 repo、不切 Smart HTTP、不改变 storage truth。
