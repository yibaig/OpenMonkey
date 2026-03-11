#!/bin/bash

echo "========================================"
echo "  🐵 OpenMonkey v0.2.0-Beta"
echo "  一键启动脚本 (macOS/Linux)"
echo "========================================"
echo ""

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "[错误] 未检测到 Node.js"
    echo "请先安装 Node.js: https://nodejs.org/"
    exit 1
fi

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "[错误] 未检测到 Rust"
    echo "请先安装 Rust: https://rustup.rs/"
    exit 1
fi

echo "[检查] 环境正常"
echo ""

# 检查依赖
if [ ! -d "node_modules" ]; then
    echo "[安装] 正在安装 Node.js 依赖..."
    echo "首次安装可能需要 5-10 分钟"
    echo ""
    npm install
    if [ $? -ne 0 ]; then
        echo "[错误] 依赖安装失败"
        exit 1
    fi
    echo "[完成] 依赖安装完成"
    echo ""
fi

# 检查配置文件
if [ ! -f "SOUL.md" ]; then
    echo "[配置] 复制灵魂配置文件..."
    cp "assets/templates/soul/SOUL.md" SOUL.md
    cp "assets/templates/soul/USER.md" USER.md
    cp "assets/templates/soul/AGENTS.md" AGENTS.md
    echo "[提示] 请编辑 SOUL.md、USER.md 和 AGENTS.md 进行配置"
    echo ""
fi

echo "========================================"
echo "  启动选项"
echo "========================================"
echo ""
echo "  1. 开发模式 (tauri:dev)"
echo "  2. 构建发布版 (tauri:build)"
echo "  3. 仅编译检查"
echo "  4. 退出"
echo ""

read -p "请选择 (1-4): " choice

case $choice in
    1)
        echo ""
        echo "[启动] 开发模式..."
        echo "提示：按 Ctrl+C 可停止"
        echo ""
        npm run tauri:dev
        ;;
    2)
        echo ""
        echo "[构建] 发布版本..."
        echo "首次构建可能需要 15-20 分钟"
        echo ""
        npm run tauri:build
        if [ $? -ne 0 ]; then
            echo "[错误] 构建失败"
            exit 1
        fi
        echo ""
        echo "[完成] 构建成功！"
        echo "安装包位置：src-tauri/target/release/bundle/"
        ;;
    3)
        echo ""
        echo "[检查] 编译验证..."
        cargo check --workspace
        if [ $? -ne 0 ]; then
            echo "[错误] 编译检查失败"
            exit 1
        fi
        echo ""
        echo "[完成] 编译检查通过！"
        ;;
    4)
        echo ""
        echo "退出"
        exit 0
        ;;
    *)
        echo "[错误] 无效选择"
        exit 1
        ;;
esac

echo ""
echo "========================================"
echo "  感谢使用 OpenMonkey!"
echo "========================================"
