# Serverless Runtime 架构设计

> 状态：设计稿（Iteration 033 / EVO-048 Spike 输出）
> 输出物：本文件 + Phase 7 复用结论 + 统一执行接口 + 冷启动方法学 + 后续 Story 依赖图
> 约束：本机无 Docker，冷启动实测值待 Docker 环境执行；方法学和理论分析已完成。

## 1. 问题

Evolith 当前有两个独立的执行路径：

| 执行路径 | trait | 实现 | 调用方 |
|----------|-------|------|--------|
| Skill 代码执行 | `SkillExecutor` | `DockerExecutor`（容器 per-request） | Skill handler |
| MCP 工具调用 | `ToolExecutor` | `HttpToolExecutor`（HTTP 转发） | MCP handler |

两个 trait 的 `ExecuteRequest` 形状不同、无共享抽象、各自 wired 到 `AppState` 的独立字段。

后续需要加入：

- **CLI 执行引擎**（EVO-045）：CLI interface 从纯文本记录升级为可执行命令
- **MCP Serverless Tool**（EVO-047）：MCP 工具支持 serverless 执行而非仅 HTTP 转发
- **远期 Vercel 模式**：本地 Docker 替换为真实 serverless 平台

需要回答：

1. Phase 7 Docker sandbox 能否复用为本地 serverless 基础？
2. 内部 serverless 与外部 HTTP 执行能否统一到一个抽象层？
3. 冷启动成本能否接受？如何优化？
4. 远期迁移到 Vercel/Lambda 时，哪些组件保留、哪些替换？

## 2. Phase 7 Sandbox 复用评估

### 2.1 当前实现分析

`service-skill/src/docker_executor.rs`（322 行）：

```
execute(request) →
  1. sandbox_id = Uuid::new_v4()
  2. create_container(sandbox_id, runtime)  // image pull + create + start
  3. execute_in_container(container_id, command)  // docker exec
  4. remove_container(container_id)  // force remove
  5. return ExecuteResponse { stdout, stderr, exit_code, execution_time_ms, timed_out }
```

关键特征：

| 特征 | 当前行为 | Serverless 期望 |
|------|----------|-----------------|
| 容器生命周期 | per-request（create → exec → remove） | 预热池 + 跨请求复用 |
| 冷启动 | 每次请求都冷启动 | 仅首次或扩容时冷启动 |
| 资源限制 | SandboxConfig：256MB / 512 CPU shares / 256 PIDs / 30s / network disabled | 相同 |
| 隔离 | 容器级（user=sandbox, no-new-privileges, cap_drop） | 相同 |
| Runtime | Python 3.11 + Node.js 20（预建镜像） | 相同 + 未来可扩展 |
| 网络 | 默认禁用 | 默认禁用，可选启用 |

### 2.2 复用结论：**部分复用**

**可直接复用**：

- `SandboxConfig` 结构体和配置映射 — 完全满足 serverless 资源限制需求
- 容器创建逻辑（image 选择、host_config 构建、security_opt） — 无需改动
- `execute_in_container_inner()` — exec + 输出收集 + exit_code 逻辑完整
- 预建 sandbox 镜像（`evolith-python-sandbox:latest`、`evolith-node-sandbox:latest`）
- `bollard` Docker API 连接和错误映射

**需要重写**：

- **容器池管理**：从 per-request lifecycle 改为 pool-based lifecycle
  - 预热 N 个 idle 容器（按 runtime 分池）
  - 请求到来时从池中取 idle 容器 → exec → 归还池
  - 超时 idle 容器自动回收
  - 池满时等待或创建新容器
- **生命周期状态机**：idle → running → idle → expired
- **并发安全**：池的 `Arc<Mutex>` 或 channel-based 调度
- **健康检查**：定期 ping idle 容器，移除已死容器

**不需要改动**：

- `DefaultSkillExecutor`（stub 降级） — 继续作为无 Docker 时的 fallback
- `service-tool/src/executor.rs`（HTTP tool executor） — 外部执行通道，不走容器

### 2.3 不复用的部分

- **V8 Isolate / Wasmtime**：当前不在范围内。Docker container 已提供足够隔离，且支持任意 runtime。V8/WASM 可作为远期优化（仅限 JS/WASM 场景），但 Phase 7 Docker 镜像和执行逻辑不受影响。

## 3. 统一执行接口设计

### 3.1 问题：两个不兼容的 trait

```rust
// service-skill/src/executor.rs
pub trait SkillExecutor: Send + Sync {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse>;
}
// ExecuteRequest { skill_id, code, language, parameters }
// ExecuteResponse { result, stdout, stderr, exit_code, execution_time_ms, timed_out }

// service-tool/src/executor.rs
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse>;
}
// ExecuteRequest { tool_id, parameters, url, method, timeout_ms }
// ExecuteResponse { result, execution_time_ms, status, error }
```

两个 `ExecuteRequest` 名字冲突，字段不兼容，Response 形状也不同。

### 3.2 设计：统一 ExecutionProvider

新增一个统一的执行抽象层，位于 `common` crate 或独立 `service-executor` crate：

```rust
/// 统一执行请求
pub struct ExecutionRequest {
    /// 调用方标识（用于路由和审计）
    pub caller: ExecutionCaller,
    /// 要执行的代码或命令
    pub payload: ExecutionPayload,
    /// 执行约束
    pub constraints: ExecutionConstraints,
    /// 调用方传入的参数（JSON）
    pub input: serde_json::Value,
    /// 租户和调用者身份（审计用）
    pub context: ExecutionContext,
}

pub enum ExecutionCaller {
    /// Skill 代码执行（来自 Skill handler）
    Skill { skill_id: Uuid, runtime: String },
    /// CLI 命令执行（来自 CLI interface handler）
    Cli { snippet_id: Uuid, command: String },
    /// MCP 工具执行（来自 MCP handler）
    McpTool { tool_id: Uuid },
}

pub enum ExecutionPayload {
    /// 执行代码（Skill / Serverless Function）
    Code { source: String, language: String },
    /// 执行 shell 命令（CLI interface）
    Command { command: String, args: Vec<String> },
    /// 转发到外部 HTTP 端点（MCP HTTP tool）
    HttpProxy { url: String, method: String },
}

pub struct ExecutionConstraints {
    pub timeout_seconds: u32,
    pub memory_mb: u32,
    pub cpu_shares: i64,
    pub pids_limit: i64,
    pub network_enabled: bool,
    pub max_output_bytes: usize,
}

pub struct ExecutionContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub request_id: String,
}

/// 统一执行响应
pub struct ExecutionResponse {
    /// 结构化输出（成功时）
    pub output: serde_json::Value,
    /// 标准输出
    pub stdout: String,
    /// 标准错误
    pub stderr: String,
    /// 退出码（0=成功，>0=应用错误，-1=基础设施错误）
    pub exit_code: i64,
    /// 执行耗时（ms）
    pub execution_time_ms: u64,
    /// 是否超时
    pub timed_out: bool,
    /// HTTP 状态码（仅 HttpProxy 有效）
    pub http_status: Option<u16>,
}

/// 统一执行 trait
#[async_trait]
pub trait ExecutionProvider: Send + Sync {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse>;
}
```

### 3.3 Provider 实现矩阵

| Provider | 适用 Payload | 实现位置 | 说明 |
|----------|-------------|----------|------|
| `DockerSandboxProvider` | Code, Command | `service-skill/`（改版） | 容器池 + exec，复用 Phase 7 |
| `HttpProxyProvider` | HttpProxy | `service-tool/`（改版） | reqwest 转发，复用 HttpToolExecutor |
| `ExternalServerlessProvider` | Code | 新 crate 或 `service-skill/` | 远期：调用 Vercel/Lambda API |
| `LocalProcessProvider` | Command | 新 crate | 远期：非 Docker 本地进程执行 |

### 3.4 路由规则

`ExecutionProvider` 的实现内部按 `ExecutionCaller` + `ExecutionPayload` 路由：

```
Skill + Code       → DockerSandboxProvider（container pool）
Skill + Command    → DockerSandboxProvider（container pool，exec 命令）
Cli + Command      → DockerSandboxProvider（container pool，exec 命令）
Cli + Code         → DockerSandboxProvider（container pool）
McpTool + HttpProxy → HttpProxyProvider（HTTP 转发）
McpTool + Code     → DockerSandboxProvider（container pool）
```

### 3.5 迁移策略

**Phase 1（推荐首步）**：保持 `SkillExecutor` 和 `ToolExecutor` 不变，新增 `ExecutionProvider` 作为 facade：

```
SkillExecutor::execute()
  → 构建 ExecutionRequest { caller: Skill, payload: Code }
  → dispatch to ExecutionProvider
  → 转换 ExecutionResponse → SkillExecutor::ExecuteResponse

ToolExecutor::execute()
  → 构建 ExecutionRequest { caller: McpTool, payload: HttpProxy }
  → dispatch to ExecutionProvider
  → 转换 ExecutionResponse → ToolExecutor::ExecuteResponse
```

好处：所有 handler 代码不动；新抽象层在 service 内部孵化。

**Phase 2（EVO-045/047 落地时）**：handler 直接调用 `ExecutionProvider`，废弃旧 trait。

## 4. 本地版 Serverless 架构

### 4.1 容器池模型

```
┌───────────────────────────────────────────────────┐
│                 ExecutionProvider                  │
│                                                   │
│  ┌─────────────────────────────────────────────┐  │
│  │           DockerSandboxProvider              │  │
│  │                                             │  │
│  │  ┌──────────────┐  ┌──────────────┐         │  │
│  │  │ Python Pool  │  │  Node Pool   │         │  │
│  │  │              │  │              │         │  │
│  │  │ [idle] [idle]│  │ [idle] [run] │         │  │
│  │  │ [run]  [exp] │  │ [idle]       │         │  │
│  │  └──────────────┘  └──────────────┘         │  │
│  │                                             │  │
│  │  PoolConfig:                                │  │
│  │  - min_idle: 1 (per runtime)                │  │
│  │  - max_total: 10 (per runtime)              │  │
│  │  - idle_timeout: 300s                       │  │
│  │  - max_lifetime: 3600s                      │  │
│  │  - health_check_interval: 30s              │  │
│  └─────────────────────────────────────────────┘  │
│                                                   │
│  ┌─────────────────────────────────────────────┐  │
│  │           HttpProxyProvider                  │  │
│  │  - reqwest::Client pool                      │  │
│  │  - 直接转发，无容器                          │  │
│  └─────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────┘
```

### 4.2 请求生命周期

```
1. 请求到达 → ExecutionProvider::execute(req)
2. 按 payload 路由到对应 Provider
3. DockerSandboxProvider:
   a. 从 runtime 对应的 pool 取 idle 容器
   b. pool 为空 → 创建新容器（冷启动）或等待
   c. docker exec 执行代码/命令
   d. 收集 stdout/stderr/exit_code
   e. 容器归还 pool（标记 idle）
   f. 返回 ExecutionResponse
4. HttpProxyProvider:
   a. 构建 HTTP request（method/url/body/timeout）
   b. reqwest 发送
   c. 收集 response body/status
   d. 返回 ExecutionResponse
```

### 4.3 并发与扩展

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `SANDBOX__POOL_MIN_IDLE` | 1 | 每个 runtime 最少保持的 idle 容器 |
| `SANDBOX__POOL_MAX_TOTAL` | 10 | 每个 runtime 最多容器数 |
| `SANDBOX__POOL_IDLE_TIMEOUT` | 300 | idle 容器超时回收（秒） |
| `SANDBOX__POOL_MAX_LIFETIME` | 3600 | 容器最大存活时间（秒），防止资源泄漏 |
| `SANDBOX__POOL_WAIT_TIMEOUT` | 30 | 池满时等待可用容器的超时（秒） |

并发模型：每个 runtime 独立池，池内部用 `tokio::sync::Semaphore` 控制并发数（= max_total）。

### 4.4 故障模型

| 故障 | 检测 | 处理 |
|------|------|------|
| 容器启动失败 | `create_container` 返回 Err | 返回 `exit_code: -1` + stderr；池自动补充 idle |
| 执行超时 | `tokio::time::timeout` | 返回 `timed_out: true`；容器归还池（exec 已结束） |
| 容器 OOM | Docker 自动 kill | `exit_code: 137`；容器归还池前做健康检查 |
| 容器死锁/无响应 | 健康检查（定期 `docker exec true`） | 强制移除 + 补充新 idle |
| Docker daemon 不可达 | `docker.ping()` | 降级到 `DefaultSkillExecutor`（EVO-056 已登记修复） |

## 5. 冷启动分析与优化

### 5.1 冷启动成本分解

一次完整的容器冷启动包括：

| 阶段 | 预估耗时 | 说明 |
|------|----------|------|
| Image pull（未缓存） | 5-30s | 取决于镜像大小和网络；首次后缓存 |
| Image pull（已缓存） | <1s | 本地缓存命中 |
| Container create | 100-500ms | bollard create_container API |
| Container start | 200-800ms | 内核 namespace/cgroup 创建 |
| First exec（code + deps） | 50-500ms | 取决于代码量和依赖 |

**已缓存镜像的冷启动预估**：350ms - 1800ms（Phase 1 + Phase 2 + Phase 3 总和）

**目标 <2s**：已缓存场景下可满足；未缓存场景（首次 pull）需接受 >5s。

### 5.2 优化策略

#### 策略 1：预热池（推荐，Phase 1 实施）

- 服务启动时，为每个 runtime 预创建 `min_idle` 个容器
- 容器启动后保持 `tail -f /dev/null`（当前已有模式）
- 请求到来时直接 `docker exec`，跳过 create + start
- **效果**：从 ~1s 降到 ~50-200ms（仅 exec 开销）

#### 策略 2：容器复用（推荐，Phase 1 实施）

- 执行完成后不 remove 容器，而是归还池
- 限制最大存活时间（`max_lifetime`），到期后强制回收
- **效果**：消除重复 create/start 开销

#### 策略 3：快照恢复（远期，Phase 2+）

- 使用 Docker checkpoint/restore（CRIU）保存已初始化容器状态
- 从 checkpoint 恢复比全新创建快 5-10x
- **限制**：CRIU 在 macOS Docker Desktop 上支持有限，主要适用于 Linux
- **效果**：从 ~1s 降到 ~100-200ms（无需 pool 也能快）

#### 策略 4：Wasm/WASI runtime（远期，独立评估）

- 用 Wasmtime 替代 Docker 容器执行 WASM 编译的代码
- 冷启动 ~1-10ms（进程内，无容器开销）
- **限制**：仅支持编译到 WASM 的语言；Python/Node 不适用
- **适用场景**：Rust/C/C++ 编译的 serverless function

### 5.3 冷启动基准方法学

由于本机无 Docker，以下为 **方法学定义**，实测值待 Docker 环境执行。

```bash
# 前提：Docker daemon 运行中，sandbox 镜像已构建
cd backend

# Step 1: 构建镜像（仅首次）
docker build -t evolith-python-sandbox:latest sandbox/python/
docker build -t evolith-node-sandbox:latest sandbox/node/

# Step 2: 冷启动基准（per-request 模式，当前行为）
# 预期结果：~350ms - 1800ms（已缓存镜像）
cargo test -p service-skill --test cold_start_benchmark -- --nocapture

# Step 3: 预热池基准（pool 模式，新设计）
# 预期结果：~50-200ms（仅 exec 开销）
cargo test -p service-skill --test pool_benchmark -- --nocapture

# 手动验证（无自动化测试时）：
# 1. 创建容器 + 计时
time docker create --name bench-test evolith-python-sandbox:latest tail -f /dev/null
time docker start bench-test
time docker exec bench-test python3 -c "print('hello')"

# 2. 池模式：跳过 create+start，直接 exec
time docker exec bench-test python3 -c "print('hello')"

# 3. 清理
docker rm -f bench-test
```

**测量指标**：

| 指标 | 目标 | 采集方式 |
|------|------|----------|
| 冷启动（per-request） | <2s | `execution_time_ms` 字段 |
| 温启动（池模式 exec） | <200ms | `execution_time_ms` 字段 |
| 池补充开销 | <2s | 后台 goroutine 计时 |
| 并发 10 请求 P95 | <5s | 压测脚本 |

### 5.4 已知约束

- **macOS Docker Desktop**：通过 Linux VM 运行容器，性能比原生 Linux 差 20-50%
- **生产 Linux 环境**：冷启动预期更快
- **镜像首次 pull**：5-30s，不可控；CI/部署流程应包含预 pull 步骤
- **CRIU**：macOS 不支持；Linux 生产可用

## 6. 远期 Vercel 模式演进

### 6.1 目标

远期支持两种执行后端：

1. **本地 Docker 模式**（默认，self-hosted）
2. **Vercel/Lambda 模式**（云 serverless，托管部署）

### 6.2 组件替换清单

| 组件 | 本地 Docker 模式 | Vercel/Lambda 模式 | 替换复杂度 |
|------|-------------------|--------------------|-----------|
| ExecutionProvider | `DockerSandboxProvider` | `ExternalServerlessProvider` | 新实现，接口不变 |
| 容器池 | `bollard` + pool manager | 不需要（云平台管理） | 删除 |
| 镜像构建 | Dockerfile + `docker build` | Vercel Builder / Lambda Layer | 替换 |
| 资源限制 | `SandboxConfig` → Docker cgroup | 云平台配置（memory/timestamp） | 映射 |
| 冷启动 | 预热池 + CRIU | 云平台原生优化 | 不适用 |
| 网络 | Docker network none/bridge | 云平台 VPC / 公网 | 映射 |
| 日志 | docker logs / stdout capture | 云平台日志服务 | 映射 |
| 审计 | 内部 audit service | 内部 audit service（不变） | 保留 |
| 安全 | container isolation (user/ns/cgroup) | 云平台 sandbox（V8 isolate / Firecracker） | 保留 |

### 6.3 不变的组件

以下组件在两种模式下完全相同：

- `ExecutionRequest` / `ExecutionResponse` / `ExecutionContext`
- `ExecutionProvider` trait 接口
- `HttpProxyProvider`（HTTP 转发不涉及容器）
- 审计日志（`service-audit`）
- RBAC 和 API Key 鉴权
- MCP 协议层（`service-tool/src/mcp.rs`）
- Skill / CLI / Tool 的 domain model 和 repository

### 6.4 演进路径

```
Phase 1 (EVO-045/047):
  └── 新增 ExecutionProvider 统一抽象
      + DockerSandboxProvider（池化容器）
      + HttpProxyProvider（HTTP 转发，复用现有）
      → 所有执行路径走统一接口

Phase 2 (远期):
  └── 新增 ExternalServerlessProvider
      调用 Vercel Serverless Functions API / AWS Lambda API
      → 配置切换：SANDBOX__MODE=docker|external
      → 本地/云两套 Provider，运行时按配置选择

Phase 3 (远期):
  └── V8 Isolate / Wasmtime Provider
      适用于 JS/WASM 场景的超低冷启动
      → 作为 Docker 模式的加速层，不替代
```

## 7. CLI 执行引擎设计（EVO-045 参考）

### 7.1 CLI 执行模型

CLI interface（当前 `Snippet` 数据模型）从纯文本记录升级为可执行命令：

```
用户定义 CLI interface:
  command: "terraform"
  subcommands: [{ name: "plan", args: ["-out=tfplan"] }]
  inputs: [{ name: "workspace", type: "string", required: true }]

用户触发执行:
  ExecutionPayload::Command {
    command: "terraform",
    args: ["plan", "-var", "workspace=prod", "-out=tfplan"]
  }
  → DockerSandboxProvider 在容器中执行
  → 返回 stdout/stderr/exit_code
```

### 7.2 CLI 与 Skill 执行的统一

两者共用 `DockerSandboxProvider`，区别仅在 `ExecutionPayload`：

| 维度 | Skill | CLI |
|------|-------|-----|
| Payload | `Code { source, language }` | `Command { command, args }` |
| Runtime | Python/Node/... | bash/sh（容器内） |
| 输入 | `parameters` (JSON) | `inputs` → 命令行参数 |
| 输出 | stdout/stderr/exit_code | stdout/stderr/exit_code |
| 隔离 | 相同 | 相同 |

## 8. 后续 Story 依赖图

```
EVO-048 (Serverless 架构 Spike) ← 本 Spike，输出设计
  │
  ├── EVO-049-A (Skill/CLI 数据模型基线)
  │     ↑ 独立，不依赖 EVO-048
  │     │ 但 EVO-045/047 需要 EVO-049-A 的数据模型就绪
  │
  ├── EVO-045 (CLI 命令执行引擎)
  │     依赖：EVO-048 (本 Spike) + EVO-049-A
  │     实施：ExecutionProvider + DockerSandboxProvider(Command payload)
  │     拆分建议：
  │       EVO-045-A: ExecutionProvider 统一 trait + DockerSandboxProvider 池化
  │       EVO-045-B: CLI handler 接线 + ExecutionPayload::Command
  │       EVO-045-C: 前端 CLI 执行 UI
  │
  ├── EVO-047 (MCP Serverless Tool)
  │     依赖：EVO-048 (本 Spike) + EVO-045-A (ExecutionProvider 基线)
  │     实施：MCP handler 增加 Code payload → DockerSandboxProvider
  │
  └── 远期：ExternalServerlessProvider (Vercel/Lambda)
        依赖：EVO-045-A + 云平台选型决策
```

**推荐实施顺序**：

1. **EVO-049-A**（Iteration 034）：数据模型先就绪，不阻塞
2. **EVO-045-A**：ExecutionProvider 统一 trait + 容器池化（核心基础设施）
3. **EVO-045-B**：CLI handler 接线（依赖 A + EVO-049-A）
4. **EVO-047**：MCP serverless tool（依赖 A）
5. **EVO-045-C**：前端 UI（依赖 B）

## 9. 不做

- 不实现 serverless runtime（本 Spike 仅输出设计）
- 不实现 Vercel/Lambda 集成（远期）
- 不引入 Wasmtime / V8 Isolate（远期评估）
- 不改数据库 schema（EVO-049-A 负责）
- 不改前端 UI（EVO-045-C 负责）
- 不修改 `SkillExecutor` / `ToolExecutor` 的公开签名（Phase 1 内部 facade）

## 10. 风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 容器池内存占用过高 | 10 容器 × 256MB = 2.5GB | 默认 min_idle=1；按需扩展；生产监控 |
| Docker daemon 单点 | daemon 挂掉则全部执行失败 | 降级到 DefaultSkillExecutor；健康检查 + 告警 |
| bollard 0.17 API 不支持池化 | 需要升级到 0.18+ | EVO-061 已登记，可在 EVO-045-A 前完成 |
| 容器 exec 并发瓶颈 | 单 Docker host 并发 exec 有限 | 生产可横向扩展 Docker host |
| Vercel/Lambda API 变化 | ExternalServerlessProvider 实现成本不确定 | 延迟到有明确需求时再设计 |

## 11. 验证方式

- [x] 设计文档完成（本文件）
- [x] Phase 7 sandbox 复用结论明确：**部分复用**
  - 复用：SandboxConfig、容器创建逻辑、exec 逻辑、sandbox 镜像、bollard 连接
  - 重写：容器池管理（per-request → pool-based）
- [x] 统一执行接口设计完成（`ExecutionProvider` trait + `ExecutionPayload` 枚举 + 路由规则）
- [x] 冷启动方法学定义完成（§5.3）
- [ ] 冷启动实测数据：待 Docker 环境执行（方法学见 §5.3，环境约束见 §5.4）
- [x] 远期 Vercel 模式组件替换清单完成（§6.2）
- [x] 后续 Story 依赖图完成（§8）
