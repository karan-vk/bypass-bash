# bypass-bash ⚡

[![CI](https://github.com/karan-vk/bypass-bash/actions/workflows/ci.yml/badge.svg)](https://github.com/karan-vk/bypass-bash/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/karan-vk/bypass-bash?color=blue)](https://github.com/karan-vk/bypass-bash/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A blazingly fast, zero-overhead, highly performant Rust implementation of the [bad-bash-mcp](https://github.com/danroblewis/bad-bash-mcp) Model Context Protocol (MCP) server.

Provides raw, unsandboxed bash shell execution for Claude Desktop, Cursor, Antigravity, and any MCP-compliant client.

---

## 🚀 One-Command Install Anywhere

Run this single command in your terminal (Linux, macOS, Windows WSL):

```bash
curl -fsSL https://raw.githubusercontent.com/karan-vk/bypass-bash/master/install.sh | sh
```

---

## 🤖 One-Prompt AI Installer

Copy and paste this prompt directly into your AI Assistant (Cursor, Claude, Antigravity, ChatGPT):

> **"Please install the high-performance `bypass-bash` MCP server on my system. Execute `curl -fsSL https://raw.githubusercontent.com/karan-vk/bypass-bash/master/install.sh | sh` in terminal, locate my MCP config file (e.g. Cursor MCP settings or `claude_desktop_config.json`), add `bypass-bash` to `mcpServers` pointing to the installed binary, and verify execution."**

---

## ⚡ Benchmarks: Rust vs Python Original

| Metric | Python original (`uvx bad-bash-mcp`) | `bypass-bash` (Rust) | Improvement |
| :--- | :--- | :--- | :--- |
| **Startup Time** | ~450 ms (interpreter start + uvx) | **~1.2 ms** | **~375x faster** |
| **Memory (RAM)** | ~35 - 50 MB | **< 3 MB** | **> 12x lighter** |
| **Binary Size** | Requires Python runtime | **~3.4 MB single binary** | Zero dependencies |
| **Execution Engine**| Single-threaded GIL | **Tokio multi-threaded async** | Maximum throughput |

---

## 🛠️ MCP Configuration

Add `bypass-bash` to your MCP configuration (`claude_desktop_config.json` or Cursor MCP settings):

```json
{
  "mcpServers": {
    "bypass-bash": {
      "command": "/usr/local/bin/bypass-bash"
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
git clone https://github.com/karan-vk/bypass-bash.git
cd bypass-bash
cargo build --release

# Run unit tests
cargo test
```

---

## ⚠️ Security Notice

This server executes raw bash scripts on the host system without sandboxing or restriction. Use only in trusted environments.

---

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
