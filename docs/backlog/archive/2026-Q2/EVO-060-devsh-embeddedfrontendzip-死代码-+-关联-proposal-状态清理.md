# EVO-060 dev.sh EMBEDDED_FRONTEND/ZIP 死代码 + 关联 proposal 状态清理

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P2
- Source: 嵌入式模式验证 2026-06-01
- Decision Context: 2026-06-03 完成：删除 4 处死代码（EMBEDDED_FRONTEND / build_frontend_zip / --features embedded-frontend / ZIP 构建逻辑）；新增 build_frontend() 函数；lite/embedded 模式改为单端口（build + backend）；后端端口改为读 SERVER__PORT 环境变量；proposal 状态已晋升；SCRIPTS-RELEASE-NOTES.md 同步

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P2
- 状态：Done
- 来源：嵌入式模式验证 2026-06-01 / Iteration 035 后续
- 用户/工程价值：消除 dev.sh 与 proposal 中指向已废 ZIP 嵌入方案的孤儿代码，避免新成员按过时模式复用。
- 背景（嵌入式模式本地验证时发现）：
  - Iteration 031 改用 `rust-embed-for-web` 的 `#[folder = "../frontend/dist/"]` 直接嵌入目录（`backend/src/frontend.rs:7`），取代了原 ZIP 方案。
  - 但 `scripts/dev.sh` 仍保留 ZIP 方案的全部脚手架：`EMBEDDED_FRONTEND` 环境变量、`build_frontend_zip()` 函数、`embedded` 子命令入口。
  - `docs/proposals/EMBEDDED-FRONTEND.md` 状态仍为 `远期目标`，未反映 Iteration 031 已用替代方案实现。
  - `Cargo.toml` 中**未定义** `embedded-frontend` feature；调用 `cargo build --features embedded-frontend` 会报 unknown feature。
  - 历史记录保留：`docs/iterations/ITERATION-030.md`（Superseded → Iteration 031）、`EVOLUTION.md` ADR-0003 引用。
- 范围（本次做）：
  1. **`scripts/dev.sh` 清理**：
     - 删除 line 10 `EMBEDDED_FRONTEND="${EMBEDDED_FRONTEND:-false}"`。
     - 删除 line 126-140 `build_frontend_zip()` 函数。
     - 删除 line 152-168 `if [ "$EMBEDDED_FRONTEND" = "true" ]; then ...` 分支。
     - 删除 line 365-368 `embedded` 子命令入口。
     - 同步更新 `--help` / usage 列表。
  2. **`docs/proposals/EMBEDDED-FRONTEND.md` 状态更新**：
     - `远期目标` → `已晋升（Iteration 031 改用 rust-embed-for-web 替代 ZIP 方案）`。
     - 移除或注释 `--features embedded-frontend` 引用（line 75、149）。
     - 顶部加改线说明 + 链接到 Iteration 031 / ADR-0003。
  3. **`docs/iterations/ITERATION-030.md` 注释补全**（按 AGENTS.md「已发布 iteration 计划基线保护」原则，保留原计划不动）：
     - 在「已改线」段落中补充：`--features embedded-frontend` 在 Cargo.toml 中未定义，Iteration 031 实际改用 `#[folder]` 无 feature flag；如需重新启用 feature flag 形式参见 ADR-0003。
- 不做：
  - 不改 backend `frontend.rs` 的 `#[folder]` 嵌入实现（工作正常）。
  - 不改 `docs/decisions/ADR-0003-embedded-frontend-rust-embed-for-web.md`（决策已生效）。
  - 不重写 `docs/iterations/ITERATION-030.md` 的计划基线（按 AGENTS.md 规则仅追加说明）。
  - 不删 `EVOLUTION.md` 历史记录。
- 验收标准：
  - [x] `rg "EMBEDDED_FRONTEND|frontend\.zip" scripts/dev.sh` 0 hits。
  - [x] `rg "embedded-frontend" scripts/dev.sh docs/proposals/ backend/Cargo.toml backend/crates/*/Cargo.toml` 0 hits（确认 feature flag 未复活）。
  - [x] `rg "embedded-frontend" docs/iterations/ITERATION-030.md` 仍保留（历史基线），但段落有改线说明。
  - [x] `docs/proposals/EMBEDDED-FRONTEND.md` 顶部状态含「已晋升」+ Iteration 031 链接。
  - [x] `bash -n scripts/dev.sh` 语法检查通过。
  - [x] `scripts/dev.sh lite` 单端口启动成功（SERVER__PORT=8090 验证）。`embedded` 子命令保留为 lite 别名（行为相同）。
  - [x] `scripts/dev.sh` 净减少 ~15 行（删 30 行死代码 + 增 15 行 build_frontend 函数）。
  - [x] `scripts/dev.sh lite|start|stop|status|logs|clean|infra|backend|frontend|embedded` 子命令均可用。
  - [x] `docs/reference/SCRIPTS-RELEASE-NOTES.md` 同步记录 dev.sh 行为变更。
- 依赖或阻塞：无。
- 解锁内容：dev.sh 与 proposal 反映 Iteration 031 终局形态；新成员复用 dev.sh 时不会看到死代码。
- 影响范围：scripts/dev.sh、docs/proposals/EMBEDDED-FRONTEND.md、docs/iterations/ITERATION-030.md、docs/reference/SCRIPTS-RELEASE-NOTES.md
- 最小验证方式：`bash -n scripts/dev.sh`；`rg "EMBEDDED_FRONTEND|frontend\.zip|embedded-frontend" scripts/dev.sh docs/proposals/ backend/` 0 hits；`bash scripts/dev.sh lite` / `bash scripts/dev.sh status` 仍可执行。
