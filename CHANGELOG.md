# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-23

### Added
- Initial release of `shell-mcp` (Model Context Protocol server in Rust).
- High-performance, zero-overhead stdio JSON-RPC implementation built on `rmcp 2.2.0` and `tokio`.
- Unsandboxed `shell` tool with optional `cwd`, `timeout_secs`, and custom `env` support.
- Matrix CI and cross-platform GitHub Actions workflows for multi-arch release binaries.
