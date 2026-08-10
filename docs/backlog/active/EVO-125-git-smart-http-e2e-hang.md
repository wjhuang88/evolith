# EVO-125 Git Smart HTTP E2E 偶发阻塞

- **类型**：Bug / Test Reliability
- **状态**：Done / Complete
- **优先级**：P1
- **依赖或阻塞**：无；不得混入 EVO-118-H-B
- **影响范围**：backend / tests / CI / docs

## 技术目标

消除 `test_git_clone_push_pull_e2e` 在真实 `git clone` 阶段偶发长时间无进展的问题，
让本地与 CI 的 workspace Gate 在有界时间内给出成功或可诊断失败，而不是无限等待。

## 当前证据

- 2026-08-09~10 的 Iteration 067 全量复验中，同一测试先前曾在约 57 秒通过。
- 后续 workspace 运行在 `git clone` 停滞 224.86 秒后人工中止；stderr 仅停在 `Cloning into '.'...`。
- 独立复验最终通过，但测试进程报告耗时 30425.94 秒，证明行为不稳定且缺少有效总超时。
- 初步假设：同步 `std::process::Command::output` 运行在 `#[actix_rt::test]` 中，可能阻塞同一运行时上的测试 HTTP Server；必须用代码与重复测试验证，不能直接当作根因。

## 验收标准

- [x] 先复现并定位阻塞点，区分 runtime starvation、Git subprocess、HTTP handler 与测试清理问题。
- [x] 真实 clone/push/pull 语义保持不变，不以 mock 或跳过测试掩盖问题。
- [x] Git 子进程有 20 秒有界 timeout，失败输出包含阶段信息；测试 Server 生命周期继续由受控 shutdown 管理。
- [x] 聚焦 E2E 连续运行至少 3 次通过，单次耗时 8.8-11.1 秒。
- [x] `cargo test --workspace` 通过，且无遗留 Git/Server 子进程。

## 最小验证

```bash
cd backend
cargo test -p api --test git_smart_http_e2e_tests test_git_clone_push_pull_e2e -- --nocapture
cargo test --workspace
```

## 不做事项

- 不改变 Smart HTTP 权限或 Git 协议契约。
- 不因测试偶发阻塞回退 EVO-115 已完成的真实 git-client E2E。

## 实际验证（2026-08-10）

- 根因：同步 `std::process::Command::output` 在 Actix 测试 runtime 中阻塞，导致最后一次
  Smart HTTP clone 偶发认证失败或长时间无进展；API key 数据库状态与直接鉴权均正常。
- 修复：完整 clone/push/pull 场景改用 `tokio::process::Command`，每个 Git 阶段设 20 秒总超时，
  `kill_on_drop(true)`，超时错误带阶段标识；未改变 Smart HTTP 协议或权限语义。
- `cargo test -p api --test git_smart_http_e2e_tests test_git_clone_push_pull_e2e -- --nocapture`：连续 3/3 通过。
- `cargo test --workspace`：未跳过任何测试并通过；`lsof -c git` 无遗留 Git 子进程，
  TCP listener 清单无测试 Server。

闭环状态：`Complete`。
