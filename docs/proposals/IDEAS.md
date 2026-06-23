# Evolith 创意与需求池

用于快速记录 Evolith 项目的灵感, 特性需求与改进想法。

## 快速添加模板

复制以下格式到对应章节：
`- [ ] **[标题]** | [一句话描述] | 优先级：[🔥/⚡/💭] | 状态：[✅/🚧/📋/💡]`

---

## 设计原则

> **Agent-First, Human-Friendly**
>
> 平台所有能力（Tool/Skill/Snippet/管理操作）必须优先面向智能体设计。每个工作台空间暴露统一的机器可读入口, 智能体访问一个 URL 即可自动发现全部资源、理解其 schema、完成安装和调用, 无需人工介入。人类操作界面（Web UI、CLI）是在此基础上的便捷层, 而非反过来。
>
> 这意味着:
> - 工具安装、技能加载、片段引用、配置生成等操作, 必须同时有 API 支持智能体自动完成
> - 开发者工具链（CLI/SDK/配置生成器）是对同一套 API 的封装, 不是独立实现
> - 平台管理操作（审核、封禁、用量查看）也应有结构化 API, 支持运维智能体自动化

---

## MCP Tools (工具封装)

- [ ] **MCP 工具市场** | 允许用户发布, 搜索与一键导入社区分享的 MCP 工具 | 优先级：🔥 | 状态：💡
- [ ] **远程工具健康检查** | 自动监控并报告远程 MCP Server 的在线状态与响应延迟 | 优先级：⚡ | 状态：📋
- [ ] **LLM 输出格式切换** | 所有 API 支持 `?format=llm` 参数, 返回去除人类可读字段的结构化 schema, 降低 token 消耗 | 优先级：🔥 | 状态：💡
- [ ] **工具注册即发布** | 创建工具时自动生成 OpenAI Function Calling / MCP Tool 格式, LLM 可直接调用无需二次适配 | 优先级：🔥 | 状态：💡

## Skills System (技能系统)

- [ ] **技能版本对比** | 可视化对比不同版本间的代码变更与 YAML 配置差异 | 优先级：⚡ | 状态：💡
- [ ] **多语言 Sandbox 支持** | 在现有 Python/Node.js 基础上增加 Go 与 Rust 的执行环境 | 优先级：💭 | 状态：📋
- [ ] **Skill 作为 Agent 编排单元** | 多个 Skill 可组合成 Pipeline, 由 LLM 按需调度执行, 而非人手动单次触发 | 优先级：⚡ | 状态：💡
- [ ] **Skill 自描述协议** | Skill 声明自己的输入/输出 schema + 前置条件 + 失败策略, LLM 可自主判断能否调用 | 优先级：⚡ | 状态：💡

## Code Snippets (代码片段)

- [ ] **代码片段智能推荐** | 根据当前编辑上下文自动匹配并推荐相关的 LLM 优化代码片段 | 优先级：🔥 | 状态：💡
- [ ] **Markdown 自动导出** | 将代码库中的文档注释一键转换为符合 Snippet 规范的文档 | 优先级：💭 | 状态：📋
- [ ] **Snippet 输出为 Function Calling** | 代码片段自动转换为 OpenAI Function Calling / MCP Tool 格式, LLM 可直接调用 | 优先级：🔥 | 状态：💡
- [ ] **上下文感知推荐 API** | LLM 传入当前代码上下文, 接口返回最匹配的 snippet 结构化列表, 而非人浏览搜索结果 | 优先级：⚡ | 状态：💡

## Platform / Infra (平台与架构)

- [ ] **平台管理员体系** | 区分平台级管理员（超管）与租户级角色, 平台管理员可管理全局租户、审核市场内容、查看全局用量、封禁/解封租户; 需要独立的管理后台界面, 与租户用户界面分离 | 优先级：🔥 | 状态：💡
- [ ] **多角色界面分层** | 按使用角色区分界面体验: Agent 开发者（工具调用+集成）、技能创作者（创作+发布+收益）、平台运营（审核+数据+运维）、租户管理员（成员+配额+计费）; 不同角色看到不同的导航/功能/仪表盘 | 优先级：🔥 | 状态：💡
- [ ] **多租户资源配额** | 限制单个租户的 Sandbox 执行时长与 API 调用频率 | 优先级：⚡ | 状态：🚧
- [ ] **分布式缓存架构** | 将内存缓存迁移至 Redis 集群以支持更高并发 | 优先级：💭 | 状态：📋
- [ ] **Agent 自动诊断接口** | `/api/v1/diagnose` 接收错误描述, LLM 自动获取日志/trace、定位根因、生成修复方案 | 优先级：🔥 | 状态：💡
- [ ] **决策 API 替代 Dashboard** | 暴露 `/api/v1/decisions` 让 LLM 获取"该做什么"的上下文, 而非给人看的统计看板 | 优先级：⚡ | 状态：💡
- [ ] **LLM-to-LLM 审计** | 审计日志输出为结构化事件流, 由另一个 LLM 做异常检测和合规审查, 而非人翻表格 | 优先级：💭 | 状态：💡

## Frontend UX (前端体验)

- [ ] **暗黑模式适配** | 完善全站 UI 的 Dark Mode 支持以提升开发者夜间使用体验 | 优先级：⚡ | 状态：📋
- [ ] **拖拽式流程画布** | 通过可视化连线编排多个 MCP 工具与技能的调用链路 | 优先级：💭 | 状态：💡

## Workspace (工作台空间)

> **2026-06-23 方向调整**：工作台空间概念**全部折叠为 git repo**（详见 [GIT-CENTRIC-PLATFORM.md](GIT-CENTRIC-PLATFORM.md) + [ADR-0004](../decisions/ADR-0004-git-centric-storage.md) + [ADR-0005](../decisions/ADR-0005-deprecate-sandbox-runtime.md) + EVO-100 Epic）。
>
> 代码层验证：`workspace_id` / `workspace_slug` 全代码库零命中；仅 6 处 cosmetic 文案（已修复）。Tenant 模型无 workspace 概念；多租户边界由 `tenant_id` 维护。
>
> 7 条原始 ideas 与新方向映射如下：
> - 工作台空间核心 → **Repo**（EVO-100 ~ EVO-103）
> - 统一智能体入口 (Agent Gateway) → Repo Context API（EVO-103）+ Discovery API（EVO-109）
> - 智能体自助操作 API → Agent Session API（EVO-106）+ Webhook Out（EVO-107）
> - 智能体适配协议 → `.evolith/agents.yaml` + `?agent=` query param（EVO-102 + 后续独立 EVO）
> - 工作台模板市场 → Template repos（git fork/clone，天然支持，Phase 5+ 评估）
> - 工作台权限隔离 → Per-repo ACL + tenant RBAC（EVO-103）
> - 工作台状态同步 → Webhook Out（EVO-107）
>
> 状态：**已整合进 GIT-CENTRIC-PLATFORM**，保留条目作为历史设计意图参考，不再单独推进。

- [ ] **工作台空间核心** | 用户可创建自定义工作台空间, 自由组合平台功能（Tools/Skills/Snippets）形成专属能力集合 | 优先级：🔥 | 状态：✅ 已整合（折叠为 Repo；EVO-100 ~ EVO-103） |
- [ ] **统一智能体入口 (Agent Gateway)** | 每个工作台暴露单一入口 URL, 智能体访问后获得该空间全部能力的结构化描述（工具列表 + 技能列表 + 片段索引 + 操作 API schema）, 可自主完成发现→安装→调用全流程, 无需人工中转 | 优先级：🔥 | 状态：✅ 已整合（EVO-103 Repo Context API + EVO-109 Discovery） |
- [ ] **智能体自助操作 API** | 统一入口返回的所有资源都附带可执行的 API endpoint, 智能体可通过 API 直接完成: 安装 Skill 到本地、生成 MCP 配置、拉取 Snippet、执行 Tool, 所有操作均为 API-first | 优先级：🔥 | 状态：✅ 已整合（EVO-106 Agent Session API + EVO-107 Webhook Out） |
- [ ] **智能体适配协议** | 入口接口支持 `?agent=opencode|cursor|claude-desktop` 参数, 自动输出目标智能体原生格式的配置（skill 目录结构 / mcp.json / rules 文件）, 智能体无需理解转换逻辑 | 优先级：🔥 | 状态：✅ 已整合（EVO-102 `.evolith/policy.yaml` + `agents.yaml` + 后续 EVO-109 跨仓适配） |
- [ ] **工作台模板市场** | 预置常见场景的工作台模板（如"前端开发"、"数据分析"、"DevOps"）, 用户一键克隆后自定义 | 优先级：⚡ | 状态：💡 Phase 5+ 评估（template repos 由 git fork 天然支持） |
- [ ] **工作台权限隔离** | 不同工作台空间之间资源隔离, 支持团队协作与独立 API Key 管理 | 优先级：⚡ | 状态：✅ 已整合（EVO-103 per-repo ACL + tenant RBAC） |
- [ ] **工作台状态同步** | 工作台配置变更时主动通知已接入的智能体, 实现热更新而非重新拉取 | 优先级：💭 | 状态：✅ 已整合（EVO-107 Webhook Out） |

## AI / LLM Integration (AI 集成)

- [ ] **Prompt 自动优化器** | 基于执行反馈自动调优技能中的 System Prompt 模板 | 优先级：🔥 | 状态：💡
- [ ] **多模型基准测试** | 在 Sandbox 中对比不同大模型对同一技能的执行成功率 | 优先级：⚡ | 状态：📋

## Developer Toolchain (开发者工具链)

> 以下工具均为统一智能体入口 API 的上层封装, 面向人类开发者提供便捷操作。

- [ ] **CLI 工具 (evolith-cli)** | 封装 Agent Gateway API, 命令行支持 `evolith install <skill>`, `evolith search`, `evolith publish`, 免登录浏览 + API Key 认证 | 优先级：🔥 | 状态：💡
- [ ] **本地开发 SDK** | Python / TypeScript SDK 封装 Agent Gateway API, 开发者可在代码中直接调用远程 Skill/Tool/Snippet | 优先级：⚡ | 状态：💡
- [ ] **Web UI 一键安装** | 平台 UI 上的"安装到智能体"按钮, 调用同一套 API 生成安装配置供用户下载或复制 | 优先级：⚡ | 状态：💡
