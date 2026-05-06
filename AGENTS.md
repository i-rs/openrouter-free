# AGENTS.md - openrouter-free

## Project Overview

- **Language**: Rust
- **Package Name**: openrouter-free
- **Edition**: 2021 (verify - user noted Cargo.toml shows "2024" which may need correction)
- **Status**: Greenfield project (empty scaffolding)

## Build Commands

```bash
cargo build    # Build the project
cargo run      # Run the binary
cargo test    # Run tests
cargo check   # Type-check only
cargo fmt     # Format code
cargo clippy  # Lint
```

## Architecture

Simple binary crate. Entry point: `src/main.rs` → `fn main()`.

## Style Conventions

- Follow standard Rust idioms (see [rust-lang style guide](https://doc.rust-lang.org/1.75.0/book/ch01-03-hello-cargo.html))
- Use `cargo fmt` for formatting
- Use `cargo clippy` before committing across all crates

## Current State

Empty project. Only `src/main.rs` with Hello World exists. No dependencies defined yet.

## Verified

- Build tested: cargo build succeeds with edition 2024 (Rust 1.85+ supports edition 2024)
- If build fails on older toolchains, change `edition = "2024"` to `edition = "2021"` in Cargo.toml