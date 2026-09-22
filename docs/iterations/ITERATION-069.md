# Iteration 069: WalGit Dependency and Toolchain Boundary

> 文档状态：Active
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
| 请求结果 | 正式启动 WalGit 重构第一切片，建立可重复的 Rust 1.90 + pinned WalGit engine dependency 基线 |
| 产物 | toolchain/Cargo/Docker/CI 版本同步；WalGit exact-SHA deps/lock；THIRD_PARTY_NOTICES；依赖/升级说明 |
| 状态同步归口 | EVO-126-A item、EVO-126 Epic、Product Backlog、Board、Iteration 069、GitHub #12/#13 |
| Story/BDD 归口 | Technical Story；BDD 不适用，使用命令级 build/test/lock/license 验收 |
| 验证证据 | Rust 1.90 fmt/check/clippy/test、cargo tree/lock 检查、文档检查、exact-head CI 与 durability/recovery |
| 残余工作归口 | service-git v2 engine boundary -> EVO-126-B；任何 upstream incompatibility 留在 A/Iteration 069，不透传到 B |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-09-22 | activation | PR #21 已合入 main（`d15217b`）；库存盘点后选择唯一 P0 Ready Story EVO-126-A，旧 Blocked/Superseded/Closed-Partial iteration disposition 如上，不阻塞本轮。 |
| 2026-09-22 | verification | WalGit `main` activation snapshot 为 `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`；upstream workspace 声明 Rust 1.90 / edition 2024 / MIT。选择 exact SHA 作为本轮 pin，禁止 floating main。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| - | - | - | - | - |

## 10. Review

- 完成：待实施。
- 未完成：EVO-126-A 全部技术验收。
- 验证结果：activation inventory 与 upstream pin snapshot 已完成；runtime 验证待实现后执行。
- 闭环状态：`Partial`
- 残余归口：尚未收口；本 Iteration 保持 Active。

## 11. Retrospective

- 做得好的：待收口。
- 需要调整的：待收口。
- 写入 EVOLUTION：若出现可复用的 Rust/WalGit dependency 或 CI 经验则在收口时决定。
