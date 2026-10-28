# SOP: API 契约优先开发

## 触发条件

- 新增或修改 API。
- 前端和后端字段不一致。
- 需要拆分一个 backlog story 的 API 边界。

## 契约文件

Evolith 当前按 transport 类型维护契约：

- Application/business GraphQL：
  - [GraphQL Application API Contract](../reference/GRAPHQL-CONTRACT.md) 维护 transport/security/error/codegen 规则；
  - [Canonical SDL](../reference/graphql/schema.graphql) 是 executable schema contract；
  - `frontend/src/graphql/operations/` 是 authored operations；
  - `frontend/src/graphql/generated/` 是 Codegen 产物，不得手改。
- REST compatibility / protocol contract：[API-CONTRACT.md](../reference/API-CONTRACT.md) 继续描述尚未迁移的 `/api/v1/*` 与协议型 HTTP 合约。
- Git Smart HTTP、MCP、health/readiness、callbacks 等 protocol-native surface 不因 GraphQL 迁移而改成 GraphQL。

契约至少必须覆盖：

- endpoint / operation 或 protocol path；
- 输入与输出结构；
- 稳定错误 code / extensions；
- authentication / authorization；
- CSRF、rate limit 与资源上限；
- generated artifacts / version history（适用时）。

## 标准流程

### GraphQL application API

```text
1. 更新 Rust schema/resolver contract
   ↓
2. 导出并审查 canonical SDL
   ↓
3. 更新 authored GraphQL operation
   ↓
4. 运行 GraphQL Codegen
   ↓
5. 更新 transport/cache integration
   ↓
6. 添加或更新安全/行为测试
   ↓
7. CI 验证 SDL + generated-client 无漂移
```

### REST / protocol-native API

```text
1. 更新 API 契约
   ↓
2. 更新前端 TypeScript 类型/API client（如有）
   ↓
3. 更新后端 DTO/handler/route
   ↓
4. 添加或更新测试
   ↓
5. 集成验证
```

## 约束

- 前端不得调用契约中不存在的 path/operation。
- GraphQL resolver 不得自行解析 JWT/API Key；必须复用现有 request principal / authorization boundary。
- GraphQL generated client 只能由 Codegen 产生；手工修改 generated 文件视为 contract drift。
- GraphQL schema 变化必须同步 committed SDL，并通过 CI drift gate。
- 后端 501 接口必须在对应契约中标注。
- 破坏性字段变更需要更新契约说明；GraphQL 删除/重命名字段必须先确认 consumer migration。
- API client 和 DTO/operation 字段名必须一致，除非有明确 adapter。

## 验收

- [ ] 对应 transport 的契约已更新。
- [ ] GraphQL 变更已同步 canonical SDL；REST/protocol 变更已同步 API contract。
- [ ] 前端 operation/type/client 已更新；GraphQL generated artifacts 来自 Codegen。
- [ ] 后端 schema/resolver 或 DTO/handler 已更新。
- [ ] Authentication / authorization / CSRF / rate-limit / resource-boundary 影响已审查。
- [ ] 如果接口是公开状态变更接口，RBAC 与 CSRF 例外已同步，并有测试覆盖。
- [ ] 如果接口通过邮件或外部链接进入，前端公开路由和 `APP__PUBLIC_URL` 已同步验证。
- [ ] 至少有局部测试或集成验证记录。
- [ ] GraphQL SDL/codegen drift gate 或对应 contract check 通过。
- [ ] 合约、实现、验证及残余状态已按 [任务收口与完成声明](TASK-CLOSURE.md) 同步。

## 相关文档

- [GraphQL Application API Contract](../reference/GRAPHQL-CONTRACT.md)
- [API Contract](../reference/API-CONTRACT.md)
- [ADR-0012 GraphQL Application API](../decisions/ADR-0012-graphql-application-api.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
