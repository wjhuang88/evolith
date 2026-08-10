# EVO-119 全局主题切换保持界面颜色一致

- 类型：bug
- 优先级：P1
- 状态：Done
- 影响范围：frontend

## 用户目标

用户切换到黑夜模式后，应用外壳、导航、卡片、表单和弹层应与内容区域一起切换，避免同一页面出现明暗混杂。

## 验收标准

- Given 用户已进入带主布局的页面
  When 切换到黑夜模式
  Then 页面背景、Header、Sidebar、语义卡片/表单 token 和文字颜色同步切换
- Given 用户刷新页面或切换路由
  When 黑夜模式仍处于选中状态
  Then 全局颜色保持黑夜主题
- Given 切换回浅色模式
  Then 原有浅色 token 恢复

## 实现与验证

- 在 `globals.css` 为 `:root.dark` 覆盖全局语义颜色 token；现有 `bg-background`、`text-foreground` 等组件无需逐页补丁即可响应主题。
- `frontend`: `bun run type-check`、`bun run build`

## 残余归口

带有独立硬编码颜色且没有 dark variant 的历史页面样式，后续随页面改造逐步迁移到语义 token；本 Story 解决全局主题 token 不生效的问题。
