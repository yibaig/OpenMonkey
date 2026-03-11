# 🚀 OpenMonkey 快速开始指南

> **5 分钟上手，开启你的 AI 助手之旅！**

---

## ⚡ 快速开始（3 步搞定）

### 步骤 1: 下载

#### 方式 A: 下载预编译包（推荐）

访问 [Releases](https://github.com/yibaig/OpenMonkey/releases) 下载最新版本：

- **Windows**: `OpenMonkey_x64_en-US.msi`
- **macOS**: `OpenMonkey_x64.dmg`
- **Linux**: `OpenMonkey_amd64.deb`

#### 方式 B: 源码编译

```bash
git clone https://github.com/yibaig/OpenMonkey.git
cd OpenMonkey
npm install
npm run tauri:dev
```

### 步骤 2: 配置

首次启动会自动创建配置文件：

```
SOUL.md      - AI 人格配置
USER.md      - 用户信息
AGENTS.md    - Agent 配置
```

**最少配置**（只需修改 `USER.md`）:

```markdown
# USER.md - 关于你

- **Name**: 你的名字
- **Timezone**: Asia/Shanghai
- **偏好**:
  - 沟通风格：简洁
  - 语言：中文
```

### 步骤 3: 启动

**Windows**: 双击 `一键启动.bat`

**macOS/Linux**:
```bash
./一键启动.sh
# 或
npm run tauri:dev
```

---

## 🎯 第一次使用

### 1. 配置飞书（可选）

如果你想通过飞书使用 OpenMonkey：

1. 访问 [飞书开放平台](https://open.feishu.cn/)
2. 创建企业应用
3. 获取 App ID 和 App Secret
4. 在 OpenMonkey 设置中填写

### 2. 测试聊天

启动后，在聊天窗口输入：

```
你好，介绍一下你自己
```

### 3. 使用技能

输入：

```
计算 25 + 17
```

---

## 📚 常用操作

### 查看帮助

```
/help
```

### 切换人格

```
/switch personality 旺财
```

### 查看记忆

```
/memory list
```

### 导入技能

```
/import skill [技能文件路径]
```

---

## 🛠️ 故障排查

### 启动失败

**问题**: 双击无反应

**解决**:
1. 检查是否安装 Rust 和 Node.js
2. 查看日志：`logs/openmonkey.log`
3. 重新运行 `一键启动.bat`

### 无法连接飞书

**问题**: 飞书消息不响应

**解决**:
1. 检查 App ID 和 Secret 是否正确
2. 确认飞书应用已发布
3. 查看飞书开放平台日志

### 编译错误

**问题**: `cargo check` 失败

**解决**:
```bash
# 更新 Rust
rustup update
# 清理缓存
cargo clean
# 重新编译
npm run tauri:build
```

---

## 📞 获取帮助

- **文档**: https://docs.openmonkey.ai
- **Issues**: https://github.com/yibaig/OpenMonkey/issues
- **邮箱**: support@openmonkey.ai
- **社区**: Discord（计划中）

---

## 🎉 下一步

- [ ] 阅读 [完整文档](docs/)
- [ ] 创建你的第一个技能
- [ ] 配置多渠道支持
- [ ] 加入社区

---

**🐵 欢迎使用 OpenMonkey！**
