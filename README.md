# shell-mcp ⚡

[![CI](https://github.com/karan-vk/shell-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/karan-vk/shell-mcp/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/karan-vk/shell-mcp?color=blue)](https://github.com/karan-vk/shell-mcp/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A blazingly fast, zero-overhead, highly performant Rust implementation of the Model Context Protocol (MCP) shell server.

Provides raw, unsandboxed bash shell execution for Claude Code, Antigravity CLI / AGY, Cursor, Windsurf, VS Code, Zed, and any MCP-compliant client.

---

## ⚡ Interactive Multi-Environment Installer

Run this single command in your terminal (Linux, macOS, Windows WSL):

```bash
curl -fsSL https://raw.githubusercontent.com/karan-vk/shell-mcp/master/install.sh | sh
```

The installer will automatically detect your platform and prompt you to pick which environment(s) to configure:

```text
  [1] Claude Code / Claude Desktop  (~/.config/Claude/claude_desktop_config.json)
  [2] Antigravity CLI / AGY / Gemini (~/.gemini/antigravity-cli/mcp_config.json)
  [3] Cursor IDE                    (~/.config/Cursor/User/globalStorage/mcp.json)
  [4] Windsurf                      (~/.codeium/windsurf/mcp_config.json)
  [5] VS Code / GitHub Copilot      (~/.config/Code/User/globalStorage/mcp.json)
  [6] Zed Editor                    (~/.config/zed/settings.json)
  [7] All Environments (1 - 6)
  [8] Custom Config JSON Path
```

---

## 🤖 Universal AI Assistant Prompt

Copy and paste this prompt directly into any AI Assistant (Claude Code, Antigravity, Cursor, Windsurf, ChatGPT):

> **"Please install the high-performance `shell-mcp` MCP server on my system. Execute `curl -fsSL https://raw.githubusercontent.com/karan-vk/shell-mcp/master/install.sh | sh` in terminal, select my active environment (Claude Code, Antigravity/AGY, Cursor, Windsurf, VS Code/Copilot, or Zed), and verify `shell-mcp` is configured in `mcpServers`."**

---

## ⚡ Performance Benchmarks: Rust vs Python Original

| Metric | Python original | `shell-mcp` (Rust) | Improvement |
| :--- | :--- | :--- | :--- |
| **Startup Time** | ~450 ms (interpreter start + uvx) | **~1.2 ms** | **~375x faster** |
| **Memory (RAM)** | ~35 - 50 MB | **< 3 MB** | **> 12x lighter** |
| **Binary Size** | Requires Python runtime | **~3.4 MB single binary** | Zero dependencies |
| **Execution Engine**| Single-threaded GIL | **Tokio multi-threaded async** | Maximum throughput |

---

## 🛠️ Manual MCP Configuration

If you prefer to configure manually, add `shell-mcp` to your environment's JSON configuration:

```json
{
  "mcpServers": {
    "shell-mcp": {
      "command": "/usr/local/bin/shell-mcp"
    }
  }
}
```

---

## 🔧 Tool Definition: `shell`

The server registers a single tool named `shell` that is 100% drop-in compatible with the original Python implementation, plus enhanced parameters.

### Arguments

| Argument | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `script` | `string` | **Yes** | The bash script/command to execute. |
| `cwd` | `string` | No | Optional working directory for execution. |
| `timeout_secs` | `integer` | No | Timeout in seconds (default: `300`). |
| `env` | `object` | No | Environment variables map (`{"KEY": "VALUE"}`). |

---

## 🔬 Development & Building from Source

```bash
# Clone & build
git clone https://github.com/karan-vk/shell-mcp.git
cd shell-mcp
cargo build --release

# Run unit tests & clippy
cargo test
cargo clippy --all-targets -- -D warnings
```

---

## ⚠️ Security Notice

This server executes raw bash scripts on the host system without sandboxing or restriction. Use only in trusted environments.

---

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
