# OpenMonkey 技能格式规范 (OM-SKILL.md)

## 技能元数据

```yaml
name: 技能名称
description: 技能的简短描述
version: 1.0.0
author: 作者名称
```

## 触发关键词

```yaml
triggers:
  - 关键词1
  - 关键词2
  - 关键词3
```

## 技能说明

```markdown
详细的技能说明和使用指南
```

## 工具依赖

```yaml
tools_required:
  - tool1
  - tool2
```

## 示例对话

```yaml
examples:
  - "用户输入示例1"
  - "用户输入示例2"
```

## 完整示例

```yaml
---
name: PDF 处理工具
description: 处理 PDF 文件，包括提取文本、合并、拆分等操作
version: 1.0.0
author: OpenMonkey Team

triggers:
  - pdf
  - pdf工具
  - 处理pdf
  - 提取pdf

tools_required:
  - pdf-reader
  - file-processor

examples:
  - "帮我提取这个 PDF 的文本内容"
  - "合并多个 PDF 文件"
  - "拆分 PDF 文档"
---

# PDF 处理工具

这是一个用于处理 PDF 文件的技能。

## 功能

- 提取 PDF 文本内容
- 合并多个 PDF 文件
- 拆分 PDF 文档
- 转换 PDF 格式

## 使用方法

当用户提到 PDF 相关任务时，本技能会自动激活。
