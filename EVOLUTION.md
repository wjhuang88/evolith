# Evolith 故障排查与经验积累

> 本文件用于把项目中的非直觉经验沉淀为可检索规则。
> 任务开始排查问题时，先看 Part 1；任务中发现新坑时，按 Part 3 写回。

---

## Part 1: 问题速查表

| # | 现象 | 可能原因 | 快速解决 |
|---|------|----------|----------|
| 1 | Docker Compose 后端没有连 PostgreSQL | 使用了 `DATABASE_TYPE` 而不是 `DATABASE__DATABASE_TYPE` | 检查 compose/env 是否使用双下划线配置键 |
| 2 | 容器化前端请求 `/auth/login` 404 | `NEXT_PUBLIC_API_URL` 缺少 `/api/v1` | 设置为 `http://localhost:8080/api/v1` 或生产 API 前缀 |
| 3 | 修改 ConfigMap/Nginx/Compose 后线上不生效 | Git 提交不等于部署刷新 | 按发布 SOP 执行重建、重启或重新 apply |
| 4 | SQLite 与 PostgreSQL 行为不一致 | 只改了一侧 migration/repository | 同步修改 `migrations/sqlite`、`migrations/postgres` 和两套 repository |
| 5 | CSRF 403 | 状态变更请求缺少 `csrf_token` cookie 或 `X-CSRF-Token` header | 先完成登录/刷新，再由 API client 自动带 header |
| 6 | Skill 执行未进入 Docker 沙箱 | Docker 初始化失败后降级到 default executor | 查看后端启动日志中的 sandbox warn |
| 7 | lite 模式种子账号不能登录 | migrations 中的测试/admin 密码哈希是占位值，且 SQLite 内存库重启即清空 | 启动后通过注册接口创建临时账号 |
| 8 | 前端改了但 release 二进制没更新 | `rust-embed-for-web` proc macro 不跟踪 dist 目录变更 | 确认 `build.rs` 中有 `cargo:rerun-if-changed` 指向前端 dist |

---

## Part 2: 经验条目

> 新经验按时间倒序追加。避免重复记录同一问题。

### 2026-05-28 方案选型应先广后深，避免对单一方案过度优化

**现象**: EVO-016 前端嵌入方案从一开始就聚焦在 ZIP 上，经历了基础嵌入（EVO-016-A）、
静态索引优化、zip crate 2.4→8.6 升级、OnceLock + Arc<ZipArchiveMetadata>、
web::block + channel 流式架构设计等大量深度工作。最终调研发现 rust-embed-for-web
直接消除了整个问题域。

**根因**: 方案选型时没有先做广度调研（grep 所有可选方案），而是直接沿用参考项目的
ZIP 方案并持续优化。对 ZIP 的深度优化本身就是浪费——如果先花 10 分钟广度搜索
"Rust embedded static files" 就会发现 rust-embed-for-web。

**规则**: 涉及技术方案选型时，必须先做广度调研（至少 3 个替代方案），
再对最优方案深度验证。具体步骤：
1. 明确问题域和约束
2. 广度搜索所有可行方案（librarian 交叉验证）
3. 快速对比淘汰到 1-2 个候选
4. 深度验证最终候选
5. 写 ADR 记录决策

**相关**: [ADR-0003](docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md)

### 2026-05-28 rust-embed-for-web 嵌入方案需要 build.rs 监听 dist 目录变更

**现象**: 用 `rust-embed-for-web` 替换 ZIP 方案后，只改前端代码（`bun run build` 产出新 dist），
`cargo build --release` 显示 0.66s 完成，实际二进制中嵌入的仍是旧版前端资源。
浏览器打开页面看不到前端变更。

**根因**: `rust-embed-for-web` 的 proc macro 在编译时读取 `#[folder]` 指向的文件并嵌入二进制。
Cargo 的增量编译只跟踪 Rust 源码（`.rs`）和 `build.rs` 输出的 `cargo:rerun-if-changed` 指令。
前端 dist 目录变更不触发 proc macro 重新执行。

**方案**: 在 `backend/build.rs` 中添加 `cargo:rerun-if-changed=../frontend/dist/index.html` 和
`cargo:rerun-if-changed=../frontend/dist/assets/` 指令。前端 dist 文件变更时 cargo 检测到
rerun 触发条件，重新编译包含 `#[derive(RustEmbed)]` 的 crate。

**规则**: 任何使用编译时文件嵌入（`include_bytes!`、`rust-embed`、`rust-embed-for-web`）
的 Rust 项目，如果嵌入内容来源不是 `.rs` 文件，必须在 `build.rs` 中声明 `cargo:rerun-if-changed`
指向实际嵌入文件或目录。

**相关**: [ADR-0003](docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md)、Iteration 031

### 2026-05-28 Story 格式要按任务性质分型，而不是机械套用户故事

**现象**: 讨论传统敏捷、Sprint、用户故事和 BDD 时，如果只引入通用 Scrum/BDD
概念，Agent 容易把技术债、治理修复、Spike 和用户可见功能都写成同一种
`作为...我希望...以便...`，或者把 BDD 场景当成空泛模板。
**根因**: Evolith 的 iteration 是可审计工作批次，既包含产品行为，也包含技术、
治理和计划基线修复；传统用户故事格式只适合行为类产出，不能覆盖所有 Agent
执行任务的验收和闭环要求。
**方案**: 以 EVO-041 / Iteration 022 明确 Sprint 与 Evolith iteration 的关系，
并将 Story 分为 Product / API / Technical / Governance / Spike；行为类工作使用
Given/When/Then，非行为类工作使用命令级或一致性验证、状态同步和残余归口。
**教训**: 用户故事规范的核心不是统一句式，而是让身份、目标、价值、范围、不做、
验收和验证都可判断；BDD 只强制用于行为验收，技术和治理工作必须有等价证据。

### 2026-05-28 Governance manifest 是 skill adoption 的可验证入口

**现象**: Evolith 已有完整 AGENTS、SOP、backlog、iteration 和经验记录，但运行
`agent-project-governance` bundled validator 时仍失败，原因是缺少
`.agent-governance/manifest.yaml`。
**根因**: 项目事实上已经采用治理结构，但没有写入 skill 可机械识别的 adoption
manifest；只有文档存在不足以让外部 skill 判断 profile、capability 状态和入口映射。
**方案**: 以 EVO-035 / Iteration 023 补齐 manifest，记录 `high-risk / conformant`
profile、标准 entrypoints、capabilities、风险门禁和迁移映射，并把 validator 作为
后续治理变更后的固定验证。
**教训**: 面向可复用治理 skill，manifest 是“已初始化”的机器可读证据；新增或改变
治理能力后，必须同步 manifest 并运行 validator。

### 2026-05-27 开始迭代必须先盘点既有 iteration

**现象**: 流程已经禁止覆写已发布计划，但用户要求“开始迭代”时，启动 SOP 仍先围绕
backlog 候选 story 展开，不能保证 Agent 先发现仍显示 `In Progress` 的 Iteration 010
或仍为 `Planned / Blocked` 的 Iteration 012。
**根因**: 规则保护了单份计划文档的基线，却没有定义 iteration 集合的启动优先级；
story WIP 检查不能替代对在途、待收口和已排期 iteration 的库存盘点。
**方案**: 登记 EVO-039 / Iteration 016；新增 iteration inventory gate，要求先为
`Active / In Progress / Review / Planned / Blocked` 作 disposition，再允许从 backlog
选新 story；将 Iteration 010 修复为待收口 Review，并同步规则到治理 skill。
**教训**: backlog 回答“还有什么可做”，iteration inventory 回答“已经承诺或正在做
什么”；开始新工作前必须先回答后者。

### 2026-05-27 治理 skill 必须为弱闭环模型提供机械收口协议

**现象**: skill 已能说明初始化、迁移和缺陷写回方法，但依赖使用模型自行判断是否完成；
能力有限的模型可能只生成部分文件就宣称任务完成，遗漏状态同步、验证证据与残余登记。
**根因**: 方法论指引覆盖了正确方向，但缺少加载即执行的完成门禁、部分完成表达方式和
面向失败模式的评估用例。
**方案**: 登记 EVO-037，在 `agent-project-governance` skill 中新增强制闭环契约、
低自由度闭环协议与过早完成评估场景；随后以 EVO-038 将同一原则落实到 Evolith
本地 `TASK-CLOSURE.md` 及入口/迭代/Git/审查流程，要求实施任务经过建账、执行、
核验、同步和交付。
**教训**: 面向能力不稳定的 Agent，关键流程不能只说明原则，还必须提供不可跳过的
停止条件、证据要求和未完成时的明确报告格式。

### 2026-05-27 已发布迭代计划必须保留为执行对照基线
**现象**: 已发布的 `Iteration 011` 原计划用于 EVO-016 Embedded Frontend refinement，外部实施时却直接将同一文件改写为 EVO-010/EVO-011 完成记录，导致原目标、依赖关系和后续 `Iteration 012` 的前置依据在当前文档中消失。
**根因**: 现有 SOP 约束了“开始前建计划”和“中途变更记录”，但没有定义 future `Planned` 文档提交后即成为不可覆写的计划基线，也没有要求启动时目标变化必须使用新的 iteration 编号。
**方案**: 新增 EVO-036；在 AGENTS、Start/Iteration/Change-Control/Doc-Check/Git SOP 与 iteration 模板中规定计划基线保护、改线新编号和后续依赖阻塞；为历史偏差补回基线说明。
**教训**: 迭代文档不仅记录结果，也保留计划与实际之间的差异证据；已发布计划被其他工作占用时，应追加偏差并另开编号，而不是覆盖原计划。

### 2026-05-26 Epic 拆分不能只定义大小阈值
**现象**: 需求进入 SOP 仅说明“超过 0.5-2 天或多个切片时作为 Epic”，但没有定义 Epic 与 Story 的职责差异、父子编号、依赖校验、分层 DoR 或跨 Epic 迭代选择方式；维护者无法稳定判断如何记录大需求。
**根因**: 初始规则是为一次既有大项拆分提供最低限度约束，沿用了 `EVO-002` 到独立 `EVO-021` 至 `EVO-025` 的历史记录方式，没有抽象成可复用需求层级模型。
**方案**: 以 EVO-034 / Iteration 008 补充 Epic / Story 方法论：新父子关系使用同前缀后缀编号、子项依赖闭包与差异化 DoR、跨 Epic 选取约束和文档一致性检查；同步抽象到治理 skill。
**教训**: 当 backlog 开始承载多阶段工作时，SOP 不仅要说“需要拆”，还必须规定关系标识、依赖和进入迭代的门槛；历史 ID 可以保留，但新规则要能直接执行。

### 2026-05-25 迭代状态不能替代命令级验收证据
**现象**: Iteration 006 将 EVO-005 标为 Done，并勾选全量测试和 Clippy 通过；复查发现 `HttpToolExecutor::new()` 因隐式系统代理探测导致应用状态初始化与测试 panic，匿名请求可触发公开 HTTP 工具执行，HTTP 失败被包装为成功 result，且新增超时测试依赖公网。`cargo fmt`、`cargo clippy -- -D warnings`、`cargo test --workspace` 均未满足记录中的完成声明。
**根因**: 迭代计划和 Done 状态在实现提交之后一次性补写，流程只要求写验证结果但未要求保留命令级证据或阻止失败门禁被勾选；出站执行与 API Key 边界未被列入强制 Navigator 检查；测试 SOP 未禁止用公网服务作为验收依赖。
**方案**: 新增 EVO-032 / Iteration 007；禁用 executor 的隐式系统代理探测、强制 `tools/call` API Key、修复 HTTP error 映射并用本地 mock 服务覆盖真实调用；全量验证还暴露并修复了日志脱敏替换同一字段时的无限循环；更新 Start/Iteration/Pairing/Testing 流程，要求计划先于实现、验收逐命令记录、外部执行边界必须审查。
**教训**: `Done` 必须由可重复的命令和高风险边界测试支撑；涉及出站请求、认证或权限的故事，没有先行计划、Navigator 结论和本地稳定验证，不得标记完成。

### 2026-05-17 迭代验收需要覆盖部署路径、公开入口和终局/过渡边界
**现象**: Iteration 004/005 文档显示完成，但复查发现 Nginx `/assets/` 反代会剥离路径前缀、邀请接受接口未同步 RBAC/CSRF 公开例外、邀请邮件/URL 缺少可用前端入口、密码重置链接误用后端监听地址，且 backlog 详情块没有随 Done 状态同步。
**根因**: 验证只覆盖了本地 build/type-check 和局部 happy path，缺少“生产反代资源路径”“公开状态变更接口的 RBAC/CSRF 双检查”“邮件链接必须指向 `APP__PUBLIC_URL`”“总表与详情块一致性”的流程防呆；同时把 Nginx 静态托管误当成终局，而不是 EVO-016 前的过渡部署策略。
**方案**: 修复 Nginx assets 反代、公开邀请接受 API/前端 join 页面、RBAC/CSRF 例外、邮件公开 URL、生产 sourcemap 默认关闭和 Bun 构建链；同步更新 Contract/Testing/Release/Iteration/Git/Local Dev SOP 与 backlog 状态。
**教训**: 迭代完成不能只看本地构建；凡是公开链接、认证例外或部署路径变更，必须同时验证 API 合约、RBAC、CSRF、前端公开路由、邮件 URL、Nginx path rewrite 和 backlog 详情块。

### 2026-05-17 SOP 要区分“当前故事变更”和“迭代期间新需求进入”
**现象**: 在 EVO-021 路由适配迭代期间，用户提出 Skill 多来源导入、版本验证和 Snippets 残留处理等后续规划；这些需求需要进入 backlog/roadmap，但并不改变当前路由适配故事。
**根因**: `CHANGE-CONTROL.md` 只说明“迭代中收到需求变更”要停手记录，未明确独立新需求应回到 `REQUIREMENT-INTAKE.md`，容易把未来需求写进当前 iteration 的 change request。
**方案**: 更新 `REQUIREMENT-INTAKE.md`、`CHANGE-CONTROL.md`、`START-ITERATION.md`、`DOC-CHECK.md`、`GIT-WORKFLOW.md` 和 `LOCAL-DEV.md`，补充独立新需求分流、Epic/Story 拆分、旧概念检查、混合文档提交和本地测试账号规则。
**教训**: 迭代期间的新输入先判断是否改变当前 story；不改变当前 story 的，进入 backlog/proposal，不污染当前迭代变更记录。

### 2026-05-16 结对开发应采用分阶段角色切换
**现象**: 在讨论极限编程结对编程时，直接让同一 Agent 在同一上下文中同时扮演 Driver 和 Navigator，可能导致目标混杂、责任不清和上下文污染。
**根因**: 双角色并行适合两个人或两个独立上下文；单上下文中更需要阶段边界和检查表，而不是角色互相争论。
**方案**: 新增 `docs/sop/PAIRING-WORKFLOW.md`，采用 Driver 实现小切片、Navigator 检查、Driver 修正、Navigator 提交前检查的顺序模式。
**教训**: AI Agent 的结对开发应优先做“分阶段审查”，而不是“同上下文双人格”；Navigator 必须基于 SOP、ADR、backlog、diff 或具体风险给结论。

### 2026-05-16 全量 cargo fmt 检查存在既有格式基线问题
**现象**: 在 EVO-017 parser 改动后运行 `cargo fmt --all -- --check`，命令失败并输出多个无关 crate 的格式差异，同时 stable rustfmt 对部分 nightly-only 配置项发出 warning。
**根因**: workspace 里已有未格式化文件或 rustfmt 配置与 stable 工具链不完全匹配；全量 fmt 检查会把无关历史差异和本次改动混在一起。
**方案**: 对本次触碰的 Rust 文件单独运行 `rustfmt <path>`，再运行局部测试；最终验证记录中明确说明全量 fmt 失败原因和局部格式化范围。
**教训**: 在格式基线不干净的仓库中，不要用全量 fmt 结果判断本次改动失败；先保证触碰文件格式化，再把全量基线问题作为独立技术债处理。

### 2026-05-16 开始迭代也需要独立 SOP
**现象**: 用户要求“提交一下然后开始一个新的迭代”时，执行过程包含选择 Ready story、创建 iteration 文件、更新 backlog、处理临时需求补充和验证链接多个固定动作，但 AGENTS 只把入口指向通用迭代工作流。
**根因**: `ITERATION-WORKFLOW.md` 描述的是迭代内循环，不足以约束“开始迭代”这个跨 backlog、iterations、验证和变更控制的流程动作。
**方案**: 新增 `docs/sop/START-ITERATION.md`，并将 AGENTS Task Router 的“开始一次迭代”入口指向该 SOP。
**教训**: 只要一个动作会同时修改 backlog 和 iteration，就应有独立 SOP；否则后续 Agent 容易只建文件、不改状态或漏掉验证。

### 2026-05-16 过长 SOP 要按任务入口拆分
**现象**: `ITERATION-WORKFLOW.md` 同时承载需求准入、Backlog refinement、迭代执行、变更控制和状态定义，AGENTS Task Router 只能把多个不同任务都指向同一个长文档。
**根因**: 初次流程改造优先保证闭环，把相邻流程放在一起；随着防呆要求提升，过长 SOP 会让后续 Agent 难以定位必读步骤。
**方案**: 拆出 `docs/sop/REQUIREMENT-INTAKE.md` 和 `docs/sop/CHANGE-CONTROL.md`，`docs/sop/ITERATION-WORKFLOW.md` 只保留迭代执行主循环，并在 AGENTS Task Router 中分别索引。
**教训**: SOP 应按任务入口拆分；当一个 SOP 同时回答“需求怎么进来”和“开发中怎么变更”时，就需要拆成独立文件并让 AGENTS 明确路由。

### 2026-05-15 迭代中需求变更需要明确变更控制入口
**现象**: EVO-001 开发到一半时，产品方向从旧 snippet 概念切换为 CLI 友好接口；原 SOP 只描述了进入迭代和完成迭代，没有说明中途变更如何暂停、拆分和重定范围。
**根因**: 迭代工作流缺少 change request 处理步骤，容易把产品 pivot 混进当前故事，造成验收标准漂移和半成品代码扩大。
**方案**: 在 `docs/sop/ITERATION-WORKFLOW.md` 增加“迭代中需求变更”流程和防呆表；新增 ADR-0002 和 EVO-017；当前迭代停止扩大 snippet 对齐工作，把旧 snippet 相关事项转入迁移故事。
**教训**: 中途变更先记录、分类、重定范围，再继续写代码；产品概念变化必须用 ADR 和 backlog story 固化。

### 2026-05-16 执行流程动作前必须先查 Task Router 和 SOP
**现象**: 用户要求"添加远期需求"，Agent 直接读了 backlog 文件并在表格中插入一行，没有先查 AGENTS.md Task Router 确认正确流程。SOP 明确规定远期想法放 `docs/proposals/`，排期才进 backlog。
**根因**: Agent 把"需求进入"当成 trivial 操作跳过了流程检查，先动手后查规则。AGENTS.md Task Router 已有明确映射"需求进入/拆分/排期 → 必读 ITERATION-WORKFLOW.md"，但未被遵循。
**方案**: 回滚 backlog 错误条目，按 SOP 将远期需求写入 `docs/proposals/AI-GATEWAY.md` 并在索引中注册。
**教训**: 任何涉及流程的操作（需求进入、迭代变更、发布部署等），先查 AGENTS.md Task Router 找到必读 SOP，再动手。即使操作本身看起来简单，流程约束可能不简单。

### 2026-05-15 工程化文档需要分层入口
**现象**: 项目文档较完整，但 AGENTS.md 同时承担项目介绍、状态记录、流程说明和任务路由，后续 agent 需要读大量内容才能找到操作步骤。
**根因**: 文档按功能主题沉淀，但缺少 reference / sop / roadmap / proposals / archive 的职责边界。
**方案**: 新增 `docs/README.md` 文档地图、`docs/sop/` 标准流程、`docs/reference/` 稳定事实、`EVOLUTION.md` 经验写回；将原有专题文档迁移到新分层目录，并在 AGENTS.md 中建立任务路由。
**教训**: 启动文档应该短而硬，复杂步骤放 SOP，稳定事实放 reference，失败经验写 EVOLUTION。

### 2026-05-15 Agent 提交需要可追溯模型和变更边界
**现象**: Agent 生成的提交如果只写普通 commit message，后续难以追踪生成模型、验证范围和脚本行为变更是否同步记录。
**根因**: Git 规则没有进入项目级 SOP，提交前检查、模型标识和脚本 release notes 依赖个人习惯。
**方案**: 新增 `docs/sop/GIT-WORKFLOW.md`，要求语义前缀、提交末尾 `[model: <name>]`、提交前查看 staged diff，脚本行为变更同步 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
**教训**: AI 参与提交必须保留模型和变更边界，Git 规范要写成 SOP，而不是口头约定。

---

## Part 3: 维护规则

触发以下情况时，会话结束前应追加经验条目：

- 操作失败后找到了根因。
- 发现文档未记录的非直觉行为。
- 多次尝试后才解决问题。
- 用户指出了遗漏、误解或业务口径错误。
- 修改流程、脚本、部署方式后发现新的操作顺序要求。

写回格式：

```markdown
### <YYYY-MM-DD> <一句话总结>
**现象**: <具体表现>
**根因**: <深层原因>
**方案**: <解决步骤>
**教训**: <一句话，供未来 Agent 记忆>
```

写回流程：

1. 读取本文全文，确认没有重复条目。
2. 如果 Part 2 超过 30 条，先把较老条目归档到 `docs/archive/evolution-<YYYY-MM-DD>.md`。
3. 追加新条目到 Part 2。
4. 在最终回复中说明已写回的经验。
