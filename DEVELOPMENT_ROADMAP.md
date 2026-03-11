# 🚀 OpenMonkey 开发路线图

> **本文档**: OpenMonkey 双版本整合计划  
> **创建日期**: 2026-03-12  
> **制定者**: 旺财 (AI Agent)

---

## 📊 当前状态

### ✅ 已完成
- [x] OpenMonkey-Trae v2.0 代码上传至 GitHub
- [x] 基础功能完整（灵魂系统/记忆系统/技能系统/多渠道）
- [x] Tauri v2 桌面应用框架
- [x] Rust 后端核心模块

### 📋 待整合
- [ ] OpenMonkey (WASM 运行时版) 核心模块
- [ ] 四层记忆系统升级（SQLite + Qdrant）
- [ ] 多 Agent 编排器
- [ ] 思维链可视化 2.0
- [ ] 人格进化系统

---

## 🎯 整合计划

### 阶段 1: 基础整合（1-2 周）
- [ ] 创建 `openmonkey-ultimate` 分支
- [ ] 整合 MemoryBank v2（SQLite + Qdrant）
- [ ] 整合 SkillAdapter v2（WASM + 传统双模式）
- [ ] 完善文档系统

### 阶段 2: 核心功能（2-3 周）
- [ ] WASM 沙箱集成
- [ ] 多 Agent 编排器
- [ ] 思维链可视化 2.0（可交互）
- [ ] 人格系统 v1

### 阶段 3: 高级功能（1-2 月）
- [ ] RPA 连接器
- [ ] 视觉引擎
- [ ] 语音接口
- [ ] 技能基因系统

### 阶段 4: 生态建设（2-3 月）
- [ ] 统一文档站
- [ ] 示例技能 50+
- [ ] SDK（TypeScript/Python）
- [ ] 社区建设

---

## 📁 项目结构

```
OpenMonkey/
├── src/                    # 前端（React + TypeScript）
│   ├── components/
│   │   ├── Chat.tsx
│   │   ├── SoulStudio.tsx
│   │   ├── SkillImporter.tsx
│   │   └── ...
│   └── store/
│       └── useAppStore.ts
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── core/
│   │   │   ├── memory_bank.rs
│   │   │   ├── soul_manager.rs
│   │   │   ├── skill_adapter.rs
│   │   │   └── ...
│   │   └── llm/
│   │       └── client.rs
│   └── Cargo.toml
├── package.json
└── README.md
```

---

## 🛠️ 技术栈

### 前端
- React 18 + TypeScript
- Zustand（状态管理）
- TailwindCSS（样式）
- Vite（构建工具）

### 后端
- Rust + Tauri v2
- SQLite（数据库）
- Tokio（异步运行时）
- Serde（序列化）

### AI
- 多模型支持（Qwen/OpenAI/Ollama）
- Prompt 模板系统
- 记忆存储与检索

---

## 📝 开发指南

### 环境准备
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js
# https://nodejs.org/

# 克隆项目
git clone https://github.com/yibaig/OpenMonkey.git
cd OpenMonkey

# 安装依赖
npm install
```

### 开发模式
```bash
# 启动开发服务器
npm run tauri:dev
```

### 构建发布
```bash
# 构建生产版本
npm run tauri:build
```

---

## 📚 相关文档

- [整合方案详细版](https://github.com/yibaig/OpenMonkey/blob/main/openmonkey-ultimate-integration-plan.md)
- [功能分析](https://github.com/yibaig/OpenMonkey/blob/main/openmonkey-analysis.md)
- [启动指南](启动文档.md)

---

## 👥 团队

- **总指挥**: Coder YiBai
- **AI Agent**: 旺财 (wangcai@openmonkey.ai)

---

## 📞 联系方式

- GitHub: https://github.com/yibaig/OpenMonkey
- 邮箱：yibaig@openmonkey.ai

---

**最后更新**: 2026-03-12
