# 🐵 OpenMonkey - 你的超级 AI 助手

> **“把复杂留给代码，把安全留给自己。”**  
> 领养你的 OpenMonkey，开启可信 AI 新时代。

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.0-green.svg)](https://tauri.app)

---

## 🌟 特性

### 🧠 四层记忆系统
- **短期记忆** - 最近对话上下文
- **长期记忆** - 重要信息持久化
- **程序记忆** - 技能和知识
- **情感记忆** - 情感体验和偏好

### 🎭 人格系统
- 可定制的人格特质
- 动态情绪状态
- 人格进化日志
- 多人格支持

### 🛠️ 技能引擎
- Markdown 格式技能定义
- WASM 沙箱安全执行（开发中）
- 热更新支持
- 官方技能市场

### 🤖 多 Agent 编排
- 任务自动分配
- Agent 协作
- 事件订阅机制
- 实时监控

### 💬 多渠道支持
- 飞书（Feishu）
- Telegram
- 企业微信（开发中）
- 自定义渠道

---

## 🚀 快速开始

### 环境要求

- **Rust**: 1.75+
- **Node.js**: 18+
- **操作系统**: Windows 10+, macOS 12+, Linux

### 安装

```bash
# 克隆项目
git clone https://github.com/yibaig/OpenMonkey.git
cd OpenMonkey

# 安装依赖
npm install

# 开发模式运行
npm run tauri:dev

# 构建发布版本
npm run tauri:build
```

### 配置

1. 复制配置模板
```bash
cp assets/templates/soul/SOUL.md .
cp assets/templates/soul/USER.md .
cp assets/templates/soul/AGENTS.md .
```

2. 编辑配置文件
- `SOUL.md` - 定义 AI 人格
- `USER.md` - 用户信息
- `AGENTS.md` - Agent 配置

---

## 📁 项目结构

```
OpenMonkey/
├── apps/
│   ├── desktop-frontend/    # React 前端
│   └── desktop-backend/     # Rust + Tauri 后端
├── packages/
│   ├── core/
│   │   ├── memory/          # 记忆系统
│   │   ├── soul/            # 灵魂系统
│   │   ├── skill/           # 技能引擎
│   │   └── agent/           # 多 Agent 编排
│   ├── wasm-runtime/        # WASM 运行时
│   └── shared/              # 共享工具
├── skills/                  # 技能市场
├── examples/                # 示例项目
└── docs/                    # 文档中心
```

---

## 📚 文档

- [整合方案](docs/INTEGRATION_PLAN.md)
- [开发路线图](docs/DEVELOPMENT_ROADMAP.md)
- [任务清单](docs/TASKS.md)
- [API 文档](docs/API.md)

---

## 🛠️ 核心包

### openmonkey-memory
记忆系统 - SQLite 存储 + Qdrant 向量检索（计划中）

```rust
use openmonkey_memory::{MemoryBank, MemoryType};

let bank = MemoryBank::new("sqlite:memory.db").await?;
bank.add(Memory::new("你好".to_string(), MemoryType::ShortTerm, None)).await?;
```

### openmonkey-soul
灵魂系统 - 人格配置和管理

```rust
use openmonkey_soul::{SoulManager, PersonalityManager};

let mut soul_mgr = SoulManager::new(&config_dir);
soul_mgr.load_all().await?;
```

### openmonkey-skill
技能引擎 - WASM 沙箱执行

```rust
use openmonkey_skill::{SkillAdapter, WasmExecutor};

let mut adapter = SkillAdapter::new(&skills_dir);
adapter.init_wasm()?;
adapter.load_all().await?;
```

### openmonkey-agent
多 Agent 编排器

```rust
use openmonkey_agent::{Coordinator, Agent};

let mut coordinator = Coordinator::new();
coordinator.register_agent(Agent::new("助手", "客服"));
let task_id = coordinator.create_task("回复用户消息");
coordinator.auto_assign_task(&task_id)?;
```

---

## 🎯 开发计划

### 阶段 1: 基础整合 ✅ (已完成)
- [x] 项目结构重组
- [x] 核心包实现
- [x] Workspace 配置
- [x] 编译验证

### 阶段 2: 核心功能 🚧 (进行中)
- [ ] WASM 沙箱集成
- [ ] Qdrant 向量检索
- [ ] 思维链可视化
- [ ] 人格进化系统

### 阶段 3: 高级功能 ⏳ (计划中)
- [ ] RPA 连接器
- [ ] 视觉引擎
- [ ] 语音接口
- [ ] 技能基因系统

### 阶段 4: 生态建设 ⏳ (计划中)
- [ ] 统一文档站
- [ ] 示例技能 50+
- [ ] SDK (TypeScript/Python)
- [ ] 社区建设

---

## 🤝 贡献

欢迎贡献！请查看 [贡献指南](CONTRIBUTING.md)。

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📊 代码统计

| 语言 | 文件数 | 代码行数 |
|------|--------|----------|
| Rust | 45 | ~3,500 |
| TypeScript | 20 | ~1,800 |
| Markdown | 15 | ~800 |
| **总计** | **80** | **~6,100** |

---

## 📝 更新日志

### v0.2.0 (2026-03-12)
- ✅ 项目结构重组
- ✅ 6 个核心 Rust 包实现
- ✅ 记忆系统（SQLite）
- ✅ 灵魂系统（人格管理）
- ✅ 多 Agent 编排器
- ✅ 技能引擎（WASM 基础）

### v0.1.0 (2026-02-25)
- 初始版本
- 基础聊天功能
- 飞书渠道集成

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## 👥 团队

- **总指挥**: Coder YiBai
- **AI Agent**: 旺财 (wangcai@openmonkey.ai)

---

## 📞 联系方式

- **GitHub**: https://github.com/yibaig/OpenMonkey
- **邮箱**: yibaig@openmonkey.ai
- **文档**: https://docs.openmonkey.ai (计划中)

---

## 🙏 致谢

感谢以下开源项目：

- [Tauri](https://tauri.app) - 桌面应用框架
- [wasmtime](https://wasmtime.dev) - WASM 运行时
- [sqlx](https://github.com/launchbadge/sqlx) - Rust SQL 库
- [Qdrant](https://qdrant.tech) - 向量数据库

---

**🐵 OpenMonkey - 聪明、可信、永远在线！**
