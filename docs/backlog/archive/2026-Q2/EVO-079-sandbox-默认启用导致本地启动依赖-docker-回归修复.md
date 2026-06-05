# EVO-079 sandbox 默认启用导致本地启动依赖 Docker 回归修复

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: bug
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-06-05 / Iteration 041 follow-up
- Decision Context: 将 `sandbox.enabled`、`.env.development`、`.env.example` 默认改为 false；显式启用 sandbox 时仍保留 Docker fail-fast

#### Source Detail Snapshot

- 类型：bug
- 优先级：P1
- 状态：Done
- 父 Epic：无
- Story 形态：Technical
- 用户价值或技术目标：
  - 为了：恢复 lite/local 开发启动不依赖 Docker 的既有约束。
  - 维护者需要：默认配置不启用 sandbox；只有显式 `SANDBOX__ENABLED=true` 时才要求 Docker executor 可初始化。
  - 以便：`cargo run` / `./scripts/dev.sh lite` 能在无 Docker 环境启动，同时不回退到 sandbox 伪成功。
- 范围：
  - `AppConfig` 的 `sandbox.enabled` 默认值改为 `false`。
  - `.env.development` / `.env.example` 的 `SANDBOX__ENABLED` 改为 `false`。
  - 同步 AGENTS、CONFIG、EVOLUTION、Iteration 041 和 Board 的状态口径。
- 不做：
  - 不恢复 Docker 初始化失败时的 fallback。
  - 不改容器池实现。
  - 不改 production compose；其默认已经是 `SANDBOX_ENABLED:-false`。
- 验收标准：
  - [x] 不设置 `SANDBOX__ENABLED` 时，配置默认 sandbox 关闭。
  - [x] `.env.development` / `.env.example` 默认 sandbox 关闭。
  - [x] `SANDBOX__ENABLED=true` 仍保留 Docker 初始化 fail-fast。
  - [x] `cargo check --workspace`、配置测试、治理校验和 `git diff --check` 通过。
- 技术备注：
  - 本修复是 Iteration 041 的 follow-up regression fix；根因是 fail-fast 行为正确，但默认值没有从“执行能力默认启用”调整成“开发启动默认不依赖 Docker”。
- 依赖或阻塞：无。
- 解锁内容：恢复本地开发体验，同时保留显式 sandbox 的失败可见性。
- 影响范围：backend config / env / docs。
- 最小验证方式：`cargo test -p infra config`；`cargo check --workspace`；文档治理 validator。
