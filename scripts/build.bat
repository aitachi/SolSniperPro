@echo off
REM SolSniper Pro 编译脚本 (Windows)

echo ==================================
echo   SolSniper Pro 构建脚本
echo ==================================

echo 检查Rust版本...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo 错误: 未安装Rust,请先安装: https://rustup.rs
    exit /b 1
)

echo 清理旧的构建文件...
cargo clean

echo 编译所有模块...
cargo build --release
if errorlevel 1 (
    echo 编译失败!
    exit /b 1
)

echo 运行测试...
cargo test --release
if errorlevel 1 (
    echo 测试失败!
    exit /b 1
)

echo 检查代码格式...
cargo fmt -- --check
if errorlevel 1 (
    echo 警告: 代码格式不规范,运行 'cargo fmt' 修复
)

echo 运行Clippy检查...
cargo clippy -- -D warnings
if errorlevel 1 (
    echo 警告: Clippy发现问题,请修复
)

echo ==================================
echo   构建完成!
echo ==================================
echo.
echo 构建产物位于: target\release\
echo.
echo 下一步:
echo 1. 配置 .env 文件
echo 2. 配置 config.toml
echo 3. 运行相关服务

pause
