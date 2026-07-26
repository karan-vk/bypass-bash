# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0](https://github.com/karan-vk/shell-mcp/compare/shell-mcp-v0.2.0...shell-mcp-v0.3.0) (2026-07-26)


### Features

* add interactive multi-environment picker for Claude Code, AGY, Cursor, Windsurf, VS Code, Zed ([d5a18a8](https://github.com/karan-vk/shell-mcp/commit/d5a18a8ceedac81bb1fad2b455a172c53cbdb01b))
* add interative shell support ([53b394f](https://github.com/karan-vk/shell-mcp/commit/53b394fae00f9138072efa5e859f686d83ca5b8a))
* add one-command install script and one-prompt AI installer instructions ([d3aadd7](https://github.com/karan-vk/shell-mcp/commit/d3aadd7a703ed0f616d2a7b19d749317c1174645))
* initial release of high-performance Rust bad-bash-mcp server ([e121385](https://github.com/karan-vk/shell-mcp/commit/e121385ca995b8058d76dd53d63e459dcf6785a7))
* setup complete OSS structure with CI, release workflows, dual licensing, and documentation ([a25d604](https://github.com/karan-vk/shell-mcp/commit/a25d604ba5e5fa0275d2179b4d31b6637b3c6536))


### Bug Fixes

* add JSONC comment parsing support for Zed settings ([b0e4eda](https://github.com/karan-vk/shell-mcp/commit/b0e4eda9d2194c94d88c4efae5c3ddc235fbe0b8))

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
