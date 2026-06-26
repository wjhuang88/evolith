# ADR-0006: Smart HTTP via `git` CLI subprocess（gix 无服务端）

## 状态

Accepted（2026-06-26）

## 背景

Evolith 作为 git 托管平台需要 Smart HTTP git 协议（`GET /repos/{id}/info/refs`、`POST /repos/{id}/git-upload-pack`、`POST /repos/{id}/git-receive-pack`），让标准 git 客户端可以 `git clone / push / pull`。项目已用 `gix` crate（纯 Rust）做 bare repo 初始化（EVO-103-A）。

调查发现：**已发布的 `gix` crate 没有任何服务端协议能力**：

- 服务端 upload-pack 仅存在于未合并的 [gitoxide PR #2465](https://github.com/GitoxideLabs/gitoxide/pull/2465)（2026-03 开，截至 2026-06 仍未合入）。
- 服务端 receive-pack 无实现计划（gix 维护者 Byron 在 [tracking issue #307](https://github.com/GitoxideLabs/gitoxide/issues/307) 明确表示“如果赶时间，建议 shell out 到 `git`”）。

所有生产级 Rust git 服务器（如 [loom](https://github.com/ghuntley/loom)）目前都用 `git --stateless-rpc` subprocess 实现全部 Smart HTTP 端点。

## 选项

- **A. 全部 3 端点用 `git` CLI subprocess**：`git upload-pack|receive-pack [--advertise-refs] --stateless-rpc <repo_path>`。需要在生产 Docker 运行时镜像中加入 `git` 二进制。upload-pack / info-refs 在 gix PR #2465 合并后可迁移到纯 gix；receive-pack 永久保持 subprocess。
- **B. 等 gix PR #2465 合并后用纯 gix 做 upload-pack / info-refs**：PR 未合并，合入时间不定；且 receive-pack 仍无 gix 方案，仍需 subprocess。会无限期阻塞 git 托管主线。
- **C. 自研服务端 git 协议**（pkt-line 帧 + packfile 生成）：工程量大、易错、无意义重复成熟的 `git` CLI。

## 决策

选 **A**。三个 Smart HTTP 端点全部通过 `git` CLI subprocess 实现。

- 调用形式：`Command::new("git")`（不经过 shell），固定参数向量 `["upload-pack"|"receive-pack", "--advertise-refs"(仅 info/refs), "--stateless-rpc", <repo_path>]`。
- 安全约束：`repo_path` 始终来自按 repo id 的 DB 查找后拼接 `base_path`，绝不取自 URL；service 参数验证为枚举 {upload-pack, receive-pack}；`tokio::time::timeout` 防止挂起。
- 生产 Docker 运行时镜像（`debian:bookworm-slim`）新增 `git` 包（见 `backend/Dockerfile` + `docs/reference/SCRIPTS-RELEASE-NOTES.md`）。
- 迁移策略：gix PR #2465 合并后，把 `info/refs` + `git-upload-pack` 迁移到纯 gix（独立 EVO 跟踪）；**`git-receive-pack` 永久保持 subprocess**（gix 无服务端 receive-pack 计划）。

## 后果

正面：

- 立即可用，复用成熟 `git` CLI，协议正确性有保障。
- 不阻塞 git 托管主线。
- 与生产级 Rust git 服务器（loom 等）做法一致。

负面 / 技术债：

- 生产镜像必须包含 `git` 二进制（约 +5–10MB；已记入 Dockerfile 与 `SCRIPTS-RELEASE-NOTES.md`）。
- subprocess 存在安全面（已缓解：无 shell、固定参数、path-from-DB、timeout）。
- 临时技术债：`upload-pack` / `info-refs` 的 subprocess 待 gix PR #2465 迁移（独立 EVO）；`receive-pack` 为永久 subprocess。

## 相关链接

- [EVO-103-B Smart HTTP git 协议](../backlog/active/EVO-103-B-smart-http-git-protocol.md)
- [EVO-103-B-2 Smart HTTP Endpoints](../backlog/active/EVO-103-B-2-smart-http-endpoints.md)
- [ADR-0004 Git-Centric Storage](ADR-0004-git-centric-storage.md)
- [EVOLUTION.md](../../EVOLUTION.md)（subprocess 决策与 actix 中间件 soundness 经验）
- gitoxide PR [#2465](https://github.com/GitoxideLabs/gitoxide/pull/2465)（服务端 upload-pack，未合并）
- gitoxide tracking issue [#307](https://github.com/GitoxideLabs/gitoxide/issues/307)（服务端 receive-pack，无计划）
- 参考实现：[ghuntley/loom](https://github.com/ghuntley/loom)
