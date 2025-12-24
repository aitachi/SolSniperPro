@echo off
echo ========================================
echo SolSniper Pro - Windows 编译工具
echo ========================================
echo.

REM 检查是否安装了 vcpkg
where vcpkg >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [✓] 检测到 vcpkg
) else (
    echo [!] 未检测到 vcpkg
    echo.
    echo 请选择一个选项:
    echo 1. 安装并使用 vcpkg ^(推荐^)
    echo 2. 手动设置 OPENSSL_DIR
    echo 3. 使用 WSL
    echo.
    choice /C 123 /M "请选择"
    if errorlevel 3 goto USE_WSL
    if errorlevel 2 goto MANUAL_OPENSSL
    if errorlevel 1 goto INSTALL_VCPKG
)

:INSTALL_VCPKG
echo.
echo 安装 vcpkg...
echo 请手动执行以下命令:
echo.
echo git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
echo cd C:\vcpkg
echo .\bootstrap-vcpkg.bat
echo .\vcpkg install openssl:x64-windows
echo .\vcpkg integrate install
echo.
pause
goto END

:MANUAL_OPENSSL
echo.
echo 请设置 OPENSSL_DIR 环境变量
echo 例如: set OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
echo.
echo 下载地址: https://slproweb.com/products/Win32OpenSSL.html
echo.
pause
goto END

:USE_WSL
echo.
echo 使用 WSL 编译:
echo.
echo wsl
echo cd /mnt/c/Users/ASUS/Desktop/B-partjob/solsinapor/SolSniperPro-main
echo sudo apt-get install -y libssl-dev pkg-config
echo cargo build
echo.
pause
goto END

:END
echo.
echo 配置完成后，运行 cargo build
pause
