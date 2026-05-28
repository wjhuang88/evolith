# Iterations 目录

本目录记录每轮迭代计划、执行和复盘。

## 文件

- [迭代模板](ITERATION-TEMPLATE.md)
- [Iteration 001](ITERATION-001.md) — 工程化文档体系与流程改造。
- [Iteration 002](ITERATION-002.md) — CLI 友好接口概念迁移。
- [Iteration 003](ITERATION-003.md) — 前端路由适配层。
- [Iteration 004](ITERATION-004.md) — 前端技术栈迁移 React + Vite + Bun（EVO-022~025）。
- [Iteration 005](ITERATION-005.md) — 认证闭环：忘记密码与邀请接受（EVO-003/004）。
- [Iteration 006](ITERATION-006.md) — MCP 工具真实执行（EVO-005）。
- [Iteration 007](ITERATION-007.md) — MCP 工具执行质量修复与流程防呆（EVO-032）。
- [Iteration 008](ITERATION-008.md) — Epic 与子需求拆分治理规则（EVO-034）。
- [Iteration 009](ITERATION-009.md) — 邮箱验证闭环（EVO-018；
  Review：代码/测试记录存在，contract/testing/roadmap reference 收口待修复）。
- [Iteration 010](ITERATION-010.md) — Skill 更新与 SKILL.md parser（EVO-006 / EVO-009；
  Review：stories Done，API contract 与迭代收口证据待核对）。
- [Iteration 011](ITERATION-011.md) — Members 与 API Keys 前端接真实 API
  （EVO-010 / EVO-011；替换了原 EVO-016 计划，已补偏差记录）。
- [Iteration 013](ITERATION-013.md) — 已发布迭代计划基线保护与改线防呆（EVO-036）。
- [Iteration 014](ITERATION-014.md) — 治理 skill 弱模型闭环执行防呆（EVO-037）。
- [Iteration 015](ITERATION-015.md) — 本项目实施任务闭环 SOP 与完成声明门禁（EVO-038）。
- [Iteration 016](ITERATION-016.md) — 迭代启动前库存盘点与既有计划优先规则（EVO-039）。
- [Iteration 021](ITERATION-021.md) — 已实现接口完成声明与参考文档状态修复（EVO-040；
  Closed：Iteration 009 / 010 收口完成）。
- [Iteration 022](ITERATION-022.md) — 敏捷实践与 BDD 验收格式适配规则（EVO-041）。
- [Iteration 023](ITERATION-023.md) — 治理 skill manifest 接入与一致性审计（EVO-035）。
- [Iteration 024](ITERATION-024.md) — Embedded Frontend 交付形态 refinement（EVO-016-A）。
- [Iteration 025](ITERATION-025.md) — 租户设置与审计详情补齐（EVO-012 / EVO-013）。
- [Iteration 026](ITERATION-026.md) — Stripe Webhook 与计费闭环恢复（EVO-014）。
- [Iteration 027](ITERATION-027.md) — Skill 发现质量与描述治理（EVO-029）。
- [Iteration 028](ITERATION-028.md) — Rustfmt 基线与 CI 命令准备（EVO-033）。
- [Iteration 029](ITERATION-029.md) — GitHub CI/CD 重建（EVO-030）。
- [Iteration 030](ITERATION-030.md) — ZIP 嵌入前端流式响应与静态索引（EVO-016-B）。

## 未来计划

以下文档仅为排期草案，未启动实施，也不代表候选事项已进入 `In Progress`：

- [Iteration 012](ITERATION-012.md) — Embedded Frontend 文件服务与单容器交付
  （Blocked for activation：需重新排期 EVO-016 refinement）。
- [Iteration 017](ITERATION-017.md) — 前端 CLI Interface 概念收口（EVO-026；
  Ready for activation：Iteration 009 / 010 已收口，激活前重核 EVO-026 DoR 与旧路由策略）。
- [Iteration 018](ITERATION-018.md) — Skill 导入基础能力细化与基线（EVO-019 / EVO-020；
  Blocked for activation：候选仍需 DoR/refinement）。
- [Iteration 019](ITERATION-019.md) — Skill 多来源导入闭环（EVO-027；
  Blocked for activation：依赖 Iteration 018 前置结论与交付）。
- [Iteration 020](ITERATION-020.md) — Skill 版本与正确性验证（EVO-028；
  Blocked for activation：依赖 Iteration 019 导入模型）。
- [Iteration 024](ITERATION-024.md) — Embedded Frontend 交付形态 refinement（EVO-016-A；
  Ready for activation：一周方案切片，解除 Iteration 012 前置）。
- [Iteration 025](ITERATION-025.md) — 租户设置与审计详情补齐（EVO-012 / EVO-013；
  Blocked for activation：候选需 Story/BDD refinement）。
- [Iteration 026](ITERATION-026.md) — Stripe Webhook 与计费闭环恢复（EVO-014；
  Blocked for activation：候选需 webhook 安全与 mock 验收 refinement）。
- [Iteration 027](ITERATION-027.md) — Skill 发现质量与描述治理（EVO-029；
  Blocked for activation：需核对与 Iteration 019/020 的边界）。
- [Iteration 028](ITERATION-028.md) — Rustfmt 基线与 CI 命令准备（EVO-033；
  Ready for activation：一周工程质量切片）。
- [Iteration 029](ITERATION-029.md) — GitHub CI/CD 重建（EVO-030；
  Blocked for activation：依赖 Iteration 028 与部署边界确认）。
- [Iteration 030](ITERATION-030.md) — ZIP 嵌入前端流式响应与静态索引（EVO-016-B；
  Active：纯流式 body + 静态索引 + HTTP 缓存策略）。

## 非终态库存

启动任何新的产品迭代前，必须先按启动 SOP 处置：

- [Iteration 009](ITERATION-009.md) — `Closed`：EVO-040 完成收口。
- [Iteration 010](ITERATION-010.md) — `Closed`：EVO-040 完成收口。
- [Iteration 012](ITERATION-012.md) — `Planned / Blocked`：继续阻塞，等待 EVO-016-B
  (Iteration 030) 完成后更新激活条件；Iteration 030 已启动流式实现。
- [Iteration 017](ITERATION-017.md) — `Planned / Ready for activation`：只发布计划
  基线；激活前重新核对 EVO-026 的 DoR 与旧 `/snippets` 路由策略。
- [Iteration 018](ITERATION-018.md) 至 [Iteration 020](ITERATION-020.md) —
  `Planned / Blocked`：按 Phase E 依赖链保留排期，待前置计划和各 story DoR 满足后
  逐轮激活。
- [Iteration 024](ITERATION-024.md) — `Planned / Ready for activation`：EVO-016-A
  refinement，解除 Iteration 012 前置。
- [Iteration 025](ITERATION-025.md) 至 [Iteration 027](ITERATION-027.md) —
  `Planned / Blocked`：Phase F 与 Skill 发现质量候选需 refinement 或依赖确认。
- [Iteration 028](ITERATION-028.md) — `Planned / Ready for activation`：EVO-033
  Rustfmt 基线和 CI 命令准备。
- [Iteration 029](ITERATION-029.md) — `Planned / Blocked`：依赖 Iteration 028 和部署边界。
- [Iteration 030](ITERATION-030.md) — `Active`：EVO-016-B ZIP 流式响应与静态索引。

## 命名

```text
ITERATION-001.md
ITERATION-002.md
...
```

## 推荐节奏

- 常规迭代：1 周。
- Agent 微迭代：一次会话只完成一个 backlog story 或一个明确切片。
- Evolith iteration 借鉴 Sprint 的小批次、目标、验收和复盘，但本质是可审计工作批次；
  不强制完整 Scrum 仪式或团队容量统计。

## 发布计划基线规则

`Planned` iteration 一旦提交即为计划基线，不是可复用的编号占位符：

1. 实际执行仍属于原目标时，在同一文档追加激活、验证、Review 和复盘，不删除原计划。
2. 实际要做另一目标或另一组 story 时，原文档追加延期/阻塞说明，新工作创建新的
   iteration 编号。
3. 已发布的后续计划依赖被改线计划时，标注 `Blocked for activation`，直至新的前置
   计划完成。

## 状态同步

迭代开始：

1. 先按 [开始一次迭代 SOP](../sop/START-ITERATION.md) 执行固定检查。
2. 盘点本目录中的 `Active / In Progress / Review / Planned / Blocked` 文档并记录
   disposition；在途或待收口迭代优先处理。
3. 已规划迭代可以按原计划激活时优先激活；继续阻塞、延期或改线时先补记录。
4. 只有既有 iteration 已处置后，才从 [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
   选择新的 Ready 项并创建 `ITERATION-<N>.md`。
5. 明确本轮不做什么。

迭代结束：

1. 更新完成情况和验证结果。
2. 更新 backlog item 状态。
3. 新经验写入 `EVOLUTION.md`。
