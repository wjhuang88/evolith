# EVO-128 — Pin CI PostgreSQL client/server major for durability gates

- **类型**：Technical / CI / urgent-fix
- **优先级**：P0
- **状态**：In Progress
- **GitHub Issue**：[#35](https://github.com/wjhuang88/evolith/issues/35)
- **触发**：GraphQL governance PR #34 `ci` #294
- **影响范围**：CI / DATA-01 durability evidence

## 问题

GitHub hosted `ubuntu-latest` 已漂移到 Ubuntu 26.04，`postgresql-client` meta package 随之变为 PostgreSQL 18；Evolith CI 的数据库 service 仍显式为 `postgres:16-alpine`。PostgreSQL 18 `pg_dump` 生成的 plain SQL 含 `SET transaction_timeout = 0;`，PostgreSQL 16 restore 时拒绝该参数，导致 DATA-01 backup/restore gate 在未修改业务代码的 PR 上失败。

## 决策

- 两条核心 workflow 固定 `runs-on: ubuntu-24.04`；
- 安装显式 `postgresql-client-16`，不再安装随发行版漂移的 meta package；
- dependency setup 显式断言 `pg_dump`/`psql` major=16；
- PostgreSQL service 继续使用 `postgres:16-alpine`；
- 不修改 `restore.sh` 去静默过滤新版 dump 语句，因为 DATA-01 证据应使用项目明确支持的 client/server major 组合。

## Acceptance

- [ ] `.github/workflows/ci.yml` 与 `data-durability-container.yml` 使用 `ubuntu-24.04`。
- [ ] 两条 workflow 安装并断言 PostgreSQL client major 16。
- [ ] PR exact-head DATA-01 backup/restore integration 重新通过。
- [ ] locked Rust/WalGit、frontend、recovery/container recreation 门禁无回归。
- [ ] GraphQL/runtime 产品代码零改动。

## 验证

- `ci` exact-head full gate；
- `data-durability-container` exact-head gate；
- job log 必须显示 `pg_dump (PostgreSQL) 16.x` 与 `psql (PostgreSQL) 16.x`。

## 残余

GitHub runner 镜像本身仍会获得 24.04 的安全/patch 更新；major-sensitive external tools 必须继续显式 pin/assert，而不是依赖 `ubuntu-latest`。
