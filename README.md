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

To create a DMG package:

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Create DMG (will use the universal binary created above)
cargo bundle --release
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

### Automated Build

This project includes a GitHub Actions workflow that automatically builds packages for macOS (DMG) and Linux (DEB) when you push to the master branch.

The workflow:
1. Builds both Intel and Apple Silicon binaries for macOS
2. Creates a universal binary by combining them
3. Creates a DMG package for macOS
4. Builds a DEB package for Linux
5. Uploads these packages as artifacts

You can find the built packages in the "Actions" tab of the GitHub repository after a successful workflow run.

## License

[MIT License](LICENSE)
