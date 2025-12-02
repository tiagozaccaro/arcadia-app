# Dockerfile for Arcadia Tauri application development
# Based on the CI workflow requirements and the build error

# Use a multi-stage build for development
FROM rust:latest as builder

# Install system dependencies required for Tauri/GTK development
# These match the dependencies installed in the CI workflow
RUN apt-get update && apt-get install -y \
    libglib2.0-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    pkg-config \
    build-essential \
    curl \
    git \
    cmake \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev \
    && rm -rf /var/lib/apt/lists/*

# Set PKG_CONFIG_PATH to include system directories where .pc files are located
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/local/lib/x86_64-linux-gnu/pkgconfig

# Verify pkg-config can find the required libraries
RUN pkg-config --libs --cflags glib-2.0 gobject-2.0 gio-2.0 && \
    echo "pkg-config successfully found all required libraries"

# Install Node.js and pnpm for frontend development
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm

# Install Tauri CLI
RUN cargo install tauri-cli

# Set working directory
WORKDIR /app

# Copy build script first
COPY scripts/build.sh /usr/local/bin/arcadia-build
RUN chmod +x /usr/local/bin/arcadia-build

# Copy all files
COPY . .

# Install frontend dependencies
RUN pnpm install

# Build the application using our build script
RUN arcadia-build build

# Final stage - use a smaller image for running
FROM debian:stable-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libglib2.0-0 \
    libgtk-3-0 \
    libwebkit2gtk-4.0-37 \
    libayatana-appindicator3-1 \
    librsvg2-2 \
    libsoup-3.0-0 \
    && rm -rf /var/lib/apt/lists/*

# Set PKG_CONFIG_PATH for runtime as well
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig:/usr/local/lib/x86_64-linux-gnu/pkgconfig

# Copy built application from builder
COPY --from=builder /app/src-tauri/target/release/arcadia-app /usr/local/bin/arcadia-app

# Set entrypoint
ENTRYPOINT ["arcadia-app"]