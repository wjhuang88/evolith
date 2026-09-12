# ADR-0006: Smart HTTP via `git` CLI subprocess（gix 无服务端）

> **2026-09-10 amendment**：本 ADR 继续作为 2026-06 至 EVO-126-H cutover 前的 **current/historical implementation rationale**；其“长期 Smart HTTP 目标”和“receive-pack 永久保持 subprocess”结论已被 [ADR-0011](ADR-0011-walgit-backed-git-data-plane.md) supersede。EVO-126-H 完成前不得反向把本状态标成“当前代码已删除 subprocess”。

## 状态

Accepted historical / **Superseded as target by ADR-0011**（原决策 2026-06-26；目标状态更新 2026-09-10）

## 背景

Evolith 作为 git 托管平台需要 Smart HTTP git 协议（`GET /repos/{id}/info/refs`、`POST /repos/{id}/git-upload-pack`、`POST /repos/{id}/git-receive-pack`），让标准 git 客户端可以 `git clone / push / pull`。项目已用 `gix` crate 做 bare repo 初始化和 Repo Context。

2026-06 调查时，已发布的 `gix` crate 没有可直接满足 Evolith 的完整服务端协议能力：

- 服务端 upload-pack 当时仅存在于未合并的 gitoxide 工作；
- 服务端 receive-pack 没有可用的完整方案；
- 为避免阻塞 Git Alpha，成熟 `git --stateless-rpc` subprocess 是当时风险最低的实现路径。

## 当时选项

- **A. 全部 3 端点用 `git` CLI subprocess**：`git upload-pack|receive-pack [--advertise-refs] --stateless-rpc <repo_path>`。
- **B. 等待 gix 服务端能力**：时间不可控，且 receive-pack 仍存在缺口。
- **C. 自研完整服务端 Git 协议**：工程量大、风险高。

## 原决策

2026-06 选择 **A**：三个 Smart HTTP 端点通过 `git` CLI subprocess 实现。

- `Command::new("git")`，不经过 shell；固定 service 枚举与参数。
- `repo_path` 来自 DB identity +受控 base path，不直接取 URL path。
- timeout/资源上限防止 unbounded child/process work。
- 生产 runtime 包含 `git` binary。

该决策成功解锁并支撑了 EVO-103/115/116/125，以及真实 clone/push/pull、Repo UI/Context 和 Durable Push Event 的 Alpha 验证。

## 2026-09-10 新目标

[ADR-0011](ADR-0011-walgit-backed-git-data-plane.md) 接受以下长期架构：

```text
Evolith Actix adapters
 -> service-git v2
 -> walgit-git / walgit-wal / walgit-store / walgit-bundle
 -> object store durable Git truth
```

因此：

- `info/refs` / `upload-pack` / `receive-pack` 的 Evolith 主路径不再以“直接对 durable filesystem repo shell out”为长期设计。
- 可继续使用 `git` CLI 的地方必须属于 WalGit lower-level primitive、明确兼容/辅助实现，或有独立 owner；不能重新形成 Evolith-owned durable filesystem engine。
- `walgit-server` 的协议 data-plane orchestration 可以按 MIT 许可证吸收/改写，但 Axum Router/Auth/UI/Product Policy 不成为 Evolith 控制面。
- receive-pack 的 durability/publication 目标改为 WalGit WAL + manifest CAS，并在其上保持 Evolith Policy/Outbox/Audit 语义。

## 后果

### 历史正面结果继续有效

- Alpha 可以立即使用标准 Git client。
- 协议正确性复用了成熟 Git CLI。
- timeout、固定参数、DB-derived path 等安全经验继续作为新实现必须保持的门禁。

### 旧实现成为迁移技术债

EVO-126-H 前：

```text
current runtime = direct git subprocess + persistent filesystem bare repo
```

EVO-126-H 后目标：

```text
current runtime = Evolith service-git v2 + WalGit-backed object-store/WAL data plane
```

只有 cutover、migration/recovery、真实 Git client 和 consumer-inventory 证据完成后，才能删除旧主路径并关闭 GIT-DP-01。

## 相关链接

- [ADR-0011 WalGit-backed Git Data Plane](ADR-0011-walgit-backed-git-data-plane.md)
- [EVO-126](../backlog/active/EVO-126-walgit-git-data-plane-refactor.md)
- [WalGit Refactor Design](../design/WALGIT-GIT-DATA-PLANE-REFACTOR.md)
- [Project Status Baseline](../reference/PROJECT-STATUS-BASELINE-2026-09-10.md)
- [EVO-103-B Smart HTTP git 协议](../backlog/active/EVO-103-B-smart-http-git-protocol.md)
- [EVO-125 Git Smart HTTP E2E Reliability](../backlog/active/EVO-125-git-smart-http-e2e-hang.md)
- [EVOLUTION.md](../../EVOLUTION.md)
