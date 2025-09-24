# Vibe Rust Template

test2

A lightweight, single-binary Rust template optimized for rapid development with
automated CI/CD.

## Features

- **Single Binary**: Everything in `src/main.rs` - no complex project structure
- **Automated Releases**: Uses conventional commits and release-please for
  semantic versioning
- **Cross-Platform Builds**: Automatically builds Linux x86_64 and macOS ARM64
  binaries
- **Optimized for Speed**: LTO and panic=abort for smaller, faster binaries
- **Minimal Dependencies**: Clean slate - add only what you need
- **Embedded Testing**: Tests live alongside code in main.rs
- **GitHub Actions Ready**: Complete CI/CD pipeline with caching

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
