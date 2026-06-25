# EVO-103-B Smart HTTP git 协议（父项，拆为 B-1/B-2）

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- 父 Epic: [EVO-103](EVO-103-repo-context-and-smart-http.md)
- 祖父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Summary

- 类型：epic（父项，拆为 B-1/B-2 子 Story）
- 优先级：P0
- 状态：Proposed
- 父 Epic：EVO-103（祖父 EVO-100）

## 拆分理由

原始 EVO-103-B 范围（git 客户端认证基础设施 + Smart HTTP 协议端点实现）包含两个独立可验收的工程结果：(1) 跨平台 auth 基础设施（Basic-Auth、RBAC 路径注册、CSRF 豁免），(2) 协议级端点实现（git subprocess 流式处理）。按 REQUIREMENT-INTAKE 端到端价值切片方法，拆为两个子 Story：B-1（auth 基础设施，可独立验证）、B-2（协议端点，硬依赖 B-1）。

**关键技术纠正**：原始 EVO-103-B item file 存在错误的技术假设——声称 `gix` 可以服务 info/refs 和 upload-pack。经研究确认：已发布的 `gix` crate（v0.78.0）**没有任何服务端协议支持**。所有 3 个 Smart HTTP 端点当前必须通过 `git` 二进制 subprocess 实现。gix 服务端 upload-pack 仅在未合并 PR #2465 中；gix 维护者明确表示服务端 receive-pack 无计划（推荐 shelling out to git）。所有生产级 Rust git 服务器（如 loom）均使用 `git --stateless-rpc` subprocess 实现全部端点。

## 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| [EVO-103-B-1](EVO-103-B-1-git-client-auth-infra.md) | Git 客户端认证基础设施（Basic-Auth + RBAC 端点挂载 + CSRF 豁免） | Done | EVO-103-A（Done） | Iteration 045 |
| [EVO-103-B-2](EVO-103-B-2-smart-http-endpoints.md) | Smart HTTP 端点实现（git subprocess 流式处理 3 个端点） | Proposed | EVO-103-B-1（硬）, EVO-103-A（Done） | - |

## Problem Or Outcome

实现 Smart HTTP git 协议端点，让用户可通过标准 git 客户端（`git clone` / `git push` / `git pull`）与 Evolith 托管的仓库交互。详细验收标准已分发到子 Story B-1/B-2。

## Goal And Non-Goals

- **Goal**：见子 Story B-1/B-2 各自 Goal。
- **Non-goals**：
  - 不实现 repo CRUD（→ EVO-103-A）。
  - 不实现 file-tree / blobs / commits / diff API（→ EVO-103-C）。
  - 不实现 SSH server（Phase 5）。
  - 不实现 LFS（Phase 5）。
  - 不使用 gix 服务端（已发布 crate 中不存在；未来 gix PR #2465 合并后可替换 info/refs + upload-pack subprocess）。

## Dependencies And Blockers

- 子 Story B-1 依赖 EVO-103-A（Done）。
- 子 Story B-2 硬依赖 B-1 完成。
- 间接依赖 `gix` crate（EVO-086 Done），但仅用于客户端操作，不用于服务端协议。

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

父项完成条件：子 Story B-1、B-2 全部 Done。
- B-1: Git 客户端认证基础设施（Basic-Auth + RBAC 端点挂载 + CSRF 豁免）
- B-2: Smart HTTP 端点实现（info/refs + upload-pack + receive-pack，全部 git subprocess）

详细验收标准见各子 Story item file。

## Validation Evidence Required

- 各子 Story 独立验证证据见对应 item file。
- 整体 E2E 链路：Basic-Auth 认证 → git clone → git push → git pull 完整通过。

## Residual Work Destination

- 当 gix PR #2465（服务端 upload-pack）合并后，可将 info/refs + upload-pack subprocess 替换为纯 gix——跟踪为未来 EVO；**receive-pack 永久保持 subprocess**（无 gix 服务端 receive-pack 计划，gix 维护者推荐 shelling out）。
- EVO-106 scoped token 将提供专用 git token（B-1 使用 API key 作为过渡方案）。
- SSH server（Phase 5）。
- LFS（Phase 5）。

## Source Snapshot

- Source: EVO-103 拆分（2026-06-25），按端到端价值切片拆为 A/B/C；B 再拆为 B-1/B-2（2026-06-25 refinement）。
- Decision context: Smart HTTP 是 git 协议层实现，依赖 EVO-103-A 的裸仓初始化；已发布 gix 无服务端协议支持，所有端点使用 `git --stateless-rpc` subprocess。
- 关键依赖：`git` CLI subprocess（`Command::new("git")`）、Actix-web 流式（`web::Payload` + `HttpResponse::streaming()`）、Basic-Auth 解析（`rbac.rs`）、CSRF 豁免（`csrf.rs`）。
- 拆分日期：2026-06-25。
