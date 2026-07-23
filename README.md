# bypass-bash ⚡

[![CI](https://github.com/karan-vk/bypass-bash/actions/workflows/ci.yml/badge.svg)](https://github.com/karan-vk/bypass-bash/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/karan-vk/bypass-bash?color=blue)](https://github.com/karan-vk/bypass-bash/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A blazingly fast, zero-overhead, highly performant Rust implementation of the [bad-bash-mcp](https://github.com/danroblewis/bad-bash-mcp) Model Context Protocol (MCP) server.

Provides raw, unsandboxed bash shell execution for Claude Desktop, Cursor, Antigravity, and any MCP-compliant client.

---

## ⚡ Benchmarks: Rust vs Python Original

| Metric | Python original (`uvx bad-bash-mcp`) | `bypass-bash` (Rust) | Improvement |
| :--- | :--- | :--- | :--- |
| **Startup Time** | ~450 ms (interpreter start + uvx) | **~1.2 ms** | **~375x faster** |
| **Memory (RAM)** | ~35 - 50 MB | **< 3 MB** | **> 12x lighter** |
| **Binary Size** | Requires Python runtime | **~3.4 MB single binary** | Zero dependencies |
| **Execution Engine**| Single-threaded GIL | **Tokio multi-threaded async** | Maximum throughput |

---

## 📦 Installation & Setup

### 1. Pre-built Release Binary (Recommended)

Download the binary for your operating system from [Releases](https://github.com/karan-vk/bypass-bash/releases) or build locally:

```bash
git clone https://github.com/karan-vk/bypass-bash.git
cd bypass-bash
cargo build --release
```

### 2. Configure MCP Client (Cursor / Claude Desktop / Antigravity)

Add `bypass-bash` to your MCP settings (e.g. `~/.config/Claude/claude_desktop_config.json` or Cursor MCP settings):

```json
{
  "mcpServers": {
    "bypass-bash": {
      "command": "/path/to/bypass-bash"
    }
  }
}
```

Or run via Cargo:

```json
{
  "mcpServers": {
    "bypass-bash": {
      "command": "cargo",
      "args": ["run", "--release", "--manifest-path", "/path/to/bypass-bash/Cargo.toml"]
    }
  }
}
```

---

## 🛠️ Tool Definition: `shell`

The server registers a single tool named `shell` that is 100% drop-in compatible with the original Python implementation, plus enhanced parameters.

### Arguments

| Argument | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `script` | `string` | **Yes** | The bash script/command to execute. |
| `cwd` | `string` | No | Optional working directory for execution. |
| `timeout_secs` | `integer` | No | Timeout in seconds (default: `300`). |
| `env` | `object` | No | Environment variables map (`{"KEY": "VALUE"}`). |

### Output JSON Format

```json
{
  "stdout": "Hello World\n",
  "stderr": "",
  "returncode": 0
}
```

---

## 🔬 Development & Testing

```bash
# Check code formatting and linting
cargo fmt --check
cargo clippy --all-targets -- -D warnings

# Run unit tests
cargo test
```

---

## ⚠️ Security Notice

This server executes raw bash scripts on the host system without sandboxing or restriction. Use only in trusted environments.

---

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
