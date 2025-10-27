# AI Terminal

A Tauri + Angular terminal application with integrated AI capabilities.
 ![AI Terminal Demo](demo.gif)
## Features

- Natural language command interpretation
- Integrated AI assistant
- Command history and auto-completion
- Cross-platform support (macOS, Windows, Linux)
- Modern UI built with Tauri and Angular

## Requirements

- Node.js 18+
- Rust and Cargo
- For AI features: [Ollama](https://ollama.ai/)
  - macOS: `brew install ollama`
  - Windows: Download from [ollama.ai](https://ollama.ai/download/windows)
  - Linux: `curl -fsSL https://ollama.com/install.sh | sh`

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/ai-terminal.git
   cd ai-terminal
   ```

2. Install dependencies and run the project:
   
   **Windows (PowerShell/Command Prompt):**
   ```cmd
   cd ai-terminal
   npm install
   npm run tauri dev
   ```
   
   **macOS/Linux:**
   ```bash
   cd ai-terminal
   npm install
   npm run tauri dev
   ```

## Installation

### Windows

For Windows users, you can build and install AI Terminal from source:

1. **Prerequisites:**
   - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) or Visual Studio with C++ development tools
   - Install [Node.js](https://nodejs.org/)
   - Install [Rust](https://rustup.rs/)

2. **Build and Install:**
   ```cmd
   git clone https://github.com/your-username/ai-terminal.git
   cd ai-terminal\ai-terminal
   npm install
   npm run tauri build
   ```

3. **Install the built package:**
   - Navigate to `src-tauri\target\release\bundle\msi\`
   - Run the generated `.msi` installer

### macOS (Homebrew)

You can install AI Terminal using Homebrew:

```bash
brew tap AiTerminalFoundation/ai-terminal
brew install --cask ai-terminal
```

After installation, you can launch the application from Spotlight or run it from the terminal:

```bash
ai-terminal
```

## Quick Guide to Using Ollama to Download `macsdeve/BetterBash3` Model

### Linux

1. **Install Ollama**

Open your terminal and run:

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

2. **Download the Model**

Run the following command:

```bash
ollama pull macsdeve/BetterBash3
```

### Windows

1. **Download Ollama**

- Visit [Ollama download page](https://ollama.ai/download/windows).
- Download the Windows installer.

2. **Install Ollama**

- Run the downloaded installer.
- Follow the installation prompts.
- Ollama will be added to your PATH automatically.

3. **Download the Model**

Open Command Prompt or PowerShell and execute:

```cmd
ollama pull macsdeve/BetterBash3
```

**Note for Windows Terminal users:** AI Terminal now fully supports Windows Terminal, Command Prompt, PowerShell, and Git Bash environments.

## Using Your Local AI (localhost:8000)

AI Terminal now supports multiple AI providers, including your local AI running on localhost:8000.

### Quick Setup for LocalAI

1. **Start your local AI server** on `localhost:8000`

2. **Configure AI Terminal:**
   - Run `setup-localhost-ai.bat` for guided setup, or
   - Use the built-in commands (see below)

3. **Built-in Configuration Commands:**
   ```bash
   # Quick setup for localhost:8000
   /localai your-model-name
   
   # Manual configuration
   /provider localai
   /host http://localhost:8000
   /model your-model-name
   /params temp=0.7 tokens=2048
   ```

### Available AI Commands

- `/help` - Show all available commands
- `/provider [ollama|localai|openai]` - Switch AI providers  
- `/localai [model]` - Quick setup for localhost:8000
- `/host [url]` - Change API endpoint
- `/model [name]` - Switch model
- `/models` - List available models
- `/params temp=X tokens=Y` - Set temperature and max tokens

### Supported AI Providers

- **Ollama** (default) - Local Ollama installation
- **LocalAI** - OpenAI-compatible local AI (localhost:8000)
- **OpenAI** - OpenAI API compatible services

### Example Usage

```bash
# Setup for localhost:8000
/localai gpt-3.5-turbo

# Ask your local AI
How do I list files in Windows?

# Switch back to Ollama
/provider ollama
```

### macOS

1. **Download Ollama**

- Visit [Ollama download page](https://ollama.com/download/mac).
- Click **Download for macOS**.

2. **Install Ollama**

- Open the downloaded `.zip` file from your `Downloads` folder.
- Drag the `Ollama.app` into your `Applications` folder.
- Open `Ollama.app` and follow any prompts.

3. **Download the Model**

Open Terminal and execute:

```bash
ollama pull macsdeve/BetterBash3
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT License](LICENSE)
