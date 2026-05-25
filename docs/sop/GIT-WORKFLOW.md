# SOP: Git 工作流

## 触发条件

- 需要提交当前变更。
- 需要整理一个 PR 或发布前变更集。
- 需要处理 agent 产生的文档、代码或脚本修改。

## 前置检查

- [ ] 先运行 `git status --short --branch`，确认分支和变更范围。
- [ ] 区分本次任务改动和用户已有改动，不要回滚无关文件。
- [ ] 如有生成物、构建产物或日志文件，确认是否应纳入版本库。
- [ ] 如修改脚本行为，已更新 `docs/reference/SCRIPTS-RELEASE-NOTES.md`。
- [ ] 如发现新坑或被用户纠正，已更新 `EVOLUTION.md`。

## 提交信息规范

提交信息使用语义前缀：

| 前缀 | 用途 |
|------|------|
| `feat:` | 新功能 |
| `fix:` | 缺陷修复 |
| `docs:` | 文档变更 |
| `refactor:` | 不改变行为的重构 |
| `test:` | 测试新增或调整 |
| `chore:` | 构建、依赖、脚本、维护 |
| `perf:` | 性能优化 |
| `security:` | 安全修复或加固 |

本项目允许中文或英文提交正文。面向项目协作和经验追溯时优先中文，保持语义前缀不变。

Agent 参与生成的提交，提交信息末尾必须注明模型：

```text
docs: 落地工程化文档治理 [model: gpt-5]
fix: 修复 Docker Compose 数据库配置键 [model: gpt-5]
```

## 标准提交流程

```bash
git status --short --branch
git diff -- <path>
git add <path>
git diff --cached
git commit -m "docs: 更新工程化文档治理 [model: gpt-5]"
```

原则：

1. 一次提交只表达一个清晰主题。
2. 提交前看 staged diff，不只看工作区 diff。
3. 不把无关格式化、构建产物、临时日志混进功能提交。
4. 不用 `git add .` 盲加，除非已经确认所有变更都属于本次任务。
5. 不使用 `git reset --hard`、`git checkout -- <file>` 回滚用户改动，除非用户明确要求。

## 变更拆分建议

| 变更类型 | 建议 |
|----------|------|
| 代码 + 测试 | 通常同一提交 |
| 代码 + 文档 | 小功能可同一提交，大功能分为 `feat` 和 `docs` |
| 脚本行为变更 | 脚本和 `docs/reference/SCRIPTS-RELEASE-NOTES.md` 同一提交 |
| 经验写回 | 可随修复提交；纯经验沉淀用 `docs:` |
| 大规模格式化 | 单独提交，避免淹没行为变更 |
| 同一 backlog / roadmap 文件同时承载迭代状态和新需求进入 | 可以同一提交，但提交信息必须覆盖两类主题；若能用 `git add -p` 干净拆分，优先拆分 |

## 混合文档提交规则

`PRODUCT-BACKLOG.md`、`IMPLEMENTATION-ROADMAP.md`、`API-CONTRACT.md` 这类文档经常同时承载状态同步、需求进入和目标契约。提交前按以下规则判断：

1. 如果多个改动服务于同一个用户请求或同一次迭代收尾，可以合并提交。
2. 如果包含互不相关的业务方向，拆成多个提交。
3. 如果同一文件里无法安全拆分 hunk，允许合并，但最终回复必须说明提交包含的主题。
4. 不要为了满足“单主题”而手工回滚同一文件中仍属于当前任务的记录。

## 提交前验证

按风险范围选择，不要求每次全量执行：

```bash
# 后端
cd backend
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace

# 前端
cd frontend
bun run type-check
bun run build
```

最终回复或 PR 描述中说明执行过哪些验证；未执行的验证要说明原因。

## 常见风险

- **只改一侧数据库实现**：数据库相关提交必须说明 SQLite/PostgreSQL 是否都覆盖。
- **脚本改动无发布说明**：脚本参数、默认值、退出码、执行顺序变化必须写 release notes。
- **经验只留在对话中**：非直觉问题必须写入 `EVOLUTION.md`。
- **提交信息缺少模型标识**：Agent 生成提交必须有 `[model: <name>]`。
