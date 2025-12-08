# 🏗️ Core Architecture & Design System

**Version**: 2.1.0 | **Stack**: Tauri 2 + Vue 3 + Rust | **Updated**: December 8, 2025

---

## 🎯 The "CyberFlash V2" Architecture

We have selected the **highest performance** and **most modern** stack available for 2025 development on macOS, Windows, and Linux.

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
│   │   │   ├── fastboot.rs     # Fastboot wrapper (NEW)
│   │   │   ├── gemini.rs       # AI integration
│   │   │   └── automation.rs   # The "Auto-Flash" Engine (NEW)
│   │   └── lib.rs              # Shared logic
│   ├── tauri.conf.json         # Tauri Config (permissions, windows)
│   └── Cargo.toml              # Rust dependencies
│
├── src/                        # ⚡ VUE 3 FRONTEND
│   ├── components/             # Vue Components
│   │   ├── features/           # Domain logic
│   │   │   ├── FlashWizard.vue # The 8-Step Wizard
│   │   │   ├── Terminal.vue    # Live Log Output
│   │   │   └── DevicePanel.vue # Status & Battery
│   ├── stores/                 # Pinia Stores
│   │   ├── flash.ts            # Flashing state machine
│   │   └── device.ts           # Device state
│
└── config/                     # 📄 CONFIGURATION (Single Source of Truth)
    ├── app-config.yaml         # Global App Settings
    ├── downloads.json          # URLs for ROM, Firmware, TWRP
    ├── phases.json             # Definition of the 8 phases
    └── theme.json              # Design tokens
```

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

## ✨ Vision UI Design System

**CyberFlash V2** adopts the "Vision UI" aesthetic—a spatial, glassmorphic design language inspired by modern spatial computing. It prioritizes depth, clarity, and semantic color usage over decorative neon effects.

### Core Principles
1.  **Spatial Glass**: Heavy blur (`backdrop-blur-xl`), translucent layers (`bg-surface/30`), and subtle borders (`border-white/10`).
2.  **Mesh Gradients**: Backgrounds are dynamic and flowing, never flat black.
3.  **Semantic Clarity**: Colors are used for status (Green=Success, Red=Error, Blue=Action), not just decoration.

### 🎨 Color Palette (Tailwind Config)

| Token | Hex | Tailwind Class | Usage |
|-------|-----|----------------|-------|
| **Primary** | `#0A84FF` | `text-primary` | Main actions, active states |
| **Surface** | `#1C1C1E` | `bg-surface` | Base layer background |
| **Success** | `#30D158` | `text-success` | Online, Connected, Flashed |
| **Error** | `#FF453A` | `text-error` | Offline, Failed, Critical |
| **Warning** | `#FFD60A` | `text-warning` | Low Battery, Unlocked |
| **Text** | `#FFFFFF` | `text-white` | Primary content |

### 💎 Component Patterns

#### The Vision Glass Card
```vue
<template>
  <div 
    class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300 hover:bg-surface/40"
  >
    <div class="relative z-10 p-6">
      <slot />
    </div>
  </div>
</template>
```

#### Action Button
```vue
<template>
  <button
    class="px-6 py-3 rounded-xl font-medium transition-all duration-200 bg-primary text-white hover:bg-primary-hover shadow-lg shadow-primary/20 active:scale-95"
  >
    <slot />
  </button>
</template>
```
