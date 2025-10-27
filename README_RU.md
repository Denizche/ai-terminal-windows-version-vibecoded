# AI Terminal - Русская документация

Умный терминал с поддержкой ИИ для всех платформ (Windows, macOS, Linux).

![AI Terminal Demo](demo.gif)

## 📋 Возможности

- 🤖 Интерпретация команд на естественном языке
- 🔧 Встроенный ИИ-ассистент
- 📚 История команд и автодополнение
- 🌐 Кроссплатформенная поддержка (Windows, macOS, Linux)
- 🎨 Современный интерфейс на Tauri + Angular
- 🔄 Поддержка нескольких провайдеров ИИ (Ollama, LocalAI, OpenAI)

## 🛠️ Системные требования

- **Node.js** 18+
- **Rust** и Cargo
- **Для Windows**: Visual Studio Build Tools или Visual Studio с C++ инструментами
- **Для ИИ функций**: 
  - Ollama, LocalAI или OpenAI-совместимый API

## 📦 Установка

### Windows

#### Способ 1: Сборка из исходного кода
1. **Установите зависимости:**
   ```cmd
   # Скачайте и установите:
   # - Node.js с https://nodejs.org/
   # - Rust с https://rustup.rs/
   # - Visual Studio Build Tools
   ```

2. **Клонируйте репозиторий:**
   ```cmd
   git clone https://github.com/your-username/ai-terminal.git
   cd ai-terminal\ai-terminal
   ```

3. **Соберите приложение:**
   ```cmd
   npm install
   npm run tauri build
   ```

4. **Установите пакет:**
   - Перейдите в `src-tauri\target\release\bundle\msi\`
   - Запустите `.msi` установщик

#### Способ 2: Автоматическая сборка
```cmd
# Запустите скрипт автоматической сборки
build-windows.bat
```

### macOS

#### Homebrew (рекомендуется)
```bash
brew tap AiTerminalFoundation/ai-terminal
brew install --cask ai-terminal
```

#### Сборка из исходного кода
```bash
git clone https://github.com/your-username/ai-terminal.git
cd ai-terminal/ai-terminal
npm install
npm run tauri dev
```

### Linux

```bash
git clone https://github.com/your-username/ai-terminal.git
cd ai-terminal/ai-terminal
npm install
npm run tauri build
```

## 🚀 Запуск в различных режимах

### 1. Режим разработки

```bash
# Windows (PowerShell/CMD)
cd ai-terminal
npm run tauri dev

# macOS/Linux
cd ai-terminal
npm run tauri dev
```

### 2. Производственная сборка

```bash
# Сборка для производства
npm run tauri build

# Найти собранные файлы:
# Windows: src-tauri\target\release\bundle\
# macOS: src-tauri/target/release/bundle/
# Linux: src-tauri/target/release/bundle/
```

### 3. Веб-режим (только фронтенд)

```bash
# Запуск без Tauri (только веб-интерфейс)
npm run start
# Откроется http://localhost:4200
```

## 🤖 Настройка ИИ

AI Terminal поддерживает несколько провайдеров ИИ:

### Режим 1: Ollama (по умолчанию)

1. **Установка Ollama:**
   ```bash
   # Windows
   # Скачайте с https://ollama.ai/download/windows
   
   # macOS
   brew install ollama
   
   # Linux
   curl -fsSL https://ollama.com/install.sh | sh
   ```

2. **Загрузка модели:**
   ```bash
   ollama pull llama3.2
   ```

3. **Настройка в AI Terminal:**
   ```bash
   /provider ollama
   /host http://localhost:11434
   /model llama3.2
   ```

### Режим 2: LocalAI (localhost:8000)

1. **Запустите ваш локальный ИИ** на порту 8000

2. **Быстрая настройка:**
   ```bash
   /localai your-model-name
   ```

3. **Ручная настройка:**
   ```bash
   /provider localai
   /host http://localhost:8000
   /model gpt-3.5-turbo
   /params temp=0.7 tokens=2048
   ```

4. **Скрипт настройки Windows:**
   ```cmd
   setup-localhost-ai.bat
   ```

### Режим 3: OpenAI API

```bash
/provider openai
/host https://api.openai.com
/model gpt-4
```

## 🎯 Поддержка терминалов

### Windows
- ✅ **Windows Terminal** (рекомендуется)
- ✅ **Command Prompt** (cmd.exe)
- ✅ **PowerShell** (Windows PowerShell & PowerShell Core)
- ✅ **Git Bash** (Unix-подобные команды)

### macOS
- ✅ **Terminal.app**
- ✅ **iTerm2**
- ✅ **Hyper**

### Linux
- ✅ **GNOME Terminal**
- ✅ **Konsole**
- ✅ **xterm**
- ✅ **Alacritty**

## 📝 Команды ИИ

### Основные команды
```bash
/help                          # Показать все команды
/provider [ollama|localai|openai]  # Переключить провайдера ИИ
/host [url]                    # Изменить API endpoint
/model [name]                  # Переключить модель
/models                        # Список доступных моделей
```

### Быстрые настройки
```bash
/localai [model]               # Настроить LocalAI на localhost:8000
/params temp=0.7 tokens=2048   # Настроить параметры ИИ
```

### Проверка статуса
```bash
/provider                      # Текущий провайдер
/host                         # Текущий хост
/model                        # Текущая модель
```

## 🔧 Расширенная конфигурация

### Переменные окружения

```bash
# Windows
set AI_PROVIDER=localai
set AI_HOST=http://localhost:8000
set AI_MODEL=gpt-3.5-turbo

# Linux/macOS
export AI_PROVIDER=localai
export AI_HOST=http://localhost:8000
export AI_MODEL=gpt-3.5-turbo
```

### Файл конфигурации

Создайте `ai-terminal-config.json`:
```json
{
  "provider": "localai",
  "host": "http://localhost:8000",
  "model": "gpt-3.5-turbo",
  "temperature": 0.7,
  "max_tokens": 2048
}
```

## 🌍 Примеры использования

### Базовое использование
```bash
# Запросите помощь на естественном языке
Как вывести список файлов в текущей директории?

# ИИ ответит:
# ```command```
# dir
# ```
```

### Смена провайдеров
```bash
# Настройка локального ИИ
/localai my-local-model

# Вопрос локальному ИИ
Как создать новую папку в Windows?

# Возврат к Ollama
/provider ollama
/model llama3.2

# Вопрос Ollama
Объясни команду mkdir
```

### Настройка параметров
```bash
# Более креативные ответы
/params temp=0.9 tokens=1024

# Более точные ответы  
/params temp=0.3 tokens=512
```

## 🛠️ Разработка

### Структура проекта
```
ai-terminal/
├── src/                    # Angular фронтенд
├── src-tauri/             # Rust бэкенд
│   ├── src/
│   │   ├── command/       # Обработка команд
│   │   ├── ollama/        # ИИ интеграция
│   │   └── utils/         # Утилиты
│   └── Cargo.toml
├── README.md
├── README_RU.md           # Этот файл
└── package.json
```

### Режим разработки с hot reload
```bash
# Терминал 1: Запуск фронтенда
npm run start

# Терминал 2: Запуск Tauri в режиме разработки
npm run tauri dev
```

### Сборка для разных платформ
```bash
# Windows (.msi)
npm run tauri build -- --target x86_64-pc-windows-msvc

# macOS (.dmg, .app)
npm run tauri build -- --target x86_64-apple-darwin

# Linux (.deb, .AppImage)
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

## 🐛 Устранение неполадок

### Windows

**Проблема:** "cargo не найден"
```cmd
# Установите Rust
winget install Rustlang.Rustup
# или скачайте с https://rustup.rs/
```

**Проблема:** Ошибка сборки C++
```cmd
# Установите Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
```

### ИИ подключение

**Проблема:** Connection refused к localhost:8000
```bash
# Проверьте, что ваш ИИ запущен
curl http://localhost:8000/v1/models

# Проверьте настройки файервола
netstat -an | findstr :8000
```

**Проблема:** Модель не найдена
```bash
# Проверьте доступные модели
/models

# Установите правильное имя модели
/model correct-model-name
```

### Права доступа

**Linux/macOS:**
```bash
# Если нет прав на выполнение
chmod +x ai-terminal
sudo ./ai-terminal
```

## 📚 API для разработчиков

### Добавление нового провайдера ИИ

1. Создайте новый вариант в `AIProvider` enum
2. Добавьте обработку в `ask_ai()` функцию
3. Реализуйте специфичную логику запросов

```rust
// src/ollama/types/ai_provider.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    Ollama,
    LocalAI,
    OpenAI,
    YourNewProvider, // Добавьте сюда
}
```

### Добавление новых команд

```rust
// src/utils/command.rs
match command.as_str() {
    "/yourcmd" => {
        // Ваша логика здесь
        Ok("Результат команды".to_string())
    }
}
```

## 🤝 Участие в разработке

1. Форкните репозиторий
2. Создайте ветку для вашей функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'Add amazing feature'`)
4. Отправьте в ветку (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для подробностей.

## 🙏 Благодарности

- [Tauri](https://tauri.app/) - За кроссплатформенную основу
- [Angular](https://angular.io/) - За реактивный фронтенд
- [Ollama](https://ollama.ai/) - За локальную ИИ поддержку
- [Rust](https://www.rust-lang.org/) - За безопасный системный код

## 📞 Поддержка

- 🐛 Сообщить о баге: [GitHub Issues](https://github.com/your-username/ai-terminal/issues)
- 💡 Предложить функцию: [GitHub Discussions](https://github.com/your-username/ai-terminal/discussions)
- 📧 Email: support@ai-terminal.dev

---

**AI Terminal** - Умный терминал будущего! 🚀