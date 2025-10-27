# AI Terminal Build Instructions (.exe)

## Available Build Scripts

You have several options for building AI Terminal into a Windows executable:

### 1. `build-windows.bat` - Simple Batch Script
**Recommended for most users**

```cmd
build-windows.bat
```

Features:
- ✅ Simple and reliable build process
- ✅ Proper error handling and user feedback
- ✅ Automatic dependency installation
- ✅ Creates ai-terminal.exe executable
- ✅ Works consistently on Windows systems

## Prerequisites

### Required:

1. **Node.js 18+**
   - Download: https://nodejs.org/
   - Verify: `node --version`

2. **Rust & Cargo**
   - Download: https://rustup.rs/
   - Verify: `cargo --version`

3. **Visual Studio Build Tools** (Windows only)
   - Download: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Or full Visual Studio with C++ components

### Optional:

4. **Git** (if cloning the repository)
   - Download: https://git-scm.com/

## Build Process

### Step 1: Preparation
```cmd
# Navigate to project folder
cd ai-terminal

# Verify all files are present
dir
# Should have: package.json, src-tauri folder
```

### Step 2: Run Build Script
Use the available script:

```cmd
# Simple build (recommended)
build-windows.bat
```

### Step 3: Wait
- First build may take 10-20 minutes
- Rust compiles many dependencies
- Subsequent builds will be faster

### Step 4: Result
After successful build, files will be in:
```
src-tauri/target/release/bundle/
├── msi/           # MSI installer (recommended)
├── nsis/          # NSIS installer
└── ...

src-tauri/target/release/
└── ai-terminal.exe # Executable file
```

## Типичные проблемы и решения

### 1. "Node.js not found"
```cmd
# Check installation
node --version
npm --version

# If not working - reinstall Node.js
```

### 2. "Rust/Cargo not found"
```cmd
# Check installation
cargo --version

# If not working:
# 1. Install Rust: https://rustup.rs/
# 2. Restart terminal
# 3. Update: rustup update
```

### 3. "Visual Studio Build Tools missing"
- Install Visual Studio Build Tools
- Or Visual Studio Community with C++ components
- Restart terminal after installation

### 4. "Rust compilation error"
```cmd
# Update Rust
rustup update

# Clean cache
cd src-tauri
cargo clean
cd ..

# Try again
```

### 5. "Windows Defender blocking build"
- Add project folder to Windows Defender exclusions
- Especially important for `src-tauri/target` folder

### 6. "Not enough memory"
- Close other applications
- Rust compilation requires a lot of memory (4GB+ recommended)

### 7. "npm errors"
```cmd
# Clean npm cache
npm cache clean --force

# Remove node_modules and reinstall
rmdir /s node_modules
npm install
```

## Additional Commands

### Manual Build (for debugging):
```cmd
# Install dependencies
npm install

# Build frontend
npm run build

# Build Tauri application
npm run tauri build

# For debug version
npm run tauri build -- --debug
```

### Run in development mode:
```cmd
npm run tauri dev
```

### Update dependencies:
```cmd
npm update
rustup update
```

## Distribution

After successful build:

1. **MSI installer** - best option for distribution
   - Users can install through standard Windows interface
   - Automatically registers in "Programs and Features"

2. **Executable file** - portable version
   - Can run without installation
   - Requires only one file

3. **NSIS installer** - alternative to MSI
   - More customizable installer
   - Smaller size

## Version updates

Update application version in:
- `package.json` - project version
- `src-tauri/tauri.conf.json` - Tauri application version

After changing version, rebuild the application.

---

**Happy building! 🚀**

If issues arise, check:
1. All dependencies are installed
2. Terminal restarted after installation
3. Antivirus is not blocking the build process