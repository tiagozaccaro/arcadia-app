# Arcadia Tauri App Build Pipeline

This document describes the new, improved build pipeline for the Arcadia Tauri application.

## Overview

The new pipeline addresses several issues with the previous setup:

1. **Cross-platform support**: Works on both macOS and Linux
2. **Simplified process**: Removes unnecessary complexity
3. **Modern tooling**: Uses current best practices
4. **CI/CD integration**: Includes GitHub Actions workflow

## Build Commands

### Development

```bash
# Start development server
npm run dev

# Start Tauri development mode
npm run tauri:dev

# Start full development (frontend + Tauri)
npm start
```

### Production Builds

```bash
# Clean build artifacts
npm run clean

# Build frontend only
npm run build

# Build Tauri app (debug)
npm run tauri:build

# Full build (frontend + Tauri debug)
npm run build:full

# Release build (frontend + Tauri release)
npm run build:release
```

### Cross-Platform Support

The new pipeline supports all major platforms:

- **macOS**: Native support with .app and .dmg bundles
- **Linux**: Native support with AppImage and .deb packages
- **Windows**: Full support with .msi and .exe installers

## New Build Script

The new `scripts/build-new.sh` replaces the old build scripts with a simpler, cross-platform approach:

```bash
#!/bin/bash

# New cross-platform build script for Arcadia Tauri application
# Simplified and modernized pipeline

set -e  # Exit on error

echo "🚀 Starting Arcadia Tauri build process..."

# Determine platform
PLATFORM="unknown"
case "$(uname -s)" in
    Darwin)
        PLATFORM="macos"
        echo "🍎 Detected macOS platform"
        ;;
    Linux)
        PLATFORM="linux"
        echo "🐧 Detected Linux platform"
        ;;
    *)
        echo "❌ Unsupported platform: $(uname -s)"
        exit 1
        ;;
esac

# Clean previous builds
echo "🧹 Cleaning previous build artifacts..."
rm -rf target || true
rm -rf dist || true

# Build frontend
echo "📦 Building frontend with Vite..."
npm run build

# Build Tauri application
echo "🦀 Building Tauri application..."
npx tauri build

echo "✅ Build completed successfully!"
echo "📁 Output location: target/release/bundle/${PLATFORM}/"
```

## CI/CD Pipeline

The new GitHub Actions workflow (`.github/workflows/ci-cd.yml`) provides:

1. **Multi-platform testing**: Builds and tests on both macOS and Linux
2. **Automatic releases**: Creates GitHub releases for main branch commits
3. **Artifact management**: Uploads build artifacts for debugging

## Key Improvements

### 1. Removed Unnecessary Complexity

- **Old scripts**: Tried to handle system dependency installation
- **New approach**: Relies on Tauri's built-in dependency management

### 2. Cross-Platform Support

- **Platform detection**: Automatically detects macOS vs Linux
- **Appropriate paths**: Uses correct paths for each platform

### 3. Modern Tooling

- **pnpm**: Uses pnpm for faster, more efficient dependency management
- **Tauri CLI**: Leverages Tauri's built-in build capabilities
- **GitHub Actions**: Modern CI/CD with artifact management

### 4. Better Error Handling

- **Clear error messages**: More descriptive error output
- **Early failure**: Fails fast on critical errors
- **Cleanup**: Proper artifact cleanup between builds

## Migration Guide

To use the new pipeline:

1. **Remove old scripts** (optional):

   ```bash
   rm scripts/build.sh scripts/rust-build.sh
   ```

2. **Use new commands**:

   ```bash
   # For development
   npm start

   # For production builds
   npm run build:release
   ```

3. **CI/CD setup**:
   - The GitHub Actions workflow is ready to use
   - Push to main branch to trigger automatic releases

## Troubleshooting

### Common Issues

**Build fails on macOS**:

- Ensure Xcode command line tools are installed: `xcode-select --install`
- Verify Rust toolchain: `rustup update`

**Dependency issues**:

- Clean and reinstall: `rm -rf node_modules && pnpm install`
- Update Rust dependencies: `cargo update`

**Tauri-specific problems**:

- Check Tauri version compatibility
- Run `cargo tauri info` for environment diagnostics
