# Skill格式规范

## 1. 概述

Evolith 技能系统兼容 Claude Skills / Agent Skills 开放标准，并扩展支持服务端代码执行、企业内版本管理和多来源导入。

规范参考：https://agentskills.io/specification。后续实现以该规范为兼容基线：`SKILL.md` 必须包含 YAML frontmatter 和 Markdown body；frontmatter 至少包含 `name` 与 `description`；`scripts/`、`references/`、`assets/` 作为渐进披露资源按需加载。

### 1.1 设计目标

- **生态兼容**：与 Claude Skills / Agent Skills 格式兼容
- **功能扩展**：支持服务端代码执行
- **易于编写**：Markdown + YAML，人类可读
- **机器友好**：结构化元数据，LLM易于解析
- **生命周期完整**：支持上传、同步、校验、版本、回滚和导入报告

### 1.2 技能类型

| 类型 | 描述 | 执行方式 | 用途 |
|------|------|----------|------|
| instruction | 纯指令型 | 客户端注入 | 编码规范、工作流程 |
| hybrid | 混合型 | 服务端执行 | 数据处理、API调用 |

## 2. 目录结构

### 2.1 基本结构

```
my-skill/
├── SKILL.md              # 入口文件（必需）
├── scripts/              # 脚本（可选）
│   └── helper.py
├── references/           # 参考文档（可选）
│   ├── REFERENCE.md
│   └── examples.md
├── assets/               # 模板、图片、数据文件（可选）
│   └── output-template.md
└── src/                  # Evolith 扩展：服务端执行代码（混合型必需）
    ├── main.py           # 入口点
    ├── requirements.txt  # 依赖
    └── lib/              # 库文件
```

### 2.2 文件说明

| 文件 | 必需 | 描述 |
|------|------|------|
| SKILL.md | ✅ | 技能定义和指令 |
| scripts/* | 可选 | Agent 可按需运行的自包含脚本 |
| references/* | 可选 | Agent 按需读取的详细参考 |
| assets/* | 可选 | 模板、图片、数据或 schema |
| src/main.py | 混合型必需 | 代码入口点 |
| src/requirements.txt | 推荐 | Python依赖 |
| src/package.json | 推荐 | Node.js依赖 |

> 兼容原则：`scripts/`、`references/`、`assets/` 使用 Agent Skills 语义；`src/` 是 Evolith 服务端执行扩展。导入第三方 Skill 时不得强制要求存在 `src/`。

## 3. SKILL.md格式

### 3.1 完整示例

```markdown
---
# === 基本信息 ===
name: data-analyzer
version: 1.0.0
description: Analyze data files (CSV, JSON, Excel) and generate statistical reports with visualizations. Use when user needs data analysis, statistical summaries, or chart generation.
author: evolith-team
tags: [data, analysis, statistics, visualization]

# === 执行配置 ===
type: hybrid                    # instruction | hybrid
execution: server               # client | server
runtime: python3.11             # python3.11 | node20 | wasm
entrypoint: src/main.py         # 代码入口点
timeout: 60                     # 超时秒数
memory: 512                     # 内存限制(MB)

# === 依赖声明 ===
dependencies:
  - pandas>=2.0.0
  - numpy>=1.24.0
  - matplotlib>=3.7.0
  - openpyxl>=3.1.0

# === 权限声明 ===
permissions:
  filesystem: read              # none | read | read-write
  network: none                 # none | outbound | full
  environment: []               # 允许的环境变量

# === 调用控制 ===
disable-model-invocation: false
user-invocable: true
argument-hint: <file_path> [--format json|html]

# === 支持文件 ===
examples:
  - examples/basic.md
  - examples/advanced.md
templates:
  - templates/report.md
---

# Data Analyzer Skill

A powerful skill for analyzing data files and generating comprehensive reports.

## Capabilities

- **Data Loading**: Supports CSV, JSON, Excel formats
- **Statistical Analysis**: Mean, median, std dev, correlations
- **Visualization**: Charts and graphs generation
- **Report Generation**: Formatted output with insights

## Usage

### Basic Analysis

```
/data-analyzer /path/to/data.csv
```

### Advanced Options

```
/data-analyzer /path/to/data.xlsx --format html --include-charts
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| file_path | string | Yes | Path to data file |
| --format | string | No | Output format: json, html, md (default: md) |
| --include-charts | flag | No | Generate visualization charts |
| --output | string | No | Output file path |

## What This Skill Does

1. **Load and Validate Data**
   - Reads the input file
   - Validates data structure
   - Detects column types

2. **Statistical Analysis**
   - Computes summary statistics
   - Identifies correlations
   - Detects anomalies

3. **Visualization** (if requested)
   - Distribution charts
   - Correlation heatmap
   - Time series plots

4. **Report Generation**
   - Formats results
   - Includes insights
   - Saves output file

## Examples

### Example 1: Basic CSV Analysis

```markdown
@examples/basic.md
```

### Example 2: Advanced Excel Analysis

```markdown
@examples/advanced.md
```

## Output Template

The report follows this structure:

```markdown
@templates/report.md
```

## Error Handling

If analysis fails, the skill will:
1. Identify the error type
2. Provide actionable suggestions
3. Log details for debugging

Common errors:
- **File not found**: Check the file path
- **Invalid format**: Ensure file is CSV, JSON, or Excel
- **Memory limit**: Try with a smaller dataset
```

### 3.2 Frontmatter字段参考

#### 基本信息字段

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| name | string | ✅ | 技能名称，用于 `/name` 调用；必须与目录名一致 |
| description | string | ✅ | 功能描述和触发条件 |
| version | string | Evolith 推荐 | 语义化版本号；缺失时导入流程可生成初始版本 |
| author | string | 推荐 | 作者标识 |
| tags | string[] | 推荐 | 标签，用于搜索和分类 |
| license | string | 可选 | 许可证名称或随包许可证文件 |
| compatibility | string | 可选 | 目标 Agent、系统依赖、网络访问等兼容性说明 |
| metadata | map | 可选 | 额外元数据，建议使用命名空间化 key |
| allowed-tools | string | 可选 | 预批准工具声明；实验字段，兼容处理 |

#### Agent Skills 兼容校验

| 校验项 | 等级 | 规则 |
|--------|------|------|
| `SKILL.md` 存在 | error | Skill 目录根部必须存在 `SKILL.md` |
| frontmatter | error | `SKILL.md` 必须以 YAML frontmatter 开始 |
| `name` | error | 1-64 字符，只能包含小写字母、数字和连字符；不能以连字符开头或结尾；不能包含连续连字符；必须匹配父目录名 |
| `description` | error | 1-1024 字符，不能为空 |
| `description` 质量 | warning | 应同时说明“做什么”和“何时使用”，并包含有助于 Agent 发现的关键词 |
| `compatibility` | warning | 如存在，建议不超过 500 字符，并只描述真实环境要求 |
| 文件引用 | warning | `SKILL.md` 中引用的文件应使用相对路径，避免深层引用链 |
| 主文档长度 | warning | `SKILL.md` 建议保持在 500 行以内，长参考材料放入 `references/` |

#### 执行配置字段

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| type | string | instruction | instruction 或 hybrid |
| execution | string | client | client 或 server |
| runtime | string | - | python3.11, node20, wasm |
| entrypoint | string | - | 代码入口点路径 |
| timeout | number | 30 | 执行超时(秒) |
| memory | number | 256 | 内存限制(MB) |

#### 依赖和权限字段

| 字段 | 类型 | 描述 |
|------|------|------|
| dependencies | string[] | 运行时依赖包 |
| permissions.filesystem | string | 文件系统权限 |
| permissions.network | string | 网络权限 |
| permissions.environment | string[] | 允许的环境变量 |

#### 调用控制字段

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| disable-model-invocation | boolean | false | 禁止LLM自动调用 |
| user-invocable | boolean | true | 用户可手动调用 |
| argument-hint | string | - | 参数提示 |

## 4. 代码规范

### 4.1 Python技能

```python
# src/main.py
import json
import sys
from typing import Any

def main(arguments: dict[str, Any]) -> dict[str, Any]:
    """
    技能入口函数
    
    Args:
        arguments: 调用参数，包含用户传入的参数
        
    Returns:
        结果字典，必须包含:
        - output: str, 主要输出内容
        - files: list[dict], 生成的文件列表（可选）
        - logs: list[str], 执行日志（可选）
    """
    file_path = arguments.get("file_path")
    format_type = arguments.get("format", "md")
    
    # 1. 验证输入
    if not file_path:
        return {
            "output": "Error: file_path is required",
            "success": False
        }
    
    # 2. 执行逻辑
    try:
        data = load_data(file_path)
        analysis = analyze(data)
        output = format_output(analysis, format_type)
        
        return {
            "output": output,
            "files": [],
            "logs": ["Loaded data", "Analysis complete"],
            "success": True
        }
    except Exception as e:
        return {
            "output": f"Error: {str(e)}",
            "success": False
        }

if __name__ == "__main__":
    # 从stdin读取参数
    args = json.load(sys.stdin)
    result = main(args)
    print(json.dumps(result))
```

### 4.2 Node.js技能

```javascript
// src/main.js
/**
 * 技能入口函数
 * @param {Object} arguments - 调用参数
 * @returns {Promise<Object>} 结果对象
 */
async function main(arguments) {
  const { file_path, format = 'md' } = arguments;

  // 1. 验证输入
  if (!file_path) {
    return {
      output: 'Error: file_path is required',
      success: false
    };
  }

  try {
    // 2. 执行逻辑
    const data = await loadData(file_path);
    const analysis = analyze(data);
    const output = formatOutput(analysis, format);

    return {
      output,
      files: [],
      logs: ['Loaded data', 'Analysis complete'],
      success: true
    };
  } catch (error) {
    return {
      output: `Error: ${error.message}`,
      success: false
    };
  }
}

// 导出函数
module.exports = { main };

// CLI入口
if (require.main === module) {
  const args = JSON.parse(process.argv[2] || '{}');
  main(args).then(result => {
    console.log(JSON.stringify(result));
  });
}
```

### 4.3 返回值规范

```typescript
interface SkillResult {
  // 必需
  output: string;           // 主要输出内容
  
  // 可选
  success?: boolean;        // 执行是否成功，默认true
  files?: FileResult[];     // 生成的文件
  logs?: string[];          // 执行日志
  metrics?: {               // 执行指标
    rows_processed?: number;
    time_taken?: number;
  };
}

interface FileResult {
  name: string;             // 文件名
  content?: string;         // 文件内容（文本）
  url?: string;             // 文件URL（二进制）
  size?: number;            // 文件大小
  mime_type?: string;       // MIME类型
}
```

## 5. 支持文件

### 5.1 示例文件

```markdown
<!-- examples/basic.md -->
## Basic CSV Analysis Example

### Input
```bash
/data-analyzer sales.csv
```

### Output
```
# Sales Data Analysis Report

## Summary Statistics
- Total Records: 1,234
- Date Range: 2024-01-01 to 2024-12-31
- Total Revenue: $1,234,567

## Key Metrics
| Metric | Value |
|--------|-------|
| Mean Sale | $1,001.23 |
| Median Sale | $850.00 |
| Std Dev | $234.56 |

## Top Products
1. Widget A: $234,567 (19%)
2. Widget B: $198,765 (16%)
3. Widget C: $156,789 (13%)
```
```

### 5.2 模板文件

```markdown
<!-- templates/report.md -->
# {{title}} Analysis Report

Generated: {{timestamp}}

## Overview

{{summary}}

## Statistics

{{statistics_table}}

## Visualizations

{{charts}}

## Insights

{{insights}}

## Recommendations

{{recommendations}}
```

## 6. 创建与导入来源

### 6.1 支持来源

| 来源 | 目标体验 | 关键要求 |
|------|----------|----------|
| Web 手写 | 适合小型 instruction skill | 表单生成规范 `SKILL.md`，保存前执行 frontmatter 和描述质量校验 |
| ZIP 上传 | 适合已有 Skill 包迁移 | 解压后必须识别单个 Skill 根目录；限制大小、路径穿越、重复文件和非法符号链接 |
| Git 仓库 | 适合团队维护和持续同步 | 支持仓库 URL、branch/tag/commit、子目录、凭据引用和导入预览 |
| SkillHub 同步 | 适合生态复用 | 记录 upstream 名称、版本、来源 URL、同步时间和校验报告 |

所有来源必须进入统一导入管线：

1. 拉取或解包到隔离临时目录。
2. 定位 Skill 根目录和 `SKILL.md`。
3. 解析 frontmatter、body、`scripts/`、`references/`、`assets/` 和 Evolith 扩展字段。
4. 运行正确性校验和描述质量检查。
5. 生成导入报告，区分 blocking error 与 warning。
6. 无 blocking error 时创建新版本或草稿；有 blocking error 时不产生可用版本。

### 6.2 导入报告

```typescript
interface SkillImportReport {
  source_type: 'manual' | 'zip' | 'git' | 'skillhub';
  source_ref?: string;
  skill_name?: string;
  version?: string;
  status: 'accepted' | 'accepted_with_warnings' | 'rejected' | 'draft';
  files: string[];
  errors: SkillValidationIssue[];
  warnings: SkillValidationIssue[];
}

interface SkillValidationIssue {
  code: string;
  message: string;
  path?: string;
  line?: number;
}
```

## 7. 打包格式

### 7.1 技能包结构

```bash
# 创建技能包
skill-package/
├── SKILL.md
├── references/
├── assets/
├── scripts/
└── src/
    ├── main.py
    ├── requirements.txt
    └── lib/

# 打包
zip -r data-analyzer.zip skill-package/
```

### 7.2 上传要求

| 要求 | 描述 |
|------|------|
| 格式 | ZIP压缩包 |
| 大小 | 最大 50MB |
| 入口 | 必须包含SKILL.md |
| 编码 | UTF-8 |
| 安全 | 禁止路径穿越、绝对路径、危险符号链接和解压后超限 |

## 8. 版本管理

### 8.1 语义化版本

```
MAJOR.MINOR.PATCH

- MAJOR: 不兼容的API变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修复
```

版本规则：

- 同一租户下 `name + version + source` 不可重复。
- 可以设置默认版本，Agent 加载未指定版本时使用默认版本。
- 回滚不删除历史版本，而是把默认版本指针切回旧版本。
- 从 Git 或 SkillHub 同步时保留 upstream version；如果 upstream 无版本，使用导入时间或 commit 生成内部版本。

### 8.2 变更日志

在SKILL.md中添加变更日志部分：

```markdown
## Changelog

### 1.1.0 (2024-02-15)
- Added support for Parquet files
- Improved memory efficiency

### 1.0.0 (2024-01-01)
- Initial release
```

## 9. 最佳实践

### 9.1 描述编写

好的描述应该：
- 清晰说明功能
- 包含触发关键词
- 说明适用场景
- 避免泛泛而谈，能帮助 Agent 判断何时激活该 Skill

```yaml
# 好
description: Analyze CSV/JSON data files and generate statistical reports with charts. Use when the user asks for data analysis, statistical summaries, anomaly detection, or visualization.

# 差
description: A data tool.
```

### 9.2 错误处理

```python
def main(arguments):
    try:
        # 业务逻辑
        pass
    except FileNotFoundError:
        return {
            "output": "Error: File not found. Please check the path.",
            "success": False
        }
    except ValueError as e:
        return {
            "output": f"Error: Invalid data format - {str(e)}",
            "success": False
        }
    except Exception as e:
        return {
            "output": f"Unexpected error: {str(e)}",
            "success": False
        }
```

### 9.3 资源管理

```python
import tempfile
import os

def main(arguments):
    # 使用临时目录
    with tempfile.TemporaryDirectory() as tmpdir:
        # 处理文件
        output_path = os.path.join(tmpdir, "output.txt")
        # ...
    
    # 临时目录自动清理
    return {"output": "Done"}
```
