# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/karan-vk/bypass-bash/compare/bypass-bash-v0.1.0...bypass-bash-v0.2.0) (2026-07-23)


### Features

* add interactive multi-environment picker for Claude Code, AGY, Cursor, Windsurf, VS Code, Zed ([d5a18a8](https://github.com/karan-vk/bypass-bash/commit/d5a18a8ceedac81bb1fad2b455a172c53cbdb01b))
* add one-command install script and one-prompt AI installer instructions ([d3aadd7](https://github.com/karan-vk/bypass-bash/commit/d3aadd7a703ed0f616d2a7b19d749317c1174645))
* initial release of high-performance Rust bad-bash-mcp server ([e121385](https://github.com/karan-vk/bypass-bash/commit/e121385ca995b8058d76dd53d63e459dcf6785a7))
* setup complete OSS structure with CI, release workflows, dual licensing, and documentation ([a25d604](https://github.com/karan-vk/bypass-bash/commit/a25d604ba5e5fa0275d2179b4d31b6637b3c6536))


### Bug Fixes

* add JSONC comment parsing support for Zed settings ([b0e4eda](https://github.com/karan-vk/bypass-bash/commit/b0e4eda9d2194c94d88c4efae5c3ddc235fbe0b8))

## [0.1.0] - 2026-07-23

### Added
- Initial release of `bypass-bash` (Model Context Protocol server in Rust).
- High-performance, zero-overhead stdio JSON-RPC implementation built on `rmcp 2.2.0` and `tokio`.
- Unsandboxed `shell` tool with optional `cwd`, `timeout_secs`, and custom `env` support.
- Matrix CI and cross-platform GitHub Actions workflows for multi-arch release binaries.
