# Git-Centric Platform：从 Skill Registry 到 Git 托管 + Vibe Coding 平台

> 状态：设计稿（2026-06-23 用户反馈驱动的方向调整）
> 输出物：本文件 + ADR-0004 + ADR-0005 + EVO-100~111（11 个 backlog 项，其中 EVO-104 已落地）
> 约束：当前迭代的 Skill/CLI/MCP 实现保留运行，但后续演进以本提案为基线

## 1. 背景与方向调整

Evolith 原定位是"企业级 AI Agent Harness 平台"：MCP 工具封装 + Skill 生命周期 + CLI 友好接口 + 多租户 + RBAC。在 Phase 0-7 落地过程中，定位逐步细化，但 Skill 执行层（Docker sandbox）始终是技术复杂度的最大来源，且与用户实际使用场景（agent 远端调用、vibe coding）存在错位。

2026-06-23 用户反馈：

1. 不再提供 skill 执行和智能体执行能力；保留 mcp/cli 作为 FaaS 服务模式。
2. 重心转向基于 git 仓库（嵌入完整 git 引擎，实现类似 GitHub 的代码管理）。
3. skill/mcp/cli 的存储基于 git 仓库联动。
4. git 仓库外挂大模型和多轮对话智能体引擎（引擎来自用户另一个项目）；Evolith 暴露集成 surface。

**核心类比**：git repo 与 skill/mcp/cli 的关系 ≈ GitHub Repo 与 GitHub Pages 的关系。

| GitHub | Evolith |
|--------|---------|
| Repo（git substrate） | Repo（git substrate + vibe coding 载体） |
| Repo 放 `index.html` → 自动 Pages 服务 | Repo 放 `SKILL.md` → 自动注册 Skill |
| `Settings → Pages → Source` | `.evolith/policy.yaml` |
| Org / repo 权限 | Tenant / repo ACL |

## 2. 架构目标

- **Git 托管是主产品**：Evolith 提供类似 GitHub 的代码托管；任何项目代码都可托管，不限于 skill/mcp/cli。
- **Skill/MCP/CLI 是 Pages 式衍生能力**：通过文件 pattern（`SKILL.md` / `interface.yaml` / `tool.yaml`）从 repo 中自动发现和索引。
- **Vibe coding 是核心使用场景**：用户在 Evolith Web UI 中浏览、编辑、与外部 agent engine 协作完成开发、提交 commit/promote。
- **Agent engine 外部构建**：Evolith 不构建 LLM loop，只暴露 Repo Context API、Agent Session API、Webhook Out、Scoped Token。
- **FaaS 执行**：MCP 工具保留 HttpProxy 形态；CLI 注册为可调用函数；不构建新的容器沙箱。

## 3. 核心架构

```
┌──────────────────────────────────────────────────────────────────────┐
│                          CLIENT LAYER                                 │
│  Web SPA (React+Vite+Bun) │ CLI (Rust, 远期) │ External Agent Engine │
└──────────────────────────────────┬───────────────────────────────────┘
                                   │ REST / Webhook / Smart HTTP
                                   ▼
┌──────────────────────────────────────────────────────────────────────┐
│                       EVOLITH API GATEWAY (Actix-web)                 │
│  /api/v1/repos    /api/v1/repos/{id}/contents  /api/v1/repos/{id}/git│
│  /api/v1/mcp      /api/v1/cli/execute          /api/v1/agent-sessions│
│  /api/v1/webhooks (push, agent events)                                │
└──────────────────────────────────┬───────────────────────────────────┘
                                   │
       ┌───────────────────────────┼────────────────────────────┐
       ▼                           ▼                            ▼
┌─────────────┐         ┌─────────────────────┐        ┌─────────────────┐
│ Git Service │         │ FaaS Executor Layer │        │ Agent Bridge    │
│  (NEW)      │         │  (existing+refact.) │        │  (NEW)          │
│             │         │                     │        │                 │
│ • gix-based │         │ • HttpProxyProvider │        │ • Session API   │
│   repo mgmt │         │   (MCP tools)       │        │ • Webhook out   │
│ • smart HTTP│         │ • CliExecutor       │        │ • Scoped tokens │
│   push/pull │         │   (HttpProxy only,  │        │ • Event log     │
│ • refs/obj  │         │    no sandbox)      │        │                 │
│ • LFS as    │         │                     │        │                 │
│   HTTP API  │         │                     │        │                 │
└──────┬──────┘         └──────────┬──────────┘        └────────┬────────┘
       │                            │                           │
       ▼                            ▼                           ▼
┌─────────────┐         ┌─────────────────────┐        ┌─────────────────┐
│ Bare git    │         │ PostgreSQL          │        │ External        │
│ repos on FS │         │ (metadata index +   │        │ Agent Engine    │
│ /srv/evolith│         │  git pointers +     │        │ (User's other   │
│  /repos/    │         │  tenant/users/      │        │  project)       │
│  {tenant}/  │         │  audit/RBAC)        │        │                 │
└─────────────┘         └─────────────────────┘        └─────────────────┘
```

**与原架构的关键差异**：

| 原架构 | 新架构 |
|--------|--------|
| Sandbox Runtime 整层（Docker + bollard） | 整层删除 |
| Skill/CLI/MCP 作为独立 service crate | service-skill/snippet 退化为 parser 库；service-tool 保留 |
| DB 存 skill/CLI 内容 TEXT 列 | DB 只存 metadata + git 指针 |
| agent execution 自建 | agent engine 外部，Evolith 仅暴露 integration surface |
| Sandbox 与 ExecutionProvider 三种 payload | 只剩 HttpProxy 一种 payload |

## 4. 数据模型重构

### 4.1 git_repos（第一类实体）

```sql
CREATE TABLE git_repos (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    name            TEXT NOT NULL,
    description     TEXT,
    default_branch  TEXT NOT NULL DEFAULT 'main',
    storage_path    TEXT NOT NULL,
    visibility      TEXT NOT NULL DEFAULT 'private',
    auto_merge      BOOLEAN NOT NULL DEFAULT FALSE,
    require_review  BOOLEAN NOT NULL DEFAULT TRUE,
    last_commit_sha TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);
```

### 4.2 资源索引表（discovered，非 authoritative）

```sql
CREATE TABLE skill_index (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL,
    repo_id         UUID NOT NULL REFERENCES git_repos(id),
    path            TEXT NOT NULL,
    name            TEXT NOT NULL,
    version         TEXT,
    description     TEXT,
    blob_oid        TEXT NOT NULL,
    ref             TEXT NOT NULL DEFAULT 'main',
    last_indexed_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(repo_id, ref, path)
);

CREATE TABLE cli_index (...);       -- path matches 'cli/*/interface.yaml' 或 'interface.yaml'
CREATE TABLE mcp_tool_index (...);  -- path matches 'mcp/*/tool.yaml' 或 'tool.yaml'
```

### 4.3 旧表处理（materialized cache 模式）

旧 `skills` / `snippets` / `tools` 表保留为 materialized cache，所有写入路径收敛到 `POST /repos/{id}/files`，由 indexer 同步双写。MVP 完成后评估是否升级为 view。

### 4.4 .evolith/policy.yaml 约定

```yaml
version: 1
default_action: require_review   # auto_merge | require_review | block
protected_paths:
  - SKILL.md
  - cli/interface.yaml
  - mcp/tool.yaml
agents:
  - name: "code-reviewer"
    scopes: ["read", "commit:src/"]
    auto_merge: true
```

## 5. Phase 切分

### Phase 1 — Git Service 基础
- EVO-100 Epic 容器
- EVO-101 `git_repos` 表 + 双轨 migration
- EVO-102 `.evolith/policy.yaml` 规范 + 解析器
- EVO-103 Repo CRUD API + Smart HTTP 协议（基于 gix）

### Phase 2 — Vibe Coding UX + Agent 集成
- EVO-104 Vibe Coding Web UI（UX 调研前置门禁：U-01 ~ U-05）
- EVO-105 Commit API + 直推直合 + Promote API
- EVO-106 Agent Session API + Scoped Token
- EVO-107 Webhook Out（push / promote → external agent）

### Phase 3 — Skill / CLI / MCP Indexer（Pages 式能力）
- EVO-108 Indexer 服务（git push 触发 + 路径 pattern 识别）
- EVO-109 Discovery API + Pages 式发现 UI
- EVO-110 旧表双写适配（保持 `GET /skills/{id}` 等接口兼容）

### Phase 4 — 废弃 Skill 沙箱执行
- EVO-111 删 `service-skill` 执行层 + bollard 依赖 + sandbox 镜像

### Phase 5 — 高级扩展
- SSH（russh）
- LFS HTTP API
- 跨 repo 搜索
- 资源级别 ACL

## 6. 与现有 backlog / proposal / ADR 的关系

### 6.1 旧 backlog 处置（详见 PRODUCT-BACKLOG.md 更新）

| 旧 ID | 旧标题 | 新状态 | 处置说明 |
|-------|--------|--------|----------|
| EVO-019 | Skill registry 服务化 | Dropped | git repos 天然作为 registry |
| EVO-020 | Storage 能力落地 | Dropped | git filesystem + LFS HTTP API（Phase 5） |
| EVO-027 | Skill 多来源创建 | Re-scoped | "创建文件到 repo"（EVO-103 + EVO-108） |
| EVO-028 | Skill 版本管理 | Dropped | git 提供原生版本管理 |
| EVO-029 | Skill 描述质量 | Re-scoped | SKILL.md frontmatter 质量（EVO-108 indexer 解析） |
| EVO-045 | CLI 命令执行引擎 | Re-scoped | CLI 作为 FaaS 注册（EVO-105 + EVO-108） |
| EVO-046 | Skill 可下载制品 | Dropped | repo 本身就是可下载制品 |
| EVO-047 | MCP Serverless 执行 | Dropped | MCP 执行维持 HttpProxy，不引入 serverless code execution |
| EVO-049 | Skill/CLI 生态兼容 Epic | Partially valid | EVO-049-A 已 Done；EVO-049-B parser 接线归 EVO-108 |
| EVO-050 | Skill/CLI 评分 | Dropped | out of MVP scope |

### 6.2 现有 proposal 状态更新

- **AI-GATEWAY.md**：从"远期想法"调整为"整合进 GIT-CENTRIC-PLATFORM（Agent Bridge 层）"。
- **AGENT-RUNTIME.md**：从"远期想法"调整为"整合进 GIT-CENTRIC-PLATFORM（Agent Session 抽象）"。
- **SERVERLESS-RUNTIME.md**：保留作历史参考；ExecutionProvider trait 大幅简化（仅保留 HttpProxy）。
- **EMBEDDED-FRONTEND.md**：维持 Done 状态（rust-embed-for-web）。
- **RUST-CLI.md**：保留，远期 CLI 客户端仍可能落地。

### 6.3 现有 ADR 影响

- **ADR-0001（React + Vite + Bun）**：维持。
- **ADR-0002（CLI 友好接口替代 Snippet）**：维持；CLI interface 名称继续使用，但存储介质从 DB 改为 git 文件。
- **ADR-0003（rust-embed-for-web）**：维持。
- 新增 **ADR-0004（Git-Centric Storage）**：DB content 列 → git 文件指针。
- 新增 **ADR-0005（Deprecate Sandbox Runtime）**：删除 skill 容器执行，仅保留 FaaS。

## 7. UX 调研前置门禁（EVO-104）

EV0-104 Vibe Coding Web UI 在进入迭代前必须完成 UX 议题决策：

- **P0 阻塞**（U-01 ~ U-05）：编辑器选型、主布局、Chat 流式架构、Agent Branch 心智模型、直推直合三态视觉反馈
- **P1 实施期可定**（U-06 ~ U-08）：Diff viewer、Commit 消息生成、文件冲突处理
- **P2 out of MVP**（U-09 ~ U-13）：实时多人协作、Live Preview、终端面板、文件拖拽、键盘快捷键

设计约束：所有 UI 实现遵循 `docs/reference/DESIGN.md` 的 Figma token；颜色/字号/间距不允许硬编码。

## 8. 不做

- 不实现 Vercel / Lambda 远期 serverless 集成（除非 EVO-080 spike 结论支持）。
- 不实现资源级别 ACL（Phase 5+）。
- 不实现 live preview pane、web 终端、Yjs 多人协作（明确 out of MVP）。
- 不实现对外部 git 托管平台（GitHub/GitLab）的代理（Phase 6+ 评估）。
- 不构建 LLM 调用 / planning / reviewer agent 自身（外部 agent engine 提供）。

## 9. 风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| `gix` push 能力缺口 | agent 写 commit 必须用 git CLI subprocess | Phase 1 临时 subprocess + 标 EVOLUTION；长期等 `gix-push` |
| `gix` 仍 pre-1.0 | 月度 breaking change 风险 | 锁定 minor 版本；CI 跟跑 |
| 现有 skill/CLI/MCP TEXT 数据迁移 | dump + commit + backfill 必须无损 | 双写 + 老路径兼容 + 回填脚本 dry-run + SHA-256 校验 |
| Sandbox 删除影响 `cargo test` | 部分集成测试会失败 | 删除前 sandbox 相关测试归类 `[ignore]` 或删除 |
| 旧 backlog 项依赖 skill 生命周期/存储语义 | 方向冲突 | Phase 0 末统一 rebase（见 §6.1） |
| Agent engine 外部项目尚未完成 | Phase 2 联调无下游 | mock external engine；不阻塞 Evolith 推进 |
| 多租户 git 仓储 disk 隔离 | tenant 越权访问他人仓 | 路径前缀硬绑 tenant_id；RBAC fuzzer 测试 |
| 仓库数量爆炸 | 1000 tenant × 100 资源 = 100K 仓 | 监控；超阈值触发 archive 或 sub-module |
| 直推直合策略误配 | auto_merge 高风险内容 | 默认 `require_review=true`；auto_merge 需 admin 显式开启 |

## 10. 验证方式

- [x] 方向文档（本文件）完成
- [x] ADR-0004 / ADR-0005 完成
- [x] EVO-100 Epic + 10 个子 Story 完成
- [x] EVO-104 Vibe Coding Web UI（UX 调研前置门禁）
- [x] PRODUCT-BACKLOG.md 更新（旧项 Dropped/Superseded 标注）
- [x] IMPLEMENTATION-ROADMAP.md 更新（Phase E 替换为 git-centric 主线）
- [ ] EVO-100 Phase 1 完成：用户可创建 repo、push/pull 代码
- [ ] EVO-104 UX 决策完成：U-01 ~ U-05 design doc 落地
- [ ] EVO-100 Phase 2 完成：vibe coding MVP + agent session
- [ ] EVO-100 Phase 3 完成：skill/CLI/MCP 自动索引发现
- [ ] EVO-111 完成：sandbox 整层删除，`cargo test --workspace` 全绿
- [ ] 外部 agent engine 联调：end-to-end session 创建 → commit → webhook 触发

## 11. 后续 Story 依赖图

```
EVO-100 (Epic: Git-Centric Platform Foundation)
  ├── EVO-101 git_repos 表 + 双轨 migration
  │     独立，不依赖其他 EVO-100 子项
  │
  ├── EVO-102 policy.yaml 规范 + 解析器
  │     独立，可与 EVO-103 同迭代推进
  │
  ├── EVO-103 Repo CRUD API + Smart HTTP
  │     依赖 EVO-101
  │
  ├── EVO-105 Commit API + 直推直合 + Promote API
  │     依赖 EVO-103（context API）+ EVO-102（policy 解析）
  │
  ├── EVO-106 Agent Session API + Scoped Token
  │     依赖 EVO-103
  │
  ├── EVO-107 Webhook Out
  │     依赖 EVO-105 + EVO-106
  │
  ├── EVO-108 Indexer 服务
  │     依赖 EVO-103（git push 事件）
  │
  ├── EVO-109 Discovery API + Pages 式发现 UI
  │     依赖 EVO-108（index）
  │
  ├── EVO-110 旧表双写适配
  │     依赖 EVO-108
  │
  └── EVO-111 废弃 Skill 沙箱执行
        独立（最后执行，作为清理收尾）

EVO-104 (Story: Vibe Coding Web UI)
  依赖 EVO-103 / EVO-105 / EVO-106（前端依赖后端 API）
  UX 调研：U-01 ~ U-05 必须在迭代开始前完成
```

**推荐迭代顺序**：

1. EVO-100 / Iteration N：EVO-101 + EVO-102 + EVO-103（git service MVP）
2. Iteration N+1：EVO-105 + EVO-106 + EVO-107（commit/promote + agent session）
3. Iteration N+2：EVO-104（vibe coding UI，UX 调研先行）
4. Iteration N+3：EVO-108 + EVO-109 + EVO-110（indexer + discovery + dual-write）
5. Iteration N+4：EVO-111（sandbox 清理收尾）

## 相关链接

- [ADR-0004 Git-Centric Storage](docs/decisions/ADR-0004-git-centric-storage.md)
- [ADR-0005 Deprecate Sandbox Runtime](docs/decisions/ADR-0005-deprecate-sandbox-runtime.md)
- [Product Backlog](../backlog/PRODUCT-BACKLOG.md)
- [实施路线图](../roadmap/IMPLEMENTATION-ROADMAP.md)
- [EVOLUTION.md](../../EVOLUTION.md)（2026-06-23 方向变更记录）
- [SERVERLESS-RUNTIME.md](SERVERLESS-RUNTIME.md)（历史参考，简化后保留）
- [AI-GATEWAY.md](AI-GATEWAY.md)（整合进本提案）
- [AGENT-RUNTIME.md](AGENT-RUNTIME.md)（整合进本提案）
