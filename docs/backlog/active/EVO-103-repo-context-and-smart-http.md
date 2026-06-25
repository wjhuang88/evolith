# EVO-103 Repo CRUD + Smart HTTP + Repo Context API

## Required Reads

- [Product Backlog](../PRODUCT-BACKLOG.md)
- [Git-Centric Platform Proposal](../../proposals/GIT-CENTRIC-PLATFORM.md)
- 父 Epic: [EVO-100](EVO-100-git-centric-platform-foundation.md)
- 依赖: [EVO-101](EVO-101-git-repos-schema.md)

## Summary

- 类型：feature / api（父项，拆为 A/B/C 子 Story）
- 优先级：P0
- 状态：Proposed
- 父 Epic：EVO-100

## 拆分理由

原始 EVO-103 范围（Repo CRUD + Smart HTTP git 协议 + Repo Context API）超过 0.5-2 天单 Story 交付窗口。按 REQUIREMENT-INTAKE 端到端价值切片方法，拆为三个独立可验收的子 Story：A（CRUD 基础）、B（git 协议）、C（上下文查询 API）。

## 子 Story

| 子 Story | 独立结果 | 状态 | 依赖 | 所属迭代 |
|----------|----------|------|------|----------|
| [EVO-103-A](EVO-103-A-repo-crud.md) | Repo CRUD API + gix::init 裸仓 + RBAC | Done | EVO-101（Done） | Iteration 044 |
| [EVO-103-B](EVO-103-B-smart-http-git-protocol.md) | Smart HTTP git 协议（clone/push/pull） | Proposed | EVO-103-A | - |
| [EVO-103-C](EVO-103-C-repo-context-api.md) | Repo Context API（file-tree/blobs/commits/diff） | Proposed | EVO-103-A | - |

## Problem Or Outcome

实现 Repo CRUD HTTP API、Smart HTTP git 协议（基于 `gix`）、Repo Context API（file-tree / blobs / commits / diff），让用户可在 Evolith 托管 git repo 并支持标准 git 客户端操作。详细验收标准已分发到子 Story A/B/C。

## Goal And Non-Goals

- **Goal**：见子 Story A/B/C 各自 Goal。
- **Non-goals**：
  - 不实现 commit API 与 promote（EVO-105）
  - 不实现 SSH server（Phase 5）
  - 不实现 LFS（Phase 5）

## Dependencies And Blockers

- 依赖 EVO-101 `git_repos` 表（Done）
- 间接依赖 `gix` crate 已升级（EVO-086 Done）
- 子 Story B/C 硬依赖 A 完成

## Governing ADRs, Specs Or Decisions

- [ADR-0004 Git-Centric Storage](../../decisions/ADR-0004-git-centric-storage.md)

## Acceptance Criteria

父项完成条件：子 Story A、B、C 全部 Done。
- A: Repo CRUD + gix::init + RBAC + seed-template
- B: Smart HTTP git 协议（info/refs + upload-pack + receive-pack）
- C: Repo Context API（file-tree + blobs + commits + diff）

详细验收标准见各子 Story item file。

## Validation Evidence Required

- 各子 Story 独立验证证据见对应 item file。
- 整体 E2E 链路：create repo → push (含 SKILL.md) → clone → file-tree → blob → commit history → diff。

## Residual Work Destination

- `gix-push` 合入后移除 subprocess 临时方案（独立 EVO，记 EVOLUTION）
- SSH（Phase 5）
- LFS（Phase 5）
- 跨 tenant 公开 repo discover（Phase 5+）

## Source Snapshot

- Source: 用户反馈 2026-06-23
- Decision context: 通用 git 托管服务的最小可用集合
- 关键依赖：`gix` crate（参考 `gitserver` 项目模式），`gix::protocol`、`gix-pack`、`gix::Tree`、`gix::Object`
- 拆分日期：2026-06-25，按端到端价值切片拆为 A/B/C
- push 临时 subprocess 走 `git receive-pack` 命令（spec 完整）；记 EVOLUTION
