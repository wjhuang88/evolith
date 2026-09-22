# EVO-126-B — service-git v2 engine boundary

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Architecture / Git Engine
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#14](https://github.com/wjhuang88/evolith/issues/14)
- **依赖或阻塞**：EVO-126-A
- **解锁内容**：EVO-126-C / D / E / F
- **影响范围**：backend / docs

## 工程目标

把当前以 filesystem helper 为中心的 `service-git` 重构为 Evolith-owned、framework-neutral 的 Git engine boundary，并在内部接入 WalGit Store/Registry 生命周期，不切换现有生产流量。

## Scope

- 定义 Evolith-owned repository identity、Git request/response、error 与 engine contract。
- 初始化 `walgit_store::DynStore`、`walgit_wal::Registry` 与 config adapter。
- 固定 Evolith tenant/repository UUID -> WalGit RepoId 的稳定映射。
- create/open/delete/list 通过 WalGit Registry 实现，并继续由 RepoApplicationService/lifecycle state 编排产品状态。
- 重新定义 Reconciler 的目标对象：DB metadata vs WalGit manifest/repository namespace。
- local disk 只作为 WalGit cache，不作为新 engine 的 durable truth。

## Acceptance

- [ ] `api` 与核心 domain 的 public contract 不暴露 `walgit_*` 类型。
- [ ] memory store 下 create/open/delete/not-found/already-exists contract 测试通过。
- [ ] 至少一个 object-store-compatible test backend 通过同一 engine contract。
- [ ] URL/repo slug 不能直接控制 object-store prefix；physical RepoId 只由已授权 DB identity 生成。
- [ ] 当前 filesystem Git runtime 仍可运行，允许新旧实现过渡但不存在长期双写设计。
- [ ] lifecycle failure 不返回假成功；新增 residual 有明确归口。

## 失败模式

- RepoId encoding 不稳定或超出 upstream validation。
- Registry/create 与 DB state 失败顺序产生无 owner 的 object-store repo。
- WalGit 类型泄漏到 API，导致未来 upstream change 扩散。

## 最小验证

```bash
cd backend
cargo test -p service-git
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

并执行 Store/Registry lifecycle integration tests。

## 不做

不移植 Smart HTTP、不切 receive-pack、不迁移现有 repo、不修改最终 production storage config。
