# AGENTS.md - openrouter-free

## Project Overview

- **Language**: Rust
- **Package Name**: openrouter-free
- **Edition**: 2024
- **Status**: Active CLI tool

## Build Commands

```bash
cargo build    # Build the project
cargo run      # Run the binary
cargo test     # Run tests
cargo check    # Type-check only
cargo fmt      # Format code
cargo clippy   # Lint
cargo build --release  # Release build
```

## Architecture

Simple binary crate. Entry point: `src/main.rs` → `fn main()`.

## Dependencies

- `clap` - CLI argument parsing
- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON serialization
- `tokio` - Async runtime
- `indicatif` - Progress bar

## Style Conventions

- Follow standard Rust idioms
- Use `cargo fmt` for formatting
- Use `cargo clippy` before committing

## Release

Uses `cargo-dist` for cross-platform builds. See `docs/release.md`.
