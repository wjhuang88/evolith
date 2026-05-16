# Iteration 002: CLI 友好接口概念迁移

> 时间：2026-05-16 起
> 目标：完成 EVO-017 的第一轮落地，把旧 snippet 主线迁移为 CLI 友好接口的产品、合约和实现基线。

## 1. 本轮目标

- 明确 Evolith 的产品定位为企业级 AI Agent Harness 平台，后续能力围绕治理、审计、权限、复用和企业交付组织。
- 基于 [ADR-0002](../decisions/ADR-0002-cli-friendly-interface-replaces-snippet.md) 明确 `CliInterface` 的领域模型、API 合约和格式文档。
- 盘点当前 snippet 实现面，决定哪些代码保留兼容、哪些进入迁移或废弃。
- 形成可继续开发的最小实现切片，避免在旧 snippet 概念上继续扩张。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-017 | Snippet 迁移为 CLI 友好接口 | P0 | Agent | In Progress |

## 3. 不做事项

- 不启动 React + Vite + Bun 前端迁移；该工作仍归属 EVO-002。
- 不启动 Rust CLI 子项目；该工作仍归属 EVO-015。
- 不做前端嵌入后端发布物；该工作仍归属 EVO-016。
- 不一次性删除所有 snippet 兼容实现；先完成概念和接口迁移边界。

## 4. 验收标准

- [ ] 新增或更新 CLI 友好接口格式文档，说明 frontmatter/schema、usage、examples、error model。
- [ ] 更新 API 合约，明确 `CliInterface` 对外字段、创建/读取/更新/删除或迁移兼容策略。
- [ ] README、roadmap 或相关入口明确企业级 AI Agent Harness 平台定位。
- [ ] 盘点并记录 snippet 相关后端、前端、数据库和文档入口的处理方式：保留兼容 / 重命名 / 废弃 / 后续迁移。
- [ ] 实现最小代码改动，避免新功能继续依赖旧 snippet 命名作为产品主线。
- [ ] 更新 backlog、roadmap 或 ADR 链接，确保旧 snippet 故事替代关系清晰。
- [ ] EVO-002 Next.js 去除保持 P0 高优先级，并明确为 EVO-017 后优先启动的工程门禁。

## 5. 验证计划

```bash
# docs
python3 - <<'PY'
from pathlib import Path
import re
missing=[]
for path in list(Path('docs').rglob('*.md'))+[Path('README.md'),Path('AGENTS.md'),Path('EVOLUTION.md')]:
    if not path.exists():
        continue
    text=path.read_text(encoding='utf-8')
    for m in re.finditer(r'\[[^\]]+\]\(([^)]+\.md)(?:#[^)]+)?\)', text):
        target=m.group(1)
        if '://' in target:
            continue
        p=(path.parent/target).resolve()
        if not p.exists():
            missing.append((str(path),target))
if missing:
    print('MISSING LINKS:')
    for src,target in missing:
        print(f'{src} -> {target}')
    raise SystemExit(1)
print('all markdown links exist')
PY

# backend, when code changes
cargo test --workspace

# frontend, when frontend changes
cd frontend
npm run build
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| 旧 snippet 实现面较大，重命名可能扩大范围 | 先做兼容边界和合约迁移，避免一次性大规模重命名 |
| 数据库表名和 API 路由可能影响现有调用 | 先记录兼容策略；需要破坏性变更时补 migration 和 release note |
| CLI interface 与未来 Rust CLI 产生格式分歧 | 本轮格式文档必须引用 EVO-015，并保留 push/pull/sync 扩展点 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-16 | Iteration 002 started. 选入 EVO-017；目标是先完成 CLI 友好接口概念迁移的合约、格式和最小实现边界。 |
| 2026-05-16 | Change request received: 用户确认项目定位为企业级 harness 平台，并要求 Next.js 去除工作提到高优先级。按 scope-change 处理：当前故事继续，补充定位和优先级文档，不切换到 EVO-002。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
| 2026-05-16 | scope-change | 接受 | 本轮补充企业级 AI Agent Harness 平台定位；EVO-002 标为 EVO-017 后优先启动的 P0 工程门禁 | 当前仅文档初始化，无需移除半成品 |

## 9. Review

- 完成：
- 未完成：
- 验证结果：

## 10. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
