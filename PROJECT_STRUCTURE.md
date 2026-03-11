# 📁 OpenMonkey-Trae 项目目录结构

> **版本**: v2.0  
> **更新日期**: 2026-03-12

---

## 🌳 完整目录树

```
OpenMonkey-Trae/
│
├── 📄 配置文件
│   ├── package.json              # Node.js 依赖配置
│   ├── tsconfig.json             # TypeScript 配置
│   ├── vite.config.ts            # Vite 构建配置
│   ├── tailwind.config.js        # TailwindCSS 配置
│   ├── postcss.config.js         # PostCSS 配置
│   ├── rust-toolchain.toml       # Rust 工具链配置
│   └── .gitignore                # Git 忽略规则
│
├── 📄 Rust 配置
│   ├── Cargo.toml                # Rust 项目配置
│   ├── Cargo.lock                # Rust 依赖锁定
│   └── build.rs                  # Rust 构建脚本
│
├── 📄 文档
│   ├── README.md                 # 项目说明
│   ├── DEVELOPMENT_ROADMAP.md    # 开发路线图
│   ├── 启动文档.md                # 启动指南
│   ├── example_skill.md          # 技能示例
│   ├── example_email_skill.md    # 邮件技能示例
│   └── example_web_search_skill.md # 搜索技能示例
│
├── 📁 src/                       # 前端源代码 (React + TypeScript)
│   ├── main.tsx                  # 应用入口
│   ├── App.tsx                   # 主应用组件
│   ├── App.css                   # 应用样式
│   ├── index.css                 # 全局样式
│   ├── types.ts                  # TypeScript 类型定义
│   │
│   ├── 📁 components/            # React 组件
│   │   ├── Chat.tsx              # 聊天界面
│   │   ├── Settings.tsx          # 设置界面
│   │   ├── ChannelSettings.tsx   # 渠道配置
│   │   ├── SkillImporter.tsx     # 技能导入器
│   │   └── SoulStudio.tsx        # 灵魂工作室
│   │
│   ├── 📁 store/                 # 状态管理 (Zustand)
│   │   └── useAppStore.ts        # 全局状态存储
│   │
│   └── 📁 assets/                # 前端资源
│       └── logo.png              # 应用 Logo
│
├── 📁 src-tauri/                 # Rust 后端源代码
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── tauri.conf.json           # Tauri 配置
│   ├── build.rs                  # 构建脚本
│   │
│   ├── 📁 src/                   # Rust 源代码
│   │   ├── main.rs               # 主入口 (桌面应用)
│   │   ├── lib.rs                # 库入口
│   │   ├── commands.rs           # Tauri 命令处理
│   │   ├── util.rs               # 工具函数
│   │   │
│   │   ├── 📁 core/              # 核心模块
│   │   │   ├── mod.rs            # 模块导出
│   │   │   ├── channel_manager.rs    # 渠道管理器
│   │   │   ├── config_store.rs       # 配置存储
│   │   │   ├── memory_bank.rs        # 记忆银行
│   │   │   ├── skill_adapter.rs      # 技能适配器
│   │   │   ├── soul_manager.rs       # 灵魂管理器
│   │   │   ├── reflector.rs          # 反射器
│   │   │   ├── secure_vault.rs       # 安全保险箱
│   │   │   ├── stats.rs              # 统计分析
│   │   │   │
│   │   │   └── 📁 channels/      # 渠道实现
│   │   │       ├── mod.rs            # 模块导出
│   │   │       ├── feishu.rs         # 飞书渠道
│   │   │       ├── feishu_ws.rs      # 飞书 WebSocket
│   │   │       ├── telegram.rs       # Telegram 渠道
│   │   │       └── wecom.rs          # 企业微信渠道
│   │   │
│   │   └── 📁 llm/               # LLM 客户端
│   │       ├── mod.rs                # 模块导出
│   │       ├── client.rs             # LLM 客户端
│   │       └── prompt_templates.rs   # Prompt 模板
│   │
│   └── 📁 icons/                 # 应用图标
│       ├── 32x32.png
│       ├── 128x128.png
│       ├── 128x128@2x.png
│       ├── favicon.ico
│       └── icon.ico
│
├── 📁 assets/                    # 模板资源
│   ├── 📁 templates/
│   │   ├── 📁 skills/            # 技能模板
│   │   │   ├── OM_SKILL_FORMAT.md      # 技能格式说明
│   │   │   ├── om_skill_pdf_tool.md    # PDF 工具技能示例
│   │   │   └── om_skill_web_design.md  # 网页设计技能示例
│   │   │
│   │   └── 📁 soul/              # 灵魂模板
│   │       ├── AGENTS.md               # Agent 配置模板
│   │       ├── SOUL.md                 # 灵魂配置模板
│   │       └── USER.md                 # 用户配置模板
│
├── 📁 public/                    # 静态资源
│   ├── icon.ico                  # 应用图标
│   └── logo.png                  # Logo
│
├── 📁 icon/                      # 额外图标包
│   ├── 32x32.png
│   ├── 128x128.png
│   ├── android-chrome-192x192.png
│   ├── android-chrome-512x512.png
│   ├── apple-touch-icon.png
│   └── favicon-*.png
│
├── 📁 dist/                      # 构建输出 (前端)
│   ├── index.html
│   ├── logo.png
│   ├── icon.ico
│   └── assets/
│       ├── index-*.css
│       └── index-*.js
│
├── 📁 node_modules/              # Node.js 依赖 (自动生成)
│
├── 📁 target/                    # Rust 编译产物 (自动生成)
│   └── debug/
│       └── .fingerprint/
│
└── 📁 .git/                      # Git 版本控制 (自动生成)
```

---

## 📊 代码统计

### 前端 (TypeScript/React)
| 文件 | 行数 | 说明 |
|------|------|------|
| `src/App.tsx` | ~300 | 主应用组件 |
| `src/components/Chat.tsx` | ~400 | 聊天界面 |
| `src/components/SoulStudio.tsx` | ~250 | 灵魂工作室 |
| `src/components/SkillImporter.tsx` | ~200 | 技能导入 |
| `src/store/useAppStore.ts` | ~150 | 状态管理 |
| **总计** | **~1,300 行** | |

### 后端 (Rust)
| 文件 | 行数 | 说明 |
|------|------|------|
| `src-tauri/src/core/memory_bank.rs` | ~400 | 记忆银行 |
| `src-tauri/src/core/soul_manager.rs` | ~350 | 灵魂管理 |
| `src-tauri/src/core/skill_adapter.rs` | ~300 | 技能适配 |
| `src-tauri/src/core/channel_manager.rs` | ~250 | 渠道管理 |
| `src-tauri/src/core/secure_vault.rs` | ~200 | 安全存储 |
| `src-tauri/src/llm/client.rs` | ~300 | LLM 客户端 |
| `src-tauri/src/commands.rs` | ~200 | 命令处理 |
| **总计** | **~2,000 行** | |

### 核心模块说明

#### 🔐 核心模块 (`src-tauri/src/core/`)

| 模块 | 功能 |
|------|------|
| `memory_bank.rs` | 四层记忆系统（短期/长期/程序/情感） |
| `soul_manager.rs` | 人格系统（SOUL/USER/AGENTS 管理） |
| `skill_adapter.rs` | 技能加载与执行（支持热更新） |
| `channel_manager.rs` | 多渠道管理（飞书/Telegram/企微） |
| `secure_vault.rs` | 敏感信息加密存储 |
| `reflector.rs` | 自我反思与学习 |
| `stats.rs` | 使用统计与分析 |

#### 🌐 渠道实现 (`src-tauri/src/core/channels/`)

| 渠道 | 文件 | 状态 |
|------|------|------|
| 飞书 | `feishu.rs` | ✅ 完整支持 |
| 飞书 WebSocket | `feishu_ws.rs` | ✅ 实时推送 |
| Telegram | `telegram.rs` | ✅ 基础支持 |
| 企业微信 | `wecom.rs` | 🚧 开发中 |

#### 🤖 LLM 客户端 (`src-tauri/src/llm/`)

| 功能 | 文件 |
|------|------|
| 多模型支持 | `client.rs` |
| Prompt 模板 | `prompt_templates.rs` |

---

## 🎯 关键文件说明

### 前端核心

#### `src/App.tsx`
```typescript
// 主应用组件
- 路由管理
- 主题切换
- 全局布局
```

#### `src/components/Chat.tsx`
```typescript
// 聊天界面
- 消息列表渲染
- 输入框处理
- 实时消息推送
- Markdown 渲染
```

#### `src/components/SoulStudio.tsx`
```typescript
// 灵魂工作室
- 人格配置编辑
- 记忆查看
- 技能管理
```

### 后端核心

#### `src-tauri/src/main.rs`
```rust
// 桌面应用入口
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_message,
            get_history,
            import_skill,
            // ...
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### `src-tauri/src/core/memory_bank.rs`
```rust
// 记忆银行
pub struct MemoryBank {
    short_term: Vec<Memory>,    // 短期记忆
    long_term: Vec<Memory>,     // 长期记忆
    procedural: Vec<Memory>,    // 程序记忆
    emotional: Vec<Memory>,     // 情感记忆
}
```

#### `src-tauri/src/core/skill_adapter.rs`
```rust
// 技能适配器
pub struct SkillAdapter {
    skills: HashMap<String, Skill>,
    wasm_runtime: Option<WasmRuntime>,
}
```

---

## 📦 依赖包

### Node.js 主要依赖
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "zustand": "^4.5.0",
    "@tauri-apps/api": "^2.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.4.0",
    "@tauri-apps/cli": "^2.0.0"
  }
}
```

### Rust 主要依赖
```toml
[dependencies]
tauri = { version = "2.0", features = [...] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
qdrant-client = "1.0"  # 向量数据库（计划中）
```

---

## 🚀 快速开始

### 1. 安装依赖
```bash
# Node.js 依赖
npm install

# Rust 依赖（自动下载）
cargo build
```

### 2. 开发模式
```bash
npm run tauri:dev
```

### 3. 构建发布
```bash
npm run tauri:build
```

---

## 📝 文件用途速查

| 要找什么 | 文件路径 |
|----------|----------|
| 修改聊天界面 | `src/components/Chat.tsx` |
| 修改记忆系统 | `src-tauri/src/core/memory_bank.rs` |
| 添加新渠道 | `src-tauri/src/core/channels/` |
| 修改技能系统 | `src-tauri/src/core/skill_adapter.rs` |
| 添加新命令 | `src-tauri/src/commands.rs` |
| 修改 UI 样式 | `src/App.css` / `src/index.css` |
| 配置 Tauri | `src-tauri/tauri.conf.json` |
| 配置构建 | `vite.config.ts` |

---

**总指挥，这就是完整的目录结构！需要我详细解释哪个模块？** 🐕📁
