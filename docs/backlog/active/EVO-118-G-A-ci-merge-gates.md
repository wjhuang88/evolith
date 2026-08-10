# EVO-118-G-A PR/Main 自动质量门禁

- **类型**：Technical / CI
- **状态**：Review / Partial
- **优先级**：P1
- **父 Epic**：[EVO-118-G](EVO-118-G-runtime-reliability-gates.md)
- **依赖**：EVO-118-B（Done）
- **当前迭代**：Iteration 056
- **影响范围**：`.github/workflows` / frontend / backend / docs

## 工程目标

让 pull request 和 main push 都自动执行当前仓库可复现的前后端质量门禁，防止只有 Tag
或人工运行才发现格式、类型、构建、lint、Clippy 或测试失败。

## 已确认现状

- `pull_request -> main` 已存在，但 `push -> main` 缺失。
- Frontend lint 仅在 release/manual 运行，未进入 PR hard gate。
- Backend fmt/check/clippy/workspace test 与 Frontend install/type-check/build 已在主 CI job。
- GitHub Branch Protection 的 required check 属于远端仓库状态，必须记录并查询真实证据；工作流文件本身不能证明规则已启用。

## 不做事项

- 不在本 Story 执行最终 production Compose clean build 或发布 Smoke；归 EVO-118-E。
- 不改 readiness、限流、SMTP 或 Redis 运行逻辑；分别归 G-B/C/D。
- 不重构现有 DATA-01 容器与恢复矩阵。

## 技术验收

- [ ] CI 同时监听 `pull_request` to main、`push` main 和 semver Tag。
- [ ] PR/main 都执行 Frontend type-check/build/lint 与 Backend fmt/check/clippy/workspace test。
- [ ] workflow 保留 exact-head、Markdown、DATA-01 恢复矩阵，不因接线而削弱已有门禁。
- [ ] Frontend lint、type-check、build 和 Backend workspace 门禁在当前工作树实际通过。
- [ ] 远端 main Branch Protection required check 已查询并记录；若权限或远端规则缺失，必须登记残余而不能声称已阻止合并。

## 验证证据要求

- `bun run type-check && bun run build && bun run lint`
- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- workflow 结构检查及 `gh api` Branch Protection 查询

## 残余工作归口

- 真实 PR/main workflow run 证据与 required check 配置记录在 Iteration 056；远端权限不足时保留为 Review residual。
- Runtime readiness、caller-aware rate limit 与 dependency fail-closed 归 G-B/C/D。

## 实际验证与残余

- Frontend `bun run lint`、`bun run type-check`、`bun run build` 通过。
- Backend `cargo fmt --all -- --check`、`cargo check --workspace --all-targets`、
  `cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace` 通过。
- Workflow YAML 静态解析通过，PR、main push、semver Tag 和 manual trigger 均已声明，
  DATA-01 恢复矩阵仍在。
- `gh api .../branches/main/protection` 返回 HTTP 403（仓库权限/计划限制），无法证明远端
  required checks 已配置；该项保留为外部治理 residual。

闭环状态：`Partial`。待具备仓库管理员权限后补查 Branch Protection，并在 Iteration 056
记录结果。
