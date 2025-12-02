#!/bin/bash

# Build script for Arcadia Tauri application
# Ensures proper environment setup for pkg-config

set -e  # Exit on error

echo "🔧 Setting up build environment..."

# Set PKG_CONFIG_PATH to include all standard system paths
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/local/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig"

echo "📋 PKG_CONFIG_PATH set to: $PKG_CONFIG_PATH"

# Verify pkg-config can find the required libraries
echo "🔍 Verifying pkg-config can find required libraries..."

if ! pkg-config --exists "glib-2.0 >= 2.70"; then
    echo "❌ Error: glib-2.0 >= 2.70 not found"
    echo "Available glib-2.0 versions:"
    pkg-config --modversion glib-2.0 2>/dev/null || echo "None found"
    exit 1
fi

if ! pkg-config --exists "gobject-2.0 >= 2.70"; then
    echo "❌ Error: gobject-2.0 >= 2.70 not found"
    echo "Available gobject-2.0 versions:"
    pkg-config --modversion gobject-2.0 2>/dev/null || echo "None found"
    exit 1
fi

if ! pkg-config --exists "gio-2.0 >= 2.70"; then
    echo "❌ Error: gio-2.0 >= 2.70 not found"
    echo "Available gio-2.0 versions:"
    pkg-config --modversion gio-2.0 2>/dev/null || echo "None found"
    exit 1
fi

echo "✅ All required libraries found!"

# Show library information
echo "📊 Library versions:"
echo "  glib-2.0:    $(pkg-config --modversion glib-2.0)"
echo "  gobject-2.0: $(pkg-config --modversion gobject-2.0)"
echo "  gio-2.0:     $(pkg-config --modversion gio-2.0)"

# Build the application
echo "🚀 Building application..."

if [ "$1" = "dev" ]; then
    echo "🔄 Starting development server..."
    pnpm dev
elif [ "$1" = "build" ]; then
    echo "📦 Building for production..."
    pnpm build && pnpm tauri build
else
    echo "🤔 Unknown build target: $1"
    echo "Usage: $0 [dev|build]"
    exit 1
fi

echo "✅ Build completed successfully!"