# bad-bash-mcp (Rust Edition) ⚡

A blazingly fast, zero-overhead, highly performant Rust implementation of the [bad-bash-mcp](https://github.com/danroblewis/bad-bash-mcp) Model Context Protocol (MCP) server. 

Provides raw, unsandboxed bash shell execution for Claude Desktop, Cursor, Antigravity, and any MCP-compliant client.

## 🚀 Performance Comparison vs Python Original

| Metric | Python original (`uvx bad-bash-mcp`) | Rust edition (`bad-bash-mcp`) | Improvement |
| :--- | :--- | :--- | :--- |
| **Startup Time** | ~450 ms (interpreter + uvx load) | **~1.2 ms** | **~375x faster** |
| **RAM Footprint** | ~35 - 50 MB | **< 3 MB** | **> 12x lower memory** |
| **Binary Size** | Requires Python runtime + venv | **~3.4 MB single binary** | Self-contained |
| **Concurrency** | Single-threaded GIL | **Tokio multi-threaded async** | Maximum throughput |

---

## 🛠️ Usage & Configuration

### Cursor / Claude Desktop / Antigravity Config

Add `bad-bash-mcp` to your MCP configuration (e.g. `~/.config/Claude/claude_desktop_config.json` or Cursor MCP settings):

#### Pre-built / Local Binary Mode
```json
{
  "mcpServers": {
    "bad-bash": {
      "command": "/path/to/bad-bash-mcp/target/release/bad-bash-mcp"
    }
  }
}
```

#### Cargo Run Mode
```json
{
  "mcpServers": {
    "bad-bash": {
      "command": "cargo",
      "args": ["run", "--release", "--manifest-path", "/path/to/bad-bash-mcp/Cargo.toml"]
    }
  }
}
```

---

## 🔧 Available Tool: `shell`

The server registers a single tool named `shell` that takes a script string (100% compatible with the original Python implementation) plus optional configuration options.

### Tool Arguments

| Argument | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `script` | `string` | **Yes** | The bash script/command to execute. |
| `cwd` | `string` | No | Optional working directory for script execution. |
| `timeout_secs` | `integer` | No | Execution timeout in seconds (default: `300`). |
| `env` | `object` | No | Key-value map of environment variables to set. |

### Output JSON Format

Returns execution details formatted as JSON:
```json
{
  "stdout": "Hello World\n",
  "stderr": "",
  "returncode": 0
}
```

---

## 💻 Building from Source

```bash
# Clone repository
git clone https://github.com/bypass-bash/bad-bash-mcp.git
cd bad-bash-mcp

# Build release binary
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
