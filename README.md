# GMFeather 🪶

[![License](https://img.shields.io/badge/license-Proprietary-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.x-blue.svg)](https://tauri.app/)
[![Vite](https://img.shields.io/badge/vite-6.x-yellow.svg)](https://vitejs.dev/)

**GMFeather** is an optimization suite designed to reduce the size of Garry's Mod addons. By applying compression techniques to assets, it helps server owners and content creators minimize download times and disk usage without compromising quality.

---

## ✨ Key Features

-   **Multi-Interface Support**: Access the optimizer via a modern, intuitive GUI or a powerful CLI for automation.
-   **Smart Compression**: Optimized handling of `VTF` (textures), `WAV/MP3` (audio), and other common GMod asset formats.
-   **Safe Processing**: Built-in validation to ensure that compressed assets remain fully functional in-game.
-   **Clean Architecture**: Engineered using Hexagonal Architecture principles for maximum reliability and maintainability.

## 🏗️ Architecture

GMFeather is built with a clear separation of concerns to ensure long-term stability:

-   **Core**: Contains the business logic and domain rules (Hexagonal Architecture).
-   **App**: A lightweight, secure desktop application powered by **Tauri** and **Vue 3**.
-   **CLI**: A robust command-line interface for batch processing and CI/CD integration.

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed on your system:

-   **Rust**: `1.95.0` or higher
-   **Node.js**: `v24.11.1` or higher
-   **Package Manager**: `npm` (included with Node.js)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/FlorentWasTaken/GMFeather.git
    cd GMFeather
    ```

2.  Install frontend dependencies:
    ```bash
    npm install
    ```

3.  Run the application in development mode:
    ```bash
    npm run dev
    ```

## 📖 Usage

> [!NOTE]
> Detailed usage instructions and examples will be available soon as the project implementation progresses.

## 🛠️ Development

To build the production version of the application:

```bash
npm run build
```

For more details on the Tauri integration, check the `src-tauri` directory.

## 📄 License

This project is proprietary. Redistribution or commercial use is strictly prohibited. See the [LICENSE](LICENSE) file for more information.
