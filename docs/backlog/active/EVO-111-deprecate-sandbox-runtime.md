# EVO-111 废弃 Skill 沙箱执行

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- [ADR-0005 Deprecate Sandbox Runtime](../../decisions/ADR-0005-deprecate-sandbox-runtime.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)

## Summary

- 类型：tech-debt / refactor
- 优先级：P0
- 状态：Proposed
- 父 Epic: EVO-100

## Problem Or Outcome

按 ADR-0005 删除 Skill 沙箱执行层（Docker + bollard + sandbox 镜像 + ExecutionProvider 中 Skill 分支），仅保留 HttpProxy FaaS 形态。完成后 `cargo test --workspace` 全绿，启动时间缩短，运维负担降低。

## Goal And Non-Goals

- **Goal**：
  1. 删除 `service-skill/src/{executor,docker_executor,docker_sandbox_provider}.rs`
  2. 删除 `backend/sandbox/` 镜像目录与 Dockerfiles
  3. 从 `service-skill/Cargo.toml` 移除 `bollard` 依赖
  4. 从 `infra/config.rs` 移除 `SandboxConfig`；从环境变量移除 `SANDBOX__*` 字段
  5. 从 `common/execution.rs` 移除 `ExecutionCaller::Skill` 变体；`ExecutionProvider` 简化为仅 HttpProxy（或直接内联到 mcp handler）
  6. 从 `AppState` 移除 `skill_executor` 字段
  7. 删除 `POST /api/v1/skills/{id}/execute` route + handler
  8. 删除前端 `skillsApi.execute()` 与 "执行" 按钮 UI
  9. Sandbox 相关集成测试用 `#[ignore = "sandbox removed per ADR-0005"]` 标注或删除
- **Non-goals**：
  - 不重新引入 sandbox（除非 EVO-080 Wasmer/WASI spike 结论明确支持）
  - 不保留 sandbox 作为 feature flag
  - 不为已删除 sandbox 创建 deprecation shim（避免误导）
  - 不实现 CLI 的本地执行（CLI 走 FaaS HttpProxy 形态）

## Dependencies And Blockers

- 无硬依赖；可作为 Phase 4 收尾执行
- 软依赖 EVO-110（旧 execute API 在 EVO-110 期间仍可能被引用；EV0-111 收口前必须完成 EV0-110）

## Governing ADRs, Specs Or Decisions

- [ADR-0005 Deprecate Sandbox Runtime](../../decisions/ADR-0005-deprecate-sandbox-runtime.md)
- [SERVERLESS-RUNTIME.md](../../proposals/SERVERLESS-RUNTIME.md)（历史参考，简化后保留）

## Acceptance Criteria

- [ ] 上述 9 项删除动作全部完成
- [ ] `grep -r "DockerExecutor\|SkillExecutor\|docker_sandbox\|RuntimePool\|bollard\|SandboxConfig\|SANDBOX__" backend/` 返回空（除 EVOLUTION.md 等历史文档外）
- [ ] `cargo test --workspace` 全绿
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` 0 errors
- [ ] `cargo build --release` 编译时间改善（与 EVO-086 依赖迁移前对比）
- [ ] 启动时间改善：本地 lite 启动 < 3s（EVO-079 修复后 < 5s，进一步缩短）
- [ ] 前端 `bun run build` 0 errors；bundle 中无 sandbox 相关依赖
- [ ] EVOLUTION.md 记录清理动作与影响
- [ ] Git commit 标 `refactor(scope): ...` + `[model: <name>]`（AGENTS.md Git Rules）

## Validation Evidence Required

- 完整 `cargo test --workspace` 输出（粘贴到 iteration 文档）
- 完整 `cargo clippy` 输出
- 编译时间对比（before / after）
- 启动时间对比
- grep 验证 sandbox 相关符号已清除

## Residual Work Destination

- EVO-080 Wasmer/WASI spike 如果结论支持非 Docker runtime，可重新设计 ExecutionProvider（独立 EVO）
- 旧 `ExecutionProvider` trait 完全简化或废弃（独立 EVO）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 详见 ADR-0005
- 丢弃成本：Phase 7 投入的 ~500 行 service-skill 执行代码 + Docker sandbox 镜像构建
- 收益：体积 / 启动 / 编译时间 / 运维负担全面改善；与新方向一致
