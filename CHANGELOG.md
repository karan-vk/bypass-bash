# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-26

### Added
- Interactive PTY-backed shell sessions via `portable-pty`. New tools: `shell_start`, `shell_write`, `shell_read`, `shell_list`, `shell_resize`, and `shell_kill`. Sessions persist across calls, so REPLs, database clients, `ssh`, and full-screen TUIs work with state preserved. Control characters (Ctrl-C, Ctrl-D) are forwarded to the process.

### Fixed
- `install.sh` aborted with `unexpected EOF while looking for matching '"'` because of a stray quote on the final banner line.
- Release assets are now published under the `shell-mcp-*` names that `install.sh` downloads. The `v0.1.0` assets were still named `bypass-bash-*`, so every install 404'd.
- Register tools with the MCP handler. The `shell` tool was defined but never wired into the router, so `tools/list` returned an empty list and no tool could be called.

## [0.1.0] - 2026-07-23

### Added
- Initial release of `shell-mcp` (Model Context Protocol server in Rust).
- High-performance, zero-overhead stdio JSON-RPC implementation built on `rmcp 2.2.0` and `tokio`.
- Unsandboxed `shell` tool with optional `cwd`, `timeout_secs`, and custom `env` support.
- Matrix CI and cross-platform GitHub Actions workflows for multi-arch release binaries.
