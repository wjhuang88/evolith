# EVO-127-C — Tenant admin/API keys/billing/audit GraphQL migration

- **父 Epic**：[EVO-127](EVO-127-graphql-application-api-migration.md)
- **类型**：Product/API Security Story
- **优先级**：P0
- **状态**：Proposed
- **GitHub Issue**：[#29](https://github.com/wjhuang88/evolith/issues/29)
- **依赖或阻塞**：EVO-127-A/B
- **影响范围**：backend / frontend / security / docs

## 目标

迁移 Members、API Keys、Billing 与 Audit application APIs 到 GraphQL，并让 Settings/Admin UI 使用 generated operations。

## Acceptance

- member/admin positive + negative permission matrix 通过。
- API key secret 仍只在创建时返回一次。
- pagination/filter 与当前实现保持语义一致。
- Settings/Admin 对这些域的 browser REST calls 为零。

## 不做

Payment provider webhook/callback 保持 protocol-native；repo/capability APIs 归其他 Story。
