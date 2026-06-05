# EVO-085 Archive index 二次压缩与 item file 拆分纠偏

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: governance
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-06-05 / backlog compaction follow-up
- Decision Context: `archive/2026-Q2/INDEX.md` 不应承载全部历史详情，已拆为 per-item archive files 并保留短索引

## Problem

第一次 backlog compaction 将 `PRODUCT-BACKLOG.md` 压缩为入口文件，但把已归档事项的长详情集中放入 `docs/backlog/archive/2026-Q2/INDEX.md`。这让 archive index 变成新的大型 dump，不符合 `backlog-compaction.md` 中 `archive/<period>/<item>.md` 保存归档 item context 的形态。

## Scope

- 将 54 个 archived detail sections 拆分为独立归档 item files。
- 将 `docs/backlog/archive/2026-Q2/INDEX.md` 缩减为短索引和读取规则。
- 将 `PRODUCT-BACKLOG.md` 的 archived links 从 `INDEX.md#anchor` 改为对应归档 item file。
- 写回 EVOLUTION，避免把“搬到 archive 的巨型单文件”误判为 compaction 完成。

## Acceptance

- [x] `archive/2026-Q2/INDEX.md` 只作为索引，不再包含所有 Source Detail Snapshot。
- [x] 每个 archived item 有独立 `.md` 文件。
- [x] `PRODUCT-BACKLOG.md` archived index 指向具体 archived item file。
- [x] Markdown link check 通过。
- [x] Governance validator 通过。

## Validation

- `sh .../validate_project_governance.sh /Users/GHuang/WorkSpace/AiProjects/evolith`
- Markdown local link check
- `git diff --check`

