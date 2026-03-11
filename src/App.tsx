import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SoulStudio from './components/SoulStudio'
import SkillImporter from './components/SkillImporter'
import Chat from './components/Chat'
import Settings from './components/Settings'
import './App.css'

type Page = 'home' | 'soul' | 'skills' | 'chat' | 'settings'

interface UsageStats {
  total_requests: number
  total_input_tokens: number
  total_output_tokens: number
  total_tokens: number
  successful_requests: number
  failed_requests: number
  last_request_time: string | null
}

interface SystemStatus {
  status: string
  status_text: string
  llm_configured: boolean
  stats: UsageStats
}

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home')
  const [systemStatus, setSystemStatus] = useState<SystemStatus | null>(null)
  const [dataDir, setDataDir] = useState<string>('')

  useEffect(() => {
    const initializeApp = async () => {
      try {
        const dataDir = await invoke<string>('get_data_dir')
        setDataDir(dataDir)
        await invoke('ensure_data_structure')

        const skillsDir = `${dataDir}/skills`
        await invoke('init_skill_adapter', { skillsDir })
        await invoke('init_memory_bank', { dbPath: `${dataDir}/memory.db` })
        await invoke('init_soul_manager', { dataDir })
        await invoke('init_reflector')
        await invoke('init_channel_manager')

        // Load system status
        await loadSystemStatus(dataDir)

        console.log('Core components initialized successfully')
      } catch (error) {
        console.error('Failed to initialize app:', error)
      }
    }

    initializeApp()
  }, [])

  const loadSystemStatus = async (dir: string) => {
    try {
      const status = await invoke<SystemStatus>('get_system_status', { dataDir: dir })
      setSystemStatus(status)
    } catch (error) {
      console.error('Failed to load system status:', error)
    }
  }

  const formatNumber = (num: number): string => {
    if (num >= 1000000) {
      return (num / 1000000).toFixed(1) + 'M'
    } else if (num >= 1000) {
      return (num / 1000).toFixed(1) + 'K'
    }
    return num.toString()
  }

  const formatTime = (timeStr: string | null): string => {
    if (!timeStr) return '从未'
    try {
      const date = new Date(timeStr)
      return date.toLocaleString('zh-CN')
    } catch {
      return timeStr
    }
  }

  const renderPage = () => {
    switch (currentPage) {
      case 'soul':
        return <SoulStudio />
      case 'skills':
        return <SkillImporter />
      case 'chat':
        return <Chat />
      case 'settings':
        return <Settings />
      default:
        return (
          <div className="min-h-screen bg-gray-900 text-white">
            <div className="container mx-auto px-4 py-8">
              <h1 className="text-4xl font-bold mb-8 text-center">OpenMonkey v2.0</h1>
              
              {/* System Status Card */}
              <div className="bg-gray-800 rounded-lg p-6 mb-6">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-xl font-semibold">系统状态</h2>
                  <div className={`px-4 py-2 rounded-full text-sm font-medium ${
                    systemStatus?.status === 'ready' 
                      ? 'bg-green-900 text-green-300' 
                      : 'bg-yellow-900 text-yellow-300'
                  }`}>
                    {systemStatus?.status_text || '加载中...'}
                  </div>
                </div>
                
                {/* Status Indicators */}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm mb-1">LLM 配置</div>
                    <div className={`text-lg font-semibold ${systemStatus?.llm_configured ? 'text-green-400' : 'text-yellow-400'}`}>
                      {systemStatus?.llm_configured ? '✓ 已配置' : '✗ 未配置'}
                    </div>
                  </div>
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm mb-1">数据目录</div>
                    <div className="text-lg font-semibold text-blue-400 truncate" title={dataDir}>
                      {dataDir ? '✓ 已初始化' : '✗ 未初始化'}
                    </div>
                  </div>
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm mb-1">技能系统</div>
                    <div className="text-lg font-semibold text-green-400">✓ 已加载</div>
                  </div>
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm mb-1">记忆系统</div>
                    <div className="text-lg font-semibold text-green-400">✓ 已就绪</div>
                  </div>
                </div>
              </div>

              {/* Usage Statistics Dashboard */}
              <div className="bg-gray-800 rounded-lg p-6 mb-6">
                <h2 className="text-xl font-semibold mb-4">使用统计</h2>
                
                {/* Main Stats */}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                  <div className="bg-gradient-to-br from-blue-900 to-blue-800 rounded-lg p-4">
                    <div className="text-blue-300 text-sm mb-1">总请求数</div>
                    <div className="text-3xl font-bold text-white">
                      {formatNumber(systemStatus?.stats.total_requests || 0)}
                    </div>
                  </div>
                  <div className="bg-gradient-to-br from-purple-900 to-purple-800 rounded-lg p-4">
                    <div className="text-purple-300 text-sm mb-1">输入 Token</div>
                    <div className="text-3xl font-bold text-white">
                      {formatNumber(systemStatus?.stats.total_input_tokens || 0)}
                    </div>
                  </div>
                  <div className="bg-gradient-to-br from-green-900 to-green-800 rounded-lg p-4">
                    <div className="text-green-300 text-sm mb-1">输出 Token</div>
                    <div className="text-3xl font-bold text-white">
                      {formatNumber(systemStatus?.stats.total_output_tokens || 0)}
                    </div>
                  </div>
                  <div className="bg-gradient-to-br from-orange-900 to-orange-800 rounded-lg p-4">
                    <div className="text-orange-300 text-sm mb-1">总 Token</div>
                    <div className="text-3xl font-bold text-white">
                      {formatNumber(systemStatus?.stats.total_tokens || 0)}
                    </div>
                  </div>
                </div>

                {/* Progress Bars */}
                <div className="space-y-4">
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-400">成功率</span>
                      <span className="text-white">
                        {systemStatus?.stats.total_requests 
                          ? Math.round((systemStatus.stats.successful_requests / systemStatus.stats.total_requests) * 100) 
                          : 0}%
                      </span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-2">
                      <div 
                        className="bg-green-500 h-2 rounded-full transition-all duration-300"
                        style={{ 
                          width: systemStatus?.stats.total_requests 
                            ? `${(systemStatus.stats.successful_requests / systemStatus.stats.total_requests) * 100}%` 
                            : '0%' 
                        }}
                      ></div>
                    </div>
                  </div>
                  
                  <div className="flex justify-between text-sm text-gray-400">
                    <span>成功: {systemStatus?.stats.successful_requests || 0}</span>
                    <span>失败: {systemStatus?.stats.failed_requests || 0}</span>
                    <span>最后请求: {formatTime(systemStatus?.stats.last_request_time || null)}</span>
                  </div>
                </div>
              </div>

              {/* Quick Actions */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <button
                  onClick={() => setCurrentPage('soul')}
                  className="bg-gray-800 rounded-lg p-6 hover:bg-gray-700 transition-colors group"
                >
                  <div className="text-3xl mb-3">🧠</div>
                  <h3 className="text-lg font-semibold mb-2 group-hover:text-blue-400">灵魂工作室</h3>
                  <p className="text-gray-400 text-sm">管理 AI 灵魂与用户配置</p>
                </button>
                
                <button
                  onClick={() => setCurrentPage('skills')}
                  className="bg-gray-800 rounded-lg p-6 hover:bg-gray-700 transition-colors group"
                >
                  <div className="text-3xl mb-3">⚡</div>
                  <h3 className="text-lg font-semibold mb-2 group-hover:text-blue-400">技能库</h3>
                  <p className="text-gray-400 text-sm">导入与管理技能</p>
                </button>
                
                <button
                  onClick={() => setCurrentPage('chat')}
                  className="bg-gray-800 rounded-lg p-6 hover:bg-gray-700 transition-colors group"
                >
                  <div className="text-3xl mb-3">💬</div>
                  <h3 className="text-lg font-semibold mb-2 group-hover:text-blue-400">聊天</h3>
                  <p className="text-gray-400 text-sm">与 OpenMonkey 对话</p>
                </button>
              </div>
            </div>
          </div>
        )
    }
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <nav className="bg-gray-800 border-b border-gray-700">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold flex items-center gap-2">
              <img src="/logo.png" alt="OpenMonkey" className="w-8 h-8" />
              OpenMonkey
            </h1>
            <div className="flex gap-2">
              <button
                onClick={() => setCurrentPage('home')}
                className={`px-4 py-2 rounded-lg ${
                  currentPage === 'home'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                首页
              </button>
              <button
                onClick={() => setCurrentPage('soul')}
                className={`px-4 py-2 rounded-lg ${
                  currentPage === 'soul'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                灵魂
              </button>
              <button
                onClick={() => setCurrentPage('skills')}
                className={`px-4 py-2 rounded-lg ${
                  currentPage === 'skills'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                技能
              </button>
              <button
                onClick={() => setCurrentPage('chat')}
                className={`px-4 py-2 rounded-lg ${
                  currentPage === 'chat'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                聊天
              </button>
              <button
                onClick={() => setCurrentPage('settings')}
                className={`px-4 py-2 rounded-lg ${
                  currentPage === 'settings'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                设置
              </button>
            </div>
          </div>
        </div>
      </nav>
      {renderPage()}
    </div>
  )
}

export default App
