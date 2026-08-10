# EVO-118-G-B Runtime Readiness 真实反映依赖

- **类型**：Technical / Reliability
- **状态**：Done / Complete
- **优先级**：P1
- **父 Epic**：[EVO-118-G](EVO-118-G-runtime-reliability-gates.md)
- **依赖**：EVO-118-G-A（完成后启动）
- **影响范围**：backend / Docker / Compose / config / tests / docs

## 工程目标

证明 liveness 只反映进程存活，readiness 在数据库或 Git Storage 不可用时返回 503，且
Docker/Compose 使用 readiness 而非永远成功的探针。

## 不做事项

- 不加入尚不存在的生产依赖检查。
- 不执行最终生产部署 Smoke；归 EVO-118-E。

## 技术验收

- [ ] `/health/live` 与 `/health/ready` 语义、路由和文档一致。
- [ ] DB/Git Storage 故障返回 503，恢复后返回 200，响应包含逐项结果。
- [ ] Dockerfile 与 production Compose healthcheck 使用 `/health/ready`。
- [ ] 运行态 smoke 证明健康与故障路径，不只依赖 unit helper。

## 验证证据要求

- Health handler unit/integration tests。
- Development smoke + dependency failure smoke。
- Docker/Compose config 静态检查。

## 残余工作归口

- 最终 production container build/startup 归 EVO-118-E。

## 实际验证与残余

- `/health/live` 与 `/health/ready` 路由已分离；liveness 固定返回 200，readiness 检查
  database 与 Git Storage 并在任一失败时返回 503。
- readiness handler 已有数据库失败、Git Storage 失败、恢复成功、读回不一致、写入失败
  和清理失败测试。
- `backend/Dockerfile` 与 `docker-compose.prod.yml` backend healthcheck 均使用
  `/health/ready`。
- 当前验证：`cargo test -p api handlers::health` 及 workspace 全量测试通过；Compose/Docker
  配置静态检查通过。

运行态 smoke 已证明：默认缺失 Git Storage 时 `/health/live` 为 200、`/health/ready`
为 503 且 `database=true/git_storage=false`；设置可写 Git Storage 后 readiness 恢复为
200 且两项均为 true。临时后端已正常停止。

最终生产镜像构建、启动和发布 Smoke 明确留给 EVO-118-E。闭环状态：`Complete`。
