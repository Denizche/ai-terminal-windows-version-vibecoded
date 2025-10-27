@echo off
cd /d "%~dp0"
echo Building AI Terminal...
echo.
call npm install
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to install dependencies
    pause
    exit /b 1
)
echo.
call npm run tauri build
if %ERRORLEVEL% NEQ 0 (
    echo Error: Failed to build application
    pause
    exit /b 1
)
echo.
echo Build completed!
echo Executable created at: src-tauri\target\release\ai-terminal.exe
pause