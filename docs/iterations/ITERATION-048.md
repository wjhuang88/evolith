# Iteration 048: Phase E'-1c Repo Context API（EVO-103-C）

> 文档状态：Closed（2026-06-26）
> 计划发布日期：2026-06-26
> 计划目标：实现 EVO-103-C Repo Context API——4 个只读 GET 端点（file-tree / blobs/{sha} / commits / diff，全部纯 gix 读取，`?ref=` 默认 main），让 Web UI 能浏览仓库文件/历史/差异。完成后 EVO-103（A+B+C）全部 Done，解锁 EVO-112 Repo Management UI。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 实现 EVO-103-C Repo Context API：4 个只读 GET 端点（file-tree / blobs/{sha} / commits / diff），全部纯 gix 读取。
- 所有 GET endpoints 接受 `?ref=` 参数（默认 `main`）。
- RBAC：tenant 作用域；公开 repo 跨 tenant list 可见，但 blobs 需权限校验。
- 这是 EVO-103（A+B+C）全部完成的最后一片；完成后解锁 EVO-112 Repo Management UI。
- 纯 gix 读取，低于 EVO-103-B 风险（无 subprocess、无 Docker 变更、无 middleware 变更）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| [EVO-103-C](../backlog/active/EVO-103-C-repo-context-api.md) | Repo Context API（file-tree/blobs/commits/diff） | 父 Epic EVO-103（祖父 EVO-100） | P0 | 启动条件: EVO-103-A Done（repo 上下文） |

## 3. 发布计划基线：不做事项

- 不实现 repo CRUD（→ EVO-103-A）。
- 不实现 Smart HTTP（→ EVO-103-B）。
- 不实现 commit / promote（→ EVO-105）。
- 不实现写操作（纯只读）。
- 不实现跨 tenant 公开 discover（Phase 5+）。

## 4. 发布计划基线：计划验收标准

### Story 格式与 BDD 适用性

- [x] EVO-103-C 标 API 形态（GET 只读 + gix 读取）；适用 BDD 验收。
- [x] 6 个 Given/When/Then 场景已在 [EVO-103-C item file](../backlog/active/EVO-103-C-repo-context-api.md) 中定义。
- [x] 技术验收：`cargo test --workspace` 全绿 + `cargo clippy --workspace --all-targets -- -D warnings` 0 errors。

### EVO-103-C 验收（来自 item file）

- [ ] `GET /repos/{id}/file-tree` 返回指定 ref 的文件树结构
- [ ] `GET /repos/{id}/blobs/{sha}` 返回指定 SHA 的 blob 内容
- [ ] `GET /repos/{id}/commits` 返回 commit 列表（按时间倒序）
- [ ] `GET /repos/{id}/diff` 返回两 ref 间的 diff
- [ ] 所有 GET endpoints 接受 `?ref=` 参数，默认 `main`
- [ ] 跨 tenant 访问私有 repo → 403
- [ ] 公开 repo 跨 tenant list 可见，但 blobs 需权限校验
- [ ] `cargo test --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 全绿

### BDD 场景（6 个，详见 item file）

1. **场景 1**：file-tree 返回指定 ref 的树
2. **场景 2**：blobs 按 SHA 获取内容
3. **场景 3**：commits 列表
4. **场景 4**：diff 两 ref 间对比
5. **场景 5**：`?ref=` 默认 main
6. **场景 6**：跨 tenant 私有 repo → 403

## 5. 发布计划基线：计划验证

```bash
# backend — 编译 + 测试
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# API 集成测试覆盖 6 个 BDD 场景
# 性能基准：file-tree（100 文件）P95 < 100ms
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| gix 读取 API 正确性 | 参考 Context7 gix 文档；使用 `gix::Tree::traverse`、`gix::Object::detach`、`gix::revwalk`、`gix::diff::tree` |
| `?ref=` 解析（默认 main，缺失 ref → 404/400） | 显式处理缺失 ref 参数，默认 main；无效 ref 返回 404 |
| RBAC tenant-scoped（跨 tenant 私有 repo → 403） | 复用现有 RBAC middleware；集成测试覆盖跨 tenant 403 场景 |
| 纯只读不误改仓库 | 仅使用 gix 读取 API，无写操作 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 启动 EVO-103-C 迭代（Phase E'-1c 首切片） |
| 产物 | service-git gix 读取函数 + repo_context handlers/routes/dto + 测试 |
| 状态同步归口 | EVO-103-C (In Progress)、EVO-103 父项子表（C Done 后 EVO-103 完成）、PRODUCT-BACKLOG、BOARD、iterations/README |
| Story/BDD 归口 | [EVO-103-C item file](../backlog/active/EVO-103-C-repo-context-api.md) 6 场景 |
| 验证证据 | `cargo test` 0 failures（排除慢速 git Smart HTTP E2E，未改动）/ `cargo clippy --workspace --all-targets -- -D warnings` 0 errors / 6 BDD 场景通过（file-tree + blob + commits + diff + `?ref=` 默认 main + cross-tenant 403）。修 actix scope-shadowing（C 路由合并进 repos::configure scope）+ clippy `manual_clamp`（`min().max()` → `clamp()`）。 |
| 残余工作归口 | 跨 tenant 公开 discover（Phase 5+）、commit/promote（EVO-105） |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-26 | activation | Iteration inventory 完成：ITERATION-046/047 Closed（EVO-103-B-1/B-2 + EVO-115 Done，git 托管核心能力 + 真实 git clone/push/pull E2E 通过）；EVO-103-B 父项转 Done（两子项 Done）。无 Active/In Progress；018-020/027 Superseded；025/026 Blocked。EVO-103-C 依赖（A Done）已满足，Proposed → Ready → In Progress，选入本轮。ITERATION-048 Active。 |
| 2026-06-26 | completion | EVO-103-C 完成：4 gix 读取端点（file-tree/blobs/commits/diff）+ BDD 测试。修 actix scope-shadowing（C 路由合并进 repos::configure scope）+ clippy clamp。EVO-103 全部完成（A+B+C Done）。 |

### 迭代启动前库存盘点（per [START-ITERATION.md](../sop/START-ITERATION.md)）

按 AGENTS.md / START-ITERATION SOP 要求，启动前先盘点既有 iteration 状态：

- **Active / In Progress**：无（046/047 Closed，Now 段为空）
- **Review**：无
- **Planned / Blocked**：Iterations 025/026（Phase F 租户/计费，与 Phase E' 独立）
- **Superseded**：Iterations 018/019/020/027（2026-06-23 方向调整 → EVO-108/109）
- **Recently Closed**：Iteration 046（EVO-103-B-2，2026-06-26）、Iteration 047（EVO-115，2026-06-26）

**disposition 决策**：
- Iterations 018-020/027 — Superseded by 2026-06-23 方向调整。不激活。
- Iterations 025/026 — Planned/Blocked（Phase F 独立）。维持阻塞。
- Iterations 046/047 — Closed。无需处置。
- 无 Active/Review 迭代，可直接从 backlog 选取 Ready story。

**新 Story 选取**：EVO-103-C 依赖 EVO-103-A Done 已满足，EVO-103-B 也 Done（父项完成），满足 DoR（6 BDD 场景、依赖满足），选入本轮。WIP 限制 = 1 story。

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  |  |  |  |  |

## 10. Review

- 完成：EVO-103-C Repo Context API（4 gix 读取端点：file-tree / blobs/{sha} / commits / diff，`?ref=` 默认 main，6 BDD 场景通过）
- 未完成：无
- 验证结果：`cargo test` 0 failures / `cargo clippy --workspace --all-targets -- -D warnings` 0 errors / 6 BDD pass
- 闭环状态：`Complete`
- 残余归口：跨 tenant 公开 discover（Phase 5+）、commit/promote（EVO-105）

## 11. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：

---

## 相关链接

- 父 Epic：[EVO-100 Git-Centric Platform Foundation](../backlog/active/EVO-100-git-centric-platform-foundation.md)
- 父 Story：[EVO-103 Repo CRUD + Smart HTTP + Repo Context API](../backlog/active/EVO-103-repo-context-and-smart-http.md)
- 子 Story：[EVO-103-C Repo Context API](../backlog/active/EVO-103-C-repo-context-api.md)
- 提案：[GIT-CENTRIC-PLATFORM](../proposals/GIT-CENTRIC-PLATFORM.md)
- ADR：[ADR-0004 Git-Centric Storage](../decisions/ADR-0004-git-centric-storage.md)
- 启动 SOP：[START-ITERATION](../sop/START-ITERATION.md)
- 闭环 SOP：[TASK-CLOSURE](../sop/TASK-CLOSURE.md)
