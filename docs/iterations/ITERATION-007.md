# Iteration 007: MCP 工具执行质量修复与流程防呆

> 时间：2026-05-25
> 目标：修复 Iteration 006 质量审查发现的运行、鉴权、错误映射和验收证据问题。

## 1. 本轮目标

- 修复 `HttpToolExecutor` 初始化 panic，恢复应用和测试可运行性。
- 收紧 `tools/call` 的 API Key 边界，防止匿名触发真实外部动作。
- 修复 HTTP 错误到 MCP error 的映射。
- 用本地可控服务补齐真实执行测试。
- 修复全量门禁暴露的日志脱敏无限循环。
- 将误验收根因写回工程流程。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-032 | Iteration 006 MCP 执行质量修复与流程防呆 | P0 | Agent | Done |

## 3. 不做事项

- 不扩展 Function 类型工具执行能力。
- 不重建 GitHub CI/CD workflow（EVO-030）。
- 不引入新的出站代理配置方案。

## 4. 验收标准

- [x] HTTP tool executor 初始化不因系统代理读取而 panic。
- [x] 匿名 `tools/call` 不可执行真实工具。
- [x] 认证后的 HTTP 工具调用成功路径有本地集成测试。
- [x] HTTP 错误状态和超时路径有本地集成测试且返回 MCP error。
- [x] 日志脱敏回归测试通过，不再使全量测试挂起。
- [x] 后端检查与测试结果如实记录。
- [x] SOP 与经验记录能降低同类误验收再次发生的概率。

## 5. 验证计划

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| 禁止匿名执行影响既有集成 | 与 API contract 对齐；公开发现仍可保留，真实执行要求 API Key |
| 禁用隐式系统代理影响需代理环境 | 当前优先保证安全、可启动和可验证；显式代理支持另行设计 |
| 全量格式检查存在历史债 | 不扩大到无关格式化；如实记录阻塞并单独治理 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-25 | Quality review found Iteration 006 marked Done while `cargo fmt`, `cargo clippy` and `cargo test --workspace` fail; inserted EVO-032 as urgent fix. |
| 2026-05-25 | Full test verification exposed an existing infinite loop in `common::sanitize_log_string`; included the narrowly scoped security utility fix because it blocks truthful workspace validation. |
| 2026-05-25 | Fixed executor initialization, execution authorization, HTTP error/size handling and sanitizer loop; local MCP integration suite passed with 7 tests. |
| 2026-05-25 | Validation: `cargo check --workspace` passed; `cargo clippy --workspace -- -D warnings` passed; `cargo test --workspace` passed. |
| 2026-05-25 | Validation: `cargo fmt --all -- --check` failed on historical untouched `service-auth`, `service-payment`, and `service-tool` formatting/config baseline; recorded as EVO-033 rather than reporting success. |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-25 | urgent-fix | Switch to EVO-032 | 暂停基于 EVO-005 Done 状态推进后续故事，先修复运行与验收门禁 | 保留 HTTP executor 主体，修正初始化、鉴权、错误映射和测试 |

## 9. Review

Navigator check: no blocking findings. The prior runtime panic, anonymous execution path,
incorrect HTTP error success result, unstable external test dependency, and sanitizer hang are
covered by code changes and repeatable tests. Residual risk: the workspace-wide rustfmt baseline
remains degraded under EVO-033, and controlled proxy support is intentionally not implemented.

## 10. Retrospective

- 完成：EVO-032 修复并验证完成，EVO-005 的 HTTP tool execution 基线恢复可信。
- 未完成：全量 rustfmt 基线清理，已拆为 EVO-033；显式出站代理支持不在本轮范围。
- 流程调整：开始迭代必须先于实现提交；验收勾选必须逐命令对应；出站执行边界必须经过 Navigator 检查并使用本地可控测试。
- 写入 EVOLUTION：已记录 Iteration 006 误验收的症状、根因、修复与可复用门禁。
