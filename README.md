# ⚡ CyberFlash V2

> **The Next-Generation Android Flashing Utility.**
> *Powered by Tauri v2, Vue 3, and Gemini 3 Pro AI.*

![Tauri](https://img.shields.io/badge/Tauri-v2.0-orange?style=for-the-badge&logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.5-green?style=for-the-badge&logo=vue.js)
![Rust](https://img.shields.io/badge/Rust-1.75+-black?style=for-the-badge&logo=rust)
![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue?style=for-the-badge&logo=typescript)
![License](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)

**CyberFlash V2** is a high-performance, cross-platform tool designed to simplify and automate the Android custom ROM flashing process. Built with a security-first mindset, it leverages **Rust** for low-level device communication (ADB/Fastboot) and **Vue 3** for a reactive, "Vision UI" interface.

---

## 🎨 Vision UI Design System

CyberFlash V2 introduces **Vision UI**, a design language inspired by modern spatial computing interfaces.
- **Glassmorphism**: Deep depth effects with `backdrop-blur-xl` and translucent layers.
- **Mesh Gradients**: Dynamic, flowing backgrounds that breathe with the application state.
- **Apple System Colors**: Semantic usage of System Blue, Green, and Red for clear status indication.
- **Spatial Layout**: Floating panels and smooth transitions.

---

## 🚀 Key Features

### 🤖 AI Neural Core (Gemini 3 Pro)
- **Intelligent Assistant**: Integrated chat interface to guide users through complex procedures.
- **Context Awareness**: The AI understands the current device state and flashing phase.
- **Error Analysis**: Automatically analyzes logs to suggest fixes for failed commands.

### ⚡ Advanced Flashing Engine
- **8-Step Wizard**: A guided, fail-safe process defined in `config/phases.json`.
- **Universal Support**: Handles `.zip`, `.img`, and `payload.bin` formats.
- **Safety Checks**: Verifies battery levels and connection stability before critical operations.

### 📱 Device Management
- **Real-time Monitoring**: Live battery, connection type, and model detection.
- **Terminal Integration**: Built-in terminal for executing raw ADB/Fastboot commands.
- **Visual Feedback**: Instant status indicators for device connectivity.

---

## 🏗️ Architecture

CyberFlash V2 abandons the resource-heavy Electron framework in favor of **Tauri v2**.

| Layer | Technology | Description |
|-------|------------|-------------|
| **Frontend** | Vue 3 + TypeScript | Reactive UI, Composition API, Pinia State Management. |
| **Styling** | Tailwind CSS v4 | Utility-first styling with CSS variables for theming. |
| **Backend** | Rust (Tauri) | Secure system access, ADB/Fastboot wrapper, AI logic. |
| **Config** | JSON/YAML | "Single Source of Truth" configuration in `config/`. |

---

## 📂 Project Structure

```
CYBERFLASH_V2/
├── src-tauri/                  # 🦀 RUST BACKEND
│   ├── src/
│   │   ├── commands/           # IPC Commands (ADB, Fastboot, AI)
│   │   └── main.rs             # Application Entry Point
│   └── tauri.conf.json         # Security & Window Config
│
├── src/                        # ⚡ VUE 3 FRONTEND
│   ├── components/
│   │   └── features/           # Domain Components (FlashWizard, DevicePanel)
│   ├── stores/                 # Pinia State Stores (device, flash, ai)
│   ├── views/                  # Main Application Views
│   └── App.vue                 # Root Component
│
├── config/                     # ⚙️ CONFIGURATION
│   ├── phases.json             # Flashing Steps Definition
│   ├── theme.json              # Design Tokens
│   └── app-config.yaml         # Global Settings
│
└── guides/                     # 📚 DOCUMENTATION
    ├── 01_CORE_ARCHITECTURE.md
    └── 07_FEATURE_ROADMAP_2026.md
```

---

## 🛠️ Getting Started

### Prerequisites
- **Node.js** (v18 or newer)
- **Rust** (Latest Stable)
- **ADB & Fastboot** installed on your system path.
- **Linux Dependencies**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`.

### Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/cyberflash-v2.git
    cd cyberflash-v2
    ```

2.  **Install Frontend Dependencies**:
    ```bash
    npm install
    ```

3.  **Run Development Server**:
    ```bash
    npm run tauri dev
    ```

### Building for Production

To create an optimized release build:

```bash
npm run tauri build
```

---

## 📚 Documentation

Detailed documentation is available in the `guides/` directory:
- **[Core Architecture](guides/01_CORE_ARCHITECTURE.md)**: Deep dive into the stack.
- **[Feature Roadmap](guides/07_FEATURE_ROADMAP_2026.md)**: Future plans and innovations.
- **[DevOps Manual](guides/02_DEVOPS_MANUAL.md)**: CI/CD and release workflows.

---

## 🗺️ Roadmap

- [x] **Phase 1**: Core Architecture & UI Overhaul (Completed)
- [ ] **Phase 2**: Rust Backend Implementation (In Progress)
- [ ] **Phase 3**: AI Neural Core Integration
- [ ] **Phase 4**: Public Beta Release

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
