# 规划: Evolith Rust CLI

> 状态：远期目标
> 目标：为用户提供本地智能体、技能、工具和代码片段的命令行管理能力。

## 1. 背景

Evolith 当前以 Web 控制台和 HTTP/MCP API 为核心。面向 Agent 开发者时，还需要一个可脚本化、可集成到本地工作流的 CLI。

参考方向：

- SkillHub / ClawHub 类工具的发布、拉取、登录、同步体验。
- OpenCode 参考项目中 `oc-platform` 的 Rust CLI 子项目组织方式。

## 2. 子项目位置

建议在前端/后端之外新增：

```text
cli/
├── Cargo.toml
├── crates/
│   ├── evolith-cli/      # CLI 入口
│   └── evolith-client/   # API client、配置、认证、错误处理
└── README.md
```

后续也可以改成根 workspace，但第一阶段建议和 `backend/` workspace 解耦，避免影响后端构建。

## 3. 核心能力

### 3.1 认证与配置

```bash
evolith auth login
evolith auth logout
evolith auth status
evolith config set api-url http://localhost:8080/api/v1
evolith config current
```

配置位置：

```text
~/.config/evolith/config.toml
~/.config/evolith/credentials.toml
```

凭证要求：

- 默认使用 API Key，不保存用户密码。
- token / api key 文件权限应限制为当前用户可读写。

### 3.2 Skill 管理

```bash
evolith skill list
evolith skill pull <name>
evolith skill push ./path/to/skill
evolith skill validate ./path/to/skill
evolith skill run <name> --input input.json
```

目标：

- 校验 `SKILL.md` frontmatter。
- 打包 skill 目录。
- 发布到 Evolith 服务端。
- 从服务端拉取到本地工作区。

### 3.3 Tool 管理

```bash
evolith tool list
evolith tool register tool.yaml
evolith tool call <name> --json args.json
```

目标：

- 支持本地 YAML/JSON 定义工具。
- 调用远程 MCP tool。
- 输出结构化 JSON，便于脚本消费。

### 3.4 Snippet 管理

```bash
evolith snippet list
evolith snippet push ./snippet.md
evolith snippet ref <id> --format direct
```

目标：

- 管理本地代码片段。
- 快速生成 LLM 引用文本。

### 3.5 本地 Agent 工作区

```bash
evolith agent init
evolith agent link-skill <name>
evolith agent sync
```

远期目标：

- 管理本地 `.evolith/` 工作区。
- 同步远程 skill/tool/snippet 到本地 agent 项目。
- 为 OpenCode、Claude Code、Codex 等工具生成适配配置。

## 4. 技术选型

| 领域 | 建议 |
|------|------|
| CLI parser | `clap` |
| HTTP client | `reqwest` |
| 配置 | `serde` + `toml` |
| 输出 | `serde_json` + table 输出 |
| 进度条 | `indicatif` |
| 错误 | `thiserror` / `anyhow` |
| 测试 | 单元测试 + golden output |

## 5. 分阶段计划

### CLI Phase 0 — Scaffold

- 新增 `cli/` 子项目。
- 实现 `evolith --version`、`config`、`auth status`。
- 建立 release 构建脚本。

### CLI Phase 1 — Remote API Client

- 实现登录/API key 配置。
- 实现 tool/skill/snippet list。
- 支持 JSON 输出。

### CLI Phase 2 — Skill / Snippet 发布

- 实现本地 skill/snippet 校验。
- 实现 push/pull。
- 和服务端格式 parser 对齐。

### CLI Phase 3 — Agent Workspace

- 实现 `.evolith/` 工作区。
- 生成不同 agent 工具的配置。
- 支持同步和本地运行。

## 6. 暂不做

- 不在第一阶段实现 TUI。
- 不在第一阶段直接控制后端部署。
- 不保存用户密码。
- 不把 CLI 放进 backend workspace，避免耦合后端 release。
