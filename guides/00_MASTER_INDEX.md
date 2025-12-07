# 📚 CyberFlash V2 Master Documentation Index

**Version**: 2.0.0 | **Stack**: Tauri 2 + Vue 3 + Rust | **Updated**: December 7, 2025

---

## 🗺️ Documentation Map

This documentation is structured to guide you from setup to deployment of the **CyberFlash V2** ROM flasher.

### 🎨 Foundation & Design
| Guide | Description |
|-------|-------------|
| **[01_GLASSMORPHIC_DESIGN_SYSTEM.md](./01_GLASSMORPHIC_DESIGN_SYSTEM.md)** | The visual language. Colors, typography, and Vue 3 glass components. |
| **[02_MACOS_LINUX_FRAMEWORK_GUIDE.md](./02_MACOS_LINUX_FRAMEWORK_GUIDE.md)** | Architecture decisions. Why Tauri 2 + Vue 3 is the optimal stack. |

### 🛠️ Development
| Guide | Description |
|-------|-------------|
| **[03_DEVELOPMENT_WORKFLOW.md](./03_DEVELOPMENT_WORKFLOW.md)** | Daily routines. Git flow, testing with Vitest, and code quality. |
| **[06_AI_INTEGRATION_GEMINI.md](./06_AI_INTEGRATION_GEMINI.md)** | **NEW**: Implementing Gemini 3 Pro for smart flashing and error prediction. |
| **[07_GEMINI_PROMPTS.md](./07_GEMINI_PROMPTS.md)** | Optimized prompts for generating code and features with Gemini 3 Pro. |

### 🚀 Operations
| Guide | Description |
|-------|-------------|
| **[04_BUILD_DEPLOYMENT.md](./04_BUILD_DEPLOYMENT.md)** | Compiling for macOS (DMG) and Linux (AppImage). CI/CD pipelines. |
| **[05_TROUBLESHOOTING.md](./05_TROUBLESHOOTING.md)** | Solutions for common ADB, build, and runtime issues. |

---

## 🌟 Quick Links

- **Project Root**: `~/CYBERFLASH_V2/`
- **Source Code**: `src/` (Vue 3) & `src-tauri/` (Rust)
- **Specs**: `specs/` (JSON configurations)
- **Scripts**: `scripts/` (Helper utilities)

## ⚡ Getting Started

1. **Read** [02_MACOS_LINUX_FRAMEWORK_GUIDE.md](./02_MACOS_LINUX_FRAMEWORK_GUIDE.md) to understand the stack.
2. **Setup** your environment using the instructions in the Framework Guide.
3. **Review** [01_GLASSMORPHIC_DESIGN_SYSTEM.md](./01_GLASSMORPHIC_DESIGN_SYSTEM.md) to understand the UI patterns.
4. **Start** coding with [03_DEVELOPMENT_WORKFLOW.md](./03_DEVELOPMENT_WORKFLOW.md).
