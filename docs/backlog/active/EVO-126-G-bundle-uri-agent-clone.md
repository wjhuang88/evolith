# EVO-126-G — Bundle URI acceleration for Agent clone workloads

- **父 Epic**：[EVO-126](EVO-126-walgit-git-data-plane-refactor.md)
- **类型**：Technical / Git Read Performance / Security-sensitive
- **优先级**：P1
- **状态**：Proposed
- **GitHub Issue**：[#19](https://github.com/wjhuang88/evolith/issues/19)
- **依赖或阻塞**：EVO-126-C、EVO-126-F
- **解锁内容**：EVO-126-H
- **影响范围**：backend / deploy / docs / tests

## 工程目标

利用 WalGit bundle primitives 为 Agent/CI 高频 clone/fetch 建立安全的 bundle-uri data path，降低每次 fresh clone 都由应用动态生成完整历史 pack 的成本。

## Scope

- 接入 `walgit-bundle` 或其当前稳定 lower-level contract。
- protocol v2 中 advertisement/list/serve 与 upload-pack remainder/fallback 协作。
- 采用明确的 bundle schedule/retention 策略，并记录与 pinned upstream 行为的关系。
- blobless/filter clone 有稳定兼容与 fallback 行为。
- bundle bytes 可通过 object-store/CDN data path 分发，但 read authorization 仍由 Evolith 控制。
- 建立 bundle hit/fallback/upload-pack metrics。

## Acceptance

- [ ] 支持 bundle-uri 的标准 Git client clone E2E 通过。
- [ ] unauthorized/cross-tenant principal 不能通过静态 bundle URL 绕过 read permission。
- [ ] signed/short-lived/equivalent access boundary 不暴露长期 bearer capability。
- [ ] bundle missing/corrupt/download failure 有明确、有界 fallback，不形成无限重试或无界 server load。
- [ ] blobless/filter 与普通 clone 的兼容矩阵有证据。
- [ ] 指标可区分 bundle hit/fallback/upload-pack。

## 最小验证

真实 Git clone bundle-uri matrix + auth negative tests + fallback/failure tests + metrics assertion。

## 不做

不实现 LFS，不在本 Story 产品化跨地域 CDN，也不实现 Agent Workspace 本身。
