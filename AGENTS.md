# SP1 Development Guide for Agentic Coding Agents

This guide contains essential information for agentic coding agents working in the SP1 repository.

**Related Documentation:**
- See [DEVELOPMENT.md](./DEVELOPMENT.md) for general development setup
- See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines
- See [.claude/CLAUDE.md](./.claude/CLAUDE.md) for Claude-specific instructions

**Workspace Configuration:**
- Rust Edition: 2021, MSRV: 1.79
- Version: 6.0.0-rc.1
- Large workspace with 50+ crates across core/, slop/, sp1-gpu/, examples/

## Build and Test Commands

### Core Commands
```bash
# Install SP1 toolchain (required before building/testing)
cargo run -p sp1-cli --no-default-features -- prove install-toolchain

# Build entire workspace
cargo build --workspace --all-targets

# Run all tests (release mode, excludes GPU crates)
cargo test --release --workspace \
  --exclude sp1-verifier \
  --exclude sp1-gpu-* \
  --features native-gnark,experimental,profiling

# Run tests with debug features for constraint debugging
RUST_LOG=info RUST_BACKTRACE=1 cargo test --features debug -- --nocapture

# Build GPU server from source
cargo install --locked --root "$HOME/.sp1" --path sp1-gpu/crates/server/
```

### Single Test Execution
```bash
# Run a specific test
cargo test path::to::module::test_name

# Run tests for a specific package
cargo test -p sp1-core-executor

# Run single test with specific features
cargo test test_name --features debug --release

# Debug constraint failures with full context
RUST_LOG=info RUST_BACKTRACE=1 cargo test test_name --release --features debug -- --nocapture

# Simple core testing (see DEVELOPMENT.md for context)
cd core && cargo test
```

### Code Quality Commands
```bash
# Format code (required, enforced by CI)
cargo +nightly fmt --all

# Check formatting without modifying
cargo +nightly fmt --all -- --check

# Run clippy lints with workspace configuration
cargo clippy --all-features --all-targets -- -D warnings -A incomplete-features

# Type checking
cargo check --workspace --all-targets --all-features
```

### Specialized Testing
```bash
# Test GPU crates (requires CUDA, single thread to avoid conflicts)
cargo test --release -p sp1-gpu-* -- --test-threads=1

# Test verifier separately
cargo test --release --package sp1-verifier -F ark

# End-to-end tests
cargo test --package sp1-prover --lib --release -- tests::test_e2e --exact --show-output

# Build verification circuits
make -C crates/prover build-circuits
```

## Code Style Guidelines

### Rust Edition and MSRV
- Edition: Rust 2021
- Minimum Supported Rust Version (MSRV): 1.79

### Import Organization
- Use `cargo +nightly fmt` which automatically reorders imports
- Import groups follow rustfmt's `reorder_imports = true` setting
- Use workspace dependencies when possible

### Code Formatting
- rustfmt.toml configuration: `reorder_imports = true`, `use_small_heuristics = "Max"`, `use_field_init_shorthand = true`
- All code must pass `cargo fmt --all -- --check`
- Maximum line length follows rustfmt defaults

### Type Guidelines
- Prefer explicit types over `impl Trait` in public APIs
- Use SP1-specific aliases as enforced by clippy.toml disallowed-types:
  - `SP1Field` instead of `slop_koala_bear::KoalaBear`
  - `SP1BasefoldConfig` instead of `slop_basefold::Poseidon2KoalaBear16BasefoldConfig`
  - `SP1MerkleTreeConfig` instead of `slop_koala_bear::Poseidon2KoalaBearConfig`
  - `SP1DiffusionMatrix` instead of `slop_koala_bear::DiffusionMatrixKoalaBear`
  - `SP1Perm` instead of `slop_koala_bear::KoalaPerm`
  - `SP1Pcs` instead of `slop_stacked::StackedPcsVerifier`
  - `SP1PcsProof` instead of `slop_stacked::StackedBasefoldProof`
- Common type aliases: `SP1ExtensionField`, `SP1GlobalContext`
- Leverage Rust's type system for compile-time guarantees

### Naming Conventions
- Follow Rust standard naming conventions
- Module names use snake_case
- Type names use PascalCase
- Constants use SCREAMING_SNAKE_CASE
- Prefer descriptive names over abbreviations

### Error Handling
- Use `Result<T, E>` for fallible operations
- Define custom error types with `thiserror` when appropriate
- Use `eyre` for application-level error handling
- Document error conditions with `#[doc]` attributes

### Documentation
- All public items must have documentation (`#![warn(missing_docs)]`)
- Use `///` for item documentation
- Use `//!` for module-level documentation
- Include examples in documentation when helpful

### Testing Patterns
- Unit tests in the same module as code
- Integration tests in `tests/` directory
- Use `rstest` for parameterized tests
- Use `serial_test` for tests requiring sequential execution

### Performance Considerations
- Use `--release` mode for performance-critical tests
- Enable `profiling` feature for profiling builds
- Use `rayon` for parallel computation when beneficial
- Consider memory efficiency with allocators

### Feature Flags
- `debug`: Enable constraint debugging and verbose logging
- `native-gnark`: Use native GNark FFI implementations
- `experimental`: Enable experimental features
- `profiling`: Enable performance profiling
- `bigint-rug`: Use rug library for big integers when needed

### Security and Safety
- No `unsafe` code unless absolutely necessary
- Audit unsafe blocks with comments explaining necessity
- Use defensive programming practices
- Validate external inputs thoroughly

### Architecture Patterns
- Follow the established directory structure (crates/, slop/, sp1-gpu/, examples/)
- Use workspace dependencies for internal crates
- Prefer composition over inheritance
- Separate concerns between executor, prover, and verifier components

### CI/CD Integration
- All PRs must pass CI checks including formatting, clippy, and tests
- Tests run on both x86-64 and ARM architectures
- GPU tests run separately on CUDA-enabled runners
- Use GitHub Actions workflows as reference for local testing

### Development Workflow
1. Install toolchain: `cargo run -p sp1-cli -- prove install-toolchain`
2. Make changes
3. Format code: `cargo +nightly fmt --all`
4. Run clippy: `cargo clippy --all-features --all-targets -- -D warnings`
5. Run tests: `cargo test --release --workspace --features native-gnark,experimental,profiling`
6. Check specific functionality as needed

### Common Gotchas
- GPU crates require CUDA setup and are excluded from main test runs
- Some tests require specific features to be enabled
- Use `RUST_LOG=info` for verbose output during debugging
- Memory-intensive tests may need increased stack size
- Always build in release mode for performance testing

### GPU Development
- GPU crates use `--test-threads=1` to avoid resource conflicts
- Set `CUDA_ARCHS=89` for RTX 3090/4090 GPUs
- GPU server needs separate installation step
- Use `cargo install --path sp1-gpu/crates/server/` for local development

### Code Patterns Found in the Wild
- Use `lazy_static!` for global constants and configuration
- Leverage workspace dependencies for internal crates
- Trait-based architecture with generic constraints
- Feature-gated modules: `#[cfg(feature = "gpu")] pub mod gpu_components`
- Structured error types with `thiserror::Error`

This guide should help agents navigate the SP1 codebase effectively while maintaining code quality and following established patterns.