# CyberFlash V2

> **Modern, Glassmorphic Android Flashing Tool powered by Tauri v2 & Vue 3.**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3-green.svg)

CyberFlash V2 is a next-generation Android flashing utility designed for Linux and macOS. It combines the performance and security of **Rust** (Tauri) with a beautiful, reactive **Vue 3** interface featuring a sophisticated Glassmorphic design system.

## 📚 Documentation

- **[Master Foundation & Architecture](MASTER_FOUNDATION.md)**: The core architectural reference.
- **[AI Instructions](.github/copilot-instructions.md)**: Guidelines for AI coding agents.
- **[Design System](config/theme-glassmorphic.json)**: The single source of truth for UI/UX.

## 🛠️ Tech Stack

- **Core**: Tauri v2 (Rust)
- **Frontend**: Vue 3 + TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **State Management**: Pinia
- **AI**: Gemini 3 Pro Integration

## 🚀 Quick Start

### Prerequisites
- Node.js (v18+)
- Rust (latest stable)
- System dependencies for Tauri (Linux: `libwebkit2gtk-4.0-dev`, etc.)

### Development

1.  **Install dependencies**:
    ```bash
    npm install
    ```

2.  **Start Development Server**:
    ```bash
    npm run tauri dev
    ```
    This will start the Vite dev server and launch the Tauri application window.

### Build

To build for production:

```bash
npm run tauri build
```

## 📂 Project Structure

- `src-tauri/`: Rust backend code (system interactions, ADB/Fastboot).
- `src/`: Vue 3 frontend code (UI, State).
- `config/`: Configuration files defining the design system and workflows.

## ⚠️ Note on Artifacts

If you see `src/main` or `src/renderer` folders, please ignore them. They are artifacts from a previous Electron-based architecture and are not used in this Tauri version.
