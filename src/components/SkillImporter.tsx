import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface Skill {
  name: string
  description: string
  version: string
  author: string
  triggers: string[]
  instructions: string
  tools_required: string[]
  examples: string[]
  enabled: boolean
}

const exampleSkills = [
  {
    name: '基础智能包',
    description: 'OpenMonkey 出厂标配。记忆系统、上下文管理、任务追踪、深度调研——开箱即智能。',
    content: `---
name: 基础智能包
description: OpenMonkey 出厂标配。记忆系统、上下文管理、任务追踪、深度调研——开箱即智能。
version: 1.0.0
author: OpenMonkey Team
triggers:
  - 记住
  - 回忆
  - 搜索
  - 任务
  - 调研
tools_required: []
examples:
  - "记住我的名字是张三"
  - "回忆一下我上次说的任务"
  - "搜索关于人工智能的信息"
---

# 基础智能包技能

这是 OpenMonkey 的基础智能包，提供以下核心功能：

## 记忆系统
- 帮助用户记住重要信息
- 支持长期记忆和短期记忆
- 自动分类和标记记忆内容

## 上下文管理
- 维护对话上下文
- 理解用户意图
- 提供连贯的对话体验

## 任务追踪
- 记录用户任务
- 跟踪任务进度
- 提醒重要事项

## 深度调研
- 搜索相关信息
- 分析研究结果
- 提供综合报告
`
  },
  {
    name: '邮件管理',
    description: '邮件监控、智能摘要、自动回复。支持飞书邮箱和IMAP。',
    content: `---
name: 邮件管理
description: 邮件监控、智能摘要、自动回复。支持飞书邮箱和IMAP。
version: 1.0.0
author: OpenMonkey Team
triggers:
  - 邮件
  - 邮箱
  - 发邮件
  - 查邮件
  - 回复邮件
tools_required:
  - email_client
examples:
  - "查看我的邮件"
  - "给我发送一封邮件"
  - "回复这封邮件"
---

# 邮件管理技能

提供完整的邮件管理功能，包括邮件监控、智能摘要和自动回复。

## 功能特性

### 邮件监控
- 实时监控新邮件
- 支持飞书邮箱和IMAP协议
- 自动邮件分类

### 智能摘要
- 自动生成邮件摘要
- 提取关键信息
- 优先级排序

### 自动回复
- 智能回复建议
- 模板管理
- 定时发送
`
  },
  {
    name: '网页搜索与分析',
    description: '实时联网搜索，深度研究，竞品分析。Brave + DuckDuckGo 多引擎。',
    content: `---
name: 网页搜索与分析
description: 实时联网搜索，深度研究，竞品分析。Brave + DuckDuckGo 多引擎。
version: 1.0.0
author: OpenMonkey Team
triggers:
  - 搜索
  - 查找
  - 调研
  - 分析
  - 网页
tools_required:
  - web_search
examples:
  - "搜索人工智能的最新发展"
  - "帮我调研一下竞品"
  - "分析这个网页的内容"
---

# 网页搜索与分析技能

提供强大的网络搜索和内容分析功能。

## 功能特性

### 多引擎搜索
- Brave 搜索引擎
- DuckDuckGo 搜索引擎
- 自动去重和排序

### 深度研究
- 多源信息收集
- 内容交叉验证
- 综合分析报告

### 竞品分析
- 竞品信息收集
- 功能对比分析
- 市场趋势分析
`
  }
]

export default function SkillImporter() {
  const [source, setSource] = useState('')
  const [preview, setPreview] = useState<Skill | null>(null)
  const [importing, setImporting] = useState(false)
  const [error, setError] = useState('')
  const [showFormat, setShowFormat] = useState(false)

  const handleImport = async () => {
    if (!source.trim()) {
      setError('请输入技能来源（Markdown 内容）')
      return
    }

    setImporting(true)
    setError('')

    try {
      const skill = await invoke<Skill>('import_skill', { source })
      setPreview(skill)
      setError('')
    } catch (err) {
      const errorMessage = String(err)
      if (errorMessage.includes('missing YAML frontmatter')) {
        setError('导入失败：技能格式不正确。请确保技能内容以 YAML frontmatter 开头（用 --- 包裹）。点击"查看格式说明"了解更多。')
      } else if (errorMessage.includes('YAML frontmatter not properly closed')) {
        setError('导入失败：YAML frontmatter 没有正确闭合。请确保在 YAML 部分结束后有第二个 --- 标记。')
      } else if (errorMessage.includes('Failed to parse YAML')) {
        setError('导入失败：YAML 格式错误。请检查 YAML 语法是否正确。常见问题：缩进必须使用空格（不能用 Tab）、列表项必须以 - 开头、字符串值如果包含特殊字符需要用引号包裹。')
      } else {
        setError('导入失败：' + errorMessage)
      }
      setPreview(null)
    } finally {
      setImporting(false)
    }
  }

  const handleLoadExample = (skillName: string) => {
    const example = exampleSkills.find(s => s.name === skillName)
    if (example) {
      setSource(example.content)
      setError('')
    }
  }

  const handleConfirmImport = async () => {
    if (!preview) return

    setImporting(true)
    try {
      await invoke('enable_skill', { skillName: preview.name, enabled: true })
      alert('技能导入成功！')
      setSource('')
      setPreview(null)
    } catch (err) {
      alert('启用技能失败：' + err)
    } finally {
      setImporting(false)
    }
  }

  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold mb-6">技能导入向导</h2>

      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <div className="flex justify-between items-center mb-2">
          <label className="block text-sm font-medium text-gray-300">
            技能来源（Markdown 内容）
          </label>
          <button
            onClick={() => setShowFormat(!showFormat)}
            className="text-sm text-blue-400 hover:text-blue-300"
          >
            {showFormat ? '隐藏' : '查看'}格式说明
          </button>
        </div>
        
        {showFormat && (
          <div className="bg-gray-900 rounded-lg p-4 mb-4 text-sm text-gray-300">
            <p className="mb-2"><strong>技能文件格式要求：</strong></p>
            <ul className="list-disc list-inside space-y-1 mb-3">
              <li>必须以 <code className="bg-gray-700 px-1 rounded">---</code> 开头和结尾的 YAML frontmatter</li>
              <li>YAML 部分必须包含：name, description, triggers</li>
              <li>可选字段：version, author, tools_required, examples</li>
              <li>YAML 之后是技能的详细说明（Markdown 格式）</li>
            </ul>
            <p className="mb-2"><strong>示例：</strong></p>
            <pre className="bg-gray-800 p-3 rounded overflow-x-auto text-xs">
{`---
name: 技能名称
description: 技能描述
version: 1.0.0
author: 作者
triggers:
  - 触发词1
  - 触发词2
tools_required:
  - 工具1
examples:
  - "示例对话1"
---

# 技能详细说明

这里是技能的详细说明内容...`}
            </pre>
          </div>
        )}

        <textarea
          value={source}
          onChange={(e) => setSource(e.target.value)}
          className="w-full h-32 bg-gray-900 text-white p-4 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm"
          placeholder="粘贴技能 Markdown 内容..."
        />
        <button
          onClick={handleImport}
          disabled={importing}
          className="mt-4 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {importing ? '解析中...' : '解析技能'}
        </button>
      </div>

      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h3 className="text-lg font-semibold mb-4">快速导入示例技能</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {exampleSkills.map((skill) => (
            <button
              key={skill.name}
              onClick={() => handleLoadExample(skill.name)}
              className="bg-gray-700 hover:bg-gray-600 rounded-lg p-4 text-left transition-colors"
            >
              <h4 className="font-medium mb-1">{skill.name}</h4>
              <p className="text-sm text-gray-400">{skill.description}</p>
            </button>
          ))}
        </div>
      </div>

      {error && (
        <div className="bg-red-900 border border-red-700 text-red-200 px-4 py-3 rounded-lg mb-6">
          {error}
        </div>
      )}

      {preview && (
        <div className="bg-gray-800 rounded-lg p-6 mb-6">
          <h3 className="text-xl font-semibold mb-4">技能预览</h3>
          <div className="space-y-4">
            <div>
              <span className="text-gray-400">名称：</span>
              <span className="ml-2 font-medium">{preview.name}</span>
            </div>
            <div>
              <span className="text-gray-400">描述：</span>
              <span className="ml-2">{preview.description}</span>
            </div>
            <div>
              <span className="text-gray-400">版本：</span>
              <span className="ml-2">{preview.version}</span>
            </div>
            <div>
              <span className="text-gray-400">作者：</span>
              <span className="ml-2">{preview.author}</span>
            </div>
            <div>
              <span className="text-gray-400">触发关键词：</span>
              <div className="ml-2 flex flex-wrap gap-2">
                {preview.triggers.map((trigger, index) => (
                  <span key={index} className="px-2 py-1 bg-blue-900 text-blue-200 rounded text-sm">
                    {trigger}
                  </span>
                ))}
              </div>
            </div>
            <div>
              <span className="text-gray-400">工具依赖：</span>
              {preview.tools_required.length > 0 ? (
                <div className="ml-2 flex flex-wrap gap-2">
                  {preview.tools_required.map((tool, index) => (
                    <span key={index} className="px-2 py-1 bg-purple-900 text-purple-200 rounded text-sm">
                      {tool}
                    </span>
                  ))}
                </div>
              ) : (
                <span className="ml-2 text-gray-500">无</span>
              )}
            </div>
            <div>
              <span className="text-gray-400">示例对话：</span>
              {preview.examples.length > 0 ? (
                <ul className="ml-2 mt-2 space-y-1">
                  {preview.examples.map((example, index) => (
                    <li key={index} className="text-gray-300">
                      • {example}
                    </li>
                  ))}
                </ul>
              ) : (
                <span className="ml-2 text-gray-500">无</span>
              )}
            </div>
            <div>
              <span className="text-gray-400">说明：</span>
              <div className="ml-2 mt-2 p-3 bg-gray-900 rounded text-gray-300 whitespace-pre-wrap">
                {preview.instructions}
              </div>
            </div>
          </div>
          <div className="mt-6 flex gap-2">
            <button
              onClick={handleConfirmImport}
              disabled={importing}
              className="px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {importing ? '导入中...' : '确认导入'}
            </button>
            <button
              onClick={() => setPreview(null)}
              className="px-6 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700"
            >
              取消
            </button>
          </div>
        </div>
      )}
    </div>
  )
}
