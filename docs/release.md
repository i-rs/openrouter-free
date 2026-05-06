# Release Guide

This project uses `cargo-dist` for automated cross-platform releases.

## Release Process

### 1. Update Version

Edit `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this
```

### 2. Commit Changes

```bash
git add .
git commit -m "release: v0.2.0"
git push
```

### 3. Create Git Tag

```bash
git tag v0.2.0
git push origin v0.2.0
```

### 4. GitHub Actions Takes Over

The release workflow will automatically:

1. Build binaries for all platforms
2. Generate installers (shell, powershell, npm, homebrew)
3. Create GitHub Release
4. Publish to npm (if configured)
5. Update Homebrew formula (if configured)

## Local Testing

### Build Locally

```bash
# Build for current platform only
dist build --artifacts=local

# Build all platforms
dist build --artifacts=all

# View outputs
ls -la target/dist/
```

### Test Installers

```bash
# Shell installer
sh target/dist/*/openrouter-free-installer.sh

# PowerShell installer (Windows)
.\target\dist\**\openrouter-free-installer.ps1
```

## CI Configuration

### Required Secrets

| Secret | Description | Required For |
|--------|-------------|--------------|
| `HOMEBREW_TAP_TOKEN` | GitHub token with push access to tap repo | Homebrew publishing |

### Environment Variables

The workflow uses these from `dist-workspace.toml`:

- `cargo-dist-version`: dist tool version
- `targets`: build platforms
- `installers`: generated installer types
- `tap`: Homebrew tap repository

### Manual npm Publishing

If npm publishing fails in CI, manually publish:

```bash
npm login
npm publish ./target/dist/npm/openrouter-free-*.tgz
```

## Platform Support

| Platform | Architectures | Status |
|----------|---------------|--------|
| macOS | aarch64 (Apple Silicon), x86_64 | Tested |
| Linux | aarch64, x86_64 | Tested |
| Windows | x86_64 | Tested |

## Troubleshooting

### Build Fails

1. Check Rust toolchain: `rustup update`
2. Verify targets installed: `rustup target list`
3. Add missing targets: `rustup target add aarch64-apple-darwin`

### Homebrew Formula Not Updated

1. Verify `HOMEBREW_TAP_TOKEN` secret exists in repo settings
2. Check token has `repo` scope
3. Verify tap repository exists and is accessible

### npm Package Not Published

1. Check `npm whoami` is logged in
2. Verify npm token has publish access
3. Package location: `target/dist/npm/`
