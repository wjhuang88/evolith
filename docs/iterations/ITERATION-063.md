# Iteration 063: Commit Evidence Deep Link

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-09
> 计划目标：完成 EVO-112-C 的 tenant-scoped Commit detail API 与 `/repos/:id/commits/:sha` 证据页，从现有 Commit 列表形成真实 deep-link 闭环。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。
> 本轮不实现 Agent/Workspace/Activity 入口、文本 patch viewer、Git 写入或生产发布。

## 1. 发布计划基线：目标

- 新增稳定、tenant-scoped 的 commit detail read API，返回完整身份、作者/提交者、parents、经验证查询 Ref 与 structured changed-file diff。
- Commit 列表 SHA 可点击进入 `/repos/:id/commits/:sha?ref=<branch>`，页面覆盖 loading/success/not-found/error 与移动布局。
- 当前普通 Git commit 即可端到端验证；未来 Agent/Workspace/Activity 只复用该稳定目标，不作为本轮前置。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-112-C | Repo Commit Evidence Detail | EVO-112 | P0 | EVO-112-B、EVO-103-C Done；ADR-0008 与 Repo Context 资源/租户边界保持不变 |

## 3. 发布计划基线：不做事项

- 不实现 textual patch/diff viewer、Agent result、Workspace、Activity、Promote、Revert、Cherry-pick 或讨论线程。
- 不修改数据库、Git 写入策略、生命周期、认证模型或生产部署。
- 不把跨 tenant SHA 的存在性暴露给无权调用者。

## 4. 发布计划基线：计划验收标准

- Story 形态：API / Product Story；EVO-112-C Given/When/Then 是验收 owner。
- Contract-first 定义 commit detail endpoint、DTO、错误码与资源上限。
- full SHA 必须精确校验；查询 Ref 必须可到达目标 commit，root commit 也返回 added-file entries。
- 复用现有 `repo:read` / tenant ownership 门禁；跨 tenant、错误 repo、未知 SHA/Ref fail closed。
- Commit list SHA 可键盘访问并 deep-link；详情页展示 message、author/committer、parents、Ref 和 changed files。
- desktop 与 390px 无横向页面溢出；changed-file 区域可独立横向滚动。
- en/zh-CN key parity、API client/types、API Contract、父 Epic/Backlog/Board/AGENTS/readiness 状态同步。

## 5. 发布计划基线：计划验证

```bash
cd backend
cargo fmt --all -- --check
cargo test -p service-git
cargo test -p api --test repo_context_e2e_tests
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

cd ../frontend
bun run type-check
bun run build
bun run lint

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
sh /Users/GHuang/.agents/skills/agent-project-governance/scripts/validate_project_governance.sh .
```

- 真实 backend/frontend + 浏览器验证 list -> detail、unknown SHA、移动布局；截图归档到 `docs/iterations/screenshots/iter-063/`。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 任意 SHA 泄露其他 Repo 对象 | 先按 tenant/repo ownership 找 Repo，再只在该 repo object database 中精确解析；跨 tenant 沿用 fail-closed 门禁 |
| Ref 只是展示字符串、实际不可达 commit | service-git 验证 commit 是请求 Ref 的 ancestor；不可达返回 404 |
| root commit 无 parent 导致 diff 失败 | 对 root tree 生成 added-file structured entries，受 `DIFF_MAX_ENTRIES` 约束 |
| 大历史或大 diff 阻塞 worker | 保留 `web::block` + Context timeout，并复用 diff entry 上限 |
| 文本 patch 扩大资源/二进制边界 | 明确不在本轮，归 EVO-104，不用假 patch 占位 |

## 7. 安全审查最小记录

| 项目 | 内容 |
|------|------|
| 受保护资产 | tenant Repo Git 对象、commit 元数据、Ref 与 changed-file 路径 |
| 攻击者/调用者 | 跨 tenant JWT、无 `repo:read` API Key、猜测 SHA 的调用者 |
| 入口 | `GET /tenant/{tenant_id}/repos/{repo_id}/commits/{sha}` |
| 信任边界 | Browser/API Key -> API -> tenant Repo object database |
| 失败模式 | 跨租户泄露、SHA 歧义、Ref 不可达却伪造上下文、无界历史/diff |
| 安全默认 | ownership/scope 先验证；无效或不可达对象拒绝；读取超时/超限有界失败 |
| 验证证据 | API Key scope、cross-tenant、unknown/unreachable SHA/Ref、root/diff bound 集成测试 |

## 8. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 持续完成全部规划迭代；本轮交付可从 Commit list 访问的真实 Commit evidence deep link |
| 产物 | API Contract、service-git commit detail、DTO/handler/route/tests、frontend client/types/page/i18n、浏览器证据 |
| 状态同步归口 | EVO-112-C、EVO-112/EVO-100、Product Backlog、Iteration 063/index、Board、AGENTS、Readiness/roadmap |
| Story/BDD 归口 | EVO-112-C Acceptance Criteria |
| 验证证据 | service/API tests、backend workspace gates、frontend gates、真实浏览器正常/负向/移动、docs/diff/governance checks |
| 残余工作归口 | textual patch/Workspace/Agent -> EVO-104/105；Activity entry -> EVO-121-E；生产 Gate -> EVO-118-E |

## 9. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-08-09 | inventory | 无 Active/Review；018-020/027 继续因 Superseded direction 阻塞，025/026 继续等待独立 refinement；056/060 Closed / Partial residual 已有 owner，不阻塞本 Story。 |
| 2026-08-09 | clarification | 将未来 Agent-result 入口从当前验收移回 EVO-105/EVO-104；Commit evidence 目标页仍完整保留，并明确 structured diff 与父项 textual diff viewer Non-goal 的边界。 |
| 2026-08-09 | activation | EVO-112-C 硬依赖闭合并满足 DoR；Iteration 063 成为唯一 Active Iteration。 |
| 2026-08-09 | driver | 按 Contract First 新增 tenant-scoped commit detail API、精确 40 位 SHA 与 Ref reachability 校验、root commit added-file evidence、Commit list deep link、详情页、错误态与 en/zh-CN 文案。 |
| 2026-08-09 | browser | 应用内浏览器因沙箱元数据缺失不可用；按 browser skill 故障降级到隔离 Chrome。真实 seeded Repo 追加第二个 commit 后，list -> detail、未知 SHA 404、桌面与 390px 布局通过，页面无横向溢出，changed-file table 独立滚动；截图归档于 `screenshots/iter-063/`。 |
| 2026-08-09 | navigator | 独立阶段复核 tenant/API Key 门禁先于 Git read、SHA 精确解析、Ref 可达性、root/first-parent diff、资源上限/timeout、Contract/DTO/client parity、404 与移动布局；无 blocking finding。 |
| 2026-08-09 | validation | Backend fmt/check/strict Clippy/workspace tests、Frontend type-check/build/lint、locale parity、Markdown links、diff check 与 governance validator 全部通过。 |
| 2026-08-09 | closure | EVO-112-C 与父项 EVO-112 达到 Done / Complete；状态先回写 owner docs，再同步 Backlog、Iteration index、Board、AGENTS、Readiness 与 roadmap。Iteration 063 Closed / Complete。 |

## 10. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-08-09 | clarification | 接受 | 消除未来入口依赖和 textual patch owner 冲突，不改变稳定 Commit evidence deep-link 目标 | future entry/patch 保留在既有 owner；本轮无半成品 |

## 11. Review

- 完成：commit detail Contract/service/API/负向测试、Commit list deep link、详情/404/移动页面、i18n 与真实浏览器证据全部交付。
- 未完成：本 Story 无未完成验收；文本 patch、Workspace/Agent/Activity 入口仍按原计划留在 EVO-104/105/121-E，未被本轮扩大。
- 验证结果：`cargo fmt --all -- --check`、`cargo check --workspace --all-targets`、strict Clippy、`cargo test --workspace`、Frontend 三门禁、locale parity、Markdown links、`git diff --check` 与治理校验通过；浏览器无 page error。
- Navigator：`Complete`，无 blocking finding。
- 闭环状态：`Complete`
- 残余归口：见闭环台账，不阻塞本 Story。

## 12. Retrospective

- 做得好的：先澄清“稳定 Commit 证据目标页”与未来 Agent 入口、文本 patch viewer 的 owner 边界，使当前 Story 能小批次独立验收；浏览器同时验证真实 Git parent/change evidence 和移动溢出边界。
- 需要调整的：应用内浏览器的沙箱元数据故障仍需平台侧修复；本轮已记录降级事实并使用隔离 Chrome，不形成项目缺陷。
- 写入 EVOLUTION：未发现新的项目级可复用陷阱；SQLite `:memory:` 已由 EVO-124 归口，浏览器连接故障属于工具环境，不写入项目演进经验。
