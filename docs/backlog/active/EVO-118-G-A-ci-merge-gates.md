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

### 2026-09-12 CI runner disk headroom follow-up

- PR #21 的 `ci` run `34500683820` 已通过 whitespace、Markdown、Frontend、DATA-01、Backend fmt/check/clippy，随后在 `Backend test (SQLite)` 链接阶段失败。
- 失败日志明确为 GitHub hosted runner `No space left on device`；没有测试断言失败证据，独立 `data-durability-container` run 同一 head 已成功。
- 根因边界：单 job 恢复 Rust `target` cache 后连续执行 check/clippy/test，测试链接阶段与既有编译产物叠加形成磁盘峰值。
- 修复保持所有质量门禁：在 workspace test 前执行 `cargo clean` 回收 check/clippy 编译产物，并将 CI test profile debug info 设为 `0` 降低链接产物体积；不跳过、不条件化、不缩小 `cargo test --workspace`。
- 新 CI run 必须证明 SQLite workspace test、application recovery drill 和后续必需步骤真实通过；若再次出现容量失败则继续按 CI 基础设施缺陷处理，不把重跑成功冒充根因闭环。

闭环状态：`Partial`。待 CI disk-headroom follow-up 的真实 PR run 通过，并在具备仓库管理员权限后补查 Branch Protection；结果继续记录到 Iteration 056。
