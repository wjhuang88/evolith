# SOP: 数据库迁移与双数据库变更

## 触发条件

- 新增或修改数据库表、字段、索引。
- 修改 repository 行为。
- 切换开发/生产数据库类型。

## 双数据库原则

Evolith 当前支持 SQLite 和 PostgreSQL：

- SQLite：开发和测试轻量模式。
- PostgreSQL：生产主路径。

数据库变更必须同时考虑：

1. `backend/migrations/sqlite/`
2. `backend/migrations/postgres/`
3. `Sqlite*Repository`
4. `Pg*Repository`
5. repository 集成测试

## 标准变更流程

1. 更新 domain model。
2. 更新 repository trait。
3. 新增 SQLite migration。
4. 新增 PostgreSQL migration。
5. 更新 SQLite repository。
6. 更新 PostgreSQL repository。
7. 增加或更新 repository tests。
8. 运行验证。

## 验证命令

```bash
cd backend
cargo test -p infra
cargo test --workspace
```

如变更涉及 PostgreSQL 特性，应使用 full 模式或本地 PostgreSQL 额外验证。

## 从 SQLite 切换到 PostgreSQL

```bash
DATABASE__DATABASE_TYPE=postgres
DATABASE__URL=postgres://evolith:dev_password@localhost:5432/evolith
cargo run
```

手工运行 migration：

```bash
cargo sqlx migrate run --source backend/migrations/postgres
```

## 注意事项

1. SQLite 和 PostgreSQL 迁移语法不同，不要机械复制。
2. UUID、JSON、时间戳、布尔值在两端类型不同。
3. 分页、大小写搜索、NULL 排序可能存在行为差异。
4. migration 一旦进入生产应视为不可修改，新增修正 migration。
5. Repository trait 不应泄漏具体数据库类型。
