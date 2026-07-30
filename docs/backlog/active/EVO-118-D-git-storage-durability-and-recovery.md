# EVO-118-D Git 存储持久化、备份与恢复演练

- **类型**：Technical / Data Durability / Deploy
- **状态**：Ready
- **优先级**：P0
- **父 Epic**：[EVO-118](EVO-118-production-readiness-and-security-hardening.md)
- **依赖**：EVO-118-A merge
- **影响范围**：deploy / backend / scripts / docs / tests

## 工程目标

确保容器重建、节点故障或数据库恢复后，Git Repo、Commit、Branch、Tag 和数据库元数据能够一起恢复，避免“数据库可恢复但代码历史永久丢失”。

## 已确认失败模式

- 生产 Compose 没有为 `GIT_STORAGE__BASE_PATH` 挂载持久卷。
- 当前备份脚本只执行 PostgreSQL `pg_dump`，不包含 Git 对象和 Ref。
- Readiness 不检查 Git 存储目录可读写。
- 缺少恢复演练、容量告警和 DB/Git 一致性盘点。

## 验收场景

### Scenario 1：容器重建不丢仓库

- **Given** Repo 已 push Commit 与 Tag
- **When** 删除并重建 Backend 容器
- **Then** clone、Commit、Branch、Tag 和 Repo Metadata 保持一致

### Scenario 2：联合备份可恢复

- **Given** PostgreSQL 与 Git Storage 已产生业务数据
- **When** 执行备份、清空环境并恢复
- **Then** 用户可重新登录、列出 Repo、clone 并校验 Commit/Tag

### Scenario 3：Git 存储不可用阻止就绪

- **Given** Git Storage 只读、路径不存在或空间耗尽
- **When** 调用 readiness 或创建/push Repo
- **Then** readiness 返回 503，写操作明确失败且不产生成功假象

## 工程要求

- Compose/K8s 显式配置 `GIT_STORAGE__BASE_PATH` 和持久 Volume。
- 备份同时覆盖 PostgreSQL、Git Repo 目录和必要版本/配置元数据。
- 定义备份一致性策略：维护窗口、快照或可重放的 reconcile 方案。
- 提供 restore 脚本或明确 SOP，并执行空环境恢复演练。
- 增加 Repo/DB 对账工具或至少可重复的盘点命令。
- 增加磁盘容量、inode、备份失败和恢复失败告警基线。
- 修改脚本时同步 `SCRIPTS-RELEASE-NOTES.md`。

## 不做事项

- 不在本 Story 实现多地域复制、Git LFS 或对象存储后端。
- 不宣称本地持久卷自动支持多实例；多实例仍需共享存储或 Repo affinity 设计。
- Repo 创建/删除业务状态机归 EVO-118-F。

## 最小验证

- `docker compose ... up` → push → 重建 backend → clone 校验。
- DB+Git 备份 → 空环境恢复 → 登录/list/clone/commit/tag 校验。
- Git 路径只读、目录丢失、空间不足故障测试。
- readiness 503/恢复 200。
- 文档、脚本 release notes 和生产配置一致性检查。

## 解锁内容

解除 DATA-01 Gate；允许 Evolith 进入外部 Alpha 的数据耐久性评估，并为 Repo 生命周期一致性提供持久存储基础。
