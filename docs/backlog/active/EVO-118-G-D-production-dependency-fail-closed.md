# EVO-118-G-D 生产关键依赖 Fail Closed

- **类型**：Technical / Security / Reliability
- **状态**：Done / Complete
- **当前迭代**：Iteration 059
- **优先级**：P1
- **父 Epic**：[EVO-118-G](EVO-118-G-runtime-reliability-gates.md)
- **依赖**：EVO-118-G-C
- **影响范围**：backend / config / mailer / cache / tests / docs

## 工程目标

按职责区分可降级缓存与安全状态：生产 SMTP 声明启用但初始化失败时阻止启动；开发
ConsoleMailer 不记录完整 token/链接；Redis 普通缓存可显式降级，但 Session、rate limit、
锁等安全状态不得伪装为进程内分布式能力。

## 验收场景

- **Given** production 配置启用 SMTP 但 host/from 等配置无效
- **When** 服务初始化
- **Then** 返回配置错误并停止启动，不切换 ConsoleMailer

- **Given** development 使用 ConsoleMailer
- **When** 生成验证、重置或邀请邮件
- **Then** 日志不包含完整 token 或可直接使用的敏感链接

- **Given** Redis 不可用
- **When** 普通缓存与安全状态分别初始化
- **Then** 普通缓存按显式策略降级；安全状态功能 fail closed 或启动失败

## 不做事项

- 不实现 Durable Outbox；归 EVO-118-H。
- 不建设完整多区域 Redis/SRE 平台。

## 验证证据要求

- production/development 配置矩阵与启动失败测试。
- ConsoleMailer 日志脱敏测试。
- Redis 普通缓存降级与安全状态 fail-closed 测试。

## 残余工作归口

- Durable event 和最终 production deployment smoke 分别归 EVO-118-H/E。

## 实际验证与残余

- Production SMTP 初始化错误返回配置错误；development 明确回退 ConsoleMailer。
- ConsoleMailer verification/reset/invitation 日志均不包含 token 或完整链接。
- Production Redis 连接失败返回配置错误；development 普通 cache 可回退内存。
- SMTP/Redis/limiter 专项测试、workspace check、strict Clippy 通过；文档链接检查通过。

Durable event 与最终 production deployment smoke 分别归 H/E。闭环状态：`Complete`。

## 威胁模型

| 项目 | 内容 |
| --- | --- |
| 受保护资产 | 邮件 token、生产安全状态、跨实例配额一致性 |
| 调用者 | 生产配置管理员、外部 Redis/SMTP、开发日志读取者 |
| 入口 | 启动配置、mailer、cache、rate-limit middleware |
| 失败模式 | SMTP/Redis 静默降级、完整 token 写日志、内存状态伪装分布式能力 |
| 安全默认 | 生产初始化失败即停止；开发普通缓存可显式降级；安全 limiter 缺失不放行 |
| 验证证据 | production/development 配置矩阵、日志字段审查、负向测试 |
