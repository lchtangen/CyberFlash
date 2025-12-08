# 🧩 Core Component Specifications (Top 20)

**Version**: 1.0.0 | **Priority**: Critical | **Target**: Cross-Platform (Linux, macOS, Windows)

This document outlines the **20 Essential Components** required to make CyberFlash V2 a production-ready, cross-platform utility. Each component consists of a **Frontend (Vue 3)** interface and a corresponding **Backend (Rust)** capability.

---

## 🔌 I. Core Connectivity & System (The Nervous System)

These components handle the raw communication between the OS, the app, and the Android device.

### 1. `DeviceConnectionHub`
*   **Frontend**: `src/components/features/DeviceConnectionHub.vue`
    *   **Role**: Global status bar indicator. Shows "Searching...", "Connected (ADB)", "Fastboot Mode".
    *   **Vision UI**: Pulsing status dots (Green/Red/Yellow).
*   **Backend**: `src-tauri/src/commands/usb_monitor.rs`
    *   **Role**: Spawns a background thread to poll USB devices every 1s. Handles hot-plugging events.

### 2. `DriverHealthCheck`
*   **Frontend**: `src/components/features/DriverHealthCheck.vue`
    *   **Role**: Modal that appears on startup if drivers are missing.
*   **Backend**: `src-tauri/src/commands/sys_driver.rs`
    *   **Role**:
        *   **Windows**: Checks Device Manager for missing drivers.
        *   **Linux**: Verifies `udev` rules for android devices.
        *   **macOS**: Checks for USB permission restrictions.

### 3. `LiveTerminal`
*   **Frontend**: `src/components/features/LiveTerminal.vue`
    *   **Role**: A high-performance, scrollable terminal window rendering colored output.
*   **Backend**: `src-tauri/src/commands/shell_stream.rs`
    *   **Role**: Creates a pseudo-terminal (PTY) to stream `stdout`/`stderr` from ADB/Fastboot directly to the frontend without buffering.

### 4. `PermissionGate`
*   **Frontend**: `src/components/features/PermissionGate.vue`
    *   **Role**: Explains why permissions are needed (e.g., "Allow USB Debugging" on phone).
*   **Backend**: `src-tauri/src/commands/os_perm.rs`
    *   **Role**: Handles `sudo` prompts (Linux) or Accessibility/Input Monitoring permissions (macOS).

### 5. `AppUpdater`
*   **Frontend**: `src/components/features/AppUpdater.vue`
    *   **Role**: "New Version Available" toast notification.
*   **Backend**: `tauri-plugin-updater`
    *   **Role**: Securely fetches and applies OTA updates for CyberFlash itself.

---

## ⚡ II. Flashing & Automation (The Muscle)

The engines that perform the actual work of modifying the device.

### 6. `FlashWizardOrchestrator`
*   **Frontend**: `src/components/features/FlashWizard.vue`
    *   **Role**: The main state machine. Manages the 8-step flow defined in `phases.json`.
*   **Backend**: `src-tauri/src/commands/automation.rs`
    *   **Role**: Executes the sequence of commands. Ensures Step 1 completes before Step 2 starts.

### 7. `UniversalRomLoader`
*   **Frontend**: `src/components/features/RomSelector.vue`
    *   **Role**: Drag-and-drop zone for `.zip`, `.img`, or `payload.bin`.
*   **Backend**: `src-tauri/src/commands/file_parser.rs`
    *   **Role**: Validates file integrity (MD5/SHA256), extracts headers to verify compatibility.

### 8. `PartitionVisualizer`
*   **Frontend**: `src/components/features/PartitionVisualizer.vue`
    *   **Role**: Graphical bar chart showing partition sizes (System, Vendor, Data).
*   **Backend**: `src-tauri/src/commands/fastboot.rs` (`get_var_all`)
    *   **Role**: Parses `fastboot getvar all` to map out the device's storage layout.

### 9. `SlotManager` (A/B)
*   **Frontend**: `src/components/features/SlotManager.vue`
    *   **Role**: Toggle switch for "Active Slot: A" vs "B".
*   **Backend**: `src-tauri/src/commands/fastboot.rs` (`set_active`)
    *   **Role**: Safely switches slots and verifies the change.

### 10. `SideloadAssistant`
*   **Frontend**: `src/components/features/SideloadAssistant.vue`
    *   **Role**: Guided UI for "Apply Update from ADB".
*   **Backend**: `src-tauri/src/commands/adb.rs` (`sideload`)
    *   **Role**: Streams the zip file over ADB with progress percentage reporting.

---

## 🛡️ III. Safety & Diagnostics (The Immune System)

Components designed to prevent bricks and data loss.

### 11. `DeviceFingerprinter`
*   **Frontend**: `src/components/features/DeviceFingerprinter.vue`
    *   **Role**: Displays "OnePlus 9 Pro (LE2123)" instead of generic "Android Device".
*   **Backend**: `src-tauri/src/commands/prop_reader.rs`
    *   **Role**: Reads `ro.product.model`, `ro.build.fingerprint` to ensure ROM matches Device.

### 12. `AIGuardian`
*   **Frontend**: `src/components/features/AIGuardian.vue`
    *   **Role**: Real-time error analysis overlay. "It looks like you forgot to unlock the bootloader."
*   **Backend**: `src-tauri/src/commands/gemini.rs`
    *   **Role**: Sends error logs to Gemini 3 Pro for instant diagnosis.

### 13. `BackupManager`
*   **Frontend**: `src/components/features/BackupManager.vue`
    *   **Role**: Checkbox list of what to backup (Apps, Photos, SMS).
*   **Backend**: `src-tauri/src/commands/adb.rs` (`backup`)
    *   **Role**: Manages `adb backup` streams to local disk.

### 14. `FactoryResetSafe`
*   **Frontend**: `src/components/features/FactoryResetSafe.vue`
    *   **Role**: "Slide to Confirm" UI to prevent accidental wipes.
*   **Backend**: `src-tauri/src/commands/wipe_logic.rs`
    *   **Role**: Executes `fastboot -w` or `adb shell recovery --wipe_data`.

### 15. `LogAnalyst`
*   **Frontend**: `src/components/features/LogAnalyst.vue`
    *   **Role**: Filterable list of logs (Errors only, Warnings only).
*   **Backend**: `src-tauri/src/commands/log_parser.rs`
    *   **Role**: Regex parsing of raw logs to categorize events.

---

## 🧠 IV. Advanced Tools (The Brain)

Specialized tools for power users.

### 16. `BootloaderUnlockFlow`
*   **Frontend**: `src/components/features/BootloaderUnlockFlow.vue`
    *   **Role**: Wizard specifically for unlocking (handling tokens for Xiaomi/Motorola).
*   **Backend**: `src-tauri/src/commands/fastboot.rs` (`oem_unlock`)
    *   **Role**: Handles the specific `fastboot flashing unlock` vs `fastboot oem unlock` commands.

### 17. `RecoveryInstaller`
*   **Frontend**: `src/components/features/RecoveryInstaller.vue`
    *   **Role**: Specialized flasher for `recovery.img` (TWRP/OrangeFox).
*   **Backend**: `src-tauri/src/commands/flash_recovery.rs`
    *   **Role**: Flashes to both `recovery_a` and `recovery_b` if needed.

### 18. `MagiskInjector`
*   **Frontend**: `src/components/features/MagiskInjector.vue`
    *   **Role**: "Root My Device" button.
*   **Backend**: `src-tauri/src/commands/image_patcher.rs`
    *   **Role**: Extracts `boot.img`, patches it with Magisk, and prepares it for flashing.

### 19. `DownloadCenter`
*   **Frontend**: `src/components/features/DownloadCenter.vue`
    *   **Role**: Browser for downloading official ROMs directly within the app.
*   **Backend**: `src-tauri/src/commands/http_client.rs`
    *   **Role**: High-speed, resumable downloads using `reqwest`.

### 20. `ThemeEngine`
*   **Frontend**: `src/components/features/ThemeEngine.vue`
    *   **Role**: Settings to toggle "Glass Intensity", "Accent Color".
*   **Backend**: `src-tauri/src/commands/config.rs`
    *   **Role**: Persists user preferences to `config/app-config.yaml`.
