# Iteration 044: Phase E'-1b Repo CRUD（EVO-103-A）

> 文档状态：Active / In Progress
> 计划发布日期：2026-06-25
> 计划目标：实现 EVO-103-A Repo CRUD（HTTP API + gix::init 裸仓 + RBAC + seed-template），解锁后续 EVO-103-B/C 与 EVO-112/EVO-105/EVO-106/EVO-108 的后端依赖。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 实现 EVO-103-A Repo CRUD：HTTP API + `gix::init` 裸仓 + RBAC + seed-template。
- 这是 Phase E'-1b 的首个切片，在 EVO-101（schema）和 EVO-102（policy.yaml）完成后，补齐用户-facing 的仓库管理能力。
- 解锁后续依赖：EVO-103-B（Smart HTTP）、EVO-103-C（Context API）、EVO-105（Commit/Promote）、EVO-106（Agent Session）、EVO-108（Indexer）、EVO-112（Repo Management UI）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-103-A](../backlog/active/EVO-103-A-repo-crud.md) | Repo CRUD（gix::init + RBAC） | 父 Epic EVO-103（祖父 EVO-100） | P0 | 启动条件: EVO-101 Done, gix crate Done (EVO-086) |

## 3. 发布计划基线：不做事项

- 不实现 EVO-103-B Smart HTTP git 协议（clone/push/pull）。
- 不实现 EVO-103-C Repo Context API（file-tree / blobs / commits / diff）。
- 不实现 EVO-105 Commit API + 直推直合 + Promote API。
- 不实现 SSH / LFS（Phase 5+）。
- 不实现跨 tenant 公开 repo discover（Phase 5+）。
- 不实现 EVO-112 Repo Management UI（前端，依赖本迭代 API 完成后启动）。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-103-A 标 API 形态（HTTP CRUD + gix::init + RBAC）；适用 BDD 验收。
- [x] 6 个 Given/When/Then 场景已在 [EVO-103-A item file](../backlog/active/EVO-103-A-repo-crud.md) 中定义。
- [x] 技术验收：`cargo test --workspace` 全绿 + `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

### EVO-103-A 验收（来自 item file）

- [ ] `POST /api/v1/repos` 创建 repo → 201 + DB 记录 + 磁盘裸仓 `/srv/evolith/repos/{tenant_id}/{repo_id}.git`
- [ ] `POST /api/v1/repos` 可选 `--seed-template` 写入 `.evolith/agents.yaml` + `.evolith/policy.yaml` + README
- [ ] `GET /api/v1/repos` 返回 caller tenant 内的 repo 列表（分页）
- [ ] `GET /api/v1/repos/{id}` 返回单个 repo 详情
- [ ] `PATCH /api/v1/repos/{id}` 更新 repo 元数据（name、description、visibility）
- [ ] `DELETE /api/v1/repos/{id}` 软删除 repo（保留 stub N 天）
- [ ] 跨 tenant 访问 → 403
- [ ] 同 tenant 内重复 name → 409 或 422
- [ ] RBAC：tenant-scoped，通过现有 `AuthenticatedUser` + RBAC middleware 校验
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 场景（6 个，详见 item file）

1. **场景 1**：创建 repo → 201 + 裸仓（含 seed-template 文件）
2. **场景 2**：list 仅返回 caller tenant
3. **场景 3**：跨 tenant 访问 → 403
4. **场景 4**：重复 name → 409/422
5. **场景 5**：更新 repo
6. **场景 6**：删除 repo（软删除）

## 5. 发布计划基线：计划验证

```bash
# backend — 编译 + 测试
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# API 集成测试覆盖 6 个 BDD 场景：
# - repo 生命周期: create → list → get → update → delete
# - 跨 tenant 403
# - 重复 name 409/422
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| `gix::init` 存储路径 `/srv/evolith/repos/{tenant_id}/{repo_id}.git` 权限/目录不存在 | 启动期确保目录存在 + 错误映射为 AppError；不假设路径已存在 |
| RBAC tenant scoping 漏洞 | 依赖现有 `AuthenticatedUser` + RBAC middleware；集成测试覆盖跨 tenant 403 场景 |
| name 唯一约束竞态 | DB UNIQUE(tenant_id, name) 约束 + 冲突映射为 409 Conflict |
| seed-template 范围蔓延 | 严格按 A 验收：仅写入 `.evolith/agents.yaml` + `.evolith/policy.yaml` + README；不实现额外文件 |
| 裸仓创建失败但 DB 记录已写入 | 事务内先 DB 后磁盘；磁盘失败时回滚 DB 记录或标记失败状态 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 启动 EVO-103-A 迭代（Phase E'-1b 首切片） |
| 产物 | （待实现: Repo CRUD handlers/routes/DTO + gix::init 集成 + 测试） |
| 状态同步归口 | EVO-103-A (In Progress)、EVO-103 父项子表、PRODUCT-BACKLOG、BOARD、iterations/README |
| Story/BDD 归口 | [EVO-103-A item file](../backlog/active/EVO-103-A-repo-crud.md) 6 场景 |
| 验证证据 | （待实现后填入） |
| 残余工作归口 | EVO-103-B/C（Smart HTTP + Context API）、EVO-105（Commit/Promote）、EVO-106（Agent Session）、EVO-108（Indexer）、EVO-112（Repo UI）、SSH/LFS（Phase 5） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-25 | activation | Iteration inventory 完成：无 Active/In Progress/Review 迭代；018/019/020/027 Superseded by 2026-06-23 方向调整；025/026 Planned/Blocked（Phase F 独立）。EVO-103 刚拆为 A/B/C，EVO-103-A Ready，选入本轮。EVO-103-A 状态 Ready → In Progress；ITERATION-044 Active。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **Active / In Progress**：无（BOARD.md `Now` 段为空；iterations/ 中无 Active 文档）
- **Review**：无
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，与 Phase E' 独立）
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整 → EVO-108/109）
- **Recently Closed**：Iteration 042（EVO-101/102，2026-06-24）、Iteration 043（EVO-113，2026-06-25）

**disposition 决策**：
- Iterations 018-020/027 — Superseded by 2026-06-23 方向调整。不激活。
- Iterations 025/026 — Planned/Blocked（Phase F 独立）。维持阻塞。
- Iterations 042/043 — Closed。无需处置。
- 无 Active/Review 迭代，可直接从 backlog 选取 Ready story。

**新 Story 选取**：EVO-103 刚拆为 A/B/C 三个子 Story，EVO-103-A 满足 DoR（6 BDD 场景、依赖 EVO-101 Done、gix crate Done），选入本轮。WIP 限制 = 1 story。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

待收口填写（迭代进行中）

## 11. Retrospective

待收口填写（迭代进行中）

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 子 Story：[EVO-103-A Repo CRUD](../backlog/active/EVO-103-A-repo-crud.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
