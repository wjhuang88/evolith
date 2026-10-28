# Iteration 069: WalGit Dependency and Toolchain Boundary

> 文档状态：Closed / Complete
> 计划发布日期：2026-09-22
> 计划目标：完成 EVO-126-A，在不改变当前 filesystem Git runtime 的前提下建立 Rust 1.90、WalGit exact-SHA dependency 与 MIT/supply-chain 可重复编译基线。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

完成 [EVO-126-A](../backlog/active/EVO-126-A-walgit-dependency-toolchain-boundary.md)：
把 Evolith toolchain 与 WalGit engine crates 放到可重复、可审计、不会随上游 `main`
漂移的编译基线上，为 EVO-126-B 解锁；本 Iteration 不改变 repository storage truth、
Smart HTTP 路径或 Repo Context runtime。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-126-A | WalGit dependency and toolchain boundary | EVO-126 | P0 | ADR-0011 / governance baseline 已由 PR #21 合入 main；无 Story 硬依赖 |

库存 disposition：

- Iteration 018/019/020/027：继续 `Blocked for activation / Superseded direction`，不激活。
- Iteration 025/026：继续 `Blocked for activation`，等待各自 refinement，不抢占当前 P0。
- Iteration 056：保持 `Closed / Partial`；Branch Protection 远端证据 residual 归 release closure，
  不阻塞 EVO-126-A。
- 其余最近 Iteration 057-068 均已 Closed；不存在必须先继续的 Active / In Progress / Review Iteration。

## 3. 发布计划基线：不做事项

- 不迁移 repository，不把 object store 设为当前 Git durable truth。
- 不切换 Smart HTTP、receive-pack、Repo Context 或 lifecycle 实现。
- 不依赖 `walgit-server` Router/AppState/Auth/UI/Product Policy。
- 不提前实现 EVO-126-B~H。
- 不把 `walgit-bundle` 作为本 Story 的必需 runtime dependency；bundle 归 EVO-126-G。

## 4. 发布计划基线：计划验收标准

- Story 形态：Technical / Dependency / Supply Chain；BDD 不适用，以可重复编译、
  lockfile、dependency inventory 和 license notice 为等价技术验收。
- [ ] Rust workspace MSRV 与 dev/container/CI 权威入口统一到 1.90。
- [ ] `walgit-store` / `walgit-config` / `walgit-git` / `walgit-wal`
  使用 upstream exact revision `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`，不存在 branch=`main` 或 floating pin。
- [ ] `Cargo.lock` 解析到同一 WalGit revision，并能在 Rust 1.90 下构建。
- [ ] `walgit-server` 不进入 Evolith runtime dependency boundary。
- [ ] `THIRD_PARTY_NOTICES.md` 保留 WalGit MIT copyright/permission notice，
  并记录 pinned revision 与后续升级复核规则。
- [ ] 现有 filesystem Git runtime 行为保持；本 Story 不声称完成 GIT-DP-01。

## 5. 发布计划基线：计划验证

```bash
cd backend
rustc --version
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo tree -p service-git

cd ..
git diff --check
python3 scripts/tests/check-markdown-links.py
```

PR exact-head CI 还必须证明现有 frontend、DATA-01 durability 与 application recovery
门禁没有因 toolchain/dependency 改动退化。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| Rust 1.90 暴露既有依赖/Clippy 不兼容 | 只修与 toolchain 升级直接相关的编译/警告；若超出 Story 窗口则 Blocked 并记录具体 crate/错误 |
| WalGit Git dependency 无法作为外部 workspace dependency 编译 | 固定 exact SHA 后先做 compile gate；失败则停止 B，不用 branch/main 或复制源码掩盖 |
| WalGit 引入 gix/aws/gcp 等大依赖导致 CI 时间/磁盘上升 | 记录 dependency graph 与 CI 证据；本 Story 不通过削弱测试门禁规避 |
| 上游 pre-1.0 API drift | exact SHA + upgrade procedure；B 起所有 WalGit 类型通过 service-git boundary 隔离 |
| 需要回滚 | 回退 Rust/toolchain、WalGit dependency 与 notice 变更即可；本 Story 无 repo 数据迁移和 runtime storage cutover |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 正式启动 WalGit 重构第一切片，建立可重复的 Rust toolchain + pinned WalGit engine dependency 基线 |
| 产物 | toolchain/Cargo/Docker/CI 版本同步；WalGit exact-SHA deps/lock；THIRD_PARTY_NOTICES；依赖/升级说明 |
| 状态同步归口 | EVO-126-A item、EVO-126 Epic、Product Backlog、Board、Iteration 069、GitHub #12/#13 |
| Story/BDD 归口 | Technical Story；BDD 不适用，使用命令级 build/test/lock/license 验收 |
| 验证证据 | amended target: Rust 1.94.1 fmt/check/clippy/test、cargo tree/lock 检查、文档检查、exact-head CI 与 durability/recovery |
| 残余工作归口 | service-git v2 engine boundary -> EVO-126-B；任何 upstream incompatibility 留在 A/Iteration 069，不透传到 B |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-09-22 | activation | PR #21 已合入 main（`d15217b`）；库存盘点后选择唯一 P0 Ready Story EVO-126-A，旧 Blocked/Superseded/Closed-Partial iteration disposition 如上，不阻塞本轮。 |
| 2026-09-22 | verification | WalGit `main` activation snapshot 为 `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`；upstream workspace 声明 Rust 1.90 / edition 2024 / MIT。选择 exact SHA 作为本轮 pin，禁止 floating main。 |
| 2026-09-22 | dependency | clean resolution 发现 WalGit gix 0.86 -> gix-protocol 0.64 仍依赖已 withdrawn/yanked 的 `bisync ^0.3`；采用本地 package-identity shim + maintained `bisync2 = 0.3.2`，不漂移 WalGit SHA。 |
| 2026-10-22 | verification | clean resolved graph 显示现有 `sqlx 0.9.0` 要求 Rust 1.94.0，WalGit 默认 store/AWS 当前 patch graph 要求 Rust 1.94.1；因此 1.90 无法作为 Evolith 集成 MSRV。 |
| 2026-10-22 | build-fix | `walgit-proto` build script 需要 `protoc`；将 `protobuf-compiler` 加入主 CI、durability CI 与 production Docker builder，并显式检查 `protoc --version`。 |
| 2026-10-23 | lock-fix | 首个 `cargo check --locked` 暴露 AWS SDK/Smithy family 类型冲突；对照 pinned WalGit 自身 `Cargo.lock`，将 AWS/Smithy family 对齐到其已验证版本组合，最终 lock 只保留 `aws-smithy-schema 0.2.0`，未漂移 WalGit SHA。 |
| 2026-10-23 | verification | technical head `2412c04bd151845d5401facb8459df742a9f9501` 的 `ci` #272 与 `data-durability-container` #114 全绿：fmt/check/clippy/workspace tests、frontend、DATA durability、locked build、application recovery 与真实 Backend container recreation 均通过。 |
| 2026-10-23 | navigator | Navigator 检查未发现 runtime/API 边界泄漏：四个 WalGit engine crates 只进入 `service-git` Cargo implementation boundary，`walgit-server` 未引入；发现 current-fact 文档漂移与 `cargo tree` 证据缺口，已进入治理修正。 |
| 2026-10-23 | gate-hardening | 将计划验证中的 `cargo tree -p service-git` 升级为 permanent `cargo tree --locked -p service-git` CI gate，显式断言四个允许 engine crates 存在、exact WalGit source rev 可见且 `walgit-server` 不存在。 |

| 2026-10-28 | closure | final exact-head `493f9f3` 的 `ci` #282 与 durability #124 全绿；PR #24 squash merge 为 `09316a6`；EVO-126-A Closed / Complete，EVO-126-B 解除硬依赖。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-10-22 | scope-change | 接受：Evolith integration toolchain/MSRV 从计划的 1.90 修正为 1.94.1；WalGit upstream 自身 MSRV 仍记录为 1.90 | 当前 Story 验收、Cargo/Docker/CI/dev 文档改为 1.94.1；不改变 data-plane 架构、Story 边界或 runtime Git path | 保留 exact WalGit pin、bisync shim、service-git dependency boundary；移除所有只为 1.90 解析失败服务的临时结论，最终 CI 不保留临时 lock update/write step |

### 9.1 修订后的当前验收标准

- [x] Evolith workspace / rust-toolchain / builder / CI / Clippy / current docs 统一为 Rust 1.94.1。
- [x] exact WalGit SHA、`walgit-server` exclusion、temporary bisync compatibility boundary 保持可审计。
- [x] committed `Cargo.lock` 可从 clean checkout 直接构建，不需要临时 `cargo update` 或 yanked crate。
- [x] Rust 1.94.1 下 fmt/check/clippy/test、dependency tree、frontend、durability/recovery exact-head CI 全部通过。
- [x] Evolith 项目许可证保持 `AGPL-3.0-only`；第三方 WalGit/bisync2 attribution 保持各自许可证事实。
- [x] filesystem Git runtime 行为不变。

## 10. Review

- 完成：activation、exact WalGit pin、dependency boundary、bisync compatibility boundary、Rust 1.94.1 scope-change、committed lockfile、`protoc` build dependency、AWS/Smithy coherent lock family 与 technical exact-head validation 已完成。
- Navigator：no blocking implementation findings。已识别并正在修正 current-fact 文档漂移，同时把 dependency-tree 人工验收升级为 permanent CI gate。
- 验证结果：technical head `2412c04` 的 `ci` #272 / durability #114 全绿；final governance head `493f9f3` 的 `ci` #282 / durability #124 再次全绿，并通过 permanent locked dependency-tree gate。
- 合并：PR #24 -> `main` merge `09316a6ba3a68fb6e7d30bbba6bf5cc563897696`。
- 闭环状态：`Complete`
- 残余归口：service-git v2 engine implementation 归 EVO-126-B；本 Iteration 无未归口残余。

## 11. Retrospective

- 做得好的：没有通过 floating WalGit main、强行锁 yanked crate或删减 object-store 默认依赖来绕过供应链/编译问题；所有临时 write/self-update workflow 都已在最终 runtime CI 前移除。
- 需要调整的：不能把上游项目的 MSRV 直接等同于集成项目可用 MSRV；必须以完整 resolved locked graph 验证。复杂 SDK family 还必须保持同一 generation 的内部类型一致，不能只看单 crate semver 可解析。
- 写入 EVOLUTION：收口时记录“integration MSRV must be derived from full locked dependency graph”，并附 AWS/Smithy coherent-lock 经验。
