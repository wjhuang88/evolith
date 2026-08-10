# EVO-118-G-C Caller-aware 分层限流

- **类型**：API / Security / Reliability
- **状态**：Done / Complete
- **当前迭代**：Iteration 058
- **优先级**：P1
- **父 Epic**：[EVO-118-G](EVO-118-G-runtime-reliability-gates.md)
- **依赖**：EVO-118-G-B
- **影响范围**：backend / middleware / auth / API Key / config / tests / docs

## 工程目标

让匿名、JWT 和 API Key 请求按稳定调用者身份分别计数，并让单 Key `rate_limit` 真正进入
请求链；共享 IP 下不同已认证调用者不得错误互相消耗配额。

## 验收场景

- **Given** 同一 IP 上有匿名、JWT 用户和两个 API Key
- **When** 其中一个调用者超过自身配额
- **Then** 只有该调用者返回 429，其他调用者不受其计数影响

- **Given** API Key 配置低于全局 API Key 上限的单 Key `rate_limit`
- **When** 超过该 Key 配额
- **Then** 返回 429，不能退回共享 IP 或仅使用全局默认值

## 不做事项

- 不把进程内 limiter 描述为多实例全局配额。
- 不在本 Story 引入通用分布式锁或 Session。

## 验证证据要求

- 匿名/JWT/API Key/不同 Key 集成矩阵。
- 429、恢复窗口和身份隔离测试。
- 配置与 API Contract 同步。

## 残余工作归口

- 多实例 Redis-backed 安全状态要求归 G-D；若当前架构无法安全满足，G-C 保持 Review 而非假完成。

## 威胁模型

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | API 可用性、租户调用配额、API Key 配额 |
| 调用者 | 匿名请求、JWT 用户、API Key、共享出口 IP 上的不同调用者 |
| 入口 | 全局 HTTP middleware |
| 信任边界 | 网络连接 -> RBAC 身份解析 -> caller-aware limiter -> handler |
| 失败模式 | 共享 IP 相互耗尽配额、单 Key 限额被忽略、未知身份绕过 |
| 安全默认 | 未认证按 peer IP；已认证按稳定 ID；单 Key 使用自身与全局上限的较小值 |
| 验证证据 | 匿名/JWT/API Key 隔离矩阵、429 与窗口恢复测试 |

## 实际验证与残余

- caller-aware middleware 已接入主服务：匿名按 peer IP、JWT 按 user ID、API Key 按 key ID。
- API Key 同时执行全局每分钟上限和持久化单 Key 每小时 `rate_limit`，两者取决于最严格者。
- 专项测试 5/5、API Key DTO 测试 5/5、workspace check、strict Clippy 和 Git clone/push/pull
  集成测试通过。
- API Contract、配置参考和 429 响应已同步；单进程共享计数器边界已明确。

多实例 Redis-backed 安全状态和 fail-closed 仍归 G-D。闭环状态：`Complete`。
