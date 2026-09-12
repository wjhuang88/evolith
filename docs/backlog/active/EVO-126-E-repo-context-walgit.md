# EVO-126-E — Repo Context on WalGit RepoHandle/ObjectAccess

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Repository Read
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#17](https://github.com/wjhuang88/evolith/issues/17)
- **依赖或阻塞**：EVO-126-B
- **解锁内容**：EVO-126-F / H
- **影响范围**：backend / docs / tests

## 工程目标

将 Tree/Blob/Commit/Diff Context 从固定 local bare repo path 迁移到 WalGit `RepoHandle` / `ObjectAccess`，保持现有 Repo UI/Commit Evidence 的契约与资源安全边界。

## Scope

- Context read 统一经过 Registry/open + freshness/sync boundary。
- `ObjectAccess::Local` 继续用 gix；Remote 模式使用 WalGit supported remote object capability，或在其能力缺口处采用明确有界 materialization。
- 保留 blob/tree/diff limits、timeout、ref reachability、tenant hiding、binary/404 等既有语义。
- API DTO/public domain 不暴露 WalGit implementation types。

## Acceptance

- [ ] 现有 Files/Commits/Commit Evidence API 回归通过。
- [ ] local cache 删除或新实例启动后仍可恢复 Context read。
- [ ] tree/blob/commit/diff 的超限、missing object/ref、unreachable commit 负向测试通过。
- [ ] 不降低 EVO-116/EVO-112-C 已建立的 resource/evidence gates。
- [ ] 大 repo remote-object/materialization 行为有明确 timeout/size upper bound。

## 最小验证

```bash
cd backend
cargo test -p service-git
cargo test --workspace
```

并复跑 Repo Context/Commit Evidence 聚焦 E2E。

## 不做

不改变产品 DTO 语义，不实现 Indexer/Discovery，不负责 production object-store migration。
