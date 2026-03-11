import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import ChannelSettings from './ChannelSettings'

interface LlmConfig {
  apiKey: string
  baseUrl: string
  model: string
}

type SettingsTab = 'llm' | 'channels'

export default function Settings() {
  const [activeTab, setActiveTab] = useState<SettingsTab>('llm')
  const [config, setConfig] = useState<LlmConfig>({
    apiKey: '',
    baseUrl: 'https://api.openai.com',
    model: 'gpt-3.5-turbo'
  })
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')
  const [savedConfig, setSavedConfig] = useState<LlmConfig | null>(null)

  useEffect(() => {
    console.log('Settings component mounted')
    loadConfig()
  }, [])

  const loadConfig = async () => {
    console.log('Loading config...')
    try {
      const savedConfig = await invoke<LlmConfig>('get_llm_config')
      console.log('Config loaded:', savedConfig)
      if (savedConfig && savedConfig.apiKey) {
        setConfig(savedConfig)
        setSavedConfig(savedConfig)
      }
    } catch (error) {
      console.log('No existing config found:', error)
    }
  }

  const handleSave = async () => {
    console.log('handleSave called')
    
    if (!config.apiKey.trim()) {
      setMessage('请输入 API 密钥')
      return
    }

    setLoading(true)
    setMessage('正在保存...')

    try {
      const configToSave = {
        apiKey: config.apiKey.trim(),
        baseUrl: config.baseUrl.trim(),
        model: config.model.trim()
      }
      console.log('Saving config:', { 
        apiKey: configToSave.apiKey.substring(0, 10) + '...', 
        baseUrl: configToSave.baseUrl,
        model: configToSave.model
      })
      
      await invoke('set_llm_config', { config: configToSave })
      
      console.log('Save successful')
      setMessage('配置保存成功！')
      
      // 立即重新加载配置以验证
      await loadConfig()
    } catch (error: any) {
      console.error('Save failed:', error)
      setMessage('保存失败：' + (error?.message || error || '未知错误'))
    } finally {
      setLoading(false)
    }
  }

  const handleDelete = async () => {
    setLoading(true)
    try {
      await invoke('delete_llm_config')
      setConfig({
        apiKey: '',
        baseUrl: 'https://api.openai.com',
        model: 'gpt-3.5-turbo'
      })
      setSavedConfig(null)
      setMessage('配置已删除')
    } catch (error: any) {
      setMessage('删除失败：' + (error?.message || error || '未知错误'))
    } finally {
      setLoading(false)
    }
  }

  const handleTest = async () => {
    setLoading(true)
    setMessage('正在测试...')
    try {
      const testConfig = await invoke<LlmConfig>('get_llm_config')
      console.log('Test result:', testConfig)
      setSavedConfig(testConfig)
      setMessage('配置读取成功！API Key 长度: ' + testConfig.apiKey.length)
    } catch (error: any) {
      console.error('Test failed:', error)
      setMessage('配置读取失败：' + (error?.message || error || '未知错误'))
    } finally {
      setLoading(false)
    }
  }

  console.log('Rendering Settings, current config:', config)

  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold mb-6">设置</h2>

      {/* Tab Navigation */}
      <div className="flex gap-2 mb-6 border-b border-gray-700 pb-2">
        <button
          onClick={() => setActiveTab('llm')}
          className={`px-4 py-2 rounded-lg ${
            activeTab === 'llm'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
          }`}
        >
          LLM 配置
        </button>
        <button
          onClick={() => setActiveTab('channels')}
          className={`px-4 py-2 rounded-lg ${
            activeTab === 'channels'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
          }`}
        >
          渠道配置
        </button>
      </div>

      {/* Tab Content */}
      {activeTab === 'llm' && (
        <div className="bg-gray-800 rounded-lg p-6 mb-6">
          <h3 className="text-xl font-semibold mb-4">LLM API 配置</h3>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                API 密钥
              </label>
              <input
                type="password"
                value={config.apiKey}
                onChange={(e) => setConfig(prev => ({ ...prev, apiKey: e.target.value }))}
                className="w-full bg-gray-900 text-white p-3 rounded-lg"
                placeholder="输入你的 API 密钥..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                API 基础 URL
              </label>
              <input
                type="text"
                value={config.baseUrl}
                onChange={(e) => setConfig(prev => ({ ...prev, baseUrl: e.target.value }))}
                className="w-full bg-gray-900 text-white p-3 rounded-lg"
                placeholder="https://api.openai.com"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                模型名称
              </label>
              <input
                type="text"
                value={config.model}
                onChange={(e) => setConfig(prev => ({ ...prev, model: e.target.value }))}
                className="w-full bg-gray-900 text-white p-3 rounded-lg"
                placeholder="gpt-3.5-turbo"
              />
            </div>

            {savedConfig && (
              <div className="bg-gray-700 p-3 rounded-lg">
                <h4 className="font-semibold mb-2">已保存的配置：</h4>
                <p className="text-sm text-gray-300">API Key: {'*'.repeat(savedConfig.apiKey.length)} ({savedConfig.apiKey.length} 字符)</p>
                <p className="text-sm text-gray-300">URL: {savedConfig.baseUrl}</p>
                <p className="text-sm text-gray-300">模型: {savedConfig.model}</p>
              </div>
            )}

            {message && (
              <div className={`p-3 rounded-lg ${message.includes('成功') ? 'bg-green-900 text-green-200' : 'bg-red-900 text-red-200'}`}>
                {message}
              </div>
            )}

            <div className="flex gap-2">
              <button
                onClick={handleSave}
                disabled={loading}
                className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
              >
                {loading ? '保存中...' : '保存设置'}
              </button>
              <button
                onClick={handleTest}
                disabled={loading}
                className="px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50"
              >
                测试配置
              </button>
              <button
                onClick={handleDelete}
                disabled={loading || !config.apiKey}
                className="px-6 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
              >
                删除配置
              </button>
            </div>
          </div>
        </div>
      )}
      
      {activeTab === 'channels' && <ChannelSettings />}
    </div>
  )
}
