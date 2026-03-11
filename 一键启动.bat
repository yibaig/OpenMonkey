# 🚀 OpenMonkey 一键启动

@echo off
echo ========================================
echo   🐵 OpenMonkey v0.2.0-Beta
echo   一键启动脚本
echo ========================================
echo.

REM 检查 Node.js
where node >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未检测到 Node.js
    echo 请先安装 Node.js: https://nodejs.org/
    pause
    exit /b 1
)

REM 检查 Rust
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未检测到 Rust
    echo 请先安装 Rust: https://rustup.rs/
    pause
    exit /b 1
)

echo [检查] 环境正常
echo.

REM 检查依赖
if not exist "node_modules" (
    echo [安装] 正在安装 Node.js 依赖...
    echo 首次安装可能需要 5-10 分钟
    echo.
    call npm install
    if %ERRORLEVEL% NEQ 0 (
        echo [错误] 依赖安装失败
        pause
        exit /b 1
    )
    echo [完成] 依赖安装完成
    echo.
)

REM 检查配置文件
if not exist "SOUL.md" (
    echo [配置] 复制灵魂配置文件...
    copy "assets\templates\soul\SOUL.md" SOUL.md
    copy "assets\templates\soul\USER.md" USER.md
    copy "assets\templates\soul\AGENTS.md" AGENTS.md
    echo [提示] 请编辑 SOUL.md、USER.md 和 AGENTS.md 进行配置
    echo.
)

echo ========================================
echo   启动选项
echo ========================================
echo.
echo   1. 开发模式 (tauri:dev)
echo   2. 构建发布版 (tauri:build)
echo   3. 仅编译检查
echo   4. 退出
echo.

set /p choice="请选择 (1-4): "

if "%choice%"=="1" goto dev
if "%choice%"=="2" goto build
if "%choice%"=="3" goto check
if "%choice%"=="4" goto end

echo [错误] 无效选择
pause
exit /b 1

:dev
echo.
echo [启动] 开发模式...
echo 提示：按 Ctrl+C 可停止
echo.
call npm run tauri:dev
goto end

:build
echo.
echo [构建] 发布版本...
echo 首次构建可能需要 15-20 分钟
echo.
call npm run tauri:build
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 构建失败
    pause
    exit /b 1
)
echo.
echo [完成] 构建成功！
echo 安装包位置：src-tauri\target\release\bundle\
pause
goto end

:check
echo.
echo [检查] 编译验证...
call cargo check --workspace
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 编译检查失败
    pause
    exit /b 1
)
echo.
echo [完成] 编译检查通过！
pause
goto end

:end
echo.
echo ========================================
echo   感谢使用 OpenMonkey！
echo ========================================
pause
