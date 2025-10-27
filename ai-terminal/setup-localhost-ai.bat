@echo off
echo Setting up AI Terminal for LocalAI on localhost:8000
echo.

echo This will configure the AI Terminal to use your local AI running on localhost:8000
echo.

set /p model_name="Enter your model name (or press Enter for default): "
if "%model_name%"=="" set model_name=gpt-3.5-turbo

echo.
echo Configuration:
echo - Provider: LocalAI (OpenAI-compatible)
echo - Host: http://localhost:8000
echo - Model: %model_name%
echo.

echo Once the AI Terminal is running, you can:
echo 1. Use /localai %model_name% to configure LocalAI
echo 2. Use /provider localai to switch to LocalAI provider
echo 3. Use /host http://localhost:8000 to set the host
echo 4. Use /params temp=0.7 tokens=2048 to adjust parameters
echo.

echo Available commands in AI Terminal:
echo - /help                    - Show all commands
echo - /localai [model]         - Quick setup for localhost:8000
echo - /provider [ollama^|localai] - Switch AI providers
echo - /host [url]              - Change API endpoint
echo - /model [name]            - Change model
echo - /params temp=X tokens=Y  - Set AI parameters
echo.

pause