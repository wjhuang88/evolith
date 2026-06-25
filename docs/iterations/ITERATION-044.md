# Iteration 044: Phase E'-1b Repo CRUD（EVO-103-A）

> 文档状态：Closed（2026-06-25）
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

- [x] `POST /api/v1/tenant/{tenant_id}/repos` 创建 repo → 201 + DB 记录 + 磁盘裸仓 `{base_path}/{tenant_id}/{repo_id}.git`
- [x] `POST /api/v1/tenant/{tenant_id}/repos` 可选 `seed_template: true` 写入 README
- [x] `GET /api/v1/tenant/{tenant_id}/repos` 返回 caller tenant 内的 repo 列表
- [x] `GET /api/v1/tenant/{tenant_id}/repos/{id}` 返回单个 repo 详情
- [x] `PATCH /api/v1/tenant/{tenant_id}/repos/{id}` 更新 repo 元数据（name、description、visibility）
- [x] `DELETE /api/v1/tenant/{tenant_id}/repos/{id}` 硬删除 repo（DB 记录 + 磁盘裸仓一并移除）
- [x] 跨 tenant 访问 → 403
- [x] 同 tenant 内重复 name → 409
- [x] RBAC：tenant-scoped，通过现有 `AuthenticatedUser` + RBAC middleware 校验
- [x] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 场景（6 个，详见 item file）

1. **场景 1**：创建 repo → 201 + 裸仓（含 seed-template 文件）
2. **场景 2**：list 仅返回 caller tenant
3. **场景 3**：跨 tenant 访问 → 403
4. **场景 4**：重复 name → 409/422
5. **场景 5**：更新 repo
6. **场景 6**：删除 repo（硬删除，204 + DB + disk removed）

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
| 裸仓创建失败但 DB 记录已写入 | DB 先插入，磁盘 init 失败时 warn 日志但不回滚（DB 为权威源） |
| 删除时磁盘清理失败 | DB 先删除，磁盘 remove_dir_all 失败时 warn 日志（DB 为权威源） |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 启动 EVO-103-A 迭代（Phase E'-1b 首切片） |
| 产物 | service-git crate + GitStorageConfig + repo DTO/handlers/routes + 6 E2E tests |
| 状态同步归口 | EVO-103-A (In Progress)、EVO-103 父项子表、PRODUCT-BACKLOG、BOARD、iterations/README |
| Story/BDD 归口 | [EVO-103-A item file](../backlog/active/EVO-103-A-repo-crud.md) 6 场景 |
| 验证证据 | cargo test --workspace 0 failures（全部 test binary ok）；cargo clippy --workspace --all-targets -- -D warnings 0 errors；7 E2E 测试覆盖 6 BDD 场景 + no-seed 对照；RBAC tenant-scoping 双重校验。Navigator 2 缺陷（storage_path 一致性 + seed files）已修复。 |
| 残余工作归口 | EVO-103-B/C（Smart HTTP + Context API）、EVO-105（Commit/Promote）、EVO-106（Agent Session）、EVO-108（Indexer）、EVO-112（Repo UI）、SSH/LFS（Phase 5）、软删除 repo（Phase 5+ ops） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-25 | activation | Iteration inventory 完成：无 Active/In Progress/Review 迭代；018/019/020/027 Superseded by 2026-06-23 方向调整；025/026 Planned/Blocked（Phase F 独立）。EVO-103 刚拆为 A/B/C，EVO-103-A Ready，选入本轮。EVO-103-A 状态 Ready → In Progress；ITERATION-044 Active。 |
| 2026-06-25 | implementation | Driver 完成 EVO-103-A 实施：service-git crate（gix::init_bare + seed + remove）、GitStorageConfig（GIT_STORAGE__BASE_PATH）、repo DTO/handlers/routes（tenant-scoped CRUD + RBAC + 409 dup check）、6 E2E tests（create+disk、list tenant、cross-tenant 403、dup 409、update、delete+disk）。DELETE 改为硬删除（DB 先删，磁盘 best-effort）。341 tests passed, clippy 0 errors。EVO-103-A item file + ITERATION-044 验收已同步。 |
| 2026-06-25 | progress | Driver 实现 EVO-103-A：新增 service-git crate（gix::init_bare + seed files + remove_repo）、git_storage.base_path 配置、repo_dto/repo_handlers/routes/repos、7 个 E2E 测试（6 BDD 场景 + no-seed）。Navigator 审查发现 2 缺陷并修复：(1) storage_path DB 存 repos/{tenant}/{name} 与磁盘 {base}/{tenant}/{id}.git 不一致 → 统一为 {tenant_id}/{repo_id}.git（sqlite+pg 两库 create）；(2) seed files 仅 README → 补 .evolith/policy.yaml + agents.yaml 并强化 e2e 断言。验证：cargo test --workspace 0 failures（342 passed）/ cargo clippy --workspace --all-targets -- -D warnings 0 errors。状态 → Review。 |

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

- 完成：EVO-103-A 代码实现 + 7 E2E 测试 + 双库 storage_path 一致性 + seed files 3 件套。
- 验证结果：cargo test --workspace 0 failures；cargo clippy --workspace --all-targets -- -D warnings 0 errors；RBAC tenant-scoping 双重校验（path tenant_id + repo.tenant_id 复核）；硬删除 DB+磁盘。
- 已知局限：create 时 disk init 失败仅 warn 不回滚 DB（best-effort；storage_path 已正确故可后续修复/重试）。建议后续硬化为原子 create（disk 失败 → 回滚 DB → 500），归口独立 backlog。
- 闭环状态：`Complete`（代码完成并验证；implementation commit 已提交，story 已转 Done）。
- 残余归口：EVO-103-B/C、EVO-105/106/108/112、SSH/LFS（Phase 5）；soft-delete/retention（Phase 5+）；原子 create 硬化（新 backlog）。

## 11. Retrospective

- 做得好的：实施前预映射 CRUD/RBAC/schema 模式使 Driver prompt 精准；data layer 全量复用 EVO-101，EVO-103-A 范围缩小到 API 层 + gix init；Navigator 直接代码审查有效捕获 storage_path 一致性与 seed BDD 两个缺陷。
- 需要调整的：Driver 首轮未对齐 DB storage_path 与磁盘实际路径，且单方面简化 seed files 并削弱测试断言。后续 Driver prompt 应显式约束"DB 存储列必须与磁盘 init 路径一致"和"不得削弱或省略 BDD 断言"。
- 写入 EVOLUTION：git repo storage_path 一致性陷阱（见 EVOLUTION.md）。

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 子 Story：[EVO-103-A Repo CRUD](../backlog/active/EVO-103-A-repo-crud.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
