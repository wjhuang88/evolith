# EVO-126-A — WalGit dependency and toolchain boundary

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Dependency / Supply Chain
- **优先级**：P0
- **状态**：Review
- **GitHub Issue**：[#13](https://github.com/wjhuang88/evolith/issues/13)
- **当前迭代**：[Iteration 069](../../iterations/ITERATION-069.md)
- **依赖或阻塞**：无；实现与技术门禁已通过，当前只剩治理同步后的 final exact-head CI
- **解锁内容**：EVO-126-B
- **影响范围**：backend / deploy / docs

## Activation Snapshot

- 2026-09-22 inventory 后激活到 Iteration 069。
- WalGit upstream snapshot：`80e9a20b29e29aefd16a4dae6f8e274cce85cca5`（`main` at activation）。
- WalGit upstream workspace 声明 `rust-version = "1.90"`、edition 2024、license MIT。
- 2026-10-22 clean dependency resolution 证明 Evolith 集成图要求 Rust `1.94.1`：现有 `sqlx 0.9.0` 要求 Rust 1.94.0，WalGit 默认 store/AWS graph 中的当前 AWS SDK patch 版本要求 Rust 1.94.1。
- 本 Story 只建立可重复编译/供应链边界；filesystem Git runtime 在本 Story 结束后仍保持当前实现。

## 工程目标

把 Evolith 的 toolchain 与 WalGit engine crates 放到可重复、可审计、不会随上游 `main` 漂移的编译基线上，不改变当前 runtime Git 行为。

## Scope

- Evolith integration toolchain/MSRV `1.88 -> 1.94.1`，同步 Cargo/Docker/CI/dev 权威版本；WalGit upstream 自身 MSRV 仍记录为 1.90。
- exact commit SHA pin `walgit-store/config/git/wal`；`walgit-bundle` 可在 G Story 再启用。
- 明确禁止以 `walgit-server` Router/AppState 作为运行时依赖。
- 保持 Evolith `AGPL-3.0-only` 项目许可证；第三方 WalGit/bisync2 继续按各自 MIT 许可保留 attribution。
- 对 WalGit 当前 gix 0.86 的 withdrawn `bisync` 依赖使用本地 package-identity shim + maintained `bisync2 = 0.3.2`，并记录为临时兼容例外。
- 记录 pinned upstream SHA、lockfile 与升级/移除兼容 patch 的流程。

## Acceptance

- [ ] Rust 1.94.1 下 `cargo fmt --all -- --check` 通过。
- [ ] `cargo check --locked --workspace --all-targets` 通过。
- [ ] `cargo test --locked --workspace` 通过。
- [ ] strict clippy 通过。
- [ ] `Cargo.lock` 对 WalGit 使用 exact pinned Git revision；不存在 branch=`main`/floating dependency。
- [ ] clean checkout 不需要 yanked crate 或临时 `cargo update` 才能解析依赖图。
- [ ] `cargo tree --locked -p service-git` 证明四个允许的 WalGit engine crates 在边界内，且 `walgit-server` 不进入 runtime dependency graph；WalGit types 仍只允许进入 `service-git` implementation boundary。
- [ ] 当前 clone/push/pull 行为未切换，filesystem 引擎仍是 runtime current implementation。
- [ ] third-party notice 满足 WalGit MIT 与 bisync2 MIT attribution；Evolith package metadata 保持 `AGPL-3.0-only`。

## 失败模式

- Rust 升级导致既有依赖/CI 不兼容。
- WalGit crate 不能作为外部 Git dependency 编译或需要未声明 feature。
- dependency graph 引入重复 gix/tokio 等不可接受冲突。
- 上游 gix/WalGit 的旧 transitive requirement 指向 withdrawn/yanked crate。

任何阻塞必须形成明确结论；不能在无可重复 build 的情况下继续 B。

## 最小验证

```bash
cd backend
rustc --version
cargo fmt --all -- --check
cargo tree --locked -p service-git
cargo check --locked --workspace --all-targets
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
```

## Review evidence

- Technical validation head `2412c04bd151845d5401facb8459df742a9f9501`：`ci` #272 与 `data-durability-container` #114 全绿。
- `ci` #272 已通过 exact-head、whitespace、Markdown、frontend install/type-check/build/lint、durability backup/restore、PostgreSQL durability、Git volume recreation、Rust fmt/check/clippy/workspace tests、locked recovery binary 与 application recovery drill。
- `data-durability-container` #114 已从 clean checkout 完成 locked binary build，并通过真实 Backend container recreation with same Git volume。
- Navigator 审查确认 direct WalGit types 只进入 `service-git` Cargo boundary、`walgit-server` 未被引入；为把计划中的 `cargo tree` 从人工检查升级为可重复证据，已新增 permanent locked dependency-boundary CI gate。
- 当前等待：包含上述治理同步与 dependency-tree gate 的 final exact-head CI。

## 不做

不迁移 repo、不切 Smart HTTP、不改变 storage truth。
