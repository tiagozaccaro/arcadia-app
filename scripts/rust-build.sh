#!/bin/bash

# Rust build wrapper script for Arcadia Tauri application
# Ensures proper environment setup for Rust/pkg-config builds

set -e  # Exit on error

echo "🔧 Setting up Rust build environment..."

# Set PKG_CONFIG_PATH to include all standard system paths
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/local/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig"

# Also set for cross-compilation targets
export PKG_CONFIG_PATH_x86_64_unknown_linux_gnu="$PKG_CONFIG_PATH"
export PKG_CONFIG_PATH_x86_64_unknown_linux_musl="$PKG_CONFIG_PATH"

echo "📋 PKG_CONFIG_PATH set to: $PKG_CONFIG_PATH"

# Verify pkg-config can find the required libraries
echo "🔍 Verifying Rust build dependencies..."

REQUIRED_LIBS=("glib-2.0" "gobject-2.0" "gio-2.0")
MISSING_LIBS=()

for lib in "${REQUIRED_LIBS[@]}"; do
    if ! pkg-config --exists "$lib"; then
        echo "❌ Error: $lib not found"
        MISSING_LIBS+=("$lib")
    else
        version=$(pkg-config --modversion "$lib")
        echo "✅ $lib version: $version"
    fi
done

if [ ${#MISSING_LIBS[@]} -gt 0 ]; then
    echo "💥 Missing required libraries: ${MISSING_LIBS[*]}"
    echo "🔧 Attempting to install missing dependencies..."

    # Try to install missing dependencies
    if command -v apt-get &> /dev/null; then
        echo "📦 Installing missing dependencies via apt-get..."
        sudo apt-get update
        for lib in "${MISSING_LIBS[@]}"; do
            case $lib in
                "glib-2.0") sudo apt-get install -y libglib2.0-dev ;;
                "gobject-2.0") sudo apt-get install -y libgobject-2.0-dev ;;
                "gio-2.0") sudo apt-get install -y libgio-2.0-dev ;;
                *)
                    echo "🚨 Unknown library: $lib, trying generic installation..."
                    sudo apt-get install -y "$lib"-dev || echo "⚠️  Failed to install $lib-dev, trying alternative..."
                    sudo apt-get install -y lib"$lib"-dev || echo "❌ Could not install $lib"
                    ;;
            esac
        done
    elif command -v brew &> /dev/null; then
        echo "🍺 Installing missing dependencies via Homebrew..."
        brew install glib gobject-introspection
    else
        echo "🚨 Cannot install missing dependencies automatically"
        echo "Please install the following manually:"
        for lib in "${MISSING_LIBS[@]}"; do
            echo "  - $lib"
        done
        exit 1
    fi

    # Verify installation
    echo "🔍 Re-verifying dependencies after installation..."
    for lib in "${MISSING_LIBS[@]}"; do
        if pkg-config --exists "$lib"; then
            echo "✅ Successfully installed $lib"
        else
            echo "❌ Failed to install $lib"
            exit 1
        fi
    done
fi

# Set Rust-specific environment variables
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
export RUSTFLAGS="${RUSTFLAGS:--C link-arg=-Wl,-rpath,/usr/local/lib}"

echo "🚀 Starting Rust build process..."

# Run the actual build command with proper environment
if [ "$1" = "debug" ]; then
    echo "🔄 Building in debug mode..."
    cargo build
elif [ "$1" = "release" ]; then
    echo "📦 Building in release mode..."
    cargo build --release
elif [ "$1" = "tauri" ]; then
    echo "🦀 Building Tauri application..."
    cargo tauri build
else
    echo "🤔 Unknown build target: $1"
    echo "Usage: $0 [debug|release|tauri]"
    exit 1
fi

echo "✅ Rust build completed successfully!"