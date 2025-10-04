# Vibe Rust Template

A lightweight, single-binary Rust template optimized for rapid development with
automated CI/CD, with support for GitHub Copilot agent workflows.

## Features

- **Organized**: Organized Rust project structure with proper module separation
- **Automated Releases**: Uses conventional commits and release-please for
  semantic versioning
- **Cross-Platform Builds**: Automatically builds Linux x86_64 and macOS ARM64
  binaries
- **Optimized for Speed**: LTO and panic=abort for smaller, faster binaries
- **Minimal Dependencies**: Clean slate - add only what you need
- **Modular Testing**: Tests organized alongside their modules following Rust
  conventions
- **GitHub Actions Ready**: Complete CI/CD pipeline with caching
- **Copilot Agent Workflow**: Automate tasks using GitHub Copilot agent

## Quick Start

```bash
# Clone and rename
git clone https://github.com/towry/vibe-rust-template.git my-project
cd my-project

# Run tests
cargo test --all-features --verbose

# Build optimized binary
cargo build --release
```

## Project Organization

This template encourages proper Rust project structure following standard
conventions:

- **`src/main.rs`**: Entry point - keep it focused on initialization and CLI
  setup
- **`src/lib.rs`**: Create this for shared library code and modules
- **`src/modules/`**: Organize your features into logical modules
- **Module files**: Use `mod.rs` for module directories or single `.rs` files
- **Tests**: Place unit tests in the same files as the code they test,
  integration tests in `tests/` directory

### Example Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Library root (optional)
├── config/
│   ├── mod.rs       # Configuration module
│   └── settings.rs  # Settings functionality
├── handlers/
│   ├── mod.rs       # Handlers module
│   └── api.rs       # API handlers
└── utils/
    └── mod.rs       # Utility functions
```

Start simple with `main.rs`, then refactor into modules as your project grows.

## Development

- **Format**: `cargo fmt --all -- --check`
- **Lint**:
  `RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features --verbose`
- **Benchmark**: `cargo bench --verbose`

## Releases

This project requires **conventional commits** (feat:, fix:, etc.) for automated
releases. Push to `main` triggers the release workflow.

## Workflow Configuration

The build workflow requires:

- **`bin_name`**: Must match your `Cargo.toml` package name (currently
  `vibe-rust`)
- Configure via:
  - Repository Settings → Environments → Create environment → Add `BIN_NAME`
    variable
  - Or update `BIN_NAME` in `.github/workflows/build-artifacts.yml`

## Copilot Agent Workflow

This template includes a GitHub Actions workflow for integrating with GitHub
Copilot agent. The workflow allows you to automate tasks such as generating code
suggestions or managing pull requests.

## PR Asset Build for Testing

You can comment `!asset` on a pull request to trigger an artifact build for that
PR.
