#!/bin/sh
set -e

# One-command installer for bypass-bash (Model Context Protocol server)
# Usage: curl -fsSL https://raw.githubusercontent.com/karan-vk/bypass-bash/master/install.sh | sh

REPO="karan-vk/bypass-bash"

info() {
    printf "\033[1;34m[INFO]\033[0m %s\n" "$1"
}

success() {
    printf "\033[1;32m[SUCCESS]\033[0m %s\n" "$1"
}

error() {
    printf "\033[1;31m[ERROR]\033[0m %s\n" "$1" >&2
    exit 1
}

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$OS" in
    linux*)  OS="linux" ;;
    darwin*) OS="macos" ;;
    msys*|mingw*|cygwin*) OS="windows" ;;
    *) error "Unsupported operating system: $OS" ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64) ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) error "Unsupported architecture: $ARCH" ;;
esac

info "Detected platform: $OS-$ARCH"

# Determine Install Directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -d "$HOME/.local/bin" ] || mkdir -p "$HOME/.local/bin"; then
    INSTALL_DIR="$HOME/.local/bin"
else
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Fetch Latest Tag
info "Fetching latest release version..."
TAG=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || true)

if [ -z "$TAG" ]; then
    TAG="v0.1.0"
fi

info "Latest version: $TAG"

# Construct Asset URL
if [ "$OS" = "windows" ]; then
    ASSET_NAME="bypass-bash-windows-amd64.zip"
else
    ASSET_NAME="bypass-bash-$OS-$ARCH.tar.gz"
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET_NAME"
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Downloading $ASSET_NAME from $DOWNLOAD_URL..."
curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME" || error "Failed to download release asset from $DOWNLOAD_URL"

info "Extracting..."
if [ "$OS" = "windows" ]; then
    unzip -q "$TEMP_DIR/$ASSET_NAME" -d "$TEMP_DIR"
    mv "$TEMP_DIR/bypass-bash.exe" "$INSTALL_DIR/"
    BINARY_PATH="$INSTALL_DIR/bypass-bash.exe"
else
    tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"
    mv "$TEMP_DIR/bypass-bash" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/bypass-bash"
    BINARY_PATH="$INSTALL_DIR/bypass-bash"
fi

success "Installed bypass-bash to $BINARY_PATH"

echo ""
echo "=========================================================="
echo "⚡ bypass-bash installation complete!"
echo "=========================================================="
echo ""
echo "To use bypass-bash in Cursor, Claude Desktop, or Antigravity, add this to your MCP config:"
echo ""
cat << EOF
{
  "mcpServers": {
    "bypass-bash": {
      "command": "$BINARY_PATH"
    }
  }
}
EOF
echo ""
