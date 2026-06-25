# EVO-089 API Contract 与实现一致性审计

## Required Reads

- Product Backlog: `../PRODUCT-BACKLOG.md`
- API Contract: `../../reference/API-CONTRACT.md`
- EVO-055 item file: `../archive/2026-Q2/EVO-055-前后端-api-契约漂移修复.md`

## Summary

- 类型：chore
- 优先级：P2
- 状态：Proposed
- 父 Epic：无

## Problem Or Outcome

历史上多次出现 API contract 与实际实现不一致（EVO-001、EVO-055 已修复部分），需要系统性审计确保：
- API Contract 文档描述与后端 routes/handlers 实现一致
- 前端 API client 调用与后端实际 endpoint 匹配
- DTO 字段名、类型、必填与 contract 描述一致

避免"看起来有功能，实际不可用"的隐蔽问题。

## Goal And Non-Goals

**目标**：
- 全量对比 API-CONTRACT.md 与 routes/mod.rs、各 handlers 实现一致性
- 对比 frontend/src/lib/api/ 与实际 endpoint 匹配情况
- 产出不一致清单并修复或更新文档

**不做**：
- 不为 501 placeholder 补实现（归口各自 backlog）
- 不改变现有 API 设计原则
- 不引入自动化 contract testing（可后置）

## Dependencies And Blockers

无硬依赖。可独立启动。

## Governing ADRs, Specs Or Decisions

- `docs/reference/API-CONTRACT.md` 是合约基线
- EVO-055 已修复部分漂移，可参考方法

## Acceptance Criteria

- [ ] 产出 API-CONTRACT vs 实现对比报告（一致/不一致/501/文档缺失）
- [ ] 所有不一致项已修复或已记录到 backlog（非 501）
- [ ] 501 placeholder 在 contract 中明确标注状态
- [ ] 前端 API client 无调用不存在 endpoint 的死代码
- [ ] `cargo test --workspace` 和 `bun run build` 仍 0 errors

## Validation Evidence Required

- 对比报告文档或 markdown 表格
- 修复 commit list 或 backlog item list
- API Contract 更新 diff

## Residual Work Destination

- 审计结果写入 `docs/reference/API-CONTRACT.md` 校验记录段
- 发现的新问题创建 backlog items

## Source Snapshot

- API Contract 文件：`docs/reference/API-CONTRACT.md`
- 后端 routes：`backend/crates/api/src/routes/mod.rs`
- 前端 API client：`frontend/src/lib/api/`