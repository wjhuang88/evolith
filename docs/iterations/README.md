# Iterations 目录

本目录记录每轮迭代的发布计划基线、实际执行、验证、Review 和 Retrospective。

开始新迭代前必须先按 [START-ITERATION](../sop/START-ITERATION.md) 处置所有非终态 Iteration。Backlog Story Done 不自动代表 Iteration Closed。

## 当前非终态库存

| Iteration | 状态 | 目标/处置 |
|-----------|------|-----------|
| [Iteration 053](ITERATION-053.md) / [Draft PR #7](https://github.com/wjhuang88/evolith/pull/7) | Active / Awaiting independent Navigator | EVO-118-D / DATA-01；Driver 实现与实现 Head exact-head CI 已完成；治理/证据提交后的最新 Head 仍需 required CI，随后按 [Navigator Review Packet](../review/EVO-118-D-navigator-review.md) 独立复验 |
| [Iteration 018](ITERATION-018.md) | Blocked for activation / Superseded direction | 旧 Skill 导入计划，被 Git-centric EVO-100/108 替代 |
| [Iteration 019](ITERATION-019.md) | Blocked for activation / Superseded direction | 旧 Skill 多来源计划，不 deliberate replan 则不激活 |
| [Iteration 020](ITERATION-020.md) | Blocked for activation / Superseded direction | 旧 Skill 版本计划，被 Git 原生历史与 Indexer 替代 |
| [Iteration 025](ITERATION-025.md) | Blocked for activation | 租户设置与审计详情；需 refinement，不抢占 EVO-118 S1 |
| [Iteration 026](ITERATION-026.md) | Blocked for activation | Stripe Webhook/计费；需安全与 Mock 验收 refinement |
| [Iteration 027](ITERATION-027.md) | Blocked for activation / Superseded direction | 旧 Skill 发现计划，被 EVO-108/109 替代 |

### 当前启动结论

- Iteration 050 已 Closed / Complete，PR #2 已 merge。
- Iteration 051 已 Closed / Complete；PR #3 已合并并解除 SEC-01。
- Iteration 052 已 Closed / Complete；Navigator accepted runtime security，CI #125 全绿，稳定权限/API 契约及治理关闭完成，SEC-02 已解除。
- Iteration 054 已 Closed / Complete；恢复 2026-06-29 的 EVO-112-A 本地历史成果，保留主线 Iteration 050 与 PR #7 的 Iteration 053，不改变 EVO-118 S1 顺序。
- Iteration 053 已在 Draft PR #7 的 owner branch 激活；EVO-118-D 是当前 `In Progress / Awaiting independent Navigator` Story，DATA-01 仍开放。
- 实现 Head `83351a2375b6537ddb83d71d84a2d22bfaaacc15` 的 `ci` run `30736864612` 与 `data-durability-container` run `30736864631` 已全绿；后续治理/证据提交改变 Head，最终接受必须使用最新 exact-head CI。
- 独立审核入口为 [EVO-118-D Navigator Review Packet](../review/EVO-118-D-navigator-review.md)；Driver 或 CI 不得替代 Navigator 最终结论。
- EVO-118-E 保持 Ready / Not Started，不得与 Iteration 053 并行实施；Iteration 054 只是已完成本地成果的恢复，不占用运行时 WIP。
- 当前严格顺序见 [Production Readiness Plan](../roadmap/PRODUCTION-READINESS-PLAN-2026-07.md)。
- 安全、数据损坏或生产构建 P0 可显式插队；普通产品工作不得绕过 EVO-118 S1。

## 最近完成的 Git-centric / Readiness Iterations

| Iteration | 状态 | 结果 |
|-----------|------|------|
| [Iteration 042](ITERATION-042.md) | Closed | EVO-101/102 Git schema + policy parser |
| [Iteration 043](ITERATION-043.md) | Closed | Direction Pivot Review Remediation |
| [Iteration 044](ITERATION-044.md) | Closed | Repo CRUD |
| [Iteration 045](ITERATION-045.md) | Closed | Git client auth infrastructure |
| [Iteration 046](ITERATION-046.md) | Closed | Smart HTTP endpoints，真实 clone/push/pull |
| [Iteration 047](ITERATION-047.md) | Closed | WWW-Authenticate + real git-client E2E |
| [Iteration 048](ITERATION-048.md) | Closed | Repo Context API |
| [Iteration 049](ITERATION-049.md) | Closed | EVO-103 acceptance hardening |
| [Iteration 050](ITERATION-050.md) | Closed / Complete | 项目体检治理基线与生产就绪重排；EVO-118-A Done |
| [Iteration 051](ITERATION-051.md) | Closed / Complete | API Key/MCP 授权边界硬化；EVO-118-B Done，SEC-01 解除 |
| [Iteration 052](ITERATION-052.md) | Closed / Complete | HTTP Tool Egress / SSRF 安全边界、稳定契约和治理关闭；EVO-118-C Done，SEC-02 解除 |
| [Iteration 054](ITERATION-054.md) | Closed / Complete | EVO-112-A Repo UI Shell 本地历史成果恢复；PR #8 merged `1216624`，exact-head CI 全绿 |

这些 Iteration 证明 Git 后端 Alpha 基础和安全 Gate 存在，但不证明平台生产就绪。剩余发布 Gate 归 [EVO-118](../backlog/active/EVO-118-production-readiness-and-security-hardening.md)。

## 历史 Iteration 索引

| 范围 | 链接与主题 |
|------|------------|
| 001 | [工程化文档体系与流程改造](ITERATION-001.md) |
| 002 | [CLI 友好接口概念迁移](ITERATION-002.md) |
| 003 | [前端路由适配层](ITERATION-003.md) |
| 004 | [React + Vite + Bun 迁移](ITERATION-004.md) |
| 005 | [认证闭环](ITERATION-005.md) |
| 006 | [MCP 工具真实执行](ITERATION-006.md) |
| 007 | [MCP 执行质量与流程防呆](ITERATION-007.md) |
| 008 | [Epic/Story 治理](ITERATION-008.md) |
| 009 | [邮箱验证闭环](ITERATION-009.md) |
| 010 | [Skill 更新与 Parser](ITERATION-010.md) |
| 011 | [Members/API Keys 前端接线](ITERATION-011.md) |
| 012 | [历史计划，后被 Iteration 031 替代](ITERATION-012.md) |
| 013 | [计划基线保护](ITERATION-013.md) |
| 014 | [弱模型闭环防呆](ITERATION-014.md) |
| 015 | [任务闭环 SOP](ITERATION-015.md) |
| 016 | [迭代库存盘点](ITERATION-016.md) |
| 017 | [前端 CLI Interface 概念收口](ITERATION-017.md) |
| 018 | [Skill 导入计划（Superseded）](ITERATION-018.md) |
| 019 | [Skill 多来源计划（Superseded）](ITERATION-019.md) |
| 020 | [Skill 版本计划（Superseded）](ITERATION-020.md) |
| 021 | [完成声明与 Reference 修复](ITERATION-021.md) |
| 022 | [敏捷与 BDD 适配](ITERATION-022.md) |
| 023 | [治理 manifest](ITERATION-023.md) |
| 024 | [Embedded Frontend refinement（Superseded）](ITERATION-024.md) |
| 025 | [租户设置与审计计划（Blocked）](ITERATION-025.md) |
| 026 | [Stripe/计费计划（Blocked）](ITERATION-026.md) |
| 027 | [Skill 发现计划（Superseded）](ITERATION-027.md) |
| 028 | [Rustfmt 基线](ITERATION-028.md) |
| 029 | [GitHub CI/CD 重建](ITERATION-029.md) |
| 030 | [ZIP 嵌入计划（Superseded）](ITERATION-030.md) |
| 031 | [rust-embed-for-web](ITERATION-031.md) |
| 032 | [依赖全量审计](ITERATION-032.md) |
| 033 | [Serverless Runtime Spike](ITERATION-033.md) |
| 034 | [Skill/CLI 数据模型基线](ITERATION-034.md) |
| 035 | [治理/API 契约漂移修复](ITERATION-035.md) |
| 036 | [Operating Board](ITERATION-036.md) |
| 037 | [近期治理漂移修复](ITERATION-037.md) |
| 038 | [假可用快速失败](ITERATION-038.md) |
| 039 | [后端死代码清理](ITERATION-039.md) |
| 040 | [前端类型与死代码清理](ITERATION-040.md) |
| 041 | [ExecutionProvider + Docker Pool](ITERATION-041.md) |
| 042~049 | 见“最近完成的 Git-centric / Readiness Iterations” |
| 050 | [项目体检治理与生产就绪重排](ITERATION-050.md) |
| 051 | [API Key 与 MCP 授权边界硬化](ITERATION-051.md) |
| 052 | [HTTP Tool 出站安全与 SSRF 防护](ITERATION-052.md)（Closed / Complete） |
| 053 | [EVO-118-D Git Durability and Recovery](ITERATION-053.md)（Active / Awaiting independent Navigator；[Draft PR #7](https://github.com/wjhuang88/evolith/pull/7)） |
| 054 | [EVO-112-A Repo UI Shell 本地历史成果恢复](ITERATION-054.md)（Closed / Complete） |

## 计划基线保护

- `Planned` 文档一旦提交，目标、候选 Story、不做事项、计划验证和风险不可被不同目标覆写。
- 同目标执行只追加事实、证据、Review 和偏差。
- 目标变化时保留原计划，标记 Deferred/Blocked/Superseded，并使用新编号。
- 原 [Two-Month Phase E' Plan](../roadmap/TWO-MONTH-PLAN-2026-07.md) 是 2026-06-29 历史计划基线；当前激活顺序由 Production Readiness Plan 接管。

## 新 Iteration 要求

使用 [Iteration Template](ITERATION-TEMPLATE.md)，至少写清：

- 目标、Story、父 Epic 和依赖；
- 不做事项；
- Story/BDD 适用性；
- 验收、验证、风险和回滚；
- 闭环台账；
- 库存 disposition；
- 实际执行、Review、Retro；
- `Complete / Partial / Blocked`。

权限、出站网络、Git 写入、数据耐久性和发布 Story 还必须读取 [Security Review SOP](../sop/SECURITY-REVIEW.md)。