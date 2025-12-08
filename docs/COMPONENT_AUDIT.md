# CyberFlash V2 - Component & Feature Audit

## 🏗️ Core Architecture
- **Framework**: Vue 3 + TypeScript + Vite
- **Backend**: Tauri v2 (Rust)
- **State Management**: Pinia
- **Styling**: Tailwind CSS

## 🧩 Feature Components (`src/components/features/`)
These are high-level functional blocks that compose the application views.

| Component | Description | Status |
|-----------|-------------|--------|
| `AIAssistantOverlay.vue` | Floating AI chat interface (Gemini/Claude) | ✅ Active |
| `AIChatInterface.vue` | Embedded chat view (alternative to overlay) | ⚠️ Legacy? |
| `CommandPalette.vue` | `Ctrl+K` quick action menu | ✅ Active |
| `DeviceStatusPanel.vue` | Displays connected device info (Battery, Model) | ✅ Active |
| `DownloadManager.vue` | Manages ROM/Tool downloads | ✅ Active |
| `FileDropZone.vue` | Drag & drop area for flashing files | ✅ Active |
| `LogStreamViewer.vue` | Real-time ADB logcat viewer | ✅ Active |
| `RomSelector.vue` | Interface for selecting ROM files | ✅ Active |
| `SettingsPanel.vue` | Configuration for App, ADB, and AI | ✅ Active |
| `StepWizard.vue` | Multi-step flashing process guide | ✅ Active |
| `ThemeAccentPicker.vue` | UI for picking theme colors | ✅ Active |

## 🎨 UI Components (`src/components/ui/`)
Reusable design system elements.

### Cyber/Neon System (Original)
| Component | Description |
|-----------|-------------|
| `CyberButton.vue` | Neon-glow button |
| `CyberTooltip.vue` | Tooltip with cyber styling |
| `DeviceCard.vue` | Card showing device details |
| `FileDragDropZone.vue` | UI for file drop |
| `FlashDNAProgressBar.vue` | Animated progress bar |
| `GlassCard.vue` | Glassmorphic container |
| `HexagonSpinner.vue` | Loading spinner |
| `HoloDeviceModel.vue` | 3D rotating device preview |
| `LogStreamTypewriter.vue` | Typewriter effect for logs |
| `ModalGlassDialog.vue` | Glass modal |
| `NeonInput.vue` | Glowing input field |
| `RomInfoCard.vue` | ROM details display |
| `StatusBadgePulse.vue` | Pulsing status indicator |
| `StepIndicator.vue` | Progress steps |
| `TerminalGlassOverlay.vue` | Terminal overlay |
| `ThemeToggle.vue` | Light/Dark switch |
| `ToggleSwitchNeon.vue` | Neon toggle switch |