# EVO-126 — WalGit-backed Git Data Plane Refactor

- **类型**：Epic / Architecture / Git Data Plane
- **优先级**：P0
- **状态**：In Progress / accepted direction
- **GitHub Issue**：[wjhuang88/evolith#12](https://github.com/wjhuang88/evolith/issues/12)
- **决策**：[ADR-0011](../../decisions/ADR-0011-walgit-backed-git-data-plane.md)
- **整体基线**：[Project Status Baseline 2026-09-10](../../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)
- **目标设计**：[WalGit Git Data Plane Refactor](../../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md)
- **执行顺序**：[Current Execution Plan 2026-09](../../roadmap/CURRENT-EXECUTION-PLAN-2026-09.md)

## 目标

将 Evolith 当前 filesystem-backed `service-git` 重构为基于 WalGit engine primitives 的 object-store/WAL Git Data Plane，同时保持 Evolith 对 tenant、RBAC、Agent identity、Policy、Commit/Promote、Audit/Outbox 和产品 UI 的所有权。

## 为什么是 Epic

该变更跨 toolchain/dependency、repository engine、Smart HTTP read/write、Repo Context、storage/recovery、bundle-uri 与最终 cutover；任何单一 `Done` 都无法真实表达完成，必须按可独立验收的 Story 小批次推进。

## 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 | GitHub |
|----------|----------|------|------|----------|--------|
| [EVO-126-A](EVO-126-A-walgit-dependency-toolchain-boundary.md) | Rust 1.90 + exact WalGit pin + license boundary | In Progress | 无 | Iteration 069 | [#13](https://github.com/wjhuang88/evolith/issues/13) |
| [EVO-126-B](EVO-126-B-service-git-v2-engine-boundary.md) | service-git v2 + Store/Registry lifecycle | Proposed | A | - | [#14](https://github.com/wjhuang88/evolith/issues/14) |
| [EVO-126-C](EVO-126-C-smart-http-read-path.md) | Smart HTTP read path v0/v2 | Proposed | B | - | [#15](https://github.com/wjhuang88/evolith/issues/15) |
| [EVO-126-D](EVO-126-D-wal-receive-pack-write-path.md) | receive-pack -> WAL publish + durable push event | Proposed | B | - | [#16](https://github.com/wjhuang88/evolith/issues/16) |
| [EVO-126-E](EVO-126-E-repo-context-walgit.md) | Repo Context parity on RepoHandle/ObjectAccess | Proposed | B | - | [#17](https://github.com/wjhuang88/evolith/issues/17) |
| [EVO-126-F](EVO-126-F-object-store-operations-migration.md) | object-store config/readiness/migration/recovery | Proposed | B; final acceptance uses C/D/E | - | [#18](https://github.com/wjhuang88/evolith/issues/18) |
| [EVO-126-G](EVO-126-G-bundle-uri-agent-clone.md) | secure bundle-uri Agent clone acceleration | Proposed | C/F | - | [#19](https://github.com/wjhuang88/evolith/issues/19) |
| [EVO-126-H](EVO-126-H-cutover-retire-legacy-git-engine.md) | default cutover + legacy Git engine retirement | Proposed | C/D/E/F/G | - | [#20](https://github.com/wjhuang88/evolith/issues/20) |

## Epic 完成条件

- Object store 是默认 Git durable truth，本地 repo/pack 仅为 cache。
- 标准 Git clone/fetch/pull/push v0/v2 与 Repo Context 在新引擎上通过。
- Push publication 使用 WAL/manifest CAS，且 Evolith durable event/audit boundary 保持。
- bundle-uri 安全授权和 fallback 验收完成。
- existing filesystem repos 有可重复、可恢复的迁移路径。
- production default 不依赖 durable local Git filesystem。
- legacy Smart HTTP/storage 主路径 consumer inventory 为零并清理。
- Security/Navigator/Data review 与文档/Gate 同步闭合；GIT-DP-01 Closed。

## 不做

- 不引入第二个 WalGit/Axum 产品控制面。
- 不以 WalGit Auth/UI/Policy 替代 Evolith 产品语义。
- 不在本 Epic 实现 EVO-105/106 Agent 产品闭环。
- 不自动把 LFS 纳入 initial cutover。

## 风险

- WalGit pre-1.0 API drift；以 exact SHA pin + service-git anti-corruption layer 控制。
- filesystem -> object-store migration 数据完整性。
- object-store availability/credential/cost/retention 运维边界。
- DB Outbox 与 object-store Git publication 跨系统一致性无法伪装为 ACID；必须以 state/reconcile/idempotency 证明。

## 验证总则

每个子 Story 独立记录测试命令、真实 Git client E2E、负向/故障证据与 residual。Epic 不直接进入 Iteration；只选择满足 DoR 的子 Story。
