# SOP: API 契约优先开发

## 触发条件

- 新增或修改 API。
- 前端和后端字段不一致。
- 需要拆分一个 backlog story 的 API 边界。

## 契约文件

API 契约维护在 [API-CONTRACT.md](../reference/API-CONTRACT.md)，包含：

- 端点路径
- 请求参数
- 响应结构
- 错误码
- 认证方式
- 公开接口的 RBAC public path 和 CSRF exempt path
- 版本历史

## 标准流程

```text
1. 更新 API 契约
   ↓
2. 更新前端 TypeScript 类型/API client
   ↓
3. 更新后端 DTO/handler/route
   ↓
4. 添加或更新测试
   ↓
5. 集成验证
```

## 约束

- 前端不得调用契约中不存在的路径。
- 后端 501 接口必须在契约中标注。
- 破坏性字段变更需要更新契约版本历史。
- API client 和 DTO 字段名必须一致，除非有明确 adapter。

## 验收

- [ ] API 契约已更新。
- [ ] 前端类型或 API client 已更新。
- [ ] 后端 DTO/handler 已更新。
- [ ] 如果接口是公开状态变更接口，RBAC 与 CSRF 例外已同步，并有测试覆盖。
- [ ] 如果接口通过邮件或外部链接进入，前端公开路由和 `APP__PUBLIC_URL` 已同步验证。
- [ ] 至少有局部测试或手工验证记录。
- [ ] 合约、实现、验证及残余状态已按 [任务收口与完成声明](TASK-CLOSURE.md) 同步。

## 相关文档

- [任务收口与完成声明](TASK-CLOSURE.md)
