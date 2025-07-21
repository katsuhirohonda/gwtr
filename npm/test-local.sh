#!/bin/bash

echo "Building gwtr binary for local testing..."

# Determine current platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map to Node.js platform names
if [ "$PLATFORM" = "darwin" ]; then
    NODE_PLATFORM="darwin"
elif [ "$PLATFORM" = "linux" ]; then
    NODE_PLATFORM="linux"
else
    echo "Unsupported platform: $PLATFORM"
    exit 1
fi

# Map architecture
if [ "$ARCH" = "x86_64" ]; then
    NODE_ARCH="x64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    NODE_ARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

PLATFORM_DIR="$NODE_PLATFORM-$NODE_ARCH"

echo "Building for $PLATFORM_DIR..."

# Build the Rust binary
cd ..
cargo build --release

# Create binary directory for testing
mkdir -p npm/binaries/$PLATFORM_DIR

# Copy the binary
if [ "$NODE_PLATFORM" = "win32" ]; then
    cp target/release/gwtr.exe npm/binaries/$PLATFORM_DIR/
else
    cp target/release/gwtr npm/binaries/$PLATFORM_DIR/
    chmod +x npm/binaries/$PLATFORM_DIR/gwtr
fi

cd npm

echo "Testing npm package locally..."
echo "You can now run: npm link"
echo "Then in any directory: gwtr --help"