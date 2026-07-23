# Contributing to bypass-bash

Thank you for considering contributing to `bypass-bash`!

## 🛠️ Development Setup

1. Prerequisites:
   - Rust 1.85+ (or latest stable)
   - Cargo

2. Clone repository:
   ```bash
   git clone https://github.com/karan-vk/bypass-bash.git
   cd bypass-bash
   ```

3. Build and test:
   ```bash
   cargo build
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

## 📜 Code Style & Pull Requests

- Format all code with `cargo fmt`.
- Ensure `cargo clippy --all-targets -- -D warnings` passes without any warnings.
- Keep commits concise and clear.
- All contributions are dual-licensed under MIT and Apache-2.0.
