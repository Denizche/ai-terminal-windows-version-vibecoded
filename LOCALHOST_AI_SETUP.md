# LocalAI Integration for AI Terminal

This document outlines the changes made to support your local AI running on localhost:8000.

## 🚀 What's New

AI Terminal now supports multiple AI providers:
- **Ollama** (default) - `http://localhost:11434`
- **LocalAI** - `http://localhost:8000` (your local AI)
- **OpenAI** - OpenAI API compatible endpoints

## 🔧 Technical Changes

### 1. New AI Provider System
- Created `AIProvider` enum with support for Ollama, LocalAI, and OpenAI
- Added flexible request/response types for different API formats
- Ollama uses simple `{model, prompt, stream}` format
- LocalAI uses OpenAI-compatible `{model, messages[], temperature, max_tokens}` format

### 2. Enhanced AI State Management
- Extended `OllamaState` to include provider type, temperature, and max_tokens
- Added provider switching capabilities
- Maintains separate configurations for each provider

### 3. Multi-Provider Request Handler
- `ask_ai()` function now routes to appropriate provider
- `ask_ollama_ai()` - handles Ollama API format
- `ask_local_ai()` - handles OpenAI-compatible format with chat messages

### 4. New Commands Added
- `/provider [name]` - Switch between AI providers
- `/localai [model]` - Quick setup for localhost:8000
- `/params temp=X tokens=Y` - Set AI parameters
- Enhanced `/help` with new commands

### 5. Tauri Integration
- Added new functions to main.rs: `set_provider`, `get_provider`, `setup_local_ai`, `set_ai_params`
- All functions are exposed to the frontend

## 📋 Quick Setup Steps

### Option 1: One Command Setup
```bash
/localai your-model-name
```

### Option 2: Manual Configuration
```bash
/provider localai
/host http://localhost:8000
/model your-model-name
/params temp=0.7 tokens=2048
```

### Option 3: GUI Setup Script
Run `setup-localhost-ai.bat` for guided configuration

## 🔌 API Compatibility

Your localhost:8000 AI should support OpenAI-compatible endpoints:

### Expected Endpoint
```
POST http://localhost:8000/v1/chat/completions
```

### Request Format
```json
{
  "model": "your-model-name",
  "messages": [
    {"role": "system", "content": "system prompt"},
    {"role": "user", "content": "user question"}
  ],
  "temperature": 0.7,
  "max_tokens": 2048,
  "stream": false
}
```

### Response Format
```json
{
  "choices": [
    {
      "message": {
        "role": "assistant",
        "content": "AI response"
      }
    }
  ]
}
```

## 🛠️ Advanced Configuration

### Environment Detection
The AI automatically detects your operating system and provides context-appropriate responses.

### Temperature Control
- Range: 0.0 (deterministic) to 1.0 (creative)
- Default: 0.7
- Set with: `/params temp=0.8`

### Token Limits
- Control response length
- Default: 2048 tokens
- Set with: `/params tokens=1024`

### Provider Status
Check current configuration:
```bash
/provider    # Shows current provider and host
/model       # Shows current model
/host        # Shows current API host
```

## 🔄 Switching Between Providers

```bash
# Use your localhost:8000 AI
/provider localai

# Switch back to Ollama
/provider ollama

# Use OpenAI API
/provider openai
/host https://api.openai.com
```

## 🚨 Troubleshooting

### Common Issues

1. **Connection refused**
   - Ensure your AI is running on localhost:8000
   - Check firewall settings

2. **Model not found**
   - Verify the model name with `/model your-actual-model-name`
   - Check your AI server's available models

3. **API format errors**
   - Ensure your localhost:8000 supports OpenAI-compatible format
   - Check the request/response format above

### Debug Commands

```bash
/provider          # Check current provider
/host              # Check current host
/model             # Check current model
/params temp=0.7   # Test parameter setting
```

## 📁 Files Modified

- `src/ollama/types/ai_provider.rs` - New provider types
- `src/ollama/types/ollama_state.rs` - Enhanced state management  
- `src/ollama/model_request/request.rs` - Multi-provider request handler
- `src/utils/command.rs` - New special commands
- `src/main.rs` - Registered new functions
- `README.md` - Updated documentation
- `setup-localhost-ai.bat` - Setup script

Your AI Terminal is now ready to work with your localhost:8000 AI! 🎉