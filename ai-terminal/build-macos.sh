#!/bin/bash

# Exit on error
set -e

echo "🚀 Building ai-terminal for macOS..."

# Build frontend
echo "📦 Building frontend..."
npm run build

# Build Tauri app
echo "🔨 Building Tauri app..."
npm run tauri build

# Create DMG
echo "📦 Creating DMG installer..."
cd src-tauri/target/release/bundle/dmg
hdiutil create -volname "ai-terminal" -srcfolder "ai-terminal.app" -ov -format UDZO "ai-terminal.dmg"

echo "✅ Build complete! DMG is available in src-tauri/target/release/bundle/dmg/ai-terminal.dmg" 