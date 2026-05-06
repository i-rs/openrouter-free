# Development Guide

## Prerequisites

- Rust 1.85+ (edition 2024 requires recent Rust)
- cargo
- Git

## Setup

```bash
git clone https://github.com/i-rs/openrouter-free.git
cd openrouter-free
cargo build
```

## Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
indicatif = "0.18.4"
reqwest = { version = "0.13.3", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.40", features = ["rt-multi-thread", "macros"] }
```

## Development Workflow

### Run in Debug Mode

```bash
cargo run -- -l 10
```

### Run Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Project Structure

```
openrouter-free/
├── src/
│   └── main.rs          # Main application
├── Cargo.toml           # Dependencies
├── Cargo.lock           # Locked dependencies
├── dist-workspace.toml  # cargo-dist configuration
├── .github/
│   └── workflows/
│       └── release.yml  # CI/CD pipeline
├── .trae/
│   └── skills/
│       └── openrouter-llm/
│           └── SKILL.md # Trae AI skill
└── docs/
    ├── installation.md  # Installation guide
    ├── release.md       # Release process
    └── development.md   # This file
```

## Common Tasks

### Add New CLI Option

1. Edit `src/main.rs` - add field to `Args` struct
2. Use the argument in `main()` function
3. Update help text
4. Test: `cargo run -- --help`

### Modify Model Output

1. Find `print_model_table()` or `print_json()` function
2. Edit formatting logic
3. Test with `cargo run -- -l 5 -j`

### Change API Endpoint

Update `OPENROUTER_API_URL` constant:

```rust
const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/models";
```

## cargo-dist Configuration

Edit `dist-workspace.toml`:

```toml
[dist]
cargo-dist-version = "0.31.0"
targets = ["aarch64-apple-darwin", "x86_64-apple-darwin", "aarch64-unknown-linux-gnu", "x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"]
installers = ["shell", "powershell", "npm"]
```

## Performance Optimization

### Binary Size

Release profile already optimized:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = "thin"         # Thin LTO
codegen-units = 1    # Better optimization
strip = true         # Remove debug info
```

### Build Time

For faster iterative builds:

```bash
# Use debug profile
cargo build

# Faster release builds
cargo build --release -p openrouter-free
```

## Debugging

### Network Issues

```bash
# Verbose HTTP logging
RUST_LOG=reqwest=debug cargo run -- -l 1
```

### JSON Parsing Errors

The API may return unexpected fields. Check with:

```bash
curl -s https://openrouter.ai/api/v1/models | head -c 1000
```
