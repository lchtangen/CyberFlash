# 🏗️ Core Architecture & Design System

**Version**: 2.0.0 | **Stack**: Tauri 2 + Vue 3 + Rust | **Updated**: December 8, 2025

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

## ✨ Glassmorphic Design System

**CyberFlash V2** adopts a refined "Glassmorphism" aesthetic. It is **not** chaotic cyberpunk; it is sophisticated, transparent, and functional.

### Core Principles
1.  **Glass Surfaces**: High transparency (`opacity-70`), heavy blur (`backdrop-blur-xl`), and subtle white borders.
2.  **Neon Accents**: Used sparingly for focus states and active indicators (Cyan & Magenta).
3.  **Floating Depth**: Elements float on different z-indexes with soft, colored shadows.

### 🎨 Color Palette (Tailwind Config)

| Token | Hex | Tailwind Class | Usage |
|-------|-----|----------------|-------|
| **Primary** | `#00F0FF` | `text-neon-cyan` | Main actions, active states, glow |
| **Secondary** | `#7000FF` | `text-neon-purple` | Gradients, secondary highlights |
| **Surface** | `#121212` | `bg-obsidian` | Deep background (behind glass) |
| **Glass** | `rgba(255,255,255,0.05)` | `bg-glass` | Card backgrounds |
| **Success** | `#00FF94` | `text-emerald-400` | Successful flash/connection |
| **Error** | `#FF0055` | `text-rose-500` | Errors, critical warnings |

### 💎 Component Patterns

#### The Base Glass Card
```vue
<template>
  <div 
    class="relative overflow-hidden rounded-2xl border border-white/10 bg-white/5 backdrop-blur-xl transition-all duration-300 hover:-translate-y-1 hover:shadow-[0_0_30px_rgba(0,240,255,0.15)]"
  >
    <div class="absolute inset-0 bg-noise opacity-[0.03] pointer-events-none"></div>
    <div class="relative z-10 p-6">
      <slot />
    </div>
  </div>
</template>
```

#### Neon Button
```vue
<template>
  <button
    class="group relative px-6 py-3 rounded-lg font-jetbrains font-bold uppercase tracking-wider transition-all duration-300 border border-cyan-400/50 text-cyan-400 hover:bg-cyan-400/10 hover:shadow-[0_0_20px_rgba(0,240,255,0.4)]"
  >
    <slot />
  </button>
</template>
```
