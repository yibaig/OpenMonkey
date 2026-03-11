import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface ChannelConfig {
  channel_type: string
  enabled: boolean
  config: any
}

interface TelegramConfig {
  bot_token: string
  chat_id?: string
}

interface FeishuConfig {
  app_id: string
  app_secret: string
  encrypt_key?: string
  verification_token?: string
  use_long_connection?: boolean
  receive_id?: string
  receive_id_type?: string
}

interface WeComConfig {
  corp_id: string
  agent_id: string
  secret: string
  token?: string
  encoding_aes_key?: string
}

export default function ChannelSettings() {
  const [channels, setChannels] = useState<ChannelConfig[]>([])
  const [selectedChannel, setSelectedChannel] = useState<string>('telegram')
  const [telegramConfig, setTelegramConfig] = useState<TelegramConfig>({
    bot_token: '',
    chat_id: ''
  })
  const [feishuConfig, setFeishuConfig] = useState<FeishuConfig>({
    app_id: '',
    app_secret: '',
    encrypt_key: '',
    verification_token: '',
    use_long_connection: false,
    receive_id: '',
    receive_id_type: 'chat_id'
  })
  const [weComConfig, setWeComConfig] = useState<WeComConfig>({
    corp_id: '',
    agent_id: '',
    secret: '',
    token: '',
    encoding_aes_key: ''
  })
  const [loading, setLoading] = useState(false)
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null)
  const [wsStatus, setWsStatus] = useState<boolean>(false)

  useEffect(() => {
    loadChannels()
    checkWsStatus()
    // 每5秒检查一次 WebSocket 状态
    const interval = setInterval(checkWsStatus, 5000)
    return () => clearInterval(interval)
  }, [])

  const checkWsStatus = async () => {
    try {
      const status = await invoke<boolean>('get_feishu_ws_status')
      setWsStatus(status)
    } catch (error) {
      setWsStatus(false)
    }
  }

  const loadChannels = async () => {
    try {
      const result = await invoke<ChannelConfig[]>('list_channels')
      setChannels(result)
      
      // Load telegram config if exists
      const telegramChannel = result.find(c => c.channel_type === 'telegram')
      if (telegramChannel) {
        setTelegramConfig(telegramChannel.config)
      }
      
      // Load feishu config if exists
      const feishuChannel = result.find(c => c.channel_type === 'feishu')
      if (feishuChannel) {
        setFeishuConfig({
          ...feishuConfig,
          ...feishuChannel.config
        })
      }
      
      // Load wecom config if exists
      const wecomChannel = result.find(c => c.channel_type === 'we_com')
      if (wecomChannel) {
        setWeComConfig(wecomChannel.config)
      }
    } catch (error) {
      console.error('Failed to load channels:', error)
    }
  }

  const handleSaveTelegram = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      await invoke('add_channel', {
        config: {
          channel_type: 'telegram',
          enabled: true,
          config: telegramConfig
        }
      })
      
      await loadChannels()
      setTestResult({ success: true, message: 'Telegram 配置已保存' })
    } catch (error) {
      setTestResult({ success: false, message: `保存失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleTestTelegram = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      const result = await invoke<string>('send_to_channel', {
        channelType: 'telegram',
        content: '这是一条来自 OpenMonkey 的测试消息'
      })
      
      setTestResult({ success: true, message: result })
    } catch (error) {
      setTestResult({ success: false, message: `测试失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleTestFeishu = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      const result = await invoke<string>('send_to_channel', {
        channelType: 'feishu',
        content: '这是一条来自 OpenMonkey 的测试消息'
      })
      
      setTestResult({ success: true, message: result })
    } catch (error) {
      setTestResult({ success: false, message: `测试失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleTestWeCom = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      const result = await invoke<string>('send_to_channel', {
        channelType: 'we_com',
        content: '这是一条来自 OpenMonkey 的测试消息'
      })
      
      setTestResult({ success: true, message: result })
    } catch (error) {
      setTestResult({ success: false, message: `测试失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleSaveFeishu = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      await invoke('add_channel', {
        config: {
          channel_type: 'feishu',
          enabled: true,
          config: feishuConfig
        }
      })
      
      await loadChannels()
      await checkWsStatus()
      setTestResult({ success: true, message: '飞书配置已保存' })
    } catch (error) {
      setTestResult({ success: false, message: `保存失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleSaveWeCom = async () => {
    setLoading(true)
    setTestResult(null)
    
    try {
      await invoke('add_channel', {
        config: {
          channel_type: 'we_com',
          enabled: true,
          config: weComConfig
        }
      })
      
      await loadChannels()
      setTestResult({ success: true, message: '企业微信配置已保存' })
    } catch (error) {
      setTestResult({ success: false, message: `保存失败: ${error}` })
    } finally {
      setLoading(false)
    }
  }

  const handleRemoveChannel = async (channelType: string) => {
    if (!confirm('确定要删除这个渠道配置吗？')) {
      return
    }
    
    try {
      await invoke('remove_channel', { channelType })
      await loadChannels()
      await checkWsStatus()
      setTestResult({ success: true, message: '渠道配置已删除' })
    } catch (error) {
      setTestResult({ success: false, message: `删除失败: ${error}` })
    }
  }

  const renderTelegramConfig = () => (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Bot Token
        </label>
        <input
          type="password"
          value={telegramConfig.bot_token}
          onChange={(e) => setTelegramConfig({ ...telegramConfig, bot_token: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="从 @BotFather 获取"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Chat ID (可选)
        </label>
        <input
          type="text"
          value={telegramConfig.chat_id || ''}
          onChange={(e) => setTelegramConfig({ ...telegramConfig, chat_id: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="从 @userinfobot 获取"
        />
        <p className="text-xs text-gray-400 mt-1">
          留空则发送给所有订阅该 bot 的用户
        </p>
      </div>
      
      <div className="flex gap-2">
        <button
          onClick={handleSaveTelegram}
          disabled={loading || !telegramConfig.bot_token}
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? '保存中...' : '保存配置'}
        </button>
        
        {channels.some(c => c.channel_type === 'telegram') && (
          <button
            onClick={handleTestTelegram}
            disabled={loading}
            className="flex-1 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? '测试中...' : '发送测试消息'}
          </button>
        )}
      </div>
      
      {/* Telegram Setup Guide */}
      <div className="mt-6 p-4 bg-gray-900 rounded-lg">
        <h4 className="font-medium text-gray-200 mb-2">Telegram对接教程</h4>
        <div className="text-sm text-gray-400 space-y-2">
          <p>1. 打开 Telegram 应用</p>
          <p>2. 搜索并开始对话 <a href="https://t.me/BotFather" target="_blank" className="text-blue-400 hover:underline">@BotFather</a></p>
          <p>3. 发送命令 /newbot 创建新机器人</p>
          <p>4. 输入机器人名称和用户名（必须以_bot结尾）</p>
          <p>5. 完成后，BotFather 会提供 Bot Token，复制保存</p>
          <p>6. 搜索并开始对话 <a href="https://t.me/userinfobot" target="_blank" className="text-blue-400 hover:underline">@userinfobot</a> 获取 Chat ID</p>
          <p>7. 向你的机器人发送一条消息，以激活对话</p>
          <p>8. 在浏览器中访问：https://api.telegram.org/bot{'{你的token}'}/getUpdates 查看对话信息</p>
        </div>
      </div>
    </div>
  )

  const renderFeishuConfig = () => (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          App ID
        </label>
        <input
          type="text"
          value={feishuConfig.app_id}
          onChange={(e) => setFeishuConfig({ ...feishuConfig, app_id: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="飞书开发者平台获取"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          App Secret
        </label>
        <input
          type="password"
          value={feishuConfig.app_secret}
          onChange={(e) => setFeishuConfig({ ...feishuConfig, app_secret: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="飞书开发者平台获取"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Encrypt Key (可选)
        </label>
        <input
          type="password"
          value={feishuConfig.encrypt_key || ''}
          onChange={(e) => setFeishuConfig({ ...feishuConfig, encrypt_key: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="消息加密密钥"
        />
      </div>
      
      {/* 长连接选项 */}
      <div className="flex items-center gap-3 p-3 bg-gray-700/50 rounded-lg">
        <input
          type="checkbox"
          id="use_long_connection"
          checked={feishuConfig.use_long_connection || false}
          onChange={(e) => setFeishuConfig({ ...feishuConfig, use_long_connection: e.target.checked })}
          className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
        />
        <label htmlFor="use_long_connection" className="text-sm text-gray-300 cursor-pointer">
          启用 WebSocket 长连接
          <span className="text-xs text-gray-500 block">无需公网 URL，可接收飞书消息</span>
        </label>
        {wsStatus && (
          <span className="ml-auto text-xs text-green-400 flex items-center gap-1">
            <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
            已连接
          </span>
        )}
      </div>
      
      {/* 接收者配置 */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            接收者类型
          </label>
          <select
            value={feishuConfig.receive_id_type || 'chat_id'}
            onChange={(e) => setFeishuConfig({ ...feishuConfig, receive_id_type: e.target.value })}
            className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          >
            <option value="chat_id">群组 (chat_id)</option>
            <option value="user_id">用户 (user_id)</option>
            <option value="open_id">开放ID (open_id)</option>
            <option value="email">邮箱 (email)</option>
          </select>
        </div>
        
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            接收者 ID
          </label>
          <input
            type="text"
            value={feishuConfig.receive_id || ''}
            onChange={(e) => setFeishuConfig({ ...feishuConfig, receive_id: e.target.value })}
            className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
            placeholder="oc_xxx 或 ou_xxx"
          />
        </div>
      </div>
      
      <div className="flex gap-2">
        <button
          onClick={handleSaveFeishu}
          disabled={loading || !feishuConfig.app_id || !feishuConfig.app_secret}
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? '保存中...' : '保存配置'}
        </button>
        
        {channels.some(c => c.channel_type === 'feishu') && (
          <button
            onClick={handleTestFeishu}
            disabled={loading}
            className="flex-1 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? '测试中...' : '发送测试消息'}
          </button>
        )}
      </div>
      
      {/* Feishu Setup Guide */}
      <div className="mt-6 p-4 bg-gray-900 rounded-lg">
        <h4 className="font-medium text-gray-200 mb-2">飞书对接教程</h4>
        <div className="text-sm text-gray-400 space-y-2">
          <p>1. 访问 <a href="https://open.feishu.cn" target="_blank" className="text-blue-400 hover:underline">飞书开放平台</a></p>
          <p>2. 登录企业账号，进入「开发者后台」</p>
          <p>3. 点击「创建企业自建应用」</p>
          <p>4. 填写应用名称和描述，点击「创建」</p>
          <p>5. 在「凭证与基础信息」中获取 App ID 和 App Secret</p>
          <p>6. 在「权限管理」中添加必要的权限（如发送消息权限）</p>
          <p>7. 在「应用功能」中设置消息接收相关配置（如需）</p>
          <p>8. 发布应用并获取管理员授权</p>
        </div>
      </div>
    </div>
  )

  const renderWeComConfig = () => (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Corp ID
        </label>
        <input
          type="text"
          value={weComConfig.corp_id}
          onChange={(e) => setWeComConfig({ ...weComConfig, corp_id: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="企业微信企业ID"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Agent ID
        </label>
        <input
          type="text"
          value={weComConfig.agent_id}
          onChange={(e) => setWeComConfig({ ...weComConfig, agent_id: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="应用Agent ID"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Secret
        </label>
        <input
          type="password"
          value={weComConfig.secret}
          onChange={(e) => setWeComConfig({ ...weComConfig, secret: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="应用Secret"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Token (可选)
        </label>
        <input
          type="password"
          value={weComConfig.token || ''}
          onChange={(e) => setWeComConfig({ ...weComConfig, token: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="消息校验Token"
        />
      </div>
      
      <div>
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Encoding AES Key (可选)
        </label>
        <input
          type="password"
          value={weComConfig.encoding_aes_key || ''}
          onChange={(e) => setWeComConfig({ ...weComConfig, encoding_aes_key: e.target.value })}
          className="w-full bg-gray-700 text-white px-4 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none"
          placeholder="消息加密密钥"
        />
      </div>
      
      <div className="flex gap-2">
        <button
          onClick={handleSaveWeCom}
          disabled={loading || !weComConfig.corp_id || !weComConfig.agent_id || !weComConfig.secret}
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? '保存中...' : '保存配置'}
        </button>
        
        {channels.some(c => c.channel_type === 'we_com') && (
          <button
            onClick={handleTestWeCom}
            disabled={loading}
            className="flex-1 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? '测试中...' : '发送测试消息'}
          </button>
        )}
      </div>
      
      {/* WeCom Setup Guide */}
      <div className="mt-6 p-4 bg-gray-900 rounded-lg">
        <h4 className="font-medium text-gray-200 mb-2">企业微信对接教程</h4>
        <div className="text-sm text-gray-400 space-y-2">
          <p>1. 登录 <a href="https://work.weixin.qq.com/" target="_blank" className="text-blue-400 hover:underline">企业微信管理后台</a></p>
          <p>2. 进入「应用管理」-「自建」-「创建应用」</p>
          <p>3. 填写应用名称和描述，上传应用logo</p>
          <p>4. 点击「创建」后，在应用详情页获取 Corp ID 和 Agent ID</p>
          <p>5. 在「管理工具」-「通讯录同步」中获取 Secret</p>
          <p>6. 在「权限管理」中添加必要的权限（如发送消息权限）</p>
          <p>7. 设置应用可见范围</p>
          <p>8. 保存配置并测试消息发送</p>
        </div>
      </div>
    </div>
  )

  const renderChannelConfig = () => {
    switch (selectedChannel) {
      case 'telegram':
        return renderTelegramConfig()
      case 'feishu':
        return renderFeishuConfig()
      case 'we_com':
        return renderWeComConfig()
      default:
        return null
    }
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">渠道配置</h2>
      
      {/* Channel Tabs */}
      <div className="flex gap-2 border-b border-gray-700 pb-2">
        {[
          { id: 'telegram', name: 'Telegram', icon: '📱' },
          { id: 'feishu', name: '飞书', icon: '💬' },
          { id: 'we_com', name: '企业微信', icon: '🏢' },
        ].map((channel) => (
          <button
            key={channel.id}
            onClick={() => setSelectedChannel(channel.id)}
            className={`px-4 py-2 rounded-lg flex items-center gap-2 ${
              selectedChannel === channel.id
                ? 'bg-blue-600 text-white'
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
          >
            <span>{channel.icon}</span>
            <span>{channel.name}</span>
            {channels.some(c => c.channel_type === channel.id) && (
              <span className="w-2 h-2 bg-green-500 rounded-full" />
            )}
          </button>
        ))}
      </div>
      
      {/* Channel Configuration */}
      <div className="bg-gray-800 rounded-lg p-6">
        {renderChannelConfig()}
      </div>
      
      {/* Configured Channels List */}
      {channels.length > 0 && (
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-lg font-medium mb-4">已配置的渠道</h3>
          <div className="space-y-2">
            {channels.map((channel) => (
              <div
                key={channel.channel_type}
                className="flex items-center justify-between bg-gray-700 rounded-lg px-4 py-3"
              >
                <div className="flex items-center gap-3">
                  <span className="text-2xl">
                    {channel.channel_type === 'telegram' && '📱'}
                    {channel.channel_type === 'feishu' && '💬'}
                    {channel.channel_type === 'we_com' && '🏢'}
                  </span>
                  <div>
                    <div className="font-medium">
                      {channel.channel_type === 'telegram' && 'Telegram'}
                      {channel.channel_type === 'feishu' && '飞书'}
                      {channel.channel_type === 'we_com' && '企业微信'}
                    </div>
                    <div className="text-sm text-gray-400">
                      {channel.enabled ? '已启用' : '已禁用'}
                      {channel.channel_type === 'feishu' && channel.config?.use_long_connection && (
                        <span className="ml-2 text-green-400">• 长连接</span>
                      )}
                    </div>
                  </div>
                </div>
                <button
                  onClick={() => handleRemoveChannel(channel.channel_type)}
                  className="text-red-400 hover:text-red-300 px-3 py-1 rounded hover:bg-red-900/20"
                >
                  删除
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
      
      {/* Test Result */}
      {testResult && (
        <div
          className={`p-4 rounded-lg ${
            testResult.success
              ? 'bg-green-900/30 text-green-300 border border-green-700'
              : 'bg-red-900/30 text-red-300 border border-red-700'
          }`}
        >
          {testResult.message}
        </div>
      )}
    </div>
  )
}
