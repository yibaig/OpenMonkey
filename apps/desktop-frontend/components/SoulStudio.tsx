import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

export default function SoulStudio() {
  const [activeTab, setActiveTab] = useState<'soul' | 'user' | 'agents'>('soul')
  const [soulContent, setSoulContent] = useState('')
  const [userContent, setUserContent] = useState('')
  const [agentsContent, setAgentsContent] = useState('')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    loadSoulFiles()
  }, [])

  const loadSoulFiles = async () => {
    try {
      const [soul, user, agents] = await Promise.all([
        invoke<string>('read_soul_file', { fileType: 'soul' }),
        invoke<string>('read_soul_file', { fileType: 'user' }),
        invoke<string>('read_soul_file', { fileType: 'agents' })
      ])
      setSoulContent(soul)
      setUserContent(user)
      setAgentsContent(agents)
    } catch (error) {
      console.error('Failed to load soul files:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSave = async () => {
    setSaving(true)
    try {
      await Promise.all([
        invoke('write_soul_file', { fileType: 'soul', content: soulContent }),
        invoke('write_soul_file', { fileType: 'user', content: userContent }),
        invoke('write_soul_file', { fileType: 'agents', content: agentsContent })
      ])
      alert('保存成功！')
    } catch (error) {
      console.error('Failed to save soul files:', error)
      alert('保存失败：' + error)
    } finally {
      setSaving(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-400">加载中...</div>
      </div>
    )
  }

  return (
    <div className="p-6">
      <div className="mb-6">
        <h2 className="text-2xl font-bold mb-4">灵魂工作室</h2>
        <div className="flex gap-2 mb-4">
          <button
            onClick={() => setActiveTab('soul')}
            className={`px-4 py-2 rounded-lg ${
              activeTab === 'soul'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
          >
            SOUL.md
          </button>
          <button
            onClick={() => setActiveTab('user')}
            className={`px-4 py-2 rounded-lg ${
              activeTab === 'user'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
          >
            USER.md
          </button>
          <button
            onClick={() => setActiveTab('agents')}
            className={`px-4 py-2 rounded-lg ${
              activeTab === 'agents'
                ? 'bg-blue-600 text-white'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
          >
            AGENTS.md
          </button>
        </div>
      </div>

      <div className="bg-gray-800 rounded-lg p-4 mb-4">
        <textarea
          value={
            activeTab === 'soul'
              ? soulContent
              : activeTab === 'user'
              ? userContent
              : agentsContent
          }
          onChange={(e) => {
            const content = e.target.value
            if (activeTab === 'soul') setSoulContent(content)
            else if (activeTab === 'user') setUserContent(content)
            else setAgentsContent(content)
          }}
          className="w-full h-96 bg-gray-900 text-white p-4 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="编辑灵魂文件内容..."
        />
      </div>

      <div className="flex gap-2">
        <button
          onClick={handleSave}
          disabled={saving}
          className="px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {saving ? '保存中...' : '保存所有更改'}
        </button>
        <button
          onClick={loadSoulFiles}
          className="px-6 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700"
        >
          重新加载
        </button>
      </div>
    </div>
  )
}
