# SOP: 经验写回与规则升级

> 本 SOP 定义什么时候更新 `EVOLUTION.md`，以及什么时候把经验升级为
> `AGENTS.md` 规则、SOP、validator 或测试。`EVOLUTION.md` 是可复用教训索引，
> 不是普通变更日志。

## 触发条件

- 用户指出遗漏、偏离、误解或错误结论。
- 失败后找到根因，或多次尝试后形成可复用处理方式。
- 发现新的项目陷阱、环境约束、配置误用或流程漂移。
- 验证、发布、提交或文档治理过程中暴露出规则缺口。
- 某条经验如果不写回，后续 Agent 很可能重复犯错。

不满足上述条件时，不写 `EVOLUTION.md`。普通功能完成、文档同步和状态更新不作为经验写回。

## 写入格式

在 `EVOLUTION.md` 追加紧凑条目：

```markdown
## YYYY-MM-DD - 简短标题

- Trigger:
- Symptom:
- Root cause:
- Fix:
- Prevention:
- Promoted to rule/check:
```

`Promoted to rule/check` 必须写明 `AGENTS.md`、某个 SOP、validator、测试，或 `none`。

## 操作步骤

1. 判断触发条件是否成立；不成立则不写。
2. 查阅 `EVOLUTION.md` 顶部速查区，避免重复记录同一经验。
3. 用上方格式追加新条目，写清触发、症状、根因、修复和预防。
4. 判断是否需要升级：
   - 会影响所有 Agent 启动行为的，更新 `AGENTS.md`。
   - 会影响固定流程的，更新对应 `docs/sop/`。
   - 可自动检查的，加入 validator 或测试。
   - 只是一次性背景的，`Promoted to rule/check` 写 `none`。
5. 若升级了规则、SOP 或测试，同步相关 Task Router、文档地图或验证记录。

## 验证

- `EVOLUTION.md` 条目包含 6 个字段。
- 新规则如果被升级，能从 `AGENTS.md` Task Router 或对应 SOP 入口找到。
- 文档链接检查和治理 validator 通过。

## 失败处理

- 写成流水账：改为根因和预防导向，删除普通变更描述。
- 只写经验不升级规则：若该经验会反复影响执行，补 `AGENTS.md`、SOP、validator 或测试。
- 重复记录：保留最新更准确条目，旧条目补交叉引用或合并。

## 相关文档

- [EVOLUTION.md](../../EVOLUTION.md)
- [任务收口与完成声明](TASK-CLOSURE.md)
- [文档一致性检查](DOC-CHECK.md)
