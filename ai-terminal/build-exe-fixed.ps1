# AI Terminal - Windows Builder (PowerShell)
# This script builds the AI Terminal application for Windows

param(
    [switch]$Clean,
    [switch]$SkipDeps,
    [switch]$Release = $true
)

# Set execution policy for this session if needed
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process -Force

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    AI Terminal - Windows Builder" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Function to check if command exists
function Test-Command {
    param($Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

# Function to get version safely
function Get-CommandVersion {
    param($Command, $VersionArg = "--version")
    try {
        $output = & $Command $VersionArg 2>$null
        return $output
    } catch {
        return "Unknown"
    }
}

# Check if we're in the correct directory
if (-not (Test-Path "package.json")) {
    Write-Host "× Error: package.json not found!" -ForegroundColor Red
    Write-Host "Please run this script from the ai-terminal directory" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Current directory: $PWD" -ForegroundColor Gray
    Write-Host "Expected files: package.json, src-tauri folder" -ForegroundColor Gray
    Write-Host ""
    Read-Host "Press Enter to exit"
    exit 1
}

if (-not (Test-Path "src-tauri")) {
    Write-Host "× Error: src-tauri folder not found!" -ForegroundColor Red
    Write-Host "This doesn't appear to be a Tauri project" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "✓ Project structure validated" -ForegroundColor Green
Write-Host ""

# Check Node.js
Write-Host "Checking Node.js installation..." -ForegroundColor Yellow
if (-not (Test-Command "node")) {
    Write-Host "× Node.js is not installed or not in PATH" -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/" -ForegroundColor Yellow
    Write-Host "Recommended version: 18.x or higher" -ForegroundColor Gray
    Read-Host "Press Enter to exit"
    exit 1
} else {
    $nodeVersion = Get-CommandVersion "node"
    Write-Host "✓ Node.js $nodeVersion found" -ForegroundColor Green
}

# Check npm
if (-not (Test-Command "npm")) {
    Write-Host "× npm is not available" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
} else {
    $npmVersion = Get-CommandVersion "npm"
    Write-Host "✓ npm $npmVersion found" -ForegroundColor Green
}

# Check Rust
Write-Host "Checking Rust installation..." -ForegroundColor Yellow
if (-not (Test-Command "cargo")) {
    Write-Host "× Rust/Cargo is not installed or not in PATH" -ForegroundColor Red
    Write-Host "Please install Rust from https://rustup.rs/" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "After installation, restart your terminal and run this script again" -ForegroundColor Gray
    Read-Host "Press Enter to exit"
    exit 1
} else {
    $cargoOutput = Get-CommandVersion "cargo"
    if ($cargoOutput -and $cargoOutput.Contains(' ')) {
        $cargoVersion = $cargoOutput.Split(' ')[1]
    } else {
        $cargoVersion = "Unknown"
    }
    Write-Host "✓ Cargo $cargoVersion found" -ForegroundColor Green
}

# Check Tauri CLI
Write-Host "Checking Tauri CLI..." -ForegroundColor Yellow
$tauriCheck = npm list @tauri-apps/cli 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "! Tauri CLI not found in project dependencies" -ForegroundColor Yellow
    Write-Host "Installing Tauri CLI..." -ForegroundColor Yellow
    npm install @tauri-apps/cli@latest --save-dev
    if ($LASTEXITCODE -ne 0) {
        Write-Host "× Failed to install Tauri CLI" -ForegroundColor Red
        Read-Host "Press Enter to exit"
        exit 1
    }
    Write-Host "✓ Tauri CLI installed" -ForegroundColor Green
} else {
    Write-Host "✓ Tauri CLI found" -ForegroundColor Green
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Starting build process..." -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Clean previous builds if requested or if Clean flag is set
if ($Clean) {
    Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
    if (Test-Path "src-tauri\target") {
        Remove-Item "src-tauri\target" -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "✓ Cleaned target directory" -ForegroundColor Green
    } else {
        Write-Host "✓ No previous builds to clean" -ForegroundColor Green
    }

    if (Test-Path "dist") {
        Remove-Item "dist" -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "✓ Cleaned dist directory" -ForegroundColor Green
    }
    Write-Host ""
}

# Install/Update dependencies
if (-not $SkipDeps) {
    Write-Host "Installing Node.js dependencies..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "× Failed to install Node.js dependencies" -ForegroundColor Red
        Write-Host ""
        Write-Host "Try running: npm cache clean --force" -ForegroundColor Gray
        Write-Host "Then run this script again" -ForegroundColor Gray
        Read-Host "Press Enter to exit"
        exit 1
    }
    Write-Host "✓ Dependencies installed successfully" -ForegroundColor Green
    Write-Host ""
} else {
    Write-Host "Skipping dependency installation (-SkipDeps flag)" -ForegroundColor Yellow
    Write-Host ""
}

# Build frontend
Write-Host "Building Angular frontend..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "× Failed to build Angular frontend" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}
Write-Host "✓ Frontend built successfully" -ForegroundColor Green
Write-Host ""

# Build Tauri app
Write-Host "Building Tauri application (this may take several minutes)..." -ForegroundColor Yellow
Write-Host "Please be patient while Rust compiles the application..." -ForegroundColor Gray
Write-Host ""

$buildCommand = if ($Release) { "tauri build" } else { "tauri build --debug" }
npm run $buildCommand

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "× Failed to build Tauri application" -ForegroundColor Red
    Write-Host ""
    Write-Host "Common solutions:" -ForegroundColor Yellow
    Write-Host "1. Make sure you have Visual Studio Build Tools installed" -ForegroundColor Gray
    Write-Host "2. Try: rustup update" -ForegroundColor Gray
    Write-Host "3. Try: cargo clean (in src-tauri folder)" -ForegroundColor Gray
    Write-Host "4. Check Windows Defender isn't blocking the build" -ForegroundColor Gray
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "    BUILD COMPLETED SUCCESSFULLY!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Find and display build artifacts
Write-Host "Build artifacts:" -ForegroundColor Yellow
Write-Host ""

$bundleDir = "src-tauri\target\release\bundle"
$releaseDir = "src-tauri\target\release"

if (Test-Path $bundleDir) {
    Write-Host "Bundle directory: $PWD\$bundleDir" -ForegroundColor Cyan
    Write-Host ""
    
    # Check for MSI installer
    $msiFiles = Get-ChildItem "$bundleDir\msi\*.msi" -ErrorAction SilentlyContinue
    if ($msiFiles) {
        Write-Host "✓ MSI Installer:" -ForegroundColor Green
        foreach ($file in $msiFiles) {
            Write-Host "  $($file.Name)" -ForegroundColor White
            $sizeMB = [math]::Round($file.Length / 1MB, 2)
            Write-Host "  Size: $sizeMB MB" -ForegroundColor Gray
        }
        Write-Host ""
    }
    
    # Check for NSIS installer
    $nsisFiles = Get-ChildItem "$bundleDir\nsis\*.exe" -ErrorAction SilentlyContinue
    if ($nsisFiles) {
        Write-Host "✓ NSIS Installer:" -ForegroundColor Green
        foreach ($file in $nsisFiles) {
            Write-Host "  $($file.Name)" -ForegroundColor White
            $sizeMB = [math]::Round($file.Length / 1MB, 2)
            Write-Host "  Size: $sizeMB MB" -ForegroundColor Gray
        }
        Write-Host ""
    }
}

# Check for executable
$exeFiles = Get-ChildItem "$releaseDir\*.exe" -ErrorAction SilentlyContinue
if ($exeFiles) {
    Write-Host "✓ Executable:" -ForegroundColor Green
    foreach ($file in $exeFiles) {
        Write-Host "  $($file.Name)" -ForegroundColor White
        $sizeMB = [math]::Round($file.Length / 1MB, 2)
        Write-Host "  Size: $sizeMB MB" -ForegroundColor Gray
    }
    Write-Host ""
}

if (-not (Test-Path $bundleDir) -and -not $exeFiles) {
    Write-Host "! No build artifacts found" -ForegroundColor Red
    Write-Host "Build may have failed or files are in unexpected location" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Test the executable by running it" -ForegroundColor Gray
Write-Host "2. To install: Run the .msi file (if available)" -ForegroundColor Gray
Write-Host "3. To distribute: Share the installer or executable" -ForegroundColor Gray
Write-Host ""

# Ask if user wants to open the build directory
$openDir = Read-Host "Open build directory? (y/n)"
if ($openDir -eq "y" -or $openDir -eq "Y") {
    if (Test-Path $bundleDir) {
        Invoke-Item "$PWD\$bundleDir"
    } elseif (Test-Path $releaseDir) {
        Invoke-Item "$PWD\$releaseDir"
    } else {
        Write-Host "Build directory not found" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "Build script completed!" -ForegroundColor Green
Write-Host ""
Read-Host "Press Enter to exit"