# EVO-126-H — Cut over and retire the legacy filesystem Git engine

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Migration Closure / Release Gate
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#20](https://github.com/wjhuang88/evolith/issues/20)
- **依赖或阻塞**：EVO-126-C / D / E / F / G
- **解锁内容**：GIT-DP-01 Close；EVO-105/106 Agent Write Loop activation
- **影响范围**：backend / deploy / docs / tests

## 工程目标

完成默认 Git engine 到 WalGit-backed data plane 的切换，证明迁移/恢复/协议/事件完整闭环，并删除或隔离不再需要的 filesystem-backed Smart HTTP/storage 主路径。

## Scope

- production/default config 切换到 WalGit-backed `service-git v2`。
- 完成 existing repo migration rehearsal 与最终 routing cutover。
- inventory 并清理 `repo_path/init_bare_repo`、旧 upload/receive-pack subprocess 主路径、filesystem-specific reconciler/backup requirement 等无 consumer 代码。
- 若底层仍使用 `git` CLI，必须明确属于 WalGit primitive/兼容实现，不再作为 Evolith durable filesystem engine owner。
- 同步 Architecture、Project Map、Config、Testing、Release、Baseline、Roadmap、Board 与 AGENTS current facts。
- 进行 Security/Navigator/Data review。

## Acceptance

- [ ] default production configuration 不要求 durable local Git repo volume。
- [ ] 新 repo 与所有已迁旧 repo 在 object-store-backed engine 上 clone/fetch/pull/push/context 通过。
- [ ] bundle-uri 授权/fallback/metrics 证据通过。
- [ ] app/cache 删除与重启后 repo 可恢复；store outage readiness 503；恢复后 smoke 通过。
- [ ] Push Durable Outbox/Audit provenance 在最终路径上通过，WAL seq 等 metadata 不破坏现有幂等语义。
- [ ] legacy Git engine consumer inventory = 0；旧配置 fail fast 或提供明确 migration error。
- [ ] docs/code search 不再把 persistent filesystem 描述为当前生产 Git durable truth（历史 ADR/Iteration/迁移说明除外）。
- [ ] Security/Navigator/Data review 无 blocking finding，所有 residual 进入 backlog。
- [ ] GIT-DP-01 与 EVO-126 可基于证据关闭。

## 最小验证

完整 migration + clean restart + real clone/push/pull + Repo Context/Commit Evidence + Outbox + bundle + readiness/recovery smoke；workspace 全量门禁；文档链接与旧术语分类检查。

## 不做

不顺手实现 EVO-105/106；不将 LFS 自动纳入 cutover。
