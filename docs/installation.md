# Installation Guide

## Quick Install

### macOS / Linux

```bash
curl -L https://github.com/i-rs/openrouter-free/releases/latest/download/openrouter-free-installer.sh | sh
```

### Windows

```powershell
irm https://github.com/i-rs/openrouter-free/releases/latest/download/openrouter-free-installer.ps1 | iex
```

### npm

```bash
npm install -g openrouter-free
```

### Homebrew (macOS/Linux)

```bash
brew install i-rs/openrouter-free/openrouter-free
```

## Manual Download

Download pre-built binaries from the [Releases](https://github.com/i-rs/openrouter-free/releases) page.

| Platform | Architecture | Download |
|----------|--------------|----------|
| macOS | Apple Silicon | `openrouter-free-aarch64-apple-darwin.tar.gz` |
| macOS | Intel | `openrouter-free-x86_64-apple-darwin.tar.gz` |
| Linux | ARM64 | `openrouter-free-aarch64-unknown-linux-gnu.tar.gz` |
| Linux | x64 | `openrouter-free-x86_64-unknown-linux-gnu.tar.gz` |
| Windows | x64 | `openrouter-free-x86_64-pc-windows-msvc.zip` |

Extract and move to your PATH:

```bash
tar -xzf openrouter-free-*.tar.gz
sudo mv openrouter-free /usr/local/bin/
```

## Build from Source

```bash
git clone https://github.com/i-rs/openrouter-free.git
cd openrouter-free
cargo install --path .
```

Or use cargo-dist for cross-platform builds:

```bash
cargo install cargo-dist
dist build
```
