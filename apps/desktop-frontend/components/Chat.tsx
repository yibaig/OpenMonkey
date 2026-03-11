import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface Message {
  role: 'user' | 'assistant'
  content: string
  timestamp: string
}

interface Skill {
  name: string
  enabled: boolean
}

const STORAGE_KEY = 'openmonkey_chat_history'
const MAX_MESSAGES = 200 // 最多保留200条消息

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [loading, setLoading] = useState(false)
  const [skills, setSkills] = useState<Skill[]>([])
  const messagesEndRef = useRef<HTMLDivElement>(null)

  // 加载历史聊天记录
  useEffect(() => {
    loadSkills()
    loadChatHistory()
  }, [])

  // 保存聊天记录到 localStorage
  useEffect(() => {
    if (messages.length > 0) {
      saveChatHistory()
    }
  }, [messages])

  useEffect(() => {
    scrollToBottom()
  }, [messages])

  const loadChatHistory = () => {
    try {
      const saved = localStorage.getItem(STORAGE_KEY)
      if (saved) {
        const parsed = JSON.parse(saved)
        if (Array.isArray(parsed)) {
          setMessages(parsed)
          console.log('Loaded chat history:', parsed.length, 'messages')
        }
      }
    } catch (error) {
      console.error('Failed to load chat history:', error)
    }
  }

  const saveChatHistory = () => {
    try {
      // 只保留最近的消息
      const messagesToSave = messages.slice(-MAX_MESSAGES)
      localStorage.setItem(STORAGE_KEY, JSON.stringify(messagesToSave))
    } catch (error) {
      console.error('Failed to save chat history:', error)
    }
  }

  const clearChatHistory = () => {
    if (confirm('确定要清空所有聊天记录吗？')) {
      setMessages([])
      localStorage.removeItem(STORAGE_KEY)
    }
  }

  const loadSkills = async () => {
    try {
      const loadedSkills = await invoke<Skill[]>('list_skills')
      setSkills(loadedSkills.filter(s => s.enabled))
    } catch (error) {
      console.error('Failed to load skills:', error)
    }
  }

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }

  const handleSend = async () => {
    if (!input.trim() || loading) return

    const userMessage: Message = {
      role: 'user',
      content: input,
      timestamp: new Date().toISOString()
    }

    setMessages(prev => [...prev, userMessage])
    setInput('')
    setLoading(true)

    try {
      const response = await invoke<string>('chat', { message: input })
      const assistantMessage: Message = {
        role: 'assistant',
        content: response,
        timestamp: new Date().toISOString()
      }
      setMessages(prev => [...prev, assistantMessage])
    } catch (error) {
      console.error('Failed to send message:', error)
      const errorMessage: Message = {
        role: 'assistant',
        content: '抱歉，发生了错误：' + error,
        timestamp: new Date().toISOString()
      }
      setMessages(prev => [...prev, errorMessage])
    } finally {
      setLoading(false)
    }
  }

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  return (
    <div className="flex flex-col h-screen">
      <div className="bg-gray-800 border-b border-gray-700 p-4">
        <div className="flex justify-between items-center">
          <h1 className="text-xl font-bold">OpenMonkey 聊天</h1>
          {messages.length > 0 && (
            <button
              onClick={clearChatHistory}
              className="px-3 py-1 text-sm bg-red-600 text-white rounded hover:bg-red-700"
            >
              清空记录
            </button>
          )}
        </div>
        {skills.length > 0 && (
          <div className="mt-2 flex flex-wrap gap-2">
            <span className="text-sm text-gray-400">已激活技能：</span>
            {skills.map((skill, index) => (
              <span key={index} className="px-2 py-1 bg-blue-900 text-blue-200 rounded text-sm">
                {skill.name}
              </span>
            ))}
          </div>
        )}
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 ? (
          <div className="text-center text-gray-500 mt-20">
              <img src="/logo.png" alt="OpenMonkey" className="w-24 h-24 mx-auto mb-4" />
              <p>开始与 OpenMonkey 对话吧！</p>
            </div>
        ) : (
          messages.map((message, index) => (
            <div
              key={index}
              className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-[70%] rounded-lg p-4 ${
                  message.role === 'user'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-100'
                }`}
              >
                <div className="text-sm text-gray-400 mb-1">
                  {message.role === 'user' ? '你' : 'OpenMonkey'} • {new Date(message.timestamp).toLocaleTimeString()}
                </div>
                <div className="whitespace-pre-wrap">{message.content}</div>
              </div>
            </div>
          ))
        )}
        {loading && (
          <div className="flex justify-start">
            <div className="bg-gray-700 text-gray-100 rounded-lg p-4">
              <div className="flex items-center gap-2">
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-500"></div>
                <span>OpenMonkey 正在思考...</span>
              </div>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      <div className="bg-gray-800 border-t border-gray-700 p-4">
        <div className="flex gap-2">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyPress={handleKeyPress}
            className="flex-1 bg-gray-900 text-white p-4 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="输入消息... (Shift+Enter 换行)"
            rows={3}
          />
          <button
            onClick={handleSend}
            disabled={loading || !input.trim()}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed self-end"
          >
            发送
          </button>
        </div>
      </div>
    </div>
  )
}
