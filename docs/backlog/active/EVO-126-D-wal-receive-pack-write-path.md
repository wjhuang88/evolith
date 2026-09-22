# EVO-126-D — WAL-backed receive-pack write path

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Git Write / Security-sensitive
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#16](https://github.com/wjhuang88/evolith/issues/16)
- **依赖或阻塞**：EVO-126-B
- **解锁内容**：EVO-126-F / H；GIT-DP-01 后的 EVO-105/106
- **影响范围**：backend / docs / tests

## 工程目标

将 Smart HTTP write path 从直接 filesystem `git receive-pack` 迁移为 WalGit receive primitives + `RepoHandle::publish_push`，同时保持 Evolith authorization、Agent write restriction、Audit/Outbox 的稳定语义。

## Scope

- 解析 receive-pack commands/capabilities/push-options 与 pack ingestion。
- publish 前经过 Evolith-owned authorization 与 push-policy hook；WalGit product policy 不成为 authority。
- 以 WAL + manifest CAS 作为 Git ref publication boundary。
- 将稳定可用的 WAL sequence/ref result 写入 Push provenance，并继续使用 Evolith Durable Outbox。
- 明确 non-fast-forward、force/delete/protected-ref、CAS conflict 与 partial per-ref result 的 owner/error contract。
- Agent Scoped Token 继续 fail closed，不因底层更换获得 generic receive-pack write。

## Acceptance

- [ ] 真实 `git push` success/conflict/reject/restart E2E 通过。
- [ ] Git success 不早于 required durable publication；不存在“客户端成功但新 ref 未 durable”的路径。
- [ ] `repo.push.completed.v1` producer/subscriber 在新路径继续满足 EVENT-01，重复/乱序仍幂等。
- [ ] 故障注入覆盖 pack ingest、CAS conflict、store failure、Outbox failure/reconcile；失败不返回假成功。
- [ ] WAL metadata/log/audit 不含 token、Authorization header 或 object-store credential。
- [ ] security/Navigator review 在 Story closure 前完成。

## 最小验证

真实 Git client push matrix + service-git/Outbox integration tests + workspace security gates。

## 不做

不在本 Story 实现 EVO-105 Commit/Promote 的完整 Policy 三态，也不开放 Agent generic push。
