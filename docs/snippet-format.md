# 代码片段格式规范

## 1. 概述

代码片段是面向大模型优化的可复用代码单元，设计目标是：
- **易于发现**：结构化元数据支持搜索
- **易于理解**：清晰的文档和示例
- **易于引用**：生成可直接使用的代码
- **Token高效**：预估消耗，按需加载

## 2. 文件格式

### 2.1 完整示例

```markdown
---
# === 基本信息 ===
id: react-use-debounce
name: useDebounce Hook
description: A React hook for debouncing values. Delays updating a value until a specified time has passed without changes. Useful for search inputs, API calls, and any scenario where you want to delay processing until user input stabilizes.

# === 分类 ===
language: typescript
framework: react
tags: [hooks, debounce, performance, user-input, delay]
category: utilities

# === 依赖 ===
dependencies:
  - name: react
    version: ^18.0.0
    required: true
peer_dependencies:
  - name: react-dom
    version: ^18.0.0

# === 元数据 ===
estimated_tokens: 180
complexity: beginner          # beginner | intermediate | advanced
license: MIT
author: evolith-team
created_at: 2024-01-15
updated_at: 2024-02-20
version: 1.0.0

# === 引用配置 ===
import_path: "@evolith/snippets/react/use-debounce"
export_name: useDebounce
---

# useDebounce Hook

A React hook that delays updating a value until a specified time has passed without changes.

## When to Use

- **Search inputs**: Wait for user to stop typing before searching
- **Auto-save**: Delay saving until changes settle
- **Resize handlers**: Throttle expensive calculations
- **API calls**: Prevent excessive requests

## Installation

```bash
npm install react
# or
yarn add react
# or
pnpm add react
```

## Usage

### Basic Example

```tsx
import { useDebounce } from '@evolith/snippets/react/use-debounce';

function SearchComponent() {
  const [searchTerm, setSearchTerm] = useState('');
  const debouncedSearch = useDebounce(searchTerm, 300);
  
  useEffect(() => {
    if (debouncedSearch) {
      searchAPI(debouncedSearch);
    }
  }, [debouncedSearch]);
  
  return (
    <input
      value={searchTerm}
      onChange={(e) => setSearchTerm(e.target.value)}
      placeholder="Search..."
    />
  );
}
```

### With Options

```tsx
import { useDebounce } from '@evolith/snippets/react/use-debounce';

function AutoSaveComponent() {
  const [content, setContent] = useState('');
  const debouncedContent = useDebounce(content, 1000);
  
  useEffect(() => {
    if (debouncedContent) {
      saveToServer(debouncedContent);
    }
  }, [debouncedContent]);
  
  return <textarea value={content} onChange={(e) => setContent(e.target.value)} />;
}
```

## Code

```tsx
import { useState, useEffect } from 'react';

/**
 * A hook that debounces a value.
 * 
 * @param value - The value to debounce
 * @param delay - Delay in milliseconds
 * @returns The debounced value
 * 
 * @example
 * const debouncedValue = useDebounce(searchTerm, 300);
 */
export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    // Set up the timeout
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    // Clean up the timeout if value or delay changes
    return () => {
      clearTimeout(timer);
    };
  }, [value, delay]);

  return debouncedValue;
}
```

## API Reference

### Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| value | `T` | Yes | - | The value to debounce |
| delay | `number` | Yes | - | Delay time in milliseconds |

### Returns

| Return | Type | Description |
|--------|------|-------------|
| debouncedValue | `T` | The debounced value that updates after the delay |

### Type Parameters

| Parameter | Constraint | Description |
|-----------|------------|-------------|
| T | any | The type of the value being debounced |

## Common Patterns

### Search with Loading State

```tsx
function SearchWithLoading() {
  const [query, setQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState([]);
  const debouncedQuery = useDebounce(query, 300);
  
  useEffect(() => {
    if (debouncedQuery) {
      setLoading(true);
      fetchResults(debouncedQuery)
        .then(setResults)
        .finally(() => setLoading(false));
    }
  }, [debouncedQuery]);
  
  return (
    <div>
      <input value={query} onChange={(e) => setQuery(e.target.value)} />
      {loading && <Spinner />}
      <ResultsList results={results} />
    </div>
  );
}
```

### Multiple Debounced Values

```tsx
function FilterComponent() {
  const [filters, setFilters] = useState({ category: '', price: 0 });
  const debouncedFilters = useDebounce(filters, 500);
  
  useEffect(() => {
    fetchFilteredProducts(debouncedFilters);
  }, [debouncedFilters]);
  
  // ...
}
```

## Performance Notes

- The hook cleans up timers on unmount, preventing memory leaks
- Each keystroke cancels the previous timer and starts a new one
- For high-frequency updates, consider using `useThrottle` instead

## Related Snippets

- [useThrottle](/snippets/react/use-throttle) - Limit update frequency
- [useDebouncedCallback](/snippets/react/use-debounced-callback) - Debounce callback functions
- [useSearch](/snippets/react/use-search) - Complete search solution

## Changelog

### 1.0.0 (2024-01-15)
- Initial release
```

### 2.2 Frontmatter字段

#### 基本信息

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| id | string | ✅ | 唯一标识符 |
| name | string | ✅ | 显示名称 |
| description | string | ✅ | 功能描述，包含触发关键词 |

#### 分类

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| language | string | ✅ | 编程语言 |
| framework | string | 推荐 | 框架名称 |
| tags | string[] | ✅ | 标签列表 |
| category | string | 推荐 | 分类 |

#### 依赖

| 字段 | 类型 | 描述 |
|------|------|------|
| dependencies | Dependency[] | 直接依赖 |
| peer_dependencies | Dependency[] | 对等依赖 |
| dev_dependencies | Dependency[] | 开发依赖 |

#### 依赖对象

```typescript
interface Dependency {
  name: string;      // 包名
  version: string;   // 版本范围
  required: boolean; // 是否必需
}
```

#### 元数据

| 字段 | 类型 | 描述 |
|------|------|------|
| estimated_tokens | number | 预估token消耗 |
| complexity | string | 复杂度级别 |
| license | string | 许可证 |
| author | string | 作者 |
| version | string | 片段版本 |

#### 引用配置

| 字段 | 类型 | 描述 |
|------|------|------|
| import_path | string | 导入路径 |
| export_name | string | 导出名称 |

## 3. 内容结构

### 3.1 必需部分

```markdown
# 片段名称

简短描述（1-2句）

## When to Use

适用场景说明

## Installation

安装依赖的命令

## Usage

基本使用示例

## Code

完整代码实现

## API Reference

参数和返回值说明
```

### 3.2 推荐部分

```markdown
## Common Patterns

常见使用模式

## Performance Notes

性能注意事项

## Related Snippets

相关片段链接

## Changelog

变更日志
```

## 4. 分类体系

### 4.1 语言分类

| 语言 | 标识 |
|------|------|
| TypeScript | typescript |
| JavaScript | javascript |
| Python | python |
| Rust | rust |
| Go | go |
| Java | java |

### 4.2 框架分类

| 框架 | 标识 | 语言 |
|------|------|------|
| React | react | typescript/javascript |
| Next.js | nextjs | typescript/javascript |
| Vue | vue | typescript/javascript |
| Express | express | typescript/javascript |
| FastAPI | fastapi | python |
| Django | django | python |
| Actix | actix | rust |

### 4.3 功能分类

| 分类 | 标识 | 描述 |
|------|------|------|
| Utilities | utilities | 通用工具函数 |
| Hooks | hooks | React Hooks |
| Components | components | UI组件 |
| API | api | API相关 |
| Database | database | 数据库操作 |
| Auth | auth | 认证授权 |
| Validation | validation | 数据验证 |
| Testing | testing | 测试相关 |

## 5. Token预估

### 5.1 计算规则

```
estimated_tokens = 
  code_tokens + 
  (documentation_tokens * 0.5) +  # 文档折半计算
  (example_tokens * 0.3)          # 示例折算
```

### 5.2 Token范围

| 范围 | 复杂度 | 示例 |
|------|--------|------|
| 50-100 | 简单 | 工具函数、类型定义 |
| 100-300 | 中等 | React Hook、小型组件 |
| 300-500 | 复杂 | 完整组件、API客户端 |
| 500+ | 高级 | 复杂业务逻辑 |

## 6. 引用格式

### 6.1 直接引用

```typescript
// 返回完整代码
import { useDebounce } from '@evolith/snippets/react/use-debounce';
```

### 6.2 内联引用

```typescript
// 返回代码片段，直接插入
const debouncedValue = useDebounce(value, 300);

// 实现代码
function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);
  useEffect(() => {
    const timer = setTimeout(() => setDebouncedValue(value), delay);
    return () => clearTimeout(timer);
  }, [value, delay]);
  return debouncedValue;
}
```

### 6.3 带依赖引用

```json
{
  "code": "import { useDebounce } from './useDebounce';\n...",
  "dependencies": [
    { "name": "react", "version": "^18.0.0", "install": "npm install react" }
  ],
  "files": [
    { "path": "hooks/useDebounce.ts", "content": "..." }
  ]
}
```

## 7. 版本管理

### 7.1 版本规则

```
MAJOR.MINOR.PATCH

- MAJOR: 不兼容的API变更
- MINOR: 向后兼容的功能新增  
- PATCH: 向后兼容的问题修复
```

### 7.2 废弃流程

1. 标记为 `deprecated: true`
2. 添加 `deprecated_message` 说明替代方案
3. 保留6个月后删除

```yaml
---
deprecated: true
deprecated_message: "Use useDebouncedValue instead for better performance."
---
```

## 8. 最佳实践

### 8.1 描述编写

```yaml
# 好
description: A React hook for debouncing values. Delays updating until 
  user stops typing. Use for search inputs, auto-save, and throttling 
  API calls.

# 差  
description: Debounce hook.
```

### 8.2 代码注释

```tsx
// 好：注释说明意图
/**
 * Cleans up timer on unmount or when value/delay changes
 * to prevent memory leaks
 */
useEffect(() => {
  return () => clearTimeout(timer);
}, [value, delay]);

// 差：无注释
useEffect(() => {
  return () => clearTimeout(timer);
}, [value, delay]);
```

### 8.3 类型定义

```tsx
// 好：完整的类型定义
export function useDebounce<T>(value: T, delay: number): T

// 差：使用any
export function useDebounce(value: any, delay: number): any
```
