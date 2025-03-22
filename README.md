# AI Terminal

A Rust-based terminal application with integrated AI capabilities.

## Features

- Modern UI built with Iced
- Integrated AI assistance
- Cross-platform support for macOS and Linux

## Requirements

- Rust 1.72 or newer
- Cargo (Rust's package manager)
- For Linux builds: GTK3 development libraries

## Development Setup

1. Clone the repository:
   ```
   git clone https://github.com/your-username/ai-terminal.git
   cd ai-terminal
   ```

2. Build and run the project:
   ```
   cd ai-terminal
   cargo run
   ```

## Building Packages

### Manual Build

#### macOS (Universal Binary)

To build a universal binary (works on both Intel and Apple Silicon):

```bash
# Build for Intel
cargo build --release --target x86_64-apple-darwin

# Build for Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Create universal binary
mkdir -p target/release
lipo -create \
  target/x86_64-apple-darwin/release/ai-terminal \
  target/aarch64-apple-darwin/release/ai-terminal \
  -output target/release/ai-terminal
```

To create an application bundle and DMG package:

```bash
# Install cargo-bundle and create-dmg
cargo install cargo-bundle
brew install create-dmg  # macOS only

# Create .app bundle
cargo bundle --release

# Sign the application (required to avoid "app is damaged" errors)
# For development/testing purposes
codesign --deep --force --options runtime --sign - "target/release/bundle/osx/AI Terminal.app"

# For distribution (requires Apple Developer account)
# codesign --deep --force --options runtime --sign "Developer ID Application: Your Name (TEAM_ID)" "target/release/bundle/osx/AI Terminal.app"

# Create DMG from the .app bundle
create-dmg \
  --volname "AI Terminal" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "AI Terminal.app" 200 190 \
  --app-drop-link 600 185 \
  "target/release/AI-Terminal-Installer.dmg" \
  "target/release/bundle/osx/AI Terminal.app"
```

#### Linux (Debian-based)

To build a DEB package:

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

# Install cargo-bundle
cargo install cargo-bundle

# Create DEB package
cargo bundle --release --format deb
```

For maximum Linux compatibility (to avoid GLIBC version issues):

```bash
# Set static linking for C runtime library
export RUSTFLAGS="-C target-feature=+crt-static"

# Create a config file for static linking
mkdir -p ~/.cargo
cat > ~/.cargo/config << EOF
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-feature=+crt-static"]
EOF

# Build with static linking
cargo bundle --release --format deb
```

Alternatively, you can use Docker to build with an older Ubuntu LTS version:

```bash
# Create a Dockerfile for building
cat > Dockerfile.build << EOF
FROM ubuntu:20.04

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential curl git pkg-config \
    libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf \
    ca-certificates

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Static linking for maximum compatibility
ENV RUSTFLAGS="-C target-feature=+crt-static"

WORKDIR /app
EOF

# Build with Docker
docker build -t ai-terminal-builder -f Dockerfile.build .
docker run -v $(pwd):/app ai-terminal-builder bash -c "cd ai-terminal && cargo install cargo-bundle && cargo bundle --release --format deb"
```

### Automated Build

This project includes a GitHub Actions workflow that automatically builds packages for macOS (DMG) and Linux (DEB) when you push to the master branch.

The workflow:
1. Builds both Intel and Apple Silicon binaries for macOS
2. Creates a universal binary by combining them
3. Creates a macOS .app bundle using cargo-bundle
4. Signs the application with a self-signed certificate (for development)
5. Packages the .app into a DMG installer
6. Builds a Linux DEB package using an older Ubuntu LTS version for maximum compatibility
7. Uploads these packages as artifacts

You can find the built packages in the "Actions" tab of the GitHub repository after a successful workflow run.

### Note about macOS Security

The macOS app built by the GitHub workflow is signed with a self-signed certificate, which isn't trusted by macOS by default. When running the application, you may see a warning about the app being damaged or from an unidentified developer.

To run the app regardless:
1. Right-click (or Control-click) on the app
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears

For proper distribution, you should sign the application with a valid Apple Developer ID certificate and notarize it with Apple's services.

### Note about Linux Compatibility

The Linux package is built with static linking for the C runtime library to maximize compatibility across different Linux distributions. This should help avoid errors related to GLIBC version mismatches. However, you still need to have a compatible version of GTK3 and WebKit libraries installed on the target system.

## License

[MIT License](LICENSE)
