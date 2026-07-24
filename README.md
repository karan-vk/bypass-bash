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

The `shell` tool runs a bash script one-shot and is 100% drop-in compatible with the original Python implementation, plus enhanced parameters. For long-lived, interactive programs, see [Interactive Sessions](#️-interactive-sessions-pty) below.

### Arguments

| Argument | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `script` | `string` | **Yes** | The bash script/command to execute. |
| `cwd` | `string` | No | Optional working directory for execution. |
| `timeout_secs` | `integer` | No | Timeout in seconds (default: `300`). |
| `env` | `object` | No | Environment variables map (`{"KEY": "VALUE"}`). |

---

## 🖥️ Interactive Sessions (PTY)

Beyond the one-shot `shell` tool, the server exposes **persistent interactive sessions** backed by a real pseudo-terminal (PTY). Programs that expect a terminal — REPLs (`python3`, `node`), database clients (`psql`), `ssh`, and full-screen TUIs — behave exactly as they would in a normal terminal. Sessions stay alive across tool calls until you kill them, so state (shell variables, REPL context, working directory) persists.

### Workflow

1. **`shell_start`** — launch a session, get back a `session_id`.
2. **`shell_write`** — send input (and control keys) to the session's stdin.
3. **`shell_read`** — drain any new output produced since the last read.
4. **`shell_list` / `shell_resize` / `shell_kill`** — manage sessions.

### Tools

| Tool | Purpose | Key Arguments |
| :--- | :--- | :--- |
| `shell_start` | Start an interactive PTY session (defaults to `bash`) | `command?`, `cwd?`, `env?`, `cols?` (120), `rows?` (30) |
| `shell_write` | Send input to a session's stdin | `session_id`, `input`, `enter?` (append newline, default `true`) |
| `shell_read` | Read new output since the last read | `session_id`, `timeout_ms?` (default `2000`) |
| `shell_list` | List sessions with running state & exit code | *(none)* |
| `shell_resize` | Resize the session's terminal | `session_id`, `cols`, `rows` |
| `shell_kill` | Terminate a session and remove it | `session_id` |

### Control keys

`shell_write` sends raw bytes, so control characters work as expected — send `"\u0003"` for **Ctrl-C** (interrupt) or `"\u0004"` for **Ctrl-D** (EOF), typically with `"enter": false`.

### Example

```jsonc
// Start a Python REPL
shell_start { "command": "python3 -q -i" }        // → { "session_id": "sh-1", ... }

// Define a variable, then use it — state persists across calls
shell_write { "session_id": "sh-1", "input": "answer = 6 * 7" }
shell_write { "session_id": "sh-1", "input": "print(answer)" }
shell_read  { "session_id": "sh-1" }               // → { "output": "... 42 ...", "running": true }

// Interrupt a hung command with Ctrl-C, then exit with Ctrl-D
shell_write { "session_id": "sh-1", "input": "\u0003", "enter": false }
shell_write { "session_id": "sh-1", "input": "\u0004", "enter": false }
shell_kill  { "session_id": "sh-1" }
```

> **Note:** `shell_read` returns the raw terminal byte stream, which for interactive programs may include ANSI escape sequences and echoed keystrokes — that is authentic PTY behavior.

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
