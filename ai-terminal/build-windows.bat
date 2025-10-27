@echo off
echo Building AI Terminal for Windows...
echo.

REM Check if Node.js is installed
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Node.js is not installed or not in PATH
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)

REM Check if Rust is installed
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Installing Node.js dependencies...
npm install
if %errorlevel% neq 0 (
    echo Error: Failed to install Node.js dependencies
    pause
    exit /b 1
)

echo.
echo Building Tauri application...
npm run tauri build
if %errorlevel% neq 0 (
    echo Error: Failed to build Tauri application
    pause
    exit /b 1
)

echo.
echo Build completed successfully!
echo You can find the installer in: src-tauri\target\release\bundle\
echo.
pause