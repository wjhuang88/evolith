# EVO-037 治理 skill 弱模型闭环执行防呆

> Archived backlog item. Routing index: [2026 Q2 Archive](INDEX.md). Active routing surface: [Product Backlog](../../PRODUCT-BACKLOG.md).

- Type: tech-debt
- Status: Done
- Priority: P1
- Source: 用户反馈 2026-05-27
- Decision Context: Iteration 014；为初始化、迁移和修复任务增加强制闭环协议

#### Source Detail Snapshot

- 类型：tech-debt
- 优先级：P1
- 状态：Done
- 用户价值或技术目标：让能力较弱的模型使用治理 skill 时也必须完成状态同步、验证、
  残余登记和可继续操作说明，减少“文件生成了但问题没有闭环”的交付。
- 范围：
  - 在 `agent-project-governance` 的主工作流中加入不可跳过的闭环契约和完成判定。
  - 提供低自由度的执行清单、闭环台账与最终输出模板，区分完成、部分完成和受阻。
  - 增加针对只创建文件却漏掉 manifest/状态/验证/残余登记的评估场景。
  - 将本次发现的过程教训写回 Evolith 的经验记录。
- 不做：
  - 不试图通过文档保证所有低能力模型都能完成复杂代码实现。
  - 不改业务代码、部署形态或尚未启动的产品故事。
  - 不提交 skill 目录已有未提交修改。
- 验收标准：
  - [x] skill 入口明确要求实施任务按“建账、执行、核验、同步、交付”完成闭环。
  - [x] 专门参考文档给出可机械执行的闭环协议和部分完成/阻塞输出要求。
  - [x] 评估用例可识别生成骨架后过早宣布完成的行为。
  - [x] 项目文档检查、`git diff --check` 和 skill 结构校验通过。
- 依赖或阻塞：skill 仓库已有未提交治理更新，必须基于现状增量写入，不覆盖无关文件。
- 影响范围：docs / external skill
- 最小验证方式：执行 Markdown 链接检查；`git diff --check`；运行 skill 的
  `quick_validate.py`。
