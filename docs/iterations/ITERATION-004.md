# Iteration 004: Vite + Bun 构建骨架

> 时间：2026-05-17 起
> 目标：完成 EVO-022，建立 Vite + Bun 构建入口和 React Router SPA 路由，与现有 Next 构建并行运行。

## 1. 本轮目标

- 在 `frontend/` 中新增 Vite 构建配置和 SPA 入口。
- 使用 React Router v6 建立完整路由树，复用 Iteration 003 的路由适配层。
- 实现 `dev:spa` / `build:spa` / `preview:spa` 双构建能力。
- 保持现有 Next 构建不受影响。

## 2. 选入故事

| ID | 标题 | 优先级 | 负责人 | 状态 |
|----|------|--------|--------|------|
| EVO-022 | Vite + Bun 构建骨架 | P0 | Agent | Planned |

## 3. 不做事项

- 不迁移 `NEXT_PUBLIC_*` 环境变量；归属 EVO-023。
- 不修改 Docker / Nginx 配置；归属 EVO-024。
- 不创建或恢复 GitHub CI/CD workflow；归属项目后段的 EVO-030。
- 不删除 Next.js 依赖、App Router 或 middleware；归属 EVO-025。
- 不改变 API client 或业务逻辑。
- 不引入 TanStack Query（保持 Zustand + Axios）。

## 4. 验收标准

- [ ] `vite.config.ts` 存在且配置了 React 插件、路径别名（`@/`）、Tailwind。
- [ ] `index.html` SPA 入口可加载。
- [ ] React Router 路由树覆盖当前所有 22 个页面路由。
- [ ] `bun run dev:spa` 启动 Vite dev server，SPA 可访问。
- [ ] `bun run build:spa` 产出 `dist/` 静态文件。
- [ ] 现有 `npm run build` / `npm run type-check` 不受影响。
- [ ] 路由适配层 `router.tsx` 在 Vite 环境使用 React Router 实现。

## 5. 验证计划

```bash
# 保留 Next 构建不破坏
cd frontend
npm run type-check
npm run build

# 新增 Vite 构建可用
bun install
bun run build:spa

# 文档一致性
cd ..
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
| Vite 路径别名与 Next 的 `@/` 冲突 | Vite 配置独立于 `tsconfig.json` paths；两者可并存 |
| React Router 与 Next App Router 路由结构差异 | 本轮只建立 Vite 侧路由树；Next 路由保持不变，双构建并行 |
| Tailwind 配置兼容性 | Vite 使用 PostCSS 插件加载 Tailwind，与现有 `tailwind.config.ts` 兼容 |
| 新增依赖影响现有构建 | 仅在 `devDependencies` 添加 vite 相关包；Next 构建链不引入 vite |

## 7. 执行记录

| 日期 | 记录 |
|------|------|
| 2026-05-17 | Iteration 004 started. 选入 EVO-022，补齐 backlog 详情块，创建迭代文件。 |
| 2026-05-17 | 前置清理：移除 `.github/workflows/ci.yml` 和 `deploy.yml`（项目尚未进入 CI 阶段，后续由 EVO-030 重建）。 |
| 2026-05-17 | 排期澄清：EVO-024 拆为“Docker / Nginx 静态 SPA 部署适配”，GitHub CI/CD 重建独立为 EVO-030，并放到项目后段处理；当前 EVO-022 验收标准不变。 |

## 8. 变更请求

| 日期 | 类型 | 决策 | 影响 | 半成品处理 |
|------|------|------|------|------------|
|  | clarification / scope-change / product-pivot / urgent-fix | 接受 / 拆分 / 暂缓 / 拒绝 |  | 保留 / 移除 / 后续清理 |

## 9. Review

- 完成：
- 未完成：
- 验证结果：

## 10. Retrospective

- 做得好的：
- 需要调整的：
- 写入 EVOLUTION：
