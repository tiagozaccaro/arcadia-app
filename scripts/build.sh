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
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        echo "🪟 Detected Windows platform"
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