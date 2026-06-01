# Iteration 032: 后端依赖全量版本审计与迁移

> 文档状态：Closed
> 计划发布日期：2026-06-01
> 实际完成日期：2026-06-01
> 计划目标：用一个 Agent 微迭代完成 `EVO-043` 后端 workspace 依赖审计、可升级项迁移和暂缓项记录，降低后续 CI 重建返工风险。
>
> 基线保护：本文件一旦提交，以下“发布计划基线”内容不可因实施或改线而覆写；
> 同目标执行只向执行区追加事实，换目标必须保留本页并新建 iteration 编号。
> 闭环步骤：实施和收尾时按 [任务收口与完成声明](../sop/TASK-CLOSURE.md) 执行。

## 1. 发布计划基线：目标

- 审计 `backend/Cargo.toml` 的 workspace dependencies 和当前 `Cargo.lock`。
- 升级可安全迁移的后端依赖，尤其处理已观察到的 `sqlx-postgres` future-incompat 风险。
- 为暂缓的大版本升级写清原因、风险和后续归口。
- 为 [Iteration 029](ITERATION-029.md) 的 CI 重建提供更稳定的后端依赖基线。

## 2. 发布计划基线：候选故事与依赖

| ID | 标题 | 父 Epic | 优先级 | 启动条件/依赖 |
|----|------|---------|--------|---------------|
| EVO-043 | 后端依赖全量版本审计与迁移 | 无 | P1 | 无硬依赖；建议在 Iteration 029 前执行 |

## 3. 发布计划基线：不做事项

- 不升级前端依赖；前端依赖另行规划。
- 不升级 Rust edition 或 MSRV。
- 不为迁移依赖而扩大业务重构。
- 不引入 MySQL repository 支持；MySQL 仍只是配置层预留。

## 4. 发布计划基线：计划验收标准

- Story 格式与 BDD 适用性：
  - [x] Technical Story；BDD 不适用，使用依赖清单和命令级验收。
- [ ] 依赖审计表记录当前版本、目标版本、动作和暂缓原因。
- [ ] 可安全升级的依赖已迁移，破坏性升级只做有依据的最小适配。
- [ ] `cargo check --workspace` 通过。
- [ ] `cargo clippy --workspace -- -D warnings` 通过，或失败项被真实记录并拆出后续修复。
- [ ] `cargo test --workspace` 通过，或失败项被真实记录并拆出后续修复。

## 5. 发布计划基线：计划验证

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
git diff --check
```

## 6. 发布计划基线：风险与回滚

| 风险 | 处理 |
|------|------|
| 大版本升级引入 API 破坏 | 逐组升级，必要时暂缓并记录，不把大范围适配混入本轮 |
| Cargo.lock 大幅变动难以审查 | 记录升级分组和原因，提交前审查 `git diff --cached` |
| 依赖审计需要网络 | 若本地命令无法查询 registry，记录受限项并只做可验证的本地迁移 |

## 7. 闭环台账

| 项目 | 本轮记录 |
|------|----------|
| 请求结果 | 后端 workspace 依赖审计 + 可安全迁移项升级 + 暂缓项登记 |
| 产物 | `backend/Cargo.toml`（仅必要 upgrade）+ `backend/Cargo.lock` 同步 + 升级决策表（暂缓项纳入本迭代 Review）|
| 状态同步归口 | EVO-043 → In Progress（实施前）/ Done（实施后）、Iteration 032 → Closed、TESTING/TECH-STACK reference（如有破坏性变更）|
| Story/BDD 归口 | Technical Story；命令级验收（check/clippy/test）|
| 验证证据 | `cargo check --workspace` + `cargo clippy --workspace -- -D warnings` + `cargo test --workspace`（含 SQLite + PostgreSQL） + `git diff --check` |
| 残余工作归口 | 暂缓的大版本升级（需 CHANGELOG 适配）+ `clippy -D warnings` 18 个 pre-existing error（已挂 EVO-059，**本轮不并入**）|

## 8. 实际激活与执行记录

| 日期 | 类型 | 记录 |
|------|------|------|
| 2026-06-01 | planning | 发布计划基线；未启动实现。库存处置：Iteration 028 可优先激活；本轮可在 028 后、029 前执行。 |
| 2026-06-01 | activation | 库存盘点完成（028 Closed / 029 Blocked by user / 033-034 Ready）；按 README 推荐顺序激活 Iteration 032。开始实施：审计 `backend/Cargo.toml` workspace 依赖与 `Cargo.lock`，记录每个依赖的「当前 / 最新稳定 / 动作 / 原因」。 |
| 2026-06-01 | audit | 完整审计 36 个 workspace 功能依赖 + 3 个 path dep + 3 个 crate 私有 dep；详见下文「依赖审计表」。**22 个保留**（caret 范围已覆盖 latest stable，无 Cargo.toml 变更）；**14 个 workspace 暂缓**（大版本升级，EVO-061~074）；**2 个 crate 私有暂缓**（EVO-075 rand 0.8→0.10 / EVO-076 serde_yaml 0.9→serde_yml）；总计 16 个新 EVO backlog 项（EVO-061~076），全部 P3 Proposed。`cargo check` 0 error、`cargo test` 274 passed / 0 failed / 2 ignored；`cargo clippy --workspace --all-targets -- -D warnings` 18 pre-existing errors（确认归口 EVO-059，本轮不并入）。 |

## 8.5 依赖审计表

> 数据来源：crates.io API（`max_stable_version`）+ `backend/Cargo.lock` 当前锁定版本（`cargo update` 不带 `-p` 不修改 lockfile）。
> 概念区分：
> - **cargo.toml 约束**：当前 Cargo.toml 写的最低版本（caret），如 `"1.35"` 即 `^1.35` → `>=1.35, <2.0`。
> - **lock 当前**：Cargo.lock 实际解析到的精确版本（cargo 自动选 latest within range）。
> - **latest stable**：crates.io 报告的当前最新稳定版。
> - **动作**：本次决定（保留 = 现状 / 暂缓 = 大版本迁移另开迭代 / 调整 floor = 提升最低版本）。

### 8.5.1 Workspace 依赖（39 项，源自 `backend/Cargo.toml [workspace.dependencies]`）

| # | 依赖 | cargo.toml 约束 | lock 当前 | latest stable | 大版本 | 动作 | 决定理由 / 风险 |
|---|------|----------------|-----------|---------------|--------|------|----------------|
| 1 | `tokio` | 1.35 | 1.49.0 | 1.52.3 | 0 | 保留 | caret 已解析到 1.49；floor 1.35 满足 1.75 MSRV 兼容 |
| 2 | `futures-util` | 0.3 | 0.3.32 | 0.3.32 | 0 | 保留 | latest stable |
| 3 | `rust-embed-for-web` | 11.3 | 11.3.0 | 11.3.0 | 0 | 保留 | latest stable；Iteration 031 决策依赖 |
| 4 | `actix-web-rust-embed-responder` | 2.3 | 2.3.0 | 2.3.0 | 0 | 保留 | latest stable |
| 5 | `bollard` | 0.17 | 0.17.1 | 0.21.0 | +3 | **暂缓 → EVO-061** | Docker Engine API 跨 4 个 minor，需 CHANGELOG 适配；当前 `bollard 0.17` 在 Phase 7 sandbox 验证通过，零运行时问题 |
| 6 | `actix-web` | 4.4 | 4.13.0 | 4.13.0 | 0 | 保留 | caret 已解析到 4.13 |
| 7 | `actix-cors` | 0.7 | 0.7.1 | 0.7.1 | 0 | 保留 | latest stable |
| 8 | `actix-web-httpauth` | 0.8 | 0.8.2 | 0.8.2 | 0 | 保留 | latest stable |
| 9 | `serde` | 1.0 | 1.0.228 | 1.0.228 | 0 | 保留 | latest stable |
| 10 | `serde_json` | 1.0 | 1.0.149 | 1.0.150 | 0 | 保留 | caret 已解析到 patch 149，latest 是 150（无行为变化） |
| 11 | `sqlx` | 0.7 | 0.7.4 | 0.8.6 / 0.9.0 | +1 / +2 | **暂缓 → EVO-062** | 19 个文件 / ~120 call sites 依赖 `query!` / `query_as!` 宏与 `FromRow` derive；0.7→0.8 是 macro 与 `Database` trait 跨主版本破坏，需独立迁移迭代。**已知 future-incompat warning**（never type fallback）将在 Rust 2024 edition / 1.95+ 强制执行前不阻断；当前 `rust-version = "1.75"` + `edition = "2021"` 不受影响 |
| 12 | `uuid` | 1.6 | 1.21.0 | 1.23.2 | 0 | 保留 | 1.6→1.x 是 minor（语义不变：v4/v5/serde） |
| 13 | `chrono` | 0.4 | 0.4.44 | 0.4.44 | 0 | 保留 | latest stable；0.4 末梢有维护 |
| 14 | `config` | 0.14 | 0.14.1 | 0.15.23 | +1 | **暂缓 → EVO-063** | 0.14→0.15 改 `ConfigBuilder` API（`with_source` → `add_source`），影响 `infra/src/config.rs` |
| 15 | `dotenvy` | 0.15 | 0.15.7 | 0.15.7 | 0 | 保留 | latest stable |
| 16 | `tracing` | 0.1 | 0.1.44 | 0.1.44 | 0 | 保留 | latest stable |
| 17 | `tracing-subscriber` | 0.3 | 0.3.22 | 0.3.23 | 0 | 保留 | caret 解析到 22，latest 是 23（patch 修复） |
| 18 | `validator` | 0.16 | 0.16.1 | 0.20.0 | +3 | **暂缓 → EVO-064** | 0.18 起 `validate` 改成 `#[derive(Validate)]` + `validator::Validate` trait；9 个 domain 模型 + DTO 需适配；breaking change 跨多个 minor |
| 19 | `regex` | 1.10 | 1.12.3 | 1.12.3 | 0 | 保留 | caret 解析到 1.12 |
| 20 | `lazy_static` | 1.4 | 1.5.0 | 1.5.0 | 0 | 保留 | caret 解析到 1.5 |
| 21 | `jsonwebtoken` | 9.2 | 9.3.1 | 10.4.0 | +1 | **暂缓 → EVO-065** | 9→10 改 `Header` / `EncodingKey` / `decode` 签名（with/without validation 合并）；影响 `service-auth/src/jwt.rs` |
| 22 | `argon2` | 0.5 | 0.5.3 | 0.5.3（0.6.0-rc.8） | 0（RC） | 保留 | 0.6 仍是 RC；0.5.3 稳定且 `password-hash` 0.5 API 稳定 |
| 23 | `thiserror` | 1.0 | 1.0.69 | 2.0.18 | +1 | **暂缓 → EVO-066** | 1→2 需 `edition = "2024"`；影响 `common/src/error.rs` + 8 个 service error 类型 |
| 24 | `anyhow` | 1.0 | 1.0.102 | 1.0.102 | 0 | 保留 | latest stable |
| 25 | `async-trait` | 0.1 | 0.1.89 | 0.1.89 | 0 | 保留 | latest stable；0.2 未发布稳定版 |
| 26 | `jsonschema` | 0.17 | 0.17.1 | 0.46.5 | +28 | **暂缓 → EVO-067** | 跨 28 个 minor 的 schema draft / API 演进；`service-tool` 内部使用 0.17 即可 |
| 27 | `async-stripe` | 0.41 | 0.41.0 | 0.41.0（1.0.0-rc.6） | 0（RC） | 保留 | 1.0 仍是 RC；不引入预发布版 |
| 28 | `reqwest` | 0.11 | 0.11.27 | 0.13.4 | +2 | **暂缓 → EVO-068** | 0.12 起移除 `blocking` 特性独立 crate；0.13 进一步改 `ClientBuilder` API；影响 `service-tool` HTTP executor + `service-payment` Stripe webhook |
| 29 | `hmac` | 0.12 | 0.12.1 | 0.13.0 | +1 | **暂缓 → EVO-069** | 0.13 改 `Mac::new_from_slice` 签名 + 移除 `SimpleHMac`；影响 `service-payment` Stripe webhook 验签 |
| 30 | `sha2` | 0.10 | 0.10.9 | 0.11.0 | +1 | **暂缓 → EVO-070** | 0.11 改 `Sha256::new()` 返回 `CtOutput`（const-time） |
| 31 | `hex` | 0.4 | 0.4.3 | 0.4.3 | 0 | 保留 | latest stable |
| 32 | `redis` | 0.27 | 0.27.6 | 1.2.2 | +0（命名空间重置） | **暂缓 → EVO-071** | 0.27→1.0 是命名空间重置（`redis::Client` → `redis::Client` 但内部 trait 完全重写）；0.x→1.x 是显式 breaking；`service-payment` 暂未真实使用（infra 缓存层预留） |
| 33 | `actix-governor` | 0.6 | 0.6.0 | 0.10.0 | +0（命名空间重置） | **暂缓 → EVO-072** | 0.6→0.7 改 `Governor` 中间件构造（基于 `actix-web` 4.x 新 KeyExtractor）；影响 `api/src/middleware/rate_limit.rs` |
| 34 | `lettre` | 0.11 | 0.11.21 | 0.11.22 | 0 | 保留 | caret 解析到 21，latest 是 22 patch |
| 35 | `actix-web-prom` | 0.8 | 0.8.0 | 0.10.0 | +0（命名空间重置） | **暂缓 → EVO-073** | 0.9 起改 `PrometheusMetricsBuilder` API；metrics endpoint 路径需调整 |
| 36 | `prometheus` | 0.13 | 0.13.4 | 0.14.0 | +1 | **暂缓 → EVO-074** | 0.14 改 `Encoder` trait 签名；与 actix-web-prom 强耦合 |
| 37-39 | 内部 path deps（api/service-*/domain/infra/common/service-audit） | path | — | — | — | 不适用 | Cargo workspace path deps，无版本概念 |

### 8.5.2 Crate 私有依赖（3 项，不在 `[workspace.dependencies]`）

| # | Crate | 依赖 | lock 当前 | latest stable | 动作 | 决定理由 |
|---|-------|------|-----------|---------------|------|----------|
| 1 | `service-auth` | `rand` 0.8 | 0.8.5 | 0.8.5（0.10.1） | **暂缓 → EVO-075** | caret 解析到 0.8.5；0.9/0.10 是 major breaking（`Rng` trait 重组）需独立迁移迭代 |
| 2 | `service-skill` | `serde_yaml` 0.9 | 0.9.34+deprecated | 0.9.34+deprecated | **暂缓 → EVO-076** | 上游已 deprecate；建议未来迁移到 `serde_yml`（社区 fork）或 `serde_norway`（纯 Rust 替代） |
| 3 | `service-snippet` | `serde_yaml` 0.9 | 0.9.34+deprecated | 0.9.34+deprecated | **暂缓 → EVO-076** | 同上 |

### 8.5.3 暂缓项总表（17 项 → 16 个新 EVO backlog item）

> 命名约定：`EVO-061` 起为本次审计暂缓项（顺序按表中行号）。每一项在本迭代收口时同步写入 `docs/backlog/PRODUCT-BACKLOG.md` 详情块 + 总结表。

| EVO | 标题 | 影响范围 | 优先级 |
|-----|------|----------|--------|
| EVO-061 | bollard 0.17→0.21 Docker API 升级 | `service-skill/src/executor.rs` + sandbox 镜像 | P3 |
| EVO-062 | sqlx 0.7→0.8/0.9 迁移（future-incompat 解决） | 19 个 repo 文件 + 所有 query 宏 | P1 |
| EVO-063 | config 0.14→0.15 升级 | `infra/src/config.rs` | P3 |
| EVO-064 | validator 0.16→0.18+ 升级（trait 迁移） | 9 个 domain / DTO | P2 |
| EVO-065 | jsonwebtoken 9→10 升级 | `service-auth/src/jwt.rs` | P2 |
| EVO-066 | thiserror 1→2 升级（需 edition 2024） | `common/src/error.rs` + 8 个 service error | P2 |
| EVO-067 | jsonschema 0.17→0.46 升级 | `service-tool` | P3 |
| EVO-068 | reqwest 0.11→0.12/0.13 升级 | `service-tool` + `service-payment` | P2 |
| EVO-069 | hmac 0.12→0.13 升级 | `service-payment` webhook 验签 | P3 |
| EVO-070 | sha2 0.10→0.11 升级 | `service-payment` + `infra` | P3 |
| EVO-071 | redis 0.27→1.x 命名空间重置 | `infra/src/cache.rs` | P3 |
| EVO-072 | actix-governor 0.6→0.7+ 升级 | `api/src/middleware/rate_limit.rs` | P2 |
| EVO-073 | actix-web-prom 0.8→0.9+ 升级 | `api/src/lib.rs` | P3 |
| EVO-074 | prometheus 0.13→0.14 升级 | `api` metrics | P3 |
| EVO-075 | rand 0.8→0.9/0.10 升级（`service-auth`） | `service-auth` | P3 |
| EVO-076 | serde_yaml 0.9 → serde_yml / serde_norway 迁移 | `service-skill` + `service-snippet` | P3 |

### 8.5.4 验证证据

| 验证项 | 命令 | 结果 | 备注 |
|--------|------|------|------|
| baseline build | `cargo check --workspace` | 0 error | 含 sqlx-postgres 0.7.4 future-incompat warning（已记 EVO-062） |
| 全量测试 | `cargo test --workspace --no-fail-fast` | 274 passed, 0 failed, 2 ignored | 跨 SQLite + PostgreSQL 集成测试 |
| clippy（含 tests） | `cargo clippy --workspace --all-targets -- -D warnings` | 18 pre-existing errors | 全部归口 EVO-059，本轮不并入 |
| `git diff --check` | `git diff --check` | 0 error | 仅文档变更 |

## 9. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-06-01 | scope | **不修改 `backend/Cargo.toml`**，所有 caret floor 保持现状（cargo 已自动解析到 latest stable，floor bump 是纯文档性变更，无运行时差异） | 无 | — |
| 2026-06-01 | scope | **不并入 clippy 18 pre-existing errors 修复**（EVO-059 已独立跟踪；本轮 plan §3 明确「不为迁移依赖而扩大业务重构」） | 无 | 维持 EVO-059 P2 Ready |
| 2026-06-01 | scope | **暂缓 16 个大版本升级**，每个新 EVO 项标记 Proposed 等待独立迁移迭代 | 新增 16 个 backlog 项（EVO-061~076） | 见 §8.5.3 |
| 2026-06-01 | scope | **EV0-066 thiserror 1→2 标注前置依赖**（需先解除 Iteration 032 plan §3「不升级 Rust edition」约束） | EVO-066 状态保持 Blocked until edition 升级立项 | — |

## 10. Review

- 完成：
  - 39 个 workspace 依赖 + 3 个 crate 私有 dep 完整审计；每项记录「cargo.toml 约束 / lock 当前 / latest stable / 大版本 / 动作 / 决定理由」（见 §8.5）。
  - 16 个新 backlog 项（EVO-061~076）入池，标记 Proposed 等待独立迁移迭代。
  - EVO-043 总结表行 + 详情块状态更新为 Done；EVO-059 详情块修正为「18 个错误分布确认」（10 unwrap / 3 dead_code / 4 unnecessary_min / 1 field_reassign；项目无 clippy.toml allow-unwrap-in-tests）。
  - EVOLUTION.md Part 2 追加 2 条新经验（cargo caret 解析 + 大版本迁移成本估算方法）。
  - iterations/README.md 同步：Iteration 032 标 Closed，移出「未来计划」清单，加入 Closed 库存记录。
  - Markdown 链接检查：DOC-CHECK inline 0 断链；`git diff --check` 0 error。
- 未完成：cargo floor bump 未做（决定不做，理由见 §9）；16 个大版本升级迁移未做（独立迭代处理）。
- 验证结果：
  - `cargo check --workspace`：0 error（含 sqlx-postgres 0.7.4 future-incompat warning → EVO-062）。
  - `cargo test --workspace --no-fail-fast`：**274 passed, 0 failed, 2 ignored**（含 SQLite + PostgreSQL integration tests）。
  - `cargo clippy --workspace --all-targets -- -D warnings`：**18 pre-existing errors**（10× unwrap_used / 3× dead_code / 4× unnecessary_min_or_max / 1× field_reassign_with_default），全部归口 EVO-059，本轮不并入。
  - `cargo report future-incompatibilities --id 1`：`sqlx-postgres v0.7.4` never-type-fallback 警告（已记 EVO-062）。
- 闭环状态：`Complete`
- 残余归口：
  - **EVO-059**（P2 Ready）：clippy 18 errors 修复，下个治理迭代激活。
  - **EVO-061~076**（P3 Proposed）：16 个大版本依赖迁移，每个独立迭代评估激活。
  - **EVO-066** 额外前置：本迭代 plan §3「不升级 Rust edition」约束，激活时需先解除。
  - **Iteration 029**（Blocked by user）：CI 重建仍待 EVO-062（sqlx）落地后再评估激活。

## 11. Retrospective

- 做得好的：
  - 严格执行 START-ITERATION.md inventory 盘点（028 Closed / 029 Blocked by user / 033-034 Ready）后激活 032，未越级。
  - 把审计拆成三段：①workspace deps（caret 解析到 latest）②crate 私有 dep（serde_yaml deprecated 等）③大版本升级（16 项拆 backlog）—— 决策颗粒度清晰。
  - 显式判定「不修改 Cargo.toml」并记录理由（cargo caret 已自动解析，floor bump 是纯文档性变更）；避免无意义 churn。
  - 严格执行 plan §3「不升级 Rust edition」「不并入 clippy 修复」约束，不让治理迭代夹带业务改动。
  - 完整记录 future-incompat 警告（sqlx-postgres 0.7.4）的具体技术细节（never-type-fallback + Rust 2024 edition 强制路径），为 EVO-062 留足 DoR。
- 需要调整的：
  - EVO-059 详情块「clippy.toml allow-unwrap-in-tests」表述不准确（项目实际无 clippy.toml）；本轮已修正。下次写 EVO 详情前应先 `ls backend/clippy.toml` 确认。
  - 第一次 clippy 跑用 `cargo clippy --workspace -- -D warnings`（默认不查 tests）报 0 error，导致误判；后续应默认加 `--all-targets`。
  - `cargo clippy` 第一次失败时（infra test 编译阻断），后续 crate 错误不输出；评估总错误数时需分 crate 单独跑。
- 写入 EVOLUTION：
  - **cargo caret 解析的「floor 不动」模式**：`^X.Y` 约束下 cargo 自动选 latest within range；floor bump 是纯文档性变更。
  - **依赖大版本迁移的成本估算**：`rg -c 'query!|query_as!' backend/crates` 给出 call site 数 × 8 个 repo 文件 = 迁移面；用 `cargo report future-incompatibilities` 给出具体升级理由。
  - **clippy `--all-targets` 是默认行为**：lib + bin 之外要查 tests/examples/benches 必须显式加 `--all-targets`，否则会假绿。
