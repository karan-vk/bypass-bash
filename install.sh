#!/bin/sh
set -e

# ==============================================================================
# bypass-bash Interactive Installer & Multi-Environment MCP Configurator
# ==============================================================================

REPO="karan-vk/bypass-bash"

info() {
    printf "\033[1;34m[INFO]\033[0m %s\n" "$1"
}

success() {
    printf "\033[1;32m[SUCCESS]\033[0m %s\n" "$1"
}

warn() {
    printf "\033[1;33m[WARN]\033[0m %s\n" "$1"
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

# Determine Installation Directory
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

# Construct Download URL
if [ "$OS" = "windows" ]; then
    ASSET_NAME="bypass-bash-windows-amd64.zip"
else
    ASSET_NAME="bypass-bash-$OS-$ARCH.tar.gz"
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET_NAME"
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Downloading $ASSET_NAME ($TAG)..."
curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME" || error "Failed to download release asset from $DOWNLOAD_URL"

info "Extracting binary..."
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

success "Successfully installed bypass-bash to: $BINARY_PATH"

# ==============================================================================
# Helper to update JSON MCP configuration files
# ==============================================================================
update_json_config() {
    FILE="$1"
    BIN_PATH="$2"

    mkdir -p "$(dirname "$FILE")"

    if command -v python3 >/dev/null 2>&1; then
        python3 - "$FILE" "$BIN_PATH" << 'EOF'
import sys, json, os

file_path = sys.argv[1]
bin_path = sys.argv[2]

data = {}
if os.path.exists(file_path) and os.path.getsize(file_path) > 0:
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            data = json.load(f)
    except Exception:
        data = {}

if "mcpServers" not in data or not isinstance(data["mcpServers"], dict):
    data["mcpServers"] = {}

data["mcpServers"]["bypass-bash"] = {
    "command": bin_path
}

with open(file_path, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2)

print(f"  \033[1;32m✓\033[0m Successfully updated: {file_path}")
EOF
    else
        warn "python3 not found; please manually add bypass-bash to $FILE"
    fi
}

# ==============================================================================
# Environment Paths Resolution
# ==============================================================================
if [ "$OS" = "macos" ]; then
    PATH_CLAUDE="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
    PATH_CURSOR="$HOME/Library/Application Support/Cursor/User/globalStorage/mcp.json"
    PATH_CODE="$HOME/Library/Application Support/Code/User/globalStorage/mcp.json"
else
    PATH_CLAUDE="$HOME/.config/Claude/claude_desktop_config.json"
    PATH_CURSOR="$HOME/.config/Cursor/User/globalStorage/mcp.json"
    PATH_CODE="$HOME/.config/Code/User/globalStorage/mcp.json"
fi

PATH_AGY="$HOME/.gemini/antigravity-cli/mcp_config.json"
PATH_WINDSURF="$HOME/.codeium/windsurf/mcp_config.json"
PATH_ZED="$HOME/.config/zed/settings.json"

# Detect TTY for interactive selection
HAS_TTY=0
if [ -t 0 ] || [ -c /dev/tty ]; then
    HAS_TTY=1
fi

echo ""
echo "================================================================================"
echo "⚡ bypass-bash Interactive Agentic Environment Configurator"
echo "================================================================================"
echo ""

if [ "$HAS_TTY" -eq 1 ]; then
    echo "Select target environments to configure with bypass-bash:"
    echo ""
    echo "  [1] Claude Code / Claude Desktop  ($PATH_CLAUDE)"
    echo "  [2] Antigravity CLI / AGY / Gemini ($PATH_AGY)"
    echo "  [3] Cursor IDE                    ($PATH_CURSOR)"
    echo "  [4] Windsurf                      ($PATH_WINDSURF)"
    echo "  [5] VS Code / GitHub Copilot      ($PATH_CODE)"
    echo "  [6] Zed Editor                    ($PATH_ZED)"
    echo "  [7] All Environments (1 - 6)"
    echo "  [8] Custom Config JSON Path"
    echo "  [0] Skip automatic configuration"
    echo ""
    printf "Enter your choice(s) separated by commas (e.g. 1,2,3 or 7): "

    if [ -c /dev/tty ]; then
        read CHOICES < /dev/tty
    else
        read CHOICES
    fi
else
    warn "Non-interactive shell detected. Auto-configuring all detected environments..."
    CHOICES="7"
fi

configure_env() {
    case "$1" in
        1) update_json_config "$PATH_CLAUDE" "$BINARY_PATH" ;;
        2) update_json_config "$PATH_AGY" "$BINARY_PATH" ;;
        3) update_json_config "$PATH_CURSOR" "$BINARY_PATH" ;;
        4) update_json_config "$PATH_WINDSURF" "$BINARY_PATH" ;;
        5) update_json_config "$PATH_CODE" "$BINARY_PATH" ;;
        6) update_json_config "$PATH_ZED" "$BINARY_PATH" ;;
        8)
            printf "Enter full path to custom MCP JSON config file: "
            if [ -c /dev/tty ]; then
                read CUSTOM_PATH < /dev/tty
            else
                read CUSTOM_PATH
            fi
            if [ -n "$CUSTOM_PATH" ]; then
                update_json_config "$CUSTOM_PATH" "$BINARY_PATH"
            fi
            ;;
    esac
}

OLD_IFS="$IFS"
IFS=","
for CHOICE in $CHOICES; do
    # Strip whitespace
    CLEAN_CHOICE=$(echo "$CHOICE" | tr -d '[:space:]')
    if [ "$CLEAN_CHOICE" = "7" ]; then
        configure_env 1
        configure_env 2
        configure_env 3
        configure_env 4
        configure_env 5
        configure_env 6
    elif [ "$CLEAN_CHOICE" = "0" ]; then
        info "Skipped automatic configuration."
    else
        configure_env "$CLEAN_CHOICE"
    fi
done
IFS="$OLD_IFS"

echo ""
success "Installation and environment configuration finished!"
echo "Binary location: $BINARY_PATH"
echo "================================================================================"
