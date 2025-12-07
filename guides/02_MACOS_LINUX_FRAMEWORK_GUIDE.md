# 🖥️ macOS & Linux Framework Guide (Vue 3 Edition)

**Version**: 2.0.0 | **Stack**: Tauri 2 + Vue 3 | **Status**: Production Ready

---

## 🎯 The "CyberFlash V2" Architecture

We have selected the **highest performance** and **most modern** stack available for 2025 development on macOS and Linux.

### 🏆 The Stack
1.  **Core**: **Tauri 2.0** (Rust) - *Smaller, faster, and more secure than Electron.*
2.  **Frontend**: **Vue 3** (Composition API) - *Reactive, performant, and developer-friendly.*
3.  **Language**: **TypeScript** (Strict) - *Type safety across the entire stack.*
4.  **State**: **Pinia** - *The official, modular state management for Vue.*
5.  **Build**: **Vite 6** - *Instant server start and lightning-fast HMR.*
6.  **AI**: **Google Gemini 3 Pro** - *Integrated via Rust backend for secure API handling.*

---

## 🏗️ Project Structure

This structure is optimized for scalability and separation of concerns.

```
CYBERFLASH_V2/
├── src-tauri/                  # 🦀 RUST BACKEND
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── commands/           # IPC Commands (exposed to frontend)
│   │   │   ├── adb.rs          # ADB wrapper
│   │   │   ├── gemini.rs       # AI integration
│   │   │   └── system.rs       # File system/OS ops
│   │   └── lib.rs              # Shared logic
│   ├── tauri.conf.json         # Tauri Config (permissions, windows)
│   └── Cargo.toml              # Rust dependencies
│
├── src/                        # ⚡ VUE 3 FRONTEND
│   ├── assets/                 # Static assets (fonts, images)
│   ├── components/             # Vue Components
│   │   ├── ui/                 # Generic UI (Buttons, Cards)
│   │   └── features/           # Domain logic (Flasher, Terminal)
│   ├── composables/            # Vue Composables (Hooks)
│   │   ├── useADB.ts
│   │   └── useGemini.ts
│   ├── stores/                 # Pinia Stores
│   │   ├── device.ts           # Device state
│   │   └── settings.ts         # App config
│   ├── views/                  # Page layouts
│   ├── App.vue                 # Root component
│   └── main.ts                 # App initialization
│
├── specs/                      # 📄 CONFIGURATION
│   ├── theme.json              # Design tokens
│   └── devices.json            # Supported device list
│
└── package.json
```

---

## 🔌 Inter-Process Communication (IPC)

We use Tauri's `invoke` command to communicate between Vue (Frontend) and Rust (Backend).

### 1. Rust Command (Backend)
```rust
// src-tauri/src/commands/adb.rs
#[tauri::command]
pub async fn get_connected_devices() -> Result<Vec<String>, String> {
    // Execute ADB command safely in Rust
    let output = std::process::Command::new("adb")
        .arg("devices")
        .output()
        .map_err(|e| e.to_string())?;
        
    // Parse and return
    Ok(parse_adb_output(output))
}
```

### 2. Vue Composable (Frontend)
```typescript
// src/composables/useADB.ts
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

export function useADB() {
  const devices = ref<string[]>([])
  const error = ref<string | null>(null)

  async function scanDevices() {
    try {
      // Call the Rust function
      devices.value = await invoke('get_connected_devices')
    } catch (e) {
      error.value = String(e)
    }
  }

  return { devices, error, scanDevices }
}
```

---

## 📦 State Management (Pinia)

We use Pinia to manage global state like connected devices and flashing progress.

```typescript
// src/stores/device.ts
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDeviceStore = defineStore('device', () => {
  const currentDevice = ref<string | null>(null)
  const isConnected = ref(false)
  const batteryLevel = ref(0)

  function setDevice(id: string) {
    currentDevice.value = id
    isConnected.value = true
  }

  return { currentDevice, isConnected, batteryLevel, setDevice }
})
```

---

## 🐧 Linux & macOS Specifics

### Linux (AppImage)
- **Dependencies**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`.
- **Distribution**: We build an **AppImage** which runs on Ubuntu, Fedora, Arch, etc. without installation.

### macOS (DMG)
- **Universal Binary**: We target both `x86_64` (Intel) and `aarch64` (Apple Silicon).
- **Permissions**: `tauri.conf.json` must configure `macOS` entitlements for USB access.

---

## 🚀 Why This Stack?

1.  **Performance**: Rust is memory-safe and blazingly fast. Vue 3's Virtual DOM is highly optimized.
2.  **Binary Size**: Tauri apps are tiny (<10MB installer) compared to Electron (>100MB).
3.  **Security**: The frontend cannot directly access the OS. All system calls go through secure Rust commands.
4.  **Developer Experience**: Vite provides instant feedback. TypeScript catches errors at compile time.
