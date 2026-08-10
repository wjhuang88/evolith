# EVO-123 本地启动端口检测只识别监听进程

- **类型**：bug / script
- **状态**：Proposed
- **优先级**：P2
- **影响范围**：`scripts/dev.sh` / local development docs / script release notes

## 技术目标

`scripts/dev.sh` 的端口占用检查只把本机监听进程视为冲突，避免任意到远端同号端口的
已建立连接阻断 `lite`、`start`、`backend` 或 `frontend` 启动。

## 已确认现状

- 2026-08-08 执行 Iteration 061 本地验收时，`lsof -ti :8080` 返回了一个
  `local-ephemeral -> remote:8080 (ESTABLISHED)` 连接的 PID。
- `lsof -nP -iTCP:8080 -sTCP:LISTEN` 同时证明本机没有监听 8080。
- `./scripts/dev.sh lite` 因此报告 `Port 8080 is already in use` 并拒绝启动。

## 不做事项

- 不终止或接管任何非 Evolith 进程。
- 不改变默认 backend/frontend 端口。
- 不在 EVO-112-B 的产品 UI Story 中顺带修改启动脚本行为。

## 技术验收

- [ ] 到远端 `:8080` 的 established 连接不会阻止本机 backend 监听 8080。
- [ ] 本机已有 TCP LISTEN 时仍拒绝启动，并报告监听 PID。
- [ ] backend 与 frontend 的端口检查使用同一监听语义。
- [ ] `bash -n scripts/dev.sh` 通过。
- [ ] 脚本行为变化同步 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。

## 依赖与最小验证

- 依赖：无；当前不阻塞 Iteration 061，后者可用显式高位端口继续验证。
- 最小验证：构造 remote-established / local-listen 两类 socket，执行端口检查与
  `scripts/dev.sh lite` 冒烟；运行 `git diff --check`。

## 残余归口

- 进入后续独立微迭代实施；完成后同步 Product Backlog、Iteration、Release Notes。
