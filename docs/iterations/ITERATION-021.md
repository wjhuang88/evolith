# Iteration 021: 已实现接口完成声明与参考文档状态修复

> 文档状态：Closed
> 计划发布日期：2026-05-27
> 计划目标：修复已实现接口的 API contract、testing reference、roadmap 状态漂移，完成 Iteration 009 / 010 的收口，消除后续规划和回归中的文档冲突风险。
>
> 基线保护：本文件一旦提交，以下"发布计划基线"内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 修复 `API-CONTRACT.md` 中邮箱验证端点（`send-verify` / `verify-email`）和 Skill 更新端点（`PUT /skills/{id}`）仍标为未实现或过期状态的内容，使其与实际代码行为一致。
- 修复 `TESTING.md` 和 roadmap（`IMPLEMENTATION-ROADMAP.md`）中对上述接口的过期描述。
- 重新运行必需验证并记录结果后，完成 Iteration 009 / 010 的收口声明。
- 使 EVO-018 的 backlog 状态从 `Review` 恢复为 `Done`（前提是验证证据充分）。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-040 | 已实现接口完成声明与参考文档状态修复 | 无 | P1 | 代码与历史测试记录已存在 |

依赖关系：本迭代前置处置 Iteration 009（Review）和 Iteration 010（Review）的收口缺口。

## 3. 发布计划基线：不做事项

- 不新增邮箱验证或 Skill 更新业务能力。
- 不改变认证策略或 API 合约中的接口行为。
- 不混入 Phase E、embedded frontend、CI/CD 或前端 Snippet 迁移实施。
- 不修改 Iteration 012 / 017–020 的阻塞状态。
- 不批量重写无关参考文档。

## 4. 发布计划基线：计划验收标准

- [x] `API-CONTRACT.md` 不再将已实现的邮箱验证与 Skill 更新端点标为未实现或返回 501。
- [x] `TESTING.md` 对邮箱验证和 Skill 更新测试的描述与实际测试位置和命令一致。
- [x] `IMPLEMENTATION-ROADMAP.md` 中 Phase C / Phase E 对应接口的状态反映实际完成情况。
- [x] `cargo test -p api` 重新执行并记录结果，确认邮箱验证和 Skill 更新测试仍通过。
- [x] Iteration 009 / 010 的收口缺口被处置：完成验收项勾选或登记残余归口。
- [x] Markdown 链接检查和 `git diff --check` 通过。

## 5. 发布计划基线：计划验证

```bash
# backend — 确认已有测试仍通过
cargo test -p api
cargo test --workspace

# 文档一致性
# (执行 docs/sop/DOC-CHECK.md 中的链接检查)
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 已有测试在当前环境无法复现 | 记录实际结果，不伪造通过；若失败则修复后重验 |
| 参考文档修改引发其他文档链接断链 | 执行文档链接检查验证 |
| Iteration 009/010 收口发现新的未覆盖验收项 | 登记残余归口而非强制关闭 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | API contract / testing / roadmap 对已实现接口的描述与代码一致；Iteration 009 / 010 可收口或残余明确归口 |
| 产物 | 修改 `API-CONTRACT.md`、`TESTING.md`、`IMPLEMENTATION-ROADMAP.md`；更新 Iteration 009 / 010 收口记录；更新 backlog EVO-018 / EVO-040 状态 |
| 状态同步归口 | backlog 总表 EVO-018 / EVO-040；Iteration 009 / 010 Review/Retrospective；`iterations/README.md` 非终态库存 |
| 验证证据 | `cargo test -p api` 输出；文档链接检查输出；`git diff --check` |
| 残余工作归口 | 新发现的验收缺口 → backlog 新条目或 Iteration 009/010 残余记录 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-05-27 | activation | Iteration 021 started. Inventory disposition：Iteration 009/010 Review → 本迭代处置收口；Iteration 012/017–020 Planned/Blocked → 继续阻塞。EVO-040 选入，backlog 状态改为 In Progress。 |
| 2026-05-27 | progress | 代码复核确认：`handlers/auth/verify.rs` 实现 send-verify / verify-email 两个 handler（100 行），`routes/auth.rs` 已注册路由，3 个 e2e 测试覆盖。`skill_handlers.rs` 实现 update_skill handler（109 行），`routes/skills.rs` 已注册路由，2 个 e2e 测试覆盖。 |
| 2026-05-27 | progress | `API-CONTRACT.md` 修复：3 个端点从 `⚠️ Not implemented (501)` 更新为完整响应描述 + 错误码。版本历史新增 2026-05-27 条目。 |
| 2026-05-27 | progress | `TESTING.md` 修复：TC-ATH-011 从 `501` 更新为 `200`；测试统计更新为 48 passed。 |
| 2026-05-27 | progress | `IMPLEMENTATION-ROADMAP.md` 修复：§3.1 501 表 3 项更新为 `✅ Implemented`；§3.3 前后端不一致表更新；§9 迁移状态表 EVO-018 / EVO-009 更新为 Done。 |
| 2026-05-27 | validation | `cargo test -p api` → 48 passed, 0 failed（23 unit + 18 auth e2e + 7 MCP e2e）。 |
| 2026-05-27 | validation | Markdown 链接检查通过；`git diff --check` 无冲突。 |
| 2026-05-27 | completion | Iteration 009 / 010 转为 `Closed`；EVO-018 backlog 状态 `Review` → `Done`；EVO-040 `In Progress` → `Done`；`iterations/README.md` 非终态库存更新。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| | | | | |

## 10. Review

- 完成：EVO-040 全部验收标准满足。`API-CONTRACT.md`、`TESTING.md`、`IMPLEMENTATION-ROADMAP.md` 三文档状态漂移修复；Iteration 009 / 010 收口闭环。
- 未完成：无。
- 验证结果：`cargo test -p api` → 48 passed, 0 failed；Markdown 链接检查通过；`git diff --check` 无冲突。
- 闭环状态：`Complete`
- 残余归口：无。

## 11. Retrospective

- 做得好的：通过探索代理并行复核代码和文档，快速定位漂移点；三文档并行修复效率高。
- 需要调整的：未来迭代应在实现完成的同一会话中同步更新参考文档，避免 Review 漂移。
- 写入 EVOLUTION：参考文档状态漂移是迭代收口的常见陷阱（已在 Iteration 009/010 Retrospective 中记录）。
