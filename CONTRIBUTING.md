# Contributing to Evolith

感谢你参与 Evolith。

Evolith 是一个以 **GNU Affero General Public License v3.0 only (`AGPL-3.0-only`)** 发布的开源项目。我们欢迎问题报告、设计讨论、文档改进、测试以及代码贡献。

## 开始之前

在投入较大的实现工作前，建议先确认现有 Issue、路线图和项目治理文档，避免重复工作或与当前架构方向冲突。

建议优先阅读：

- [README](./README.md)
- [AGENTS.md](./AGENTS.md)
- [文档地图](./docs/README.md)
- [项目地图](./docs/reference/PROJECT-MAP.md)
- [架构设计](./docs/reference/ARCHITECTURE.md)
- [实施路线图](./docs/roadmap/IMPLEMENTATION-ROADMAP.md)
- [本地开发 SOP](./docs/sop/LOCAL-DEV.md)

如果你计划修改核心架构、安全边界、Git 写入链路、认证授权、Webhook、外部执行或生产配置，请同时遵守仓库中的相关 ADR、SOP 和安全审查要求。

## 如何贡献

### 报告问题

提交 Issue 时，请尽量提供：

- 清晰的问题描述；
- 可复现步骤或最小复现；
- 期望行为与实际行为；
- 相关日志、错误信息或环境信息；
- 如果适用，说明可能涉及的模块或文档。

安全漏洞不要在公开 Issue 中披露敏感利用细节；请先与项目维护者建立私下沟通渠道。

### 提交代码

Pull Request 应尽量保持单一目标，并说明：

- 解决的问题或对应 Issue；
- 设计与实现方式；
- 影响范围和兼容性；
- 已执行的测试与验证；
- 是否涉及迁移、配置、权限、安全或数据模型变化。

如果变更会影响公开 API、架构边界或长期维护方式，应同步更新相应文档。

## 本地验证

后端常用检查：

```bash
cd backend
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
```

前端常用检查：

```bash
cd frontend
bun install --frozen-lockfile
bun run type-check
bun run build
```

根据变更范围，可能还需要执行仓库文档、集成测试或安全审查流程。

## 贡献许可

除非另有明确书面约定，被项目接受并合入 Evolith 的贡献将作为 Evolith 的一部分按照 **`AGPL-3.0-only`** 发布。

提交贡献即表示你确认：

- 你有权提交该贡献；
- 该贡献不是你无权重新发布的第三方代码、文档、资源或其他材料；
- 你同意该贡献在被合入 Evolith 后按照 `AGPL-3.0-only` 提供给下游用户。

贡献者保留其依法拥有的版权。当前项目**不要求 CLA**；如果未来贡献治理发生变化，会在生效前通过仓库文档明确说明，不追溯改变已经依照现有许可接受的贡献。

## 第三方内容

引入第三方代码、资源、模型、数据、生成内容或依赖时，应说明来源和许可证，并确认其条款与 Evolith 的分发方式兼容。

不要直接复制来源不明、许可证不明或与你无权重新许可的内容。

## 维护原则

维护者会根据项目方向、实现质量、兼容性、安全性、测试覆盖、文档完整性和长期维护成本审阅贡献。

被关闭或暂不合并并不意味着贡献本身没有价值；项目可能因为架构时机、范围边界或当前路线图而选择延后。

## License

Evolith is licensed under the GNU Affero General Public License v3.0 only. See [`LICENSE`](./LICENSE).
