# CLI 友好接口格式规范

> 本格式替代旧 snippet 主线，目标是让大模型和本地 CLI 都能稳定发现、理解和调用企业内部能力。

## 设计目标

- 面向 LLM：提供清晰 usage、examples、error model 和参数语义。
- 面向 CLI：描述稳定命令、子命令、参数 schema 和 JSON-friendly 输入输出。
- 面向企业治理：保留 tags、visibility、owner、version 等元数据，便于权限、审计和同步。
- 面向后续 Rust CLI：同一份描述可被前端管理，也可被 CLI push / pull / sync。

## 文件结构

CLI Interface 使用 YAML frontmatter + Markdown body：

```markdown
---
name: deploy-service
version: 1.0.0
summary: Deploy a service to a target environment.
command: evolith deploy
subcommands:
  - service
tags:
  - deploy
  - platform
visibility: tenant
inputs:
  - name: service
    type: string
    required: true
    description: Service name.
  - name: env
    type: enum
    required: true
    values: [dev, staging, prod]
    description: Target environment.
output:
  type: object
  description: Deployment result.
examples:
  - title: Deploy to staging
    command: evolith deploy service --service api --env staging
    input:
      service: api
      env: staging
    output:
      deployment_id: dep_123
      status: queued
error_model:
  - code: SERVICE_NOT_FOUND
    message: Service does not exist or is not visible to the caller.
    retryable: false
---

## Usage

Use this interface when an agent needs to deploy an existing service.

## Notes

- Production deployments may require admin approval.
- Prefer JSON output when called by another agent.
```

## Frontmatter 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 稳定接口名，建议 kebab-case |
| `version` | string | 是 | 接口描述版本 |
| `summary` | string | 是 | 一句话说明 |
| `command` | string | 是 | 主命令或命令前缀 |
| `subcommands` | string[] | 否 | 子命令路径 |
| `tags` | string[] | 否 | 检索和治理标签 |
| `visibility` | string | 否 | `private` / `tenant` / `public`，默认 `private` |
| `inputs` | Parameter[] | 否 | 参数 schema |
| `output` | Output | 否 | 输出描述 |
| `examples` | Example[] | 否 | LLM 和 CLI 使用示例 |
| `error_model` | Error[] | 否 | 稳定错误语义 |

## Parameter

```yaml
name: env
type: enum
required: true
description: Target environment.
values: [dev, staging, prod]
default: dev
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 参数名 |
| `type` | string | 是 | `string` / `number` / `boolean` / `enum` / `object` / `array` |
| `required` | boolean | 否 | 默认 `false` |
| `description` | string | 否 | 面向人和 LLM 的说明 |
| `values` | string[] | 否 | enum 可选值 |
| `default` | any | 否 | 默认值 |

## Example

```yaml
title: Deploy to staging
command: evolith deploy service --service api --env staging
input:
  service: api
  env: staging
output:
  deployment_id: dep_123
  status: queued
```

## Error Model

```yaml
code: SERVICE_NOT_FOUND
message: Service does not exist or is not visible to the caller.
retryable: false
```

## 与旧 Snippet 的关系

| 旧 Snippet 字段 | CLI Interface 字段 | 迁移策略 |
|-----------------|---------------------|----------|
| `name` / `title` | `name` | 保留语义，稳定为接口名 |
| `content` | Markdown body | 保留为 usage / notes |
| `code` | `command` / `examples.command` | 不再作为“可插入代码”，改为可调用命令示例 |
| `language` | `inputs` / `output` / tags | 不再作为主分类；必要时放入 tags |
| `framework` | tags | 迁移为标签 |
| `dependencies` | Markdown body 或后续 runtime 字段 | 本轮不强制迁移 |

当前实现仍保留 `/api/v1/snippets` 兼容路径。新功能应使用 `CliInterface` 术语和本文格式；实际 API 路由重命名在后续迁移故事中处理。
