@echo off
setlocal enabledelayedexpansion
cls

echo ========================================
echo    AI Terminal - Windows Builder
echo ========================================
echo.

REM Set colors
for /F %%a in ('echo prompt $E ^| cmd') do set "ESC=%%a"

REM Check if running from correct directory
if not exist "package.json" (
    echo %ESC%[91mError: package.json not found!%ESC%[0m
    echo Please run this script from the ai-terminal directory
    echo.
    echo Current directory: %CD%
    echo Expected files: package.json, src-tauri folder
    echo.
    pause
    exit /b 1
)

REM Check if it's a Tauri project
if not exist "src-tauri" (
    echo %ESC%[91mError: src-tauri folder not found!%ESC%[0m
    echo This doesn't appear to be a Tauri project
    pause
    exit /b 1
)

echo %ESC%[92m✓ Project structure validated%ESC%[0m
echo.

REM Check Node.js
echo Checking Node.js installation...
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo %ESC%[91m✗ Node.js is not installed or not in PATH%ESC%[0m
    echo Please install Node.js from https://nodejs.org/
    echo Recommended version: 18.x or higher
    pause
    exit /b 1
) else (
    for /f %%i in ('node --version') do set NODE_VERSION=%%i
    echo %ESC%[92m✓ Node.js !NODE_VERSION! found%ESC%[0m
)

REM Check npm
npm --version >nul 2>&1
if %errorlevel% neq 0 (
    echo %ESC%[91m✗ npm is not available%ESC%[0m
    pause
    exit /b 1
) else (
    for /f %%i in ('npm --version') do set NPM_VERSION=%%i
    echo %ESC%[92m✓ npm !NPM_VERSION! found%ESC%[0m
)

REM Check Rust
echo Checking Rust installation...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo %ESC%[91m✗ Rust/Cargo is not installed or not in PATH%ESC%[0m
    echo Please install Rust from https://rustup.rs/
    echo.
    echo After installation, restart your terminal and run this script again
    pause
    exit /b 1
) else (
    for /f "tokens=2" %%i in ('cargo --version') do set CARGO_VERSION=%%i
    echo %ESC%[92m✓ Cargo !CARGO_VERSION! found%ESC%[0m
)

REM Check Tauri CLI
echo Checking Tauri CLI...
npm list @tauri-apps/cli >nul 2>&1
if %errorlevel% neq 0 (
    echo %ESC%[93m! Tauri CLI not found in project dependencies%ESC%[0m
    echo Installing Tauri CLI...
    npm install @tauri-apps/cli@latest --save-dev
    if %errorlevel% neq 0 (
        echo %ESC%[91m✗ Failed to install Tauri CLI%ESC%[0m
        pause
        exit /b 1
    )
) else (
    echo %ESC%[92m✓ Tauri CLI found%ESC%[0m
)

echo.
echo ========================================
echo Starting build process...
echo ========================================
echo.

REM Clean previous builds
echo %ESC%[93mCleaning previous builds...%ESC%[0m
if exist "src-tauri\target" (
    rmdir /s /q "src-tauri\target" 2>nul
    echo %ESC%[92m✓ Cleaned target directory%ESC%[0m
) else (
    echo %ESC%[92m✓ No previous builds to clean%ESC%[0m
)

if exist "dist" (
    rmdir /s /q "dist" 2>nul
    echo %ESC%[92m✓ Cleaned dist directory%ESC%[0m
)
echo.

REM Install/Update dependencies
echo %ESC%[93mInstalling Node.js dependencies...%ESC%[0m
npm install
if %errorlevel% neq 0 (
    echo %ESC%[91m✗ Failed to install Node.js dependencies%ESC%[0m
    echo.
    echo Try running: npm cache clean --force
    echo Then run this script again
    pause
    exit /b 1
)
echo %ESC%[92m✓ Dependencies installed successfully%ESC%[0m
echo.

REM Build frontend
echo %ESC%[93mBuilding Angular frontend...%ESC%[0m
npm run build
if %errorlevel% neq 0 (
    echo %ESC%[91m✗ Failed to build Angular frontend%ESC%[0m
    pause
    exit /b 1
)
echo %ESC%[92m✓ Frontend built successfully%ESC%[0m
echo.

REM Build Tauri app
echo %ESC%[93mBuilding Tauri application (this may take several minutes)...%ESC%[0m
echo Please be patient while Rust compiles the application...
echo.

npm run tauri build
if %errorlevel% neq 0 (
    echo.
    echo %ESC%[91m✗ Failed to build Tauri application%ESC%[0m
    echo.
    echo Common solutions:
    echo 1. Make sure you have Visual Studio Build Tools installed
    echo 2. Try: rustup update
    echo 3. Try: cargo clean (in src-tauri folder)
    echo 4. Check Windows Defender isn't blocking the build
    pause
    exit /b 1
)

echo.
echo %ESC%[92m========================================%ESC%[0m
echo %ESC%[92m    BUILD COMPLETED SUCCESSFULLY!%ESC%[0m
echo %ESC%[92m========================================%ESC%[0m
echo.

REM Find and display build artifacts
echo %ESC%[93mBuild artifacts:%ESC%[0m
echo.

set "BUNDLE_DIR=src-tauri\target\release\bundle"
if exist "%BUNDLE_DIR%" (
    echo %ESC%[96mBundle directory: %CD%\%BUNDLE_DIR%%ESC%[0m
    echo.
    
    REM Check for MSI installer
    if exist "%BUNDLE_DIR%\msi" (
        echo %ESC%[92m✓ MSI Installer:%ESC%[0m
        for %%f in ("%BUNDLE_DIR%\msi\*.msi") do (
            echo   %%~nxf
            set "MSI_PATH=%%f"
        )
        echo.
    )
    
    REM Check for NSIS installer
    if exist "%BUNDLE_DIR%\nsis" (
        echo %ESC%[92m✓ NSIS Installer:%ESC%[0m
        for %%f in ("%BUNDLE_DIR%\nsis\*.exe") do (
            echo   %%~nxf
            set "NSIS_PATH=%%f"
        )
        echo.
    )
    
    REM Check for executable
    if exist "src-tauri\target\release\*.exe" (
        echo %ESC%[92m✓ Executable:%ESC%[0m
        for %%f in ("src-tauri\target\release\*.exe") do (
            echo   %%~nxf
            set "EXE_PATH=%%f"
        )
        echo.
    )
) else (
    echo %ESC%[91m! Bundle directory not found%ESC%[0m
    echo Looking for executable in target\release...
    if exist "src-tauri\target\release\*.exe" (
        echo %ESC%[92m✓ Found executable:%ESC%[0m
        for %%f in ("src-tauri\target\release\*.exe") do echo   %%~nxf
    )
)

echo.
echo %ESC%[93mNext steps:%ESC%[0m
echo 1. Test the executable by running it
echo 2. To install: Run the .msi file (if available)
echo 3. To distribute: Share the installer or executable
echo.

REM Ask if user wants to open the build directory
set /p OPEN_DIR="Open build directory? (y/n): "
if /i "%OPEN_DIR%"=="y" (
    if exist "%BUNDLE_DIR%" (
        explorer "%CD%\%BUNDLE_DIR%"
    ) else (
        explorer "%CD%\src-tauri\target\release"
    )
)

echo.
echo %ESC%[92mBuild script completed!%ESC%[0m
echo.
pause