# EVO-126-C — Smart HTTP read path on WalGit primitives

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Git Protocol
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#15](https://github.com/wjhuang88/evolith/issues/15)
- **依赖或阻塞**：EVO-126-B
- **解锁内容**：EVO-126-G / H
- **影响范围**：backend / docs / tests

## 工程目标

吸收并改写 `walgit-server` 的 Smart HTTP read-side orchestration，在 Evolith `service-git` 内建立 framework-neutral 的 `info/refs`、protocol v0/v2、`ls-refs` 与 `upload-pack` 路径。

## Scope

- 复用 `walgit-git` 的 protocol primitives；按需要吸收 pkt-line、streaming、advertisement cache 的 server-side 设计。
- `info/refs` 只做 refs-level sync，不为 advertisement 强制 materialize 全仓。
- `upload-pack` 在 read guard 生命周期内完成有界 streaming。
- Actix 只负责 HTTP/auth adapter，不让 `service-git` 依赖 Axum Router/AppState/Auth。
- 保留现有 request timeout、body/resource bounds、Git error/WWW-Authenticate 语义。

## Acceptance

- [ ] 标准 Git client clone/fetch/pull v0 与 v2 E2E 通过。
- [ ] local cache 删除/新实例启动后，可由 object store 恢复 read path。
- [ ] advertisement/cache freshness 与 manifest/version 绑定，不返回已知 stale refs。
- [ ] 认证/tenant hiding 继续由 Evolith boundary 控制；WalGit Authenticator 不进入产品路径。
- [ ] client disconnect、timeout、超限等负向测试有界结束，不遗留无界后台工作。

## 最小验证

```bash
cd backend
cargo test -p service-git
cargo test --workspace
```

并运行真实 `git clone` / `git fetch` / `git pull` 的 v0/v2 集成矩阵。

## 不做

不迁移 receive-pack 写路径，不启用 bundle-required policy，不实现 Agent write。
