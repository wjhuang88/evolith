# Iteration 010: Skill 更新与 CLI Interface 格式基线

> 状态：Closed
> 计划目标：在认证闭环之后补齐 Skill 生命周期的更新入口，并建立 CLI 友好接口格式解析基线。按 WIP 限制先选入 EVO-009。
>
> 收口说明（2026-05-27）：Iteration 021 / EVO-040 已修复 `API-CONTRACT.md` 中
> `PUT /skills/{id}` 从 `⚠️ Not implemented (501)` 更新为完整响应描述。
> `IMPLEMENTATION-ROADMAP.md` Phase E Skill update 状态更新为 Done。
> `cargo test -p api` → 48 passed, 0 failed，含 `test_update_skill_success` 和
> `test_update_skill_not_found` 两个 Skill 更新 e2e 测试。所有验收标准满足，迭代关闭。

## 1. 计划边界

本轮把两个 Phase E 候选 Story 放在同一迭代中，是因为二者共同建立后续导入、版本验证与
界面迁移依赖的格式和更新基础。启动前仍需分别满足 DoR，并检查是否需要拆为两个微迭代。

## 2. 候选故事

| ID | 标题 | 所属 Epic | 优先级 | 当前状态 | 依赖/顺序 |
|----|------|-----------|--------|----------|-----------|
| EVO-006 | Skill 更新接口 | 无 | P1 | Done | EVO-009 后实施 |
| EVO-009 | SKILL.md 与 CLI interface frontmatter parser | 无 | P1 | Done | 先实施 |

## 3. 目标范围

- 实现或细化 `PUT /skills/{id}` 的真实更新闭环。
- 为 SKILL.md / CLI interface frontmatter 建立可验证 parser 契约与测试。
- 为 EVO-027 / EVO-028 的导入和版本校验能力建立稳定输入边界。

## 4. 不做事项

- 不同时实现 ZIP / Git / SkillHub 导入（EVO-027）。
- 不实现完整版本回滚或描述评分（EVO-028 / EVO-029）。
- 不清理全部前端 Snippet 残留（EVO-026）。

## 5. 计划验收标准

- [x] Skill 更新接口不再为 501，契约、错误处理和权限边界明确。
- [x] frontmatter parser 对有效和无效示例有测试覆盖。
- [x] 格式选择与后续导入/版本管理的依赖关系写入相关参考文档。（EVO-040 已同步 `API-CONTRACT.md` Skills 章节，CLI interface 兼容策略已在 contract 中记录。）
- [x] 如两个故事不能在 WIP 与验证范围内共同完成，启动时拆成顺序微迭代。

## 6. 计划验证

```bash
cargo test -p service-skill
cargo test -p api
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## 7. 风险与进入条件

| 风险 | 计划控制 |
|------|----------|
| 更新接口与未来版本模型反复重写 | 启动前确定最小版本语义与 API contract |
| parser 延续旧 Snippet 产品概念 | 验收中明确 CLI interface 是新主线，旧命名仅作兼容参考 |
| 两个 P1 故事扩大范围 | 按 WIP 和 DoR 检查，必要时只选入一个 Story |

## 8. 计划记录

| 日期 | 记录 |
|------|------|
| 2026-05-26 | Future iteration planned only. 候选 EVO-006 / EVO-009；未启动、未改变 backlog 状态。 |
| 2026-05-26 | Iteration 010 started. 按 WIP 限制先选入 EVO-009（SKILL.md parser），EVO-006 待 EVO-009 完成后再排期。EVO-009 backlog 详情块已补齐。 |
| 2026-05-26 | EVO-009 Done：SkillParser 从 stub 升级为生产级解析器，复用 CLI interface parser 模式。6 个单元测试覆盖。cargo check/clippy/test 通过。 |
| 2026-05-26 | EVO-006 Done：实现 PUT /skills/{id} 全栈（domain UpdateSkill + repo update + handler + 测试）。API 不再返回 501。cargo check/clippy/test 通过。 |
| 2026-05-27 | Inventory audit: stories 已为 Done，但“格式选择与后续导入/版本管理的依赖关系写入相关参考文档”仍无本页完成证据；状态由 `In Progress` 修正为 `Review`，启动新产品迭代前应先处置该收口缺口。 |
| 2026-05-27 | Planning audit: `API-CONTRACT.md` 仍将已实现的 `PUT /skills/{id}` 标为 `501 Not implemented`；将该参考文档漂移一并归口 EVO-040。 |
| 2026-05-27 | Planning disposition: 本迭代维持 `Review`，后续 Phase E 文档仅作为 future plan 发布；在本项核验证据完成或残余被明确归口前，不激活新的产品 iteration。 |
| 2026-05-27 | EVO-040 / Iteration 021 完成参考文档修复。`API-CONTRACT.md` 中 `PUT /skills/{id}` 从 `⚠️ Not implemented (501)` 更新为完整响应描述（含错误码）。`IMPLEMENTATION-ROADMAP.md` Phase E Skill update 状态更新为 Done。`cargo test -p api` → 48 passed, 0 failed。迭代转为 `Closed`。 |

## 10. Review

- 完成：EVO-006 Skill 更新接口（全栈实现 + 2 个 e2e 测试）；EVO-009 SKILL.md parser（6 个单元测试）；API contract / roadmap 参考文档收口。
- 未完成：无。
- 验证结果：`cargo test -p api` → 48 passed, 0 failed（含 `test_update_skill_success`、`test_update_skill_not_found`）；`cargo test -p service-skill` parser 测试通过。
- 闭环状态：`Complete`
- 残余归口：无。

## 11. Retrospective

- 做得好的：两个 story 按 WIP 限制顺序实施，parser 复用 CLI interface 模式减少重复代码。
- 需要调整的：实现完成后应立即同步 API contract，避免 Review 漂移。
- 写入 EVOLUTION：同 Iteration 009 — 参考文档状态漂移需要在实现完成时同步处理。
