# GMFeather 🪶

[![License](https://img.shields.io/badge/license-Proprietary-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app/)
[![Vite](https://img.shields.io/badge/vite-6.x-yellow.svg)](https://vitejs.dev/)

**GMFeather** is an optimization suite designed to reduce the size of Garry's Mod addons. By applying compression techniques to assets, it helps server owners and content creators minimize download times and disk usage without compromising quality.

---

## ✨ Key Features

-   **Multi-Interface Support**: Ready for both CLI automation and a modern GUI (GUI in development).
-   **Image Compression**: High-performance optimization for `PNG` (via Oxipng) and `JPG` (via Jpeg-turbo).
-   **Smart Resizing**: Automatically downscale images to maximum dimensions while maintaining aspect ratio.
-   **Safe Processing**: Built-in backup system in local AppData and validation to ensure asset integrity.
-   **Clean Architecture**: Engineered using Hexagonal Architecture principles with strict SRP enforcement.
-   **Quality Control**: Automated pre-commit hooks and strict typing for maximum reliability.

## 🏗️ Architecture

GMFeather is built with a clear separation of concerns:

-   **Core**: The business logic and domain rules, independent of any framework.
-   **CLI**: A robust command-line interface for batch processing.
-   **App**: Desktop application powered by **Tauri** and **Vue 3** (Coming soon).

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:

-   **Rust**: `1.95.0` or higher
-   **Node.js**: `v24.11.1` or higher
-   **NPM**: Included with Node.js

### 🌍 Environment Configuration

Copy the example environment file:

```bash
cp .env.example .env
```

| Variable | Default | Description |
| :--- | :--- | :--- |
| `RUST_LOG` | `info` | Logging level (debug, info, warn, error) |

### 📦 Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/FlorentWasTaken/GMFeather.git
    cd GMFeather
    ```

2.  Install dependencies:
    ```bash
    npm install
    ```

## 📖 CLI Usage

The GMFeather CLI allows you to optimize assets in bulk. You can run it using cargo:

```bash
cargo run --bin gmfeather -- [COMMAND] [ARGS]
```

### 🖼️ Optimize Assets
Optimize images (PNG/JPG) in a file or directory recursively.

```bash
# Basic optimization
gmfeather optimize ./addons/my_addon

# With resizing and dry run
gmfeather optimize ./addons/textures --max-width 1024 --max-height 1024 --dry-run

# Disable backups (not recommended)
gmfeather optimize ./addons/textures --no-backup
```

### 🔙 Rollback
Restore files from their last backup.

```bash
# Rollback a specific file
gmfeather rollback ./addons/my_addon/texture.png

# Rollback an entire directory
gmfeather rollback ./addons/my_addon
```

### 🧹 Clean
Remove stored backups to free up space.

```bash
gmfeather clean ./addons/my_addon
```

## 🛠️ Development

To build the project:

```bash
# Build the CLI
cargo build --release --bin gmfeather

# Build the Tauri App
npm run tauri build
```

## 📄 License

This project is proprietary. Redistribution or commercial use is strictly prohibited. See the [LICENSE](LICENSE) file for more information.
