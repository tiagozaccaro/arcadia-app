# Arcadia App

A Tauri application built with React, TypeScript, and Rust, featuring an extension system for game management.

## Technologies

- **Frontend**: React 19, TypeScript, Vite, TailwindCSS
- **Backend**: Rust (Tauri), SQLite
- **Build Tools**: Vite, Cargo, pnpm
- **Testing**: Vitest (frontend), Cargo test (backend)
- **Linting**: ESLint (frontend), Clippy (backend)

## Development

### Prerequisites

- Node.js (LTS)
- Rust (stable)
- pnpm
- Docker (for containerized development)

### Setup

#### Native Development (requires system dependencies)

```bash
pnpm install
pnpm tauri dev
```

#### Docker Development (recommended for Linux/macOS)

```bash
# Build and start development container
docker-compose up arcadia-dev

# Or build and run in background
docker-compose up -d arcadia-dev
```

### Scripts

- `pnpm dev` - Start development server
- `pnpm build` - Build for production
- `pnpm lint` - Lint frontend code
- `pnpm test` - Run frontend tests
- `pnpm tauri build` - Build Tauri app

### Build Script

For troubleshooting build issues, use the build script:

```bash
# Development mode
./scripts/build.sh dev

# Production build
./scripts/build.sh build
```

The build script automatically:

- Sets up proper PKG_CONFIG_PATH
- Verifies all required system libraries are available
- Provides detailed error messages if dependencies are missing

### Docker Build

```bash
# Build production image
docker-compose build

# Run production build
docker-compose run arcadia-build
```

### System Dependencies

For native development, you need to install system dependencies:

**Ubuntu/Debian:**

```bash
sudo apt-get update && sudo apt-get install -y \
    libglib2.0-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    pkg-config
```

**macOS (Homebrew):**

```bash
brew install glib pkg-config
export PKG_CONFIG_PATH=$(brew --prefix)/opt/glib/lib/pkgconfig
```

## CI/CD

This project uses GitHub Actions for continuous integration. All pull requests must pass the following checks:

- **Frontend Linting**: ESLint with zero warnings
- **Frontend Testing**: Vitest unit tests
- **Backend Linting**: Clippy with no warnings
- **Backend Testing**: Cargo unit tests
- **Cross-Platform Build**: Successful compilation on Ubuntu, macOS, and Windows

### Branch Protection

The `main` branch is protected and requires:

- All CI checks to pass
- No merge commits (squash or rebase only)
- At least one approval for PRs

To configure branch protection:

1. Go to repository Settings > Branches
2. Add rule for `main` branch
3. Enable required status checks and select all CI jobs
4. Enable required pull request reviews

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
