# GitHub Copilot Instructions

This is a **single-binary Rust project** optimized for rapid development with
automated CI/CD. The project emphasizes simplicity, conventional commits for
automated releases, and cross-platform binary distribution.

## Project Architecture

- **Single binary crate**: All code lives in `src/main.rs` with embedded tests
- **Minimal dependencies**: Add dependencies incrementally as needed
- **Release automation**: Uses Google's release-please for semantic versioning
  based on conventional commits, it handle Cargo.toml versioning and changelog,
  so do not edit manually

## Key Development Workflows

### Testing

```bash
# Run all tests (includes embedded tests in main.rs)
cargo test --all-features --verbose

# Run formatting check
cargo fmt --all -- --check

# Run linting with warnings as errors
RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features --verbose

# Run benchmarks
cargo bench --verbose
```

### Release Process

- **Conventional Commits**: This project REQUIRES conventional commit messages
  (feat:, fix:, etc.) for automated releases
- **Automated releases**: Push to `main` triggers release-please workflow that:
  1. Analyzes commit history
  2. Creates/updates release PR with changelog
  3. On merge, creates GitHub release and builds cross-platform binaries
- **Binary distribution**: Automatically builds for Linux x86_64 and macOS ARM64

### CI/CD Pipeline Structure

The `.github/workflows/` follows a dependency chain:

1. **rust.yml**: fmt → clippy → test → bench (on push only)
2. **release.yml**: Triggers on main branch, calls build-artifacts.yml if
   release created
3. **build-artifacts.yml**: Reusable workflow for cross-platform binary builds

## Project-Specific Conventions

### Binary Naming

- Default binary name: `template` (from Cargo.toml)
- Release binaries: `vibe-rust-template` (overridden in build workflow)
- Customize via `bin_name` input in build-artifacts workflow

### Profile Configuration

```toml
[profile.release]
lto = true          # Link-time optimization for smaller binaries
panic = "abort"     # Smaller binaries, faster startup
```

### Test Patterns

Tests are embedded directly in `src/main.rs` using:

- Standard tests: `#[test]`
- Expected panic tests: `#[test] #[should_panic(expected = "...")]`

### Cargo Features

- Uses `default = []` feature set
- Run tests with `--all-features` to ensure compatibility

## Common Development Tasks

### Adding Dependencies

1. Add to `[dependencies]` or `[dev-dependencies]` in Cargo.toml
2. Update keywords/categories if the dependency changes project purpose
3. Consider if it affects the release profile optimization settings

### Creating a New Feature

1. Use conventional commit: `feat: add new functionality`
2. Add tests directly in the same file when possible
3. Update README.md if it changes usage patterns

### Debugging Build Issues

- Check `target/` directory for compilation artifacts
- Workflow caching uses prefixed keys (`v1-rust-{fmt,clippy,test,bench}`)
- Binary builds target specific platforms - check matrix in build-artifacts.yml

## File Structure Context

- `CHANGELOG.md`: Maintained by release-please, don't edit manually
- `CODE_OF_CONDUCT.md`: Standard contributor covenant
- `target/`: Standard Rust build artifacts, heavily cached in CI
- `.github/FUNDING.yml`: GitHub Sponsors configuration

## External Integrations

- **release-please**: Requires `MY_RELEASE_PLEASE_TOKEN` secret for triggering
  workflows
- **GitHub Actions**: Extensive caching strategy for Rust toolchain and cargo
  artifacts
- **Cross-compilation**: Handles Linux and macOS builds automatically on release

When working on this project, prioritize simplicity and maintainability. The
structure is designed for rapid iteration while maintaining production-ready
CI/CD practices.
