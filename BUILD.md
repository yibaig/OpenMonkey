# 🏗️ OpenMonkey 构建指南

> **版本**: v0.2.0-Beta  
> **最后更新**: 2026-03-12

---

## 📋 环境要求

### 必需工具

- **Rust**: 1.75+ 
  ```bash
  # 安装 Rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js**: 18+
  ```bash
  # 下载安装：https://nodejs.org/
  ```

- **Git**: 最新版

### 可选工具

- **VS Code**: 推荐编辑器
- **Tauri CLI**: 自动安装

---

## 🔨 开发模式

### 1. 克隆项目

```bash
git clone https://github.com/yibaig/OpenMonkey.git
cd OpenMonkey
```

### 2. 安装依赖

```bash
# 安装 Node.js 依赖
npm install
```

### 3. 启动开发服务器

```bash
# 开发模式运行（自动热更新）
npm run tauri:dev
```

**注意**: 首次运行会下载 Rust 依赖，可能需要 5-10 分钟。

---

## 📦 构建发布版本

### Windows

```bash
# 构建 Windows 安装包
npm run tauri:build
```

**输出位置**: `src-tauri/target/release/bundle/`
- `.msi` 安装包
- `.exe` 便携版

### macOS

```bash
npm run tauri:build
```

**输出位置**: `src-tauri/target/release/bundle/`
- `.dmg` 安装包
- `.app` 应用

### Linux

```bash
npm run tauri:build
```

**输出位置**: `src-tauri/target/release/bundle/`
- `.deb` (Debian/Ubuntu)
- `.rpm` (Fedora/RHEL)
- `.AppImage` (通用)

---

## ⚙️ 配置指南

### 1. 配置灵魂文件

首次启动前，复制配置模板：

```bash
# Windows PowerShell
Copy-Item assets\templates\soul\SOUL.md .
Copy-Item assets\templates\soul\USER.md .
Copy-Item assets\templates\soul\AGENTS.md .
```

### 2. 编辑配置

- `SOUL.md` - 定义 AI 人格
- `USER.md` - 用户信息
- `AGENTS.md` - Agent 配置

### 3. 配置渠道

#### 飞书（Feishu）

1. 登录 [飞书开放平台](https://open.feishu.cn/)
2. 创建企业应用
3. 获取 App ID 和 App Secret
4. 在设置中填写

#### Telegram

1. 联系 [@BotFather](https://t.me/BotFather)
2. 创建 Bot
3. 获取 Bot Token
4. 在设置中填写

---

## 🐛 常见问题

### 1. Rust 编译失败

**问题**: `error: package 'xxx' cannot be built`

**解决**:
```bash
# 更新 Rust 工具链
rustup update
# 清理缓存
cargo clean
# 重新构建
npm run tauri:build
```

### 2. Node.js 依赖安装失败

**问题**: `npm install` 卡住或失败

**解决**:
```bash
# 使用国内镜像
npm config set registry https://registry.npmmirror.com
# 重新安装
npm install
```

### 3. Windows 构建失败

**问题**: 缺少 Visual Studio 构建工具

**解决**:
1. 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. 选择 "C++ 生成工具"
3. 重启后重试

### 4. macOS 签名问题

**问题**: 应用无法打开

**解决**:
```bash
# 临时解决：移除隔离属性
xattr -cr /Applications/OpenMonkey.app
```

---

## 📊 构建时间参考

| 平台 | 首次构建 | 增量构建 |
|------|----------|----------|
| Windows | ~15 分钟 | ~3 分钟 |
| macOS | ~20 分钟 | ~5 分钟 |
| Linux | ~18 分钟 | ~4 分钟 |

---

## 🎯 验证构建

### 运行测试

```bash
# Rust 测试
cargo test

# 前端测试
npm test
```

### 检查构建产物

```bash
# Windows
dir src-tauri\target\release\bundle

# macOS/Linux
ls src-tauri/target/release/bundle/
```

---

## 📞 获取帮助

- **GitHub Issues**: https://github.com/yibaig/OpenMonkey/issues
- **文档**: https://docs.openmonkey.ai
- **邮箱**: support@openmonkey.ai

---

**🐵 构建愉快！**
