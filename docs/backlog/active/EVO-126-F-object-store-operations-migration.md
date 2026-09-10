# EVO-126-F — Object-store operations, migration and recovery

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Data Durability / Deploy
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#18](https://github.com/wjhuang88/evolith/issues/18)
- **依赖或阻塞**：EVO-126-B；最终验收联合 EVO-126-C/D/E
- **解锁内容**：EVO-126-G / H
- **影响范围**：backend / deploy / docs / tests

## 工程目标

把 production Git storage 的 config、readiness、durability、lifecycle reconcile、backup/recovery 与 existing-repo migration 从 filesystem 假设迁移到 WalGit object-store/WAL 模型。

## Scope

- 建立 Evolith-owned object-store config adapter；开发/测试与生产 backend 边界明确。
- readiness 验证 Store 必需 read/write/CAS 能力，关键依赖失败时 503/fail closed。
- Repo lifecycle 从 DB/FS reconcile 改为 DB/WalGit manifest namespace reconcile。
- 设计并实现 existing bare repo -> WalGit import：inventory、connectivity、refs/tags/default branch/object format 校验、dry-run、idempotency、失败恢复。
- 重定义 backup/recovery responsibility：object store versioning/retention/replication 与 DB/Git publication consistency point 明确。
- destructive restore/migration 延续 DATA-01 的 fail-safe 原则，不覆盖未知非空目标。

## Acceptance

- [ ] 旧 filesystem repo 在测试环境迁入后 clone/context/push 均通过。
- [ ] app 容器/local cache 被删除后 repository durable data 不丢失。
- [ ] object store unavailable/permission denied/CAS unavailable 时 readiness 503，不返回假健康。
- [ ] migration 中断后可安全重试或明确回滚；原 repo 不被破坏。
- [ ] DB-only / object-store-only / migrated repo inventory 与 reconcile 策略可观察。
- [ ] 恢复验收以真实 Git clone + Context read 为证据，不只检查 object keys。
- [ ] DATA-01/DATA-02 旧引擎历史证据保留，不被重写为未完成。

## 最小验证

migration/recovery rehearsal + object-store fault matrix + real Git client smoke + readiness runtime smoke。

## 不做

不把 object store 宣称为天然生产就绪；最终 cutover/GIT-DP-01 由 EVO-126-H 关闭。
