# `.evolith/policy.yaml` 规范

## 概述

`.evolith/policy.yaml` 是 Evolith 仓库级策略文件，定义仓库的协作规则、分支保护路径和 Agent 权限范围。
类似 GitHub Pages 的 `Settings → Pages → Source` 配置，但面向仓库的 AI 协作场景。

每个 Evolith 托管的 git 仓库根目录下可以包含此文件。缺失时，使用 `git_repos` 表中
`auto_merge` / `require_review` 默认字段值。

## 文件位置

- 路径：`.evolith/policy.yaml`（仓库根目录）
- 编码：UTF-8
- 格式：YAML 1.2

## Schema

```yaml
version: 1                       # Schema 版本；缺失时按 v1 处理
default_action: require_review   # 默认策略
protected_paths:                 # 受保护的文件路径（glob 模式）
  - SKILL.md
  - cli/interface.yaml
  - mcp/tool.yaml
agents:                          # Agent 级策略覆盖
  - name: "code-reviewer"        # Agent 名称
    scopes:                      # 权限范围
      - "read"
      - "commit:src/"
    auto_merge: true             # 该 Agent 是否可绕过 review
```

## 字段说明

### `version`（可选）

Schema 版本号。当前仅支持 `1`。缺失时默认按 v1 处理以保证向后兼容。

### `default_action`（必选）

仓库默认策略，当没有 Agent 特定规则或路径特定规则匹配时生效。

| 值 | 含义 |
|----|------|
| `auto_merge` | 允许自动合并（无需人工 review） |
| `require_review` | 需要人工 review 后才能合并 |
| `block` | 禁止合并操作用于关键仓库 |

### `protected_paths`（可选）

文件路径 glob 模式列表。匹配这些模式的文件在 `require_review` 或 `block` 策略下受保护。
Agent 修改这些路径时必须满足对应的 review 要求。

支持的 glob 模式：
- `*` — 匹配单一路径段中的任意字符（不含 `/`）
- `**` — 匹配任意级目录
- `?` — 匹配单一路径段中的单个字符

### `agents`（可选）

Agent 级策略覆盖列表。每个 Agent 可以拥有独立的 scope 和 auto_merge 设置。

#### Agent 字段

| 字段 | 类型 | 必选 | 说明 |
|------|------|------|------|
| `name` | string | 是 | Agent 名称标识 |
| `scopes` | array | 否 | 权限范围列表；默认为 `["read"]` |
| `auto_merge` | boolean | 否 | 是否允许自动合并；覆盖 `default_action` |

#### 权限范围（Scope）

| 值 | 含义 |
|----|------|
| `read` | 只读访问 |
| `commit:<path-glob>` | 允许对匹配路径的文件直接提交（无需 review） |
| `promote` | 允许 promote/merge 操作 |

示例：
```yaml
scopes:
  - "read"
  - "commit:src/"
  - "promote"
```

## 策略优先级

1. Agent 特定 `auto_merge` 设置（最高优先级，仅当 scope 允许时生效）
2. `protected_paths` 匹配（触发 `default_action`）
3. `default_action`（默认策略）
4. `git_repos` 表 `auto_merge` / `require_review` 字段（最低优先级，policy.yaml 缺失时使用）

## 向后兼容策略

- `version` 字段缺失时，解析器按 v1 处理
- 未知字段被忽略（允许未来版本新增字段）
- 空的 `policy.yaml` 等同于缺失，使用 DB 默认值
- 未来版本升级时，`version` 字段必须递增

## 推荐配置

### 通用 skill/CLI/MCP 仓库

```yaml
version: 1
default_action: require_review
protected_paths:
  - SKILL.md
  - cli/interface.yaml
  - mcp/tool.yaml
```

### CI/CD 自动部署仓库

```yaml
version: 1
default_action: block
agents:
  - name: "deploy-bot"
    scopes: ["read", "promote"]
    auto_merge: true
```

### 开放协作仓库

```yaml
version: 1
default_action: auto_merge
```
