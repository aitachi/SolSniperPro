@echo off
REM SolSniper Pro - Windows 一键启动 WSL 设置脚本

echo ==========================================
echo SolSniper Pro - WSL 环境自动设置
echo ==========================================
echo.
echo 此脚本将在 WSL 中自动配置 Rust 编译环境
echo.
echo 需要的时间: 约 5-10 分钟
echo 需要的权限: sudo (会提示输入密码)
echo.
pause

echo.
echo 正在启动 WSL 并运行设置脚本...
echo.

REM 转到项目目录并在 WSL 中运行设置脚本
cd /d "%~dp0"
wsl -d Ubuntu-22.04 bash -c "cd '/mnt/c/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main' && chmod +x setup_wsl.sh && ./setup_wsl.sh"

echo.
echo ==========================================
echo 设置完成！
echo ==========================================
echo.
echo 现在可以使用以下命令编译和运行:
echo.
echo   wsl
echo   cd /mnt/c/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main
echo   cargo build --release
echo   cargo run --bin solsniper-api-server
echo.
pause
