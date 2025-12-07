# 🏗️ Master Foundation & Architecture
**Stack**: Tauri v2 + Vue 3 + Vite + TypeScript
**Status**: In Development | **Updated**: December 7, 2025

---

## 🎯 Core Architecture

### The "Performance & Security" Stack

We have selected this stack for maximum performance, security, and binary efficiency.

| Layer | Technology | Version | Why Selected |
|-------|-----------|---------|--------------|
| **Frontend** | Vue | 3.4+ | Composition API, high performance, easy reactivity. |
| **Desktop** | Tauri | 2.0 | Rust-based, secure, tiny binary size (<10MB), low memory usage. |
| **Build** | Vite | 6.0 | Lightning fast HMR, optimized builds. |
| **Language** | TypeScript | 5.6 | Strict mode, type safety, better DX. |
| **State** | Pinia | 2.1 | Official Vue store, intuitive, type-safe. |
| **Styling** | TailwindCSS | 3.4 | Utility-first, rapid UI development, easy glassmorphism. |
| **AI** | Gemini 3 Pro | API | Streaming responses, vision capabilities, function calling. |

### Why Tauri over Electron?
1.  **Security**: Rust backend provides memory safety and strict isolation.
2.  **Performance**: Native system webview (WebKit/WebView2) means no bundled Chrome.
3.  **Binary Size**: ~5-10MB vs ~100MB+ for Electron.
4.  **Resource Usage**: Significantly lower RAM and CPU footprint.

---

## 📂 Project Structure

The project follows a strict separation of concerns between the Rust backend and Vue frontend.

```
CyberFlash_V2/
├── config/                 # Configuration (Single Source of Truth)
│   ├── theme-glassmorphic.json # Design tokens & UI specs
│   ├── components.json     # Component specs
│   └── phases.json         # Flashing workflow steps
├── src-tauri/              # Rust Backend
│   ├── src/
│   │   ├── lib.rs          # Command definitions & setup
│   │   └── main.rs         # Entry point
│   └── tauri.conf.json     # App configuration
├── src/                    # Vue 3 Frontend
│   ├── components/         # Vue components
│   ├── stores/             # Pinia stores
│   ├── App.vue             # Root component
│   ├── main.ts             # Entry point
│   └── style.css           # Global styles & Tailwind directives
├── scripts/                # Shell scripts for ADB/Fastboot (Legacy/Dev)
└── docs/                   # Documentation
```

---

## 🔌 Core Systems

### 1. ADB/Fastboot Integration
- **Implementation**: Rust `std::process::Command` (via Tauri Commands).
- **Interface**: Frontend invokes Tauri commands (`invoke('adb_devices')`).
- **Safety**: Rust handles process execution and output parsing securely.
- **Streaming**: Output streamed to frontend via Tauri Events.

### 2. State Management (Pinia)
- **Stores**: 
  - `device`: Connection status, device info.
  - `flash`: Current flashing phase, progress, logs.
  - `settings`: User preferences (theme, paths).
- **Persistence**: `pinia-plugin-persistedstate` (if needed).

### 3. AI Integration (Gemini 3 Pro)
- **Mode**: Streaming API.
- **Context**: System prompt includes `config/` specs and current device state.
- **Capabilities**: Error analysis, log interpretation, guided troubleshooting.

---

## 🔒 Security & Performance

### Security
- **Isolation**: Frontend cannot directly access system calls; must go through Rust commands.
- **Permissions**: Tauri capability system restricts what API endpoints are available.
- **CSP**: Strict Content Security Policy configured in `tauri.conf.json`.

### Performance Targets
- **Startup**: < 0.5 seconds.
- **Memory**: < 100MB (Idle).
- **Animations**: 60fps (GPU accelerated via CSS/Glassmorphism).

---

## 🚀 Cross-Platform Strategy

### Linux & macOS
- **Format**: `.deb`, `.AppImage` (Linux); `.dmg`, `.app` (macOS).
- **Dependencies**: Uses system `adb`/`fastboot` or bundles them via Tauri sidecars.
