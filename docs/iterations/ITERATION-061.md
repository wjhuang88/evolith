# Iteration 061: Repo Detail Read-only

> 文档状态：Closed / Complete
> 计划发布日期：2026-08-08
> 计划目标：完成 EVO-112-B 的真实 Repo Detail 只读浏览与受控设置流程，并用桌面/移动浏览器证据验收。
>
> 基线保护：本文件建立时实现骨架已经存在；以下基线记录实际剩余验收，不改写该流程偏差。换目标必须新建编号。
> 本轮不执行 EVO-118-E；最终生产构建/部署仍在产品开发和 legacy cleanup 全部完成后执行。

## 1. 发布计划基线：目标

- 交付可从 `/repos` 进入、可 deep-link 的 Overview / Files / Commits / Settings。
- 真实消费 Repo Context 与 Repo CRUD API，不展示假 Workspace、假 Resources 或静态业务数据。
- 补齐 loading、empty、error、not-found、移动端和删除确认状态。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|-----------|--------|---------------|
| EVO-112-B | Repo Detail Read-only | EVO-112 / EVO-100 | P0 | EVO-112-A、EVO-103-C、EVO-116 Done；EVO-118-F 与 G 直接边界已交付；H 最小 Outbox 边界已落地，后续残余不阻塞只读 UI |

## 3. 发布计划基线：不做事项

- 不实现代码编辑、Commit/Promote、Workspace、branch/tag 切换、diff viewer 或 Resources。
- 不关闭 EVO-118-H / EVENT-01，不执行 EVO-118-E / DEPLOY-01。
- 不顺带重构最终 App Shell 或删除 legacy Registry UI。

## 4. 发布计划基线：计划验收标准

- Product Story；EVO-112-B 中的 Given/When/Then 场景继续作为验收 owner。
- `/repos/:id` 规范进入 Overview，四个子路由可直接访问且刷新不丢失当前 Tab。
- Files 可浏览嵌套目录和文本 blob，明确显示 binary/empty/error 状态。
- Commits 显示提交摘要、作者、相对时间，并提供有界加载。
- Settings 支持 Story 声明的元数据字段、clone URL 复制和输入仓库名的删除确认。
- 不存在的 Repo 有明确 not-found 状态；桌面和移动布局无重叠、截断或不可操作控件。

## 5. 发布计划基线：计划验证

```bash
cd frontend
bun run type-check
bun run build
bun run lint

cd ..
python3 scripts/tests/check-markdown-links.py
git diff --check
```

- 本地启动后注册真实账号，创建真实 Repo，验证 Overview / Files / Commits / Settings / not-found。
- Playwright 桌面与移动截图覆盖 Files、Commits、Settings、404。
- 本 Story 不改后端；记录后端全量测试未重跑，不把前序后端改动归入本 Story 证据。

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 现有骨架与原验收不一致 | 以 EVO-112-B owner 的验收为准逐项补齐；未完成则保持 Partial |
| 本地端口或依赖在沙箱不可用 | 使用已授权的高位端口；记录环境限制与实际运行证据 |
| 删除操作误触 | 强制输入仓库名确认；测试失败路径，不创建通用危险操作抽象 |
| API 返回空仓或二进制内容 | 显式 empty/binary 状态，不把错误吞成空数据 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 继续全部已规划开发；本轮完成 Repo Detail 可运行验收切片 |
| 产物 | Repo Detail 路由、四个页面状态、API 接线、i18n、浏览器证据 |
| 状态同步归口 | EVO-112-B、EVO-112 父项、Product Backlog、Iteration index、Board |
| Story/BDD 归口 | EVO-112-B Acceptance Criteria |
| 验证证据 | 前端三门禁、Markdown links、diff check、真实浏览器 smoke 与截图 |
| 残余工作归口 | 编辑/Workspace -> EVO-104；Commit/Promote -> EVO-105；首次使用 -> EVO-120；Resources -> EVO-109 |

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-08-08 | deviation | Repo Detail 骨架与 API 接线先于本 Iteration 文档产生；本记录不把该顺序改写为正常流程。 |
| 2026-08-08 | activation | Iteration 060 以 Closed / Partial 收口；EVO-118-H 残余保留后，切换到 EVO-112-B。 |
| 2026-08-08 | validation | 现有骨架已通过 type-check、build、lint；代码审查确认子路由、完整 Settings、clone URL、目录浏览和浏览器证据仍缺。 |
| 2026-08-08 | implementation | 补齐四个子路由、clone URL、Settings 全字段与删除确认、嵌套目录、空/二进制状态、相对时间、有界加载、路由切换状态重置、中英文 key parity 和 390px header 响应式。 |
| 2026-08-08 | browser-validation | 在 `127.0.0.1:18080` backend、`127.0.0.1:13001` frontend 与临时 Git storage 上注册真实账号并创建 Repo；验证 Overview/Files/Commits/Settings/404、README、嵌套目录、空/二进制文件、clipboard、PATCH Saved 和临时 Repo DELETE 后列表移除。 |
| 2026-08-08 | navigator | 首轮浏览器审查发现 i18n namespace 嵌套错误与 390px header 横向溢出；修复并复验无溢出。终轮代码审查补齐跨 Repo/Tab 的异步状态重置，无 blocking finding。 |
| 2026-08-08 | closure | 前端 type-check/build/lint、Markdown links、`git diff --check` 与治理校验通过；EVO-112-B Done / Complete，Iteration 061 Closed / Complete。 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-08-08 | scope-change | 接受并新建编号 | 060 不再扩大；061 承接已开始的产品 UI 实施 | Repo Detail 骨架保留并补齐；Outbox 基础保留，残余归 H |

## 10. Review

- 完成：EVO-112-B 计划范围全部实现；四个子路由和规范化入口可用，Repo Context/CRUD、状态处理、桌面/移动布局及安全删除确认均已真实验收。
- 未完成：无本 Story 验收缺口。代码编辑、Workspace、Commit/Promote、Onboarding、Resources 等非目标继续由既有 Story 承接。
- 验证结果：
  - `bun run type-check`、`bun run build`、`bun run lint`：通过；仅保留 Browserslist 数据陈旧、`module.register()` deprecation 与 bundle size 非阻断 warning。
  - Headless Chrome：真实账号、真实 Repo 与双 commit；canonical/deep-link、README、嵌套/空/二进制、commits `limit=50`、clipboard、PATCH、DELETE、404 与 390px viewport 通过。
  - 截图：`docs/iterations/screenshots/iter-061/00-overview-desktop.png` 至 `05-files-mobile.png`。
  - `python3 scripts/tests/check-markdown-links.py` 与 `git diff --check`：通过；治理 validator 退出码 0，并报告 1 个既有 Iteration 058 验证命令识别 warning。
  - 后端全量测试：未重跑；本 Story 无后端改动，按发布计划基线明确记录。
- Navigator 结论：`Complete`，无 blocking finding。
- 闭环状态：`Complete`
- 残余归口：见闭环台账；另登记 EVO-123 修复本地端口监听误判，不阻塞本 Iteration。

## 11. Retrospective

- 需要调整的：发现实现先于 iteration 记录时，应立即登记偏差并以真实未完成验收建立新基线，不能直接补写 Done。
- 可复用经验：`lsof -ti :PORT` 会把到远端同号端口的 established 连接误判为本机监听；已登记 EVO-123，后续按独立 Story 修复。
- 浏览器审查价值：API 成功不等于页面可用；本轮真实渲染发现并修复了 i18n namespace 与 390px header 溢出。
