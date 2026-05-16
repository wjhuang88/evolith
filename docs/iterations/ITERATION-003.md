# Iteration 003: 前端路由适配层

> 时间：2026-05-16 起
> 目标：完成 EVO-021，为 Next.js 去除建立路由适配边界，降低后续 Vite + React Router 迁移风险。

## 1. 本轮目标

- 将页面和共享组件从 `next/link` / `next/navigation` 直连中解耦。
- 新增统一路由适配层，当前仍委托 Next，后续可替换为 React Router。
- 保持当前 Next 构建和类型检查可用。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-021 | 前端路由适配层 | P0 | Agent | Done |

## 3. 不做事项

- 不新增 Vite 构建入口；归属 EVO-022。
- 不迁移 `NEXT_PUBLIC_*` 环境变量；归属 EVO-023。
- 不修改 Docker / Nginx；归属 EVO-024。
- 不创建或恢复 GitHub CI/CD workflow；归属 EVO-030。
- 不删除 Next.js 依赖、middleware 或 App Router 文件；归属 EVO-025。

## 4. 验收标准

- [x] 新增前端路由适配层，封装 `Link`、`useRouter`、`usePathname`、`useParams`、`useSearchParams`。
- [x] 业务页面和共享组件不再直接导入 `next/link` 或 `next/navigation`。
- [x] 当前 Next 阶段 `npm run type-check` 通过。
- [x] 记录 Navigator 审查结论。

## 5. 验证计划

```bash
cd frontend
npm run type-check

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

git diff --check
```

## 6. 风险与回滚

| 风险 | 处理 |
|------|------|
| 适配层类型和 Next 现有类型不一致 | 先用 Next 原始 hook 类型透传，避免提前抽象过深 |
| 大量机械替换漏掉个别 import | 用 `rg "next/link|next/navigation"` 检查 |
| 后续 React Router 替换时行为差异 | 本轮只建立边界；后续 EVO-022/EVO-025 再替换实现 |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-16 | Iteration 003 started. EVO-002 被拆分为 EVO-021 至 EVO-025；本轮只处理路由适配层。启用分阶段结对流程。 |
| 2026-05-16 | Driver: 新增 `frontend/src/lib/router.tsx`，并把业务页面、AuthGuard、AuthInitializer、Header、LayoutWrapper 的 `next/link` / `next/navigation` 导入改为内部适配层。 |
| 2026-05-16 | Navigator check: no blocking findings. Residual risk: 适配层当前仍委托 Next；后续 EVO-022/EVO-025 替换 React Router 实现时需要验证 hook 语义。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  | clarification / scope-change / product-pivot / urgent-fix | 接受 / 拆分 / 暂缓 / 拒绝 |  | 保留 / 移除 / 后续清理 |

## 9. Review

- 完成：
  - 将 EVO-002 拆分为 EVO-021 至 EVO-025。
  - 创建 Iteration 003，选入 EVO-021。
  - 新增 `frontend/src/lib/router.tsx` 路由适配层。
  - 业务页面和共享组件不再直接导入 `next/link` / `next/navigation`。
- 未完成：
  - 未新增 Vite 入口，留给 EVO-022。
  - 未迁移 `NEXT_PUBLIC_*`，留给 EVO-023。
  - 未删除 Next 依赖，留给 EVO-025。
- 验证结果：
  - `npm run type-check`：通过。
  - Markdown 相对链接检查：通过。
  - `git diff --check`：通过。

## 10. Retrospective

- 做得好的：先建立路由边界，没有直接大规模切换构建系统，降低了 Vite 迁移风险。
- 需要调整的：后续 EVO-022 引入 React Router 时，需要重点验证 `useParams`、`useSearchParams` 和 redirect 行为。
- 写入 EVOLUTION：无新增非直觉经验。
